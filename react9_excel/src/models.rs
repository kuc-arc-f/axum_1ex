use sqlx::{PgPool, Row};
use sqlx::postgres::PgPoolOptions;
use sqlx::FromRow;
use chrono::{Utc, NaiveDate, DateTime, NaiveDateTime};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
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
    pub title: String,
    pub content: String,
    pub completed: Option<bool>,
    pub content_type: Option<String>,
    pub is_public: Option<bool>,
    pub food_orange: Option<bool>,
    pub food_apple: Option<bool>,
    pub food_banana: Option<bool>,
    pub food_melon: Option<bool>,
    pub food_grape: Option<bool>,
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
}

#[derive(Debug, Deserialize)]
pub struct DeleteTodo {
    pub id: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UpdateTodo {
    pub id: i32,
    pub title: Option<String>,
    pub content: Option<String>,
    pub completed: Option<bool>,
    pub content_type: Option<String>,
    pub is_public: Option<bool>,
    pub food_orange: Option<bool>,
    pub food_apple: Option<bool>,
    pub food_banana: Option<bool>,
    pub food_melon: Option<bool>,
    pub food_grape: Option<bool>,
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

#[derive(Debug, Serialize , Deserialize, FromRow)]
pub struct TodoXlsItems {
    pub id: i32,
    pub title: Option<String>,
    pub content: Option<String>,
}