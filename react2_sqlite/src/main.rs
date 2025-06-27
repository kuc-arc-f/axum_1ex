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

mod mod_sqlite;

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
        .route("/api/list", get(mod_sqlite::list_todos))
        .route("/api/create", post(mod_sqlite::create_todo))
        .route("/api/delete", post(mod_sqlite::delete_todo))
        .route("/api/update", post(mod_sqlite::update_todo))  
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

