use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use dotenvy::dotenv;
use libsql::Builder;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::env;
use std::sync::Arc;
use tower_http::trace::TraceLayer;

mod mod_purchase;

// JSON-RPC 2.0 Request
#[derive(Debug, Deserialize)]
struct JsonRpcRequest {
    jsonrpc: String,
    method: String,
    #[serde(default)]
    params: Option<Value>,
    id: Option<Value>,
}

// JSON-RPC 2.0 Response
#[derive(Debug, Serialize)]
struct JsonRpcResponse {
    jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<JsonRpcError>,
    id: Option<Value>,
}

// JSON-RPC 2.0 Error
#[derive(Debug, Serialize)]
struct JsonRpcError {
    code: i32,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,
}

// MCP Server State
#[derive(Clone)]
struct AppState {
    server_name: String,
    version: String,
    db_path: String,
}
#[derive(Debug, Deserialize,Serialize)]
struct PurchaseParams {
    name: String,
    price: i32,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    // ロギング初期化
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    // データベース初期化
    let db_path = ":memory:";

    // アプリケーションステート
    let state = Arc::new(AppState {
        server_name: "MCP Server Example".to_string(),
        version: "1.0.0".to_string(),
        db_path: db_path.to_string(),
    });

    // ルーター設定
    let app = Router::new()
        .route("/", post(handle_jsonrpc))
        .route("/mcp", post(handle_jsonrpc))
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    // サーバー起動
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    tracing::info!("MCP Server listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

// JSON-RPC ハンドラー
async fn handle_jsonrpc(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(request): Json<JsonRpcRequest>,
) -> impl IntoResponse 
{
    tracing::info!("Received request: method={}, id={:?}", request.method, request.id);

    // Authorization ヘッダーの検証
    let api_key = env::var("API_KEY").unwrap_or_else(|_| String::new());
    
    if !api_key.is_empty() {
        let auth_header = headers.get("Authorization")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");
        
        tracing::info!("auth={}", auth_header);
        
        if auth_header != api_key {
            tracing::info!("NG, auth-key");
            return (
                StatusCode::UNAUTHORIZED,
                Json(JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    result: None,
                    error: Some(JsonRpcError {
                        code: -32001,
                        message: "Unauthorized: Invalid API key".to_string(),
                        data: None,
                    }),
                    id: request.id,
                }),
            );
        }
        
        tracing::info!("ok, auth-key");
    }
        
    // JSON-RPC 2.0 バージョンチェック
    if request.jsonrpc != "2.0" {
        return (
            StatusCode::OK,
            Json(JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: None,
                error: Some(JsonRpcError {
                    code: -32600,
                    message: "Invalid Request".to_string(),
                    data: None,
                }),
                id: request.id,
            }),
        );
    }

    // メソッドディスパッチ
    let result = match request.method.as_str() {
        "initialize" => handle_initialize(&state, request.params),
        "tools/list" => handle_tools_list(),
        "tools/call" => handle_tools_call(state, request.params).await,
        "resources/list" => handle_resources_list(),
        "resources/read" => handle_resources_read(request.params),
        "prompts/list" => handle_prompts_list(),
        _ => Err(JsonRpcError {
            code: -32601,
            message: "Method not found".to_string(),
            data: None,
        }),
    };

    let response = match result {
        Ok(result) => JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            result: Some(result),
            error: None,
            id: request.id,
        },
        Err(error) => JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            result: None,
            error: Some(error),
            id: request.id,
        },
    };

    (StatusCode::OK, Json(response))
}

// MCP initialize メソッド
fn handle_initialize(state: &AppState, _params: Option<Value>) -> Result<Value, JsonRpcError> {
    Ok(json!({
        "protocolVersion": "2024-11-05",
        "serverInfo": {
            "name": state.server_name,
            "version": state.version
        },
        "capabilities": {
            "tools": {},
            "resources": {},
            "prompts": {}
        }
    }))
}

// tools/list メソッド
fn handle_tools_list() -> Result<Value, JsonRpcError> {
    Ok(json!({
        "tools": [
            {
                "name": "echo",
                "description": "Echo back the input message",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "message": {
                            "type": "string",
                            "description": "Message to echo"
                        }
                    },
                    "required": ["message"]
                }
            },
            {
                "name": "add",
                "description": "Add two numbers",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "a": { "type": "number" },
                        "b": { "type": "number" }
                    },
                    "required": ["a", "b"]
                }
            },
            {
                "name": "add_todo",
                "description": "Add a new todo item to the database",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "title": {
                            "type": "string",
                            "description": "The todo item title/content"
                        }
                    },
                    "required": ["title"]
                }
            },
            {
                "name": "purchase",
                "description": "品名と価格を受け取り、値をAPIに送信します。",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "name": {
                            "type": "string",
                            "description": "購入する品名"
                        },
                        "price": {
                            "type": "number",
                            "description": "価格"
                        }
                    },
                    "required": ["name", "price"]
                }
            }            
        ]
    }))
}

