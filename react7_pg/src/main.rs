use axum::{
    extract::State,
    http::StatusCode,
    response::{Json, Html, IntoResponse},
    routing::{get, post},
    Router,
};

use chrono::{Utc, NaiveDate, DateTime, NaiveDateTime};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{PgPool, Row};
use sqlx::postgres::PgPoolOptions;
use sqlx::FromRow;
use std::sync::Arc;
use tokio;
use tower_http::services::ServeDir;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: i32,
    name: String,
    email: String,
}
#[derive(Clone)]
struct AppState {
    pool: PgPool,
}

#[derive(Debug, Serialize , Deserialize, FromRow)]
pub struct Todo {
    pub id: i32,
    pub title: Option<String>,
    pub content: Option<String>,
    pub completed: bool,
    pub content_type: Option<String>,
    pub is_public: bool,
    pub food_orange: bool,
    pub food_apple: bool,
    pub food_banana: bool,
    pub food_melon: bool,
    pub food_grape: bool,
    pub pub_date1: Option<NaiveDate>,
    pub pub_date2: Option<NaiveDate>,
    pub pub_date3: Option<NaiveDate>,
    pub pub_date4: Option<NaiveDate>,
    pub pub_date5: Option<NaiveDate>,
    pub pub_date6: Option<NaiveDate>,
    pub qty1: Option<String>,
    pub qty2: Option<String>,
    pub qty3: Option<String>,
    pub qty4: Option<String>,
    pub qty5: Option<String>,
    pub qty6: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}


