use axum::{
    body::Body,
    extract::{Path, State},
    http::{header, StatusCode, HeaderName},
    response::{Json, IntoResponse, Response},
};
use chrono::Utc;
use serde_json::{json, Value};

use sqlx::{PgPool, Row};
use std::fs;
use std::sync::Arc;
//use std::path::Path;
use umya_spreadsheet::*;
use uuid::Uuid;

pub async fn get_todos(State(state): State<super::models::AppState>) -> Result<String, StatusCode> {
    println!("# /api/list");

    let todSql = "SELECT id, title, content , completed , content_type ,
    is_public, food_orange, food_apple, food_banana, food_melon, food_grape, 
    pub_date1, pub_date2, pub_date3, pub_date4, pub_date5, pub_date6,
    qty1, qty2, qty3, qty4, qty5, qty6,
    created_at, updated_at
    FROM todos ORDER BY id DESC
    ";
    //println!("# todSql={}", &todSql);

    let todo_items: Vec<super::models::TodoResponse> = sqlx::query_as::<_, super::models::TodoResponse>(&todSql)
        .fetch_all(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    //println!("Mapped structs > {:?}", todo_items);

    let out = serde_json::to_string(&todo_items).unwrap();    
    Ok(out.to_string())
}


pub async fn create_todo(
    State(state): State<super::models::AppState>,
    Json(payload): Json<super::models::CreateTodo>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    println!("# /api/create");
    println!("{:?}", &payload);

    let result = sqlx::query(
        "INSERT INTO todos (title, content, completed, content_type, is_public, food_orange, food_apple, food_banana, food_melon, food_grape, pub_date1, pub_date2, pub_date3, pub_date4, pub_date5, pub_date6, qty1, qty2, qty3, qty4, qty5, qty6) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22)"
    )
    .bind(&payload.title)
    .bind(&payload.content)
    .bind(payload.completed.unwrap_or(false))
    .bind(&payload.content_type)
    .bind(payload.is_public.unwrap_or(false))
    .bind(payload.food_orange.unwrap_or(false))
    .bind(payload.food_apple.unwrap_or(false))
    .bind(payload.food_banana.unwrap_or(false))
    .bind(payload.food_melon.unwrap_or(false))
    .bind(payload.food_grape.unwrap_or(false))
    .bind(&payload.pub_date1)
    .bind(&payload.pub_date2)
    .bind(&payload.pub_date3)
    .bind(&payload.pub_date4)
    .bind(&payload.pub_date5)
    .bind(&payload.pub_date6)
    .bind(&payload.qty1)
    .bind(&payload.qty2)
    .bind(&payload.qty3)
    .bind(&payload.qty4)
    .bind(&payload.qty5)
    .bind(&payload.qty6)
    .execute(&state.pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to create todo: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(json!({
        "ret": 200,
        "message": "Todo created successfully",
    })))
}

pub async fn delete_todo(
    State(state): State<super::models::AppState>,
    Json(payload): Json<super::models::DeleteTodo>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    println!("# /api/delete");
    println!("{:?}", payload);

    sqlx::query("DELETE FROM todos WHERE id = $1")
        .bind(&payload.id)
        .execute(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({
        "message": "Todo deleted successfully",
        "id": payload.id
    })))
}

