use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use chrono::Utc;
use serde_json::{json, Value};

use sqlx::{PgPool, Row};
use crate::database::DbPool;
use crate::models::{CreateTodo, Todo, UpdateTodo};
use std::sync::Arc;


pub async fn create_todo(
    State(pool): State<DbPool>,
    Json(payload): Json<CreateTodo>,
) -> Result<Json<Todo>, StatusCode> {
    let todo = sqlx::query_as::<_, Todo>(
        r#"
        INSERT INTO todos (title, content, created_at, updated_at)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
    )
    .bind(&payload.title)
    .bind(&payload.content)
    .bind(Utc::now())
    .bind(Utc::now())
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(todo))
}

pub async fn get_todo(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
) -> Result<Json<Todo>, StatusCode> {
    let todo = sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE id = $1")
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(todo))
}

pub async fn update_todo(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateTodo>,
) -> Result<Json<Todo>, StatusCode> {
    let todo = sqlx::query_as::<_, Todo>(
        r#"
        UPDATE todos 
        SET title = COALESCE($1, title), 
            content = COALESCE($2, content),
            updated_at = $3
        WHERE id = $4
        RETURNING *
        "#,
    )
    .bind(&payload.title)
    .bind(&payload.content)
    .bind(Utc::now())
    .bind(id)
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(todo))
}

pub async fn delete_todo(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
) -> Result<Json<Value>, StatusCode> {
    let result = sqlx::query("DELETE FROM todos WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(Json(json!({ "message": "Todo deleted successfully" })))
}