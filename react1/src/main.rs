use axum::{
    routing::get, 
    Router,
    response::{Html, IntoResponse},
};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    // `public` フォルダのパス
    let public_dir = "public";

    // `ServeDir` ミドルウェアを初期化
    let serve_dir = ServeDir::new(public_dir);

    let app = Router::new()
        .nest_service("/", serve_dir)
        .route("/foo", get(get_foo));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> String {
    String::from("root\n")
}

async fn get_foo() -> Html<&'static str> {
    let s1 = "<html>
  <head>
    <title>welcome</title>
  </head>
  <body>
    <div id='app'></div>
    <script type='module' src='/client.js'></script>
  <body>
</html>
";
    Html(&s1)
}

