use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post}, 
    Router,
    response::{Html, IntoResponse, Json},
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{sqlite::SqlitePool, Row};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    id: i64,
    title: String,
    content: Option<String>,
    created_at: Option<String>,
    updated_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTodo {
    title: String,
    content: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DeleteTodo {
    id: i64,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodo {
    id: i64,
    title: String,
    content: Option<String>,
}

pub async fn list_todos(State(pool): State<Arc<SqlitePool>>) -> Result<Json<Vec<Todo>>, StatusCode> {
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

pub async fn create_todo(
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

pub async fn delete_todo(
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

pub async fn update_todo(
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
