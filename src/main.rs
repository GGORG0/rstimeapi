use axum::{
    Router,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse as _, Response},
    routing::get,
};
use chrono::Utc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/", get(current_utc_time))
        .route("/unix", get(unix_time))
        .route("/unixms", get(unix_time_ms))
        .route("/rfc3339", get(rfc3339_time));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Server running on http://{}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}

fn text_response(body: String) -> Response {
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "text/plain".parse().unwrap());
    (StatusCode::OK, headers, body).into_response()
}

async fn current_utc_time() -> Response {
    let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    text_response(now)
}

async fn unix_time() -> Response {
    let timestamp = Utc::now().timestamp();
    text_response(timestamp.to_string())
}

async fn unix_time_ms() -> Response {
    let timestamp = Utc::now().timestamp_millis();
    text_response(timestamp.to_string())
}

async fn rfc3339_time() -> Response {
    let rfc3339 = Utc::now().to_rfc3339();
    text_response(rfc3339)
}