fn purchase(product_name: String, price: i32) -> String {
    format!("「{}」を{}円で購入しました。", product_name, price)
}
// tools/call メソッド
async fn handle_tools_call(state: Arc<AppState>, params: Option<Value>) -> Result<Value, JsonRpcError> {
    let params = params.ok_or(JsonRpcError {
        code: -32602,
        message: "Invalid params".to_string(),
        data: None,
    })?;

    let tool_name = params["name"].as_str().ok_or(JsonRpcError {
        code: -32602,
        message: "Tool name is required".to_string(),
        data: None,
    })?;

    let arguments = &params["arguments"];

    match tool_name {
        "echo" => {
            let message = arguments["message"].as_str().unwrap_or("No message");
            Ok(json!({
                "content": [
                    {
                        "type": "text",
                        "text": format!("Echo: {}", message)
                    }
                ]
            }))
        }
        "add" => {
            let a = arguments["a"].as_f64().unwrap_or(0.0);
            let b = arguments["b"].as_f64().unwrap_or(0.0);
            Ok(json!({
                "content": [
                    {
                        "type": "text",
                        "text": format!("Result: {}", a + b)
                    }
                ]
            }))
        },
        "purchase" => {
            let name = arguments["name"].as_str().ok_or(JsonRpcError {
                code: -32602,
                message: "name is required".to_string(),
                data: None,
            })?;    
            let price = arguments["price"].as_i64().unwrap_or(0);        
            let result_text = mod_purchase::purchase_handler(name.to_string(), price as i32).await;
            let out_text = format!("{} {} 円 登録しました", name, price);
            Ok(json!({
                "content": [
                    {
                        "type": "text",
                        "text": format!("Result: {}", out_text),
                    }
                ]
            }))
        }        
        "add_todo" => {
            let title = arguments["title"].as_str().ok_or(JsonRpcError {
                code: -32602,
                message: "Title is required".to_string(),
                data: None,
            })?;

            let db = Builder::new_local(&state.db_path).build().await.map_err(|e| JsonRpcError {
                code: -32603,
                message: format!("Database connection error: {}", e),
                data: None,
            })?;

            let conn = db.connect().map_err(|e| JsonRpcError {
                code: -32603,
                message: format!("Database connection error: {}", e),
                data: None,
            })?;

            conn.execute("INSERT INTO todo (title) VALUES (?)", [title])
                .await
                .map_err(|e| JsonRpcError {
                    code: -32603,
                    message: format!("Database insert error: {}", e),
                    data: None,
                })?;

            let last_id = conn.last_insert_rowid();

            Ok(json!({
                "content": [
                    {
                        "type": "text",
                        "text": format!("Todo added successfully with ID: {}", last_id)
                    }
                ]
            }))
        }
        _ => Err(JsonRpcError {
            code: -32602,
            message: format!("Unknown tool: {}", tool_name),
            data: None,
        }),
    }
}

// resources/list メソッド
fn handle_resources_list() -> Result<Value, JsonRpcError> {
    Ok(json!({
        "resources": [
            {
                "uri": "file:///example.txt",
                "name": "Example Resource",
                "description": "An example resource",
                "mimeType": "text/plain"
            }
        ]
    }))
}

// resources/read メソッド
fn handle_resources_read(params: Option<Value>) -> Result<Value, JsonRpcError> {
    let params = params.ok_or(JsonRpcError {
        code: -32602,
        message: "Invalid params".to_string(),
        data: None,
    })?;

    let uri = params["uri"].as_str().ok_or(JsonRpcError {
        code: -32602,
        message: "URI is required".to_string(),
        data: None,
    })?;

    Ok(json!({
        "contents": [
            {
                "uri": uri,
                "mimeType": "text/plain",
                "text": "This is the content of the resource"
            }
        ]
    }))
}

// prompts/list メソッド
fn handle_prompts_list() -> Result<Value, JsonRpcError> {
    Ok(json!({
        "prompts": [
            {
                "name": "example_prompt",
                "description": "An example prompt",
                "arguments": []
            }
        ]
    }))
}
