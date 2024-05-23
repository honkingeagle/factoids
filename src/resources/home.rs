use super::ApiError;
use askama::Template;
use axum::response::Html;

#[derive(Template)]
#[template(path = "base.html")]
struct HomeTemplate;

pub async fn home() -> Result<Html<String>, ApiError> {
    let template = HomeTemplate;

    let html = template.render()?;

    Ok(Html(html))
}
