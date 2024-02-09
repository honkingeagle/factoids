pub mod resources;

use axum::{routing::get, Router};
use sqlx::PgPool;
use std::sync::Arc;
use tower_http::services::ServeDir;

type SharedState = Arc<AppState>;

pub struct AppState {
    pub pool: PgPool,
}

impl AppState {
    pub fn new(pool: PgPool) -> AppState {
        AppState { pool }
    }
}

pub fn create_router(state: SharedState) -> Router {
    Router::new()
        .route("/", get(resources::home).post(resources::create_slang_word))
        .route(
            "/slangword/:id",
            get(resources::get_slang_word).delete(resources::delete_slang_word),
        )
        .nest_service("/public", ServeDir::new("public"))
        .with_state(state)
}
