use axum::{
    extract::State,
    http::StatusCode,
    response::{Json, Html, IntoResponse},
    routing::{get, post},
    Router,
};

use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;
use sqlx::{PgPool, Row};
use sqlx::postgres::PgPoolOptions;
use sqlx::FromRow;
use std::sync::Arc;
use tokio;
use tower_http::services::ServeDir;
use reqwest::Error;

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
}


#[derive(Debug, Deserialize)]
pub struct CreateTodo {
    title: String,
    content: String,
}

#[derive(Debug, Deserialize)]
pub struct DeleteTodo {
    id: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodo {
    id: i32,
    title: String,
    content: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AdkInitRequest {
    appName: String,
    userId: String,
    sessionId: String,
    messages: String,
}
#[derive(Debug, Deserialize)]
pub struct AdkRunRequest {
    text: String,
}


const ADK_API_URL: &str ="http://localhost:8000";
/**
*
* @param
*
* @return
*/
pub async fn adk_init_handler(
  State(state): State<AppState>,
  Json(payload): Json<AdkInitRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {

  let client = reqwest::Client::new();
  let path_s1 = String::from("/apps/");
  let path_s2 =  &payload.appName;
  let path_s3 = "/users/";
  let path_s4= &payload.userId;
  let path_s5 = "/sessions/";
  let path_s6 = &payload.sessionId;
  let path = path_s1 + path_s2 + path_s3 + path_s4 + path_s5 + path_s6;
  let url = ADK_API_URL.to_string() + &path;
  println!("url={}", url);
  let sendBody = "{}";

  // JSON文字列をValue型にデコード
  let value: Value = serde_json::from_str(&sendBody).expect("REASON");
  println!("{}", sendBody );

  let client = reqwest::Client::new();
  let response: Result<reqwest::Response, Error> = client.post(url)
      .json(&value)
      .send()
      .await;

  match response {
    Ok(resp) => {
        let status = resp.status();
        println!("HTTP Status: {}", status.to_string());

        if status.is_success() {
          println!("Request was successful!");
          let body = resp.text().await.unwrap();
          return Ok(Json(json!({
            "status": 200,
            "body": &body.to_string(),
          })))
        } 
        else if status.is_client_error() {
          println!("Client error occurred!");
          let body = resp.text().await.unwrap();
          return Ok(Json(json!({
            "status": 400,
            "body": &body.to_string(),
          })))
        }
        else if status.is_server_error() {
          println!("Server error occurred!");
          let body = resp.text().await.unwrap();
          return Ok(Json(json!({
            "status": 500,
            "body": &body.to_string(),
          })))
        }
    }
    Err(err) => {
        eprintln!("Request failed: {}", err);
    }
  }      
  Ok(Json(json!({
      "status": 500,
      "body": "",
  })))
}
/**
*
* @param
*
* @return
*/
pub async fn adk_run_handler(
  State(state): State<AppState>,
  Json(payload): Json<AdkRunRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {

  let client = reqwest::Client::new();
  let path_s1 = String::from("/run");
  let url = ADK_API_URL.to_string() + &path_s1;
  println!("url={}", url);
  // JSON文字列をValue型にデコード
  let value: Value = serde_json::from_str(&payload.text).expect("REASON");
  println!("{}", &value );

  let client = reqwest::Client::new();
  let response: Result<reqwest::Response, Error> = client.post(url)
      .json(&value)
      .send()
      .await;

  match response {
    Ok(resp) => {
        let status = resp.status();
        println!("HTTP Status: {}", status.to_string());

        if status.is_success() {
          println!("Request was successful!");
          let body = resp.text().await.unwrap();
          return Ok(Json(json!({
            "status": 200,
            "body": &body.to_string(),
          })))
        } 
        else if status.is_client_error() {
          println!("Client error occurred!");
          let body = resp.text().await.unwrap();
          return Ok(Json(json!({
            "status": 400,
            "body": &body.to_string(),
          })))
        }
        else if status.is_server_error() {
          println!("Server error occurred!");
          let body = resp.text().await.unwrap();
          return Ok(Json(json!({
            "status": 500,
            "body": &body.to_string(),
          })))
        }
    }
    Err(err) => {
        eprintln!("Request failed: {}", err);
    }
  }      
  Ok(Json(json!({
      "status": 500,
      "body": "",
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
    <link href='/static/main.css' rel='stylesheet' /> 
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

        .route("/api/adk_run", post(adk_run_handler))
        .route("/api/adk_init", post(adk_init_handler))

        .route("/", get(root))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("Server running on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}