pub async fn update_todo(
    State(state): State<super::models::AppState>,
    Json(payload): Json<super::models::UpdateTodo>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    println!("# /api/update");
    //println!("{:?}", &payload);

    let mut tx = state.pool.begin().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let todSql = "SELECT id, title, content , completed , content_type ,
    is_public, food_orange, food_apple, food_banana, food_melon, food_grape, 
    pub_date1, pub_date2, pub_date3, pub_date4, pub_date5, pub_date6,
    qty1, qty2, qty3, qty4, qty5, qty6,
    created_at, updated_at
    FROM todos WHERE id = $1 LIMIT 1
    ";
    let select_item: Vec<super::models::TodoResponse> = sqlx::query_as::<_, super::models::TodoResponse>(&todSql)
        .bind(&payload.id)
        .fetch_all(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    //println!("# todo.select-1");
    //println!("{:?}", &select_item);
    
    let mut todo: super::models::UpdateTodo = payload.clone();
    //println!("# todo.select-2");
    //println!("{:?}", &todo);

    if let Some(title) = payload.title {
        todo.title = Some(title);
    }
    if let Some(content) = payload.content {
        todo.content = Some(content);
    }
    if let Some(completed) = payload.completed {
        todo.completed = Some(completed);
    }
    if let Some(content_type) = payload.content_type {
        todo.content_type = Some(content_type);
    }
    if let Some(is_public) = payload.is_public {
        todo.is_public = Some(is_public);
    }
    if let Some(food_orange) = payload.food_orange {
        todo.food_orange = Some(food_orange);
    }
    if let Some(food_apple) = payload.food_apple {
        todo.food_apple = Some(food_apple);
    }
    if let Some(food_banana) = payload.food_banana {
        todo.food_banana = Some(food_banana);
    }
    if let Some(food_melon) = payload.food_melon {
        todo.food_melon = Some(food_melon);
    }
    if let Some(food_grape) = payload.food_grape {
        todo.food_grape = Some(food_grape);
    }
    if let Some(pub_date1) = payload.pub_date1 {
        todo.pub_date1 = Some(pub_date1);
    }
    if let Some(pub_date2) = payload.pub_date2 {
        todo.pub_date2 = Some(pub_date2);
    }
    if let Some(pub_date3) = payload.pub_date3 {
        todo.pub_date3 = Some(pub_date3);
    }
    if let Some(pub_date4) = payload.pub_date4 {
        todo.pub_date4 = Some(pub_date4);
    }
    if let Some(pub_date5) = payload.pub_date5 {
        todo.pub_date5 = Some(pub_date5);
    }
    if let Some(pub_date6) = payload.pub_date6 {
        todo.pub_date6 = Some(pub_date6);
    }
    if let Some(qty1) = payload.qty1 {
        todo.qty1 = Some(qty1);
    }
    if let Some(qty2) = payload.qty2 {
        todo.qty2 = Some(qty2);
    }
    if let Some(qty3) = payload.qty3 {
        todo.qty3 = Some(qty3);
    }
    if let Some(qty4) = payload.qty4 {
        todo.qty4 = Some(qty4);
    }
    if let Some(qty5) = payload.qty5 {
        todo.qty5 = Some(qty5);
    }
    if let Some(qty6) = payload.qty6 {
        todo.qty6 = Some(qty6);
    }

    sqlx::query("UPDATE todos SET title = $1, content = $2, completed = $3, content_type = $4, is_public = $5, food_orange = $6, food_apple = $7, food_banana = $8, food_melon = $9, food_grape = $10, pub_date1 = $11, pub_date2 = $12, pub_date3 = $13, pub_date4 = $14, pub_date5 = $15, pub_date6 = $16, qty1 = $17, qty2 = $18, qty3 = $19, qty4 = $20, qty5 = $21, qty6 = $22, updated_at = now() WHERE id = $23")
        .bind(&todo.title)
        .bind(&todo.content)
        .bind(todo.completed)
        .bind(&todo.content_type)
        .bind(todo.is_public)
        .bind(todo.food_orange)
        .bind(todo.food_apple)
        .bind(todo.food_banana)
        .bind(todo.food_melon)
        .bind(todo.food_grape)
        .bind(todo.pub_date1)
        .bind(todo.pub_date2)
        .bind(todo.pub_date3)
        .bind(todo.pub_date4)
        .bind(todo.pub_date5)
        .bind(todo.pub_date6)
        .bind(&todo.qty1)
        .bind(&todo.qty2)
        .bind(&todo.qty3)
        .bind(&todo.qty4)
        .bind(&todo.qty5)
        .bind(&todo.qty6)
        .bind(todo.id)
        .execute(&mut *tx)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    tx.commit().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({
        "message": "Todo updated successfully",
        "id": payload.id
    })))
}

