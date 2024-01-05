use axum::{response::IntoResponse, routing::get, Router};

pub fn get_routes() -> Router {
    Router::new().route("/", get(get_another_page))
}
async fn get_another_page() -> impl IntoResponse {
    "Hello, World!"
}
