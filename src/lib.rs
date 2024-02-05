pub mod resources;

use axum::{routing::get, Router};
use tower_http::services::ServeDir;

pub fn create_router() -> Router {
    Router::new()
    .route("/", get(resources::home))
    .route("/slangword", get(resources::get_slang_word))
    .nest_service("/public", ServeDir::new("public"))
}