use axum::{routing::get, Router};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    // `public` フォルダのパス
    let public_dir = "public";

    // `ServeDir` ミドルウェアを初期化
    let serve_dir = ServeDir::new(public_dir);
    //.route("/", get(root))
    let app = Router::new()
        .nest_service("/", serve_dir)
        .route("/foo", get(get_foo));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> String {
    String::from("root\n")
}

async fn get_foo() -> String {
    String::from("get_foo\n")
}

