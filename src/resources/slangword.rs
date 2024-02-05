use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse}
};

#[derive(Template)]
#[template(path = "slangword.html")]
struct SlangWordTemplate;

pub async fn get_slang_word() -> impl IntoResponse {
    let template = SlangWordTemplate;

    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to render template. Error: {err}"),
        )
            .into_response(),
    }
}