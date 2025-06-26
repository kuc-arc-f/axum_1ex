use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post}, 
    Router,
    response::{Html, IntoResponse, Json},
};
use tower_http::services::ServeDir;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{sqlite::SqlitePool, Row};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    id: i64,
    title: String,
    content: Option<String>,
    created_at: Option<String>,
    updated_at: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CreateTodo {
    title: String,
    content: Option<String>,
}

#[derive(Debug, Deserialize)]
struct DeleteTodo {
    id: i64,
}

#[derive(Debug, Deserialize)]
struct UpdateTodo {
    id: i64,
    title: String,
    content: Option<String>,
}

#[tokio::main]
async fn main() {
    let pool = SqlitePool::connect("sqlite:todos.db").await.unwrap();
    
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            content TEXT,
            created_at TEXT,
            updated_at TEXT
        )
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();

    let app_state = Arc::new(pool);

    // `public` フォルダのパス
    let public_dir = "public/static";

    // `ServeDir` ミドルウェアを初期化
    let serve_dir = ServeDir::new(public_dir);

    let app = Router::new()
        .nest_service("/static", serve_dir)
        .route("/", get(root))
        .route("/foo", get(get_foo))
        .route("/api/list", get(list_todos))
        .route("/api/create", post(create_todo))
        .route("/api/delete", post(delete_todo))
        .route("/api/update", post(update_todo))        
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Html<&'static str> {
    let s1 = "<!doctype html>
<html>
  <head>
    <meta charset='UTF-8' />
    <meta name='viewport' content='width=device-width, initial-scale=1.0' />
    <title>welcome</title>
    <script src='https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4'></script>
  </head>
  <body>
    <div id='app'></div>
    <script type='module' src='/static/client.js'></script>
  <body>
</html>
";
  Html(&s1)
}

async fn get_foo() -> String {
    String::from("foo\n")
}


async fn list_todos(State(pool): State<Arc<SqlitePool>>) -> Result<Json<Vec<Todo>>, StatusCode> {
    let rows = sqlx::query("SELECT id, title, content, created_at, updated_at FROM todos")
        .fetch_all(pool.as_ref())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let todos: Vec<Todo> = rows
        .into_iter()
        .map(|row| Todo {
            id: row.get("id"),
            title: row.get("title"),
            content: row.get("content"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
        .collect();

    Ok(Json(todos))
}


async fn create_todo(
    State(pool): State<Arc<SqlitePool>>,
    Json(payload): Json<CreateTodo>,
) -> Result<Json<Todo>, StatusCode> {
    let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    
    let result = sqlx::query(
        "INSERT INTO todos (title, content, created_at, updated_at) VALUES (?, ?, ?, ?)"
    )
    .bind(&payload.title)
    .bind(&payload.content)
    .bind(&now)
    .bind(&now)
    .execute(pool.as_ref())
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let todo_id = result.last_insert_rowid();

    let todo = Todo {
        id: todo_id,
        title: payload.title,
        content: payload.content,
        created_at: Some(now.clone()),
        updated_at: Some(now),
    };

    Ok(Json(todo))
}

async fn delete_todo(
    State(pool): State<Arc<SqlitePool>>,
    Json(payload): Json<DeleteTodo>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let result = sqlx::query("DELETE FROM todos WHERE id = ?")
        .bind(payload.id)
        .execute(pool.as_ref())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(Json(json!({
        "message": "Todo deleted successfully",
        "id": payload.id
    })))
}

async fn update_todo(
    State(pool): State<Arc<SqlitePool>>,
    Json(payload): Json<UpdateTodo>,
) -> Result<Json<Todo>, StatusCode> {
    let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    
    let result = sqlx::query(
        "UPDATE todos SET title = ?, content = ?, updated_at = ? WHERE id = ?"
    )
    .bind(&payload.title)
    .bind(&payload.content)
    .bind(&now)
    .bind(payload.id)
    .execute(pool.as_ref())
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    let row = sqlx::query("SELECT id, title, content, created_at, updated_at FROM todos WHERE id = ?")
        .bind(payload.id)
        .fetch_one(pool.as_ref())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let todo = Todo {
        id: row.get("id"),
        title: row.get("title"),
        content: row.get("content"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    };

    Ok(Json(todo))
}