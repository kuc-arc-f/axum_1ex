use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// #[derive(Debug, Serialize, Deserialize, FromRow)]
#[derive(Debug, Deserialize, FromRow)]
pub struct Todo {
    pub id: i32,
    pub title: Option<String>,
    pub content: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTodo {
    pub title: Option<String>,
    pub content: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodo {
    pub title: Option<String>,
    pub content: Option<String>,
}