#[derive(Debug, Deserialize)]
pub struct CreateTodo {
    title: String,
    content: String,
    completed: Option<bool>,
    content_type: Option<String>,
    is_public: Option<bool>,
    food_orange: Option<bool>,
    food_apple: Option<bool>,
    food_banana: Option<bool>,
    food_melon: Option<bool>,
    food_grape: Option<bool>,
    pub_date1: Option<NaiveDate>,
    pub_date2: Option<NaiveDate>,
    pub_date3: Option<NaiveDate>,
    pub_date4: Option<NaiveDate>,
    pub_date5: Option<NaiveDate>,
    pub_date6: Option<NaiveDate>,
    qty1: Option<String>,
    qty2: Option<String>,
    qty3: Option<String>,
    qty4: Option<String>,
    qty5: Option<String>,
    qty6: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DeleteTodo {
    id: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UpdateTodo {
    id: i32,
    title: Option<String>,
    content: Option<String>,
    completed: Option<bool>,
    content_type: Option<String>,
    is_public: Option<bool>,
    food_orange: Option<bool>,
    food_apple: Option<bool>,
    food_banana: Option<bool>,
    food_melon: Option<bool>,
    food_grape: Option<bool>,
    pub_date1: Option<NaiveDate>,
    pub_date2: Option<NaiveDate>,
    pub_date3: Option<NaiveDate>,
    pub_date4: Option<NaiveDate>,
    pub_date5: Option<NaiveDate>,
    pub_date6: Option<NaiveDate>,
    qty1: Option<String>,
    qty2: Option<String>,
    qty3: Option<String>,
    qty4: Option<String>,
    qty5: Option<String>,
    qty6: Option<String>,
}

#[derive(Debug, Serialize , Deserialize, FromRow)]
pub struct TodoResponse {
    pub id: i32,
    pub title: String,
    pub content: Option<String>,
    pub completed: Option<bool>,
    pub content_type: Option<String>,
    pub is_public: bool,
    pub food_orange: bool,
    pub food_apple: bool,
    pub food_banana: bool,
    pub food_melon: bool,
    pub food_grape: bool,
    pub_date1: Option<NaiveDate>,
    pub pub_date2: Option<NaiveDate>,
    pub pub_date3: Option<NaiveDate>,
    pub pub_date4: Option<NaiveDate>,
    pub pub_date5: Option<NaiveDate>,
    pub pub_date6: Option<NaiveDate>,
    pub qty1: Option<String>,
    pub qty2: Option<String>,
    pub qty3: Option<String>,
    pub qty4: Option<String>,
    pub qty5: Option<String>,
    pub qty6: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime> 
}

pub async fn get_todos(State(state): State<AppState>) -> Result<String, StatusCode> {
    println!("# /api/list");
    //let todo_items: Vec<Todo> = sqlx::query_as::<_, Todo>("SELECT * FROM todos ORDER BY id DESC")

    let todSql = "SELECT id, title, content , completed , content_type ,
    is_public, food_orange, food_apple, food_banana, food_melon, food_grape, 
    pub_date1, pub_date2, pub_date3, pub_date4, pub_date5, pub_date6,
    qty1, qty2, qty3, qty4, qty5, qty6,
    created_at, updated_at
    FROM todos
    ";
    println!("# todSql={}", &todSql);

    let todo_items: Vec<TodoResponse> = sqlx::query_as::<_, TodoResponse>(&todSql)
        .fetch_all(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    println!("Mapped structs > {:?}", todo_items);

    let out = serde_json::to_string(&todo_items).unwrap();    
    Ok(out.to_string())
}


pub async fn create_todo(
    State(state): State<AppState>,
    Json(payload): Json<CreateTodo>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    println!("# /api/create");
    println!("{:?}", &payload);

    let result = sqlx::query(
        "INSERT INTO todos (title, content, completed, content_type, is_public, food_orange, food_apple, food_banana, food_melon, food_grape, pub_date1, pub_date2, pub_date3, pub_date4, pub_date5, pub_date6, qty1, qty2, qty3, qty4, qty5, qty6) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22)"
    )
    .bind(&payload.title)
    .bind(&payload.content)
    .bind(payload.completed.unwrap_or(false))
    .bind(&payload.content_type)
    .bind(payload.is_public.unwrap_or(false))
    .bind(payload.food_orange.unwrap_or(false))
    .bind(payload.food_apple.unwrap_or(false))
    .bind(payload.food_banana.unwrap_or(false))
    .bind(payload.food_melon.unwrap_or(false))
    .bind(payload.food_grape.unwrap_or(false))
    .bind(&payload.pub_date1)
    .bind(&payload.pub_date2)
    .bind(&payload.pub_date3)
    .bind(&payload.pub_date4)
    .bind(&payload.pub_date5)
    .bind(&payload.pub_date6)
    .bind(&payload.qty1)
    .bind(&payload.qty2)
    .bind(&payload.qty3)
    .bind(&payload.qty4)
    .bind(&payload.qty5)
    .bind(&payload.qty6)
    .execute(&state.pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to create todo: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(json!({
        "ret": 200,
        "message": "Todo created successfully",
    })))
}

pub async fn delete_todo(
    State(state): State<AppState>,
    Json(payload): Json<DeleteTodo>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    println!("# /api/delete");
    println!("{:?}", payload);

    sqlx::query("DELETE FROM todos WHERE id = $1")
        .bind(&payload.id)
        .execute(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({
        "message": "Todo deleted successfully",
        "id": payload.id
    })))
}


pub async fn update_todo(
    State(state): State<AppState>,
    Json(payload): Json<UpdateTodo>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    println!("# /api/update");
    println!("{:?}", &payload);

    let mut tx = state.pool.begin().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let todSql = "SELECT id, title, content , completed , content_type ,
    is_public, food_orange, food_apple, food_banana, food_melon, food_grape, 
    pub_date1, pub_date2, pub_date3, pub_date4, pub_date5, pub_date6,
    qty1, qty2, qty3, qty4, qty5, qty6,
    created_at, updated_at
    FROM todos WHERE id = $1 LIMIT 1
    ";
    let select_item: Vec<TodoResponse> = sqlx::query_as::<_, TodoResponse>(&todSql)
        .bind(&payload.id)
        .fetch_all(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    /*
    let result = sqlx::query(
        "SELECT * FROM todos WHERE id = $1 LIMIT 1"        
    )
    .bind(&payload.id)
    .execute(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let count = select_item.len();    
    */
    println!("# todo.select-1");
    println!("{:?}", &select_item);
    
    let mut todo: UpdateTodo = payload.clone();
    println!("# todo.select-2");
    println!("{:?}", &todo);

    if let Some(title) = payload.title {
        todo.title = Some(title);
    }
    if let Some(content) = payload.content {
        todo.content = Some(content);
    }
    if let Some(completed) = payload.completed {
        todo.completed = Some(completed);
    }
    if let Some(content_type) = payload.content_type {
        todo.content_type = Some(content_type);
    }
    if let Some(is_public) = payload.is_public {
        todo.is_public = Some(is_public);
    }
    if let Some(food_orange) = payload.food_orange {
        todo.food_orange = Some(food_orange);
    }
    if let Some(food_apple) = payload.food_apple {
        todo.food_apple = Some(food_apple);
    }
    if let Some(food_banana) = payload.food_banana {
        todo.food_banana = Some(food_banana);
    }
    if let Some(food_melon) = payload.food_melon {
        todo.food_melon = Some(food_melon);
    }
    if let Some(food_grape) = payload.food_grape {
        todo.food_grape = Some(food_grape);
    }
    if let Some(pub_date1) = payload.pub_date1 {
        todo.pub_date1 = Some(pub_date1);
    }
    if let Some(pub_date2) = payload.pub_date2 {
        todo.pub_date2 = Some(pub_date2);
    }
    if let Some(pub_date3) = payload.pub_date3 {
        todo.pub_date3 = Some(pub_date3);
    }
    if let Some(pub_date4) = payload.pub_date4 {
        todo.pub_date4 = Some(pub_date4);
    }
    if let Some(pub_date5) = payload.pub_date5 {
        todo.pub_date5 = Some(pub_date5);
    }
    if let Some(pub_date6) = payload.pub_date6 {
        todo.pub_date6 = Some(pub_date6);
    }
    if let Some(qty1) = payload.qty1 {
        todo.qty1 = Some(qty1);
    }
    if let Some(qty2) = payload.qty2 {
        todo.qty2 = Some(qty2);
    }
    if let Some(qty3) = payload.qty3 {
        todo.qty3 = Some(qty3);
    }
    if let Some(qty4) = payload.qty4 {
        todo.qty4 = Some(qty4);
    }
    if let Some(qty5) = payload.qty5 {
        todo.qty5 = Some(qty5);
    }
    if let Some(qty6) = payload.qty6 {
        todo.qty6 = Some(qty6);
    }

    sqlx::query("UPDATE todos SET title = $1, content = $2, completed = $3, content_type = $4, is_public = $5, food_orange = $6, food_apple = $7, food_banana = $8, food_melon = $9, food_grape = $10, pub_date1 = $11, pub_date2 = $12, pub_date3 = $13, pub_date4 = $14, pub_date5 = $15, pub_date6 = $16, qty1 = $17, qty2 = $18, qty3 = $19, qty4 = $20, qty5 = $21, qty6 = $22, updated_at = now() WHERE id = $23")
        .bind(&todo.title)
        .bind(&todo.content)
        .bind(todo.completed)
        .bind(&todo.content_type)
        .bind(todo.is_public)
        .bind(todo.food_orange)
        .bind(todo.food_apple)
        .bind(todo.food_banana)
        .bind(todo.food_melon)
        .bind(todo.food_grape)
        .bind(todo.pub_date1)
        .bind(todo.pub_date2)
        .bind(todo.pub_date3)
        .bind(todo.pub_date4)
        .bind(todo.pub_date5)
        .bind(todo.pub_date6)
        .bind(&todo.qty1)
        .bind(&todo.qty2)
        .bind(&todo.qty3)
        .bind(&todo.qty4)
        .bind(&todo.qty5)
        .bind(&todo.qty6)
        .bind(todo.id)
        .execute(&mut *tx)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    tx.commit().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({
        "message": "Todo updated successfully",
        "id": payload.id
    })))
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

#[tokio::main]
async fn main() {
    // `public` フォルダのパス
    let public_dir = "public/static";

    // `ServeDir` ミドルウェアを初期化
    let serve_dir = ServeDir::new(public_dir);

    let database_url = "postgresql://postgresql:admin@localhost/postgresql".to_string();

    let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect("postgres://postgres:admin@localhost/postgres")
    .await.expect("Failed to create pool");

    let state = AppState { pool };

    let app = Router::new()
        .nest_service("/static", serve_dir)
        .route("/api/list", get(get_todos))
        .route("/api/create", post(create_todo))
        .route("/api/delete", post(delete_todo))
        .route("/api/update", post(update_todo))  
        .route("/", get(root))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("Server running on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
