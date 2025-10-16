use axum::{
    extract::State,
    http::StatusCode,
    http::HeaderMap,
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use dotenvy::dotenv;
use std::env;
use libsql::Database;
use libsql::Builder;
use libsql::Connection;
use libsql::params;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tracing::{info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Debug, Deserialize)]
pub struct JsonRpcRequest {
    jsonrpc: String,
    id: Option<Value>,
    method: String,
    params: Option<Value>,
}

#[derive(Debug, Deserialize)]
struct AddTenParams {
    value: i32,
}

#[derive(Debug, Deserialize,Serialize)]
pub struct PurchaseParams {
    name: String,
    price: i32,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    id: i64,
    data: String,
    created_at: String,
    updated_at: String,
}


pub fn purchase(product_name: String, price: i32) -> String {
    format!("「{}」を{}円で購入しました。", product_name, price)
}
//Result<String, String>
pub async fn purchase_handler(
    name: String, price: i32
) -> String
{
    let url = env::var("TURSO_DATABASE_URL").expect("TURSO_DATABASE_URL must be set");
    let token = env::var("TURSO_AUTH_TOKEN").expect("TURSO_AUTH_TOKEN must be set");
    tracing::info!("TURSO_DATABASE_URL={}", url);
    let db = Builder::new_remote(url, token).build().await.unwrap();
    let conn = db.connect().unwrap();    

    let post_data = PurchaseParams {
        name: name.clone(),
        price: price
    };    
    let json_string_variable = serde_json::to_string(&post_data).expect("JSON convert error");
    println!("変換されたJSON文字列: {}", json_string_variable); 
    let sql = format!("INSERT INTO item_price (data) VALUES ('{}')", &json_string_variable);
    let mut result = conn
        .execute(&sql, ())
        .await
        .unwrap();

    let result_str = purchase(name, price);

    //Ok(result_str.to_string())
    result_str.to_string()
}
