use axum::{
    body::Body,
    http::{header, StatusCode, HeaderName},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use std::fs;
use std::path::Path;
use tokio::net::TcpListener;
use umya_spreadsheet::*;
use uuid::Uuid;

async fn edit_download_excel() -> impl IntoResponse {
    let inPath = Path::new("input.xlsx");
    let mut book = reader::xlsx::read(inPath).unwrap();  // 普通に読み込み

    let sheet = book.get_sheet_by_name_mut("Sheet1").unwrap();
    // セル A1 に文字列
    sheet.get_cell_mut("A2").set_value("こんにちは、Rust!");
    sheet.get_cell_mut("B2").set_value("123");
    sheet.get_cell_mut("C2").set_value("C-123");

    let file_name = format!("{}.xlsx", Uuid::new_v4());
    let path = Path::new(&file_name);

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

async fn download_excel() -> impl IntoResponse {
    let mut book = new_file();
    let _ = book.new_sheet("Sheet1");

    let sheet = book.get_sheet_by_name_mut("Sheet1").unwrap();
    sheet.get_cell_mut("A1").set_value("テスト");
    let style = sheet.get_style_mut("A1");
    style.set_background_color(Color::COLOR_YELLOW);
    style
        .get_borders_mut()
        .get_bottom_mut()
        .set_border_style(Border::BORDER_MEDIUM);

    book.insert_new_row("Sheet1", &2, &1);
    book.insert_new_column("Sheet1", "B", &1);

    let file_name = format!("{}.xlsx", Uuid::new_v4());
    let path = Path::new(&file_name);

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
                    "attachment; filename=\"result.xlsx\"",
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

#[tokio::main]
async fn main() {
    let app = Router::new().route("/download", get(download_excel))
    .route("/edit_download", get(edit_download_excel))
    ;
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Listening on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}