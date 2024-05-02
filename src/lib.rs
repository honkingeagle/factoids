pub mod resources;

use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use std::sync::Arc;
use tower_http::services::ServeDir;

type SharedState = Arc<AppState>;

pub struct AppState {
    pub pool: PgPool,
}

pub struct AppStateBuilder {
    pub pool: Option<PgPool>,
}

impl AppStateBuilder {
    pub fn new() -> AppStateBuilder {
        AppStateBuilder { pool: None }
    }

    pub fn pool(mut self, pool: PgPool) -> AppStateBuilder {
        self.pool = Some(pool);
        self
    }

    pub fn build(self) -> AppState {
        AppState {
            pool: self.pool.unwrap(),
        }
    }
}


pub fn create_router(state: SharedState) -> Router {
    Router::new()
        .route("/", get(resources::home))
        .route("/slangwords", get(resources::get_slang_words))
        .route("/slangword/create", post(resources::create_slang_word))
        .route("/slangword/store", get(resources::go_to_store_page))
        .route("/slangword/update/:id", get(resources::go_to_update_page))
        .route(
            "/slangword/:id",
            get(resources::get_slang_word)
                .put(resources::update_slang_word)
                .delete(resources::delete_slang_word),
        )
        .nest_service("/public", ServeDir::new("public"))
        .with_state(state)
}
