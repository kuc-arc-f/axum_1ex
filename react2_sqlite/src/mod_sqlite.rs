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