pub async fn dowload_handler(State(state): State<super::models::AppState>) -> impl IntoResponse {
    println!("# /test");

    let todSql = "SELECT id, title, content 
    FROM todos ORDER BY id DESC
    ";
    println!("# todSql={}", &todSql);
    //TodoXlsItems

    let todo_items: Vec<super::models::TodoXlsItems> = sqlx::query_as::<_, super::models::TodoXlsItems>(&todSql)
        .fetch_all(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR).unwrap();

    //println!("Mapped structs > {:?}", todo_items);

    let inPath = std::path::Path::new("input.xlsx");
    let mut book = reader::xlsx::read(inPath).unwrap();  // 普通に読み込み

    let sheet = book.get_sheet_by_name_mut("Sheet1").unwrap();

    let mut countNum = 2;
    for v in &todo_items {
        println!("countNum={}", countNum);
        //println!("{:?}", v);
        let cell_num = countNum.to_string();
        let cell_pos_a = format!("{}{}", "A", cell_num);
        let cell_pos_b = format!("{}{}", "B", cell_num);
        let cell_pos_c = format!("{}{}", "C", cell_num);
        let cell_val_a = v.id.to_string();
        let cell_val_b = &v.title;
        let cell_val_c = &v.content;
        println!("cell_pos_a={}", &cell_pos_a);
        println!("cell_val_a={}", &cell_val_a);
        //println!("cell_val_b={}", &cell_val_b.to_string().unwrap());
        println!("cell_val_b={}", &cell_val_b.clone().unwrap());
        sheet.get_cell_mut(cell_pos_a.to_string()).set_value(&cell_val_a);
        sheet.get_cell_mut(cell_pos_b.to_string()).set_value(&cell_val_b.clone().unwrap());
        sheet.get_cell_mut(cell_pos_c.to_string()).set_value(&cell_val_c.clone().unwrap());
        countNum += 1;
    }
    let file_name = format!("{}.xlsx", Uuid::new_v4());
    let path = std::path::Path::new(&file_name);

    match writer::xlsx::write(&book, path) {
        Ok(_) => {
            let file_content = match fs::read(path) {
                Ok(content) => content,
                Err(_) => {
                    return Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(Body::from("Failed to read Excel file."))
                        .unwrap();
                }
            };

            let _ = fs::remove_file(path);

            let headers = [
                (
                    header::CONTENT_TYPE,
                    "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
                ),
                (
                    header::CONTENT_DISPOSITION,
                    "attachment; filename=\"edit_result.xlsx\"",
                ),
            ];
            Response::builder()
                .status(StatusCode::OK)
                .header(headers[0].0.clone(), headers[0].1)
                .header(headers[1].0.clone(), headers[1].1)
                .body(Body::from(file_content))
                .unwrap()
        }
        Err(_) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from("Failed to create Excel file."))
            .unwrap(),
    }
}

pub async fn edit_download_excel() -> impl IntoResponse {
    let inPath = std::path::Path::new("input.xlsx");
    let mut book = reader::xlsx::read(inPath).unwrap();  // 普通に読み込み

    let sheet = book.get_sheet_by_name_mut("Sheet1").unwrap();
    // セル A1 に文字列
    sheet.get_cell_mut("A2").set_value("こんにちは、Rust!");
    sheet.get_cell_mut("B2").set_value("123");
    sheet.get_cell_mut("C2").set_value("C-123");

    let file_name = format!("{}.xlsx", Uuid::new_v4());
    let path = std::path::Path::new(&file_name);

    match writer::xlsx::write(&book, path) {
        Ok(_) => {
            let file_content = match fs::read(path) {
                Ok(content) => content,
                Err(_) => {
                    return Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(Body::from("Failed to read Excel file."))
                        .unwrap();
                }
            };

            let _ = fs::remove_file(path);

            let headers = [
                (
                    header::CONTENT_TYPE,
                    "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
                ),
                (
                    header::CONTENT_DISPOSITION,
                    "attachment; filename=\"edit_result.xlsx\"",
                ),
            ];
            Response::builder()
                .status(StatusCode::OK)
                .header(headers[0].0.clone(), headers[0].1)
                .header(headers[1].0.clone(), headers[1].1)
                .body(Body::from(file_content))
                .unwrap()
        }
        Err(_) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from("Failed to create Excel file."))
            .unwrap(),
    }
}