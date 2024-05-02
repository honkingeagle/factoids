use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

#[derive(Template)]
#[template(path = "base.html")]
struct HomeTemplate;

pub async fn home() -> impl IntoResponse {
    let template = HomeTemplate;

    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(err) => (
            StatusCode::NOT_FOUND,
            format!("Failed to render template! {err}"),
        )
            .into_response(),
    }
}
