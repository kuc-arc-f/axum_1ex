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
mod models;
mod handlers;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: i32,
    name: String,
    email: String,
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

    let state = models::AppState { pool };

    let app = Router::new()
        .nest_service("/static", serve_dir)
        .route("/api/list", get(handlers::get_todos))
        .route("/api/create", post(handlers::create_todo))
        .route("/api/delete", post(handlers::delete_todo))
        .route("/api/update", post(handlers::update_todo))
        .route("/edit_download", get(handlers::edit_download_excel)) 
        .route("/download", get(handlers::dowload_handler)) 
        .route("/", get(root))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("Server running on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
