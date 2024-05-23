pub mod delete;
pub mod get_all;
pub mod get_one;
pub mod home;
pub mod search;
pub mod store;
pub mod store_page;
pub mod update;
pub mod update_page;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
pub use delete::delete_slang_word;
pub use get_all::get_slang_words;
pub use get_one::get_slang_word;
pub use home::home;
pub use search::search_slang_word;
pub use store::create_slang_word;
pub use store_page::go_to_store_page;
use thiserror::Error;
pub use update::update_slang_word;
pub use update_page::go_to_update_page;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("failed to render template: `{0}`")]
    RenderHtmlError(#[from] askama::Error),
    #[error("database error: `{0}`")]
    SqlxError(#[from] sqlx::Error),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let error_handler = match self {
            ApiError::RenderHtmlError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            ApiError::SqlxError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
        };

        error_handler.into_response()
    }
}

#[derive(sqlx::FromRow, serde::Deserialize)]
pub struct SlangWord {
    pub id: i32,
    pub word: String,
    pub synonym: String,
    pub description: String,
}
