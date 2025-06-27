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
    content_type: Option<String>,
    public_type: Option<String>,
    food_orange: Option<i32>,
    food_apple: Option<i32>,
    food_banana: Option<i32>,
    food_melon: Option<i32>,
    food_grape: Option<i32>,
    category_food: Option<i32>,
    category_drink: Option<i32>,
    category_gadget: Option<i32>,
    category_sport: Option<i32>,
    category_government: Option<i32>,
    category_internet: Option<i32>,
    category_smartphone: Option<i32>,
    country_jp: Option<String>,
    country_en: Option<String>,
    prefecture_jp: Option<String>,
    prefecture_en: Option<String>,
    created_at: Option<String>,
    updated_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTodo {
    title: String,
    content: Option<String>,
    content_type: Option<String>,
    public_type: Option<String>,
    food_orange: Option<i32>,
    food_apple: Option<i32>,
    food_banana: Option<i32>,
    food_melon: Option<i32>,
    food_grape: Option<i32>,
    category_food: Option<i32>,
    category_drink: Option<i32>,
    category_gadget: Option<i32>,
    category_sport: Option<i32>,
    category_government: Option<i32>,
    category_internet: Option<i32>,
    category_smartphone: Option<i32>,
    country_jp: Option<String>,
    country_en: Option<String>,
    prefecture_jp: Option<String>,
    prefecture_en: Option<String>,
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
    content_type: Option<String>,
    public_type: Option<String>,
    food_orange: Option<i32>,
    food_apple: Option<i32>,
    food_banana: Option<i32>,
    food_melon: Option<i32>,
    food_grape: Option<i32>,
    category_food: Option<i32>,
    category_drink: Option<i32>,
    category_gadget: Option<i32>,
    category_sport: Option<i32>,
    category_government: Option<i32>,
    category_internet: Option<i32>,
    category_smartphone: Option<i32>,
    country_jp: Option<String>,
    country_en: Option<String>,
    prefecture_jp: Option<String>,
    prefecture_en: Option<String>,
}

