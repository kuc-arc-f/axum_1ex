use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{PgPool, Row};
use sqlx::postgres::PgPoolOptions;
use sqlx::FromRow;
use std::sync::Arc;
use tokio;

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

pub async fn get_todos(State(state): State<AppState>) -> Result<String, StatusCode> {
    let todos = sqlx::query_as::<_, Todo>("SELECT id , title, content FROM todos ORDER BY created_at DESC")
        .fetch_all(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // 5) 構造体へマッピングして一覧取得
    let todoItems: Vec<Todo> = sqlx::query_as::<_, Todo>("SELECT id, title, content FROM todos")
        .fetch_all(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    //println!("Mapped structs > {:?}", todoItems);

    let out = serde_json::to_string(&todoItems).unwrap();
    Ok(out.to_string())
}


pub async fn create_todo(
    State(state): State<AppState>,
    Json(payload): Json<CreateTodo>,
) -> Result<String, StatusCode> {
    let result = sqlx::query(
        "INSERT INTO todos (title, content) VALUES ($1, $2) RETURNING id",
    )
    .bind(&payload.title)
    .bind(&payload.content)
    .execute(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    //Ok(Json(todo))
    Ok("OK".to_string())
}

pub async fn delete_todo(
    State(state): State<AppState>,
    Json(payload): Json<DeleteTodo>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    println!("# /api/delete");
    println!("{:?}", payload);

    let result = sqlx::query("DELETE FROM todos WHERE id = $1")
        .bind(&payload.id)
        .execute(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    //println!("# /api/delete END");
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
    println!("{:?}", payload);

    let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    
    //"UPDATE todos SET title = $1, content = $2, updated_at = $3 WHERE id = $4"
    let result = sqlx::query(
        "UPDATE todos SET title = $1, content = $2 WHERE id = $3"
    )
    .bind(&payload.title)
    .bind(&payload.content)
    .bind(&payload.id)
    .execute(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    println!("# /api/update END");

    //Ok(Json(todo))
    Ok(Json(json!({
        "message": "Todo update successfully",
        "id": payload.id
    })))

}


#[tokio::main]
async fn main() {
    let database_url = "postgresql://postgresql:admin@localhost/postgresql".to_string();

    let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect("postgres://postgres:admin@localhost/postgres")
    .await.expect("Failed to create pool");

    let state = AppState { pool };

    let app = Router::new()
        .route("/api/list", get(get_todos))
        .route("/api/create", post(create_todo))
        .route("/api/delete", post(delete_todo))
        .route("/api/update", post(update_todo))  
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("Server running on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}