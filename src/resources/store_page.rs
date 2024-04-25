use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

#[derive(Template)]
#[template(path = "slangwords/store.html")]
pub struct StoreTemplate;

pub async fn go_to_store_page() -> impl IntoResponse {
    let template = StoreTemplate;

    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to render template!"),
        )
            .into_response(),
    }
}