pub async fn list_todos(State(pool): State<Arc<SqlitePool>>) -> Result<Json<Vec<Todo>>, StatusCode> {
    let rows = sqlx::query("SELECT id, title, content, content_type, public_type, food_orange, food_apple, food_banana, food_melon, food_grape, category_food, category_drink, category_gadget, category_sport, category_government, category_internet, category_smartphone, country_jp, country_en, prefecture_jp, prefecture_en, created_at, updated_at FROM todos")
        .fetch_all(pool.as_ref())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let todos: Vec<Todo> = rows
        .into_iter()
        .map(|row| Todo {
            id: row.get("id"),
            title: row.get("title"),
            content: row.get("content"),
            content_type: row.get("content_type"),
            public_type: row.get("public_type"),
            food_orange: row.get("food_orange"),
            food_apple: row.get("food_apple"),
            food_banana: row.get("food_banana"),
            food_melon: row.get("food_melon"),
            food_grape: row.get("food_grape"),
            category_food: row.get("category_food"),
            category_drink: row.get("category_drink"),
            category_gadget: row.get("category_gadget"),
            category_sport: row.get("category_sport"),
            category_government: row.get("category_government"),
            category_internet: row.get("category_internet"),
            category_smartphone: row.get("category_smartphone"),
            country_jp: row.get("country_jp"),
            country_en: row.get("country_en"),
            prefecture_jp: row.get("prefecture_jp"),
            prefecture_en: row.get("prefecture_en"),
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
    println!("#create_todo");

    let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    
    let result = sqlx::query(
        "INSERT INTO todos (title, content, content_type, public_type, food_orange, food_apple, food_banana, food_melon, food_grape, category_food, category_drink, category_gadget, category_sport, category_government, category_internet, category_smartphone, country_jp, country_en, prefecture_jp, prefecture_en, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&payload.title)
    .bind(&payload.content)
    .bind(&payload.content_type)
    .bind(&payload.public_type)
    .bind(&payload.food_orange)
    .bind(&payload.food_apple)
    .bind(&payload.food_banana)
    .bind(&payload.food_melon)
    .bind(&payload.food_grape)
    .bind(&payload.category_food)
    .bind(&payload.category_drink)
    .bind(&payload.category_gadget)
    .bind(&payload.category_sport)
    .bind(&payload.category_government)
    .bind(&payload.category_internet)
    .bind(&payload.category_smartphone)
    .bind(&payload.country_jp)
    .bind(&payload.country_en)
    .bind(&payload.prefecture_jp)
    .bind(&payload.prefecture_en)
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
        content_type: payload.content_type,
        public_type: payload.public_type,
        food_orange: payload.food_orange,
        food_apple: payload.food_apple,
        food_banana: payload.food_banana,
        food_melon: payload.food_melon,
        food_grape: payload.food_grape,
        category_food: payload.category_food,
        category_drink: payload.category_drink,
        category_gadget: payload.category_gadget,
        category_sport: payload.category_sport,
        category_government: payload.category_government,
        category_internet: payload.category_internet,
        category_smartphone: payload.category_smartphone,
        country_jp: payload.country_jp,
        country_en: payload.country_en,
        prefecture_jp: payload.prefecture_jp,
        prefecture_en: payload.prefecture_en,
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
        "UPDATE todos SET title = ?, content = ?, content_type = ?, public_type = ?, food_orange = ?, food_apple = ?, food_banana = ?, food_melon = ?, food_grape = ?, category_food = ?, category_drink = ?, category_gadget = ?, category_sport = ?, category_government = ?, category_internet = ?, category_smartphone = ?, country_jp = ?, country_en = ?, prefecture_jp = ?, prefecture_en = ?, updated_at = ? WHERE id = ?"
    )
    .bind(&payload.title)
    .bind(&payload.content)
    .bind(&payload.content_type)
    .bind(&payload.public_type)
    .bind(&payload.food_orange)
    .bind(&payload.food_apple)
    .bind(&payload.food_banana)
    .bind(&payload.food_melon)
    .bind(&payload.food_grape)
    .bind(&payload.category_food)
    .bind(&payload.category_drink)
    .bind(&payload.category_gadget)
    .bind(&payload.category_sport)
    .bind(&payload.category_government)
    .bind(&payload.category_internet)
    .bind(&payload.category_smartphone)
    .bind(&payload.country_jp)
    .bind(&payload.country_en)
    .bind(&payload.prefecture_jp)
    .bind(&payload.prefecture_en)
    .bind(&now)
    .bind(payload.id)
    .execute(pool.as_ref())
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    let row = sqlx::query("SELECT id, title, content, content_type, public_type, food_orange, food_apple, food_banana, food_melon, food_grape, category_food, category_drink, category_gadget, category_sport, category_government, category_internet, category_smartphone, country_jp, country_en, prefecture_jp, prefecture_en, created_at, updated_at FROM todos WHERE id = ?")
        .bind(payload.id)
        .fetch_one(pool.as_ref())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let todo = Todo {
        id: row.get("id"),
        title: row.get("title"),
        content: row.get("content"),
        content_type: row.get("content_type"),
        public_type: row.get("public_type"),
        food_orange: row.get("food_orange"),
        food_apple: row.get("food_apple"),
        food_banana: row.get("food_banana"),
        food_melon: row.get("food_melon"),
        food_grape: row.get("food_grape"),
        category_food: row.get("category_food"),
        category_drink: row.get("category_drink"),
        category_gadget: row.get("category_gadget"),
        category_sport: row.get("category_sport"),
        category_government: row.get("category_government"),
        category_internet: row.get("category_internet"),
        category_smartphone: row.get("category_smartphone"),
        country_jp: row.get("country_jp"),
        country_en: row.get("country_en"),
        prefecture_jp: row.get("prefecture_jp"),
        prefecture_en: row.get("prefecture_en"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    };

    Ok(Json(todo))
}
