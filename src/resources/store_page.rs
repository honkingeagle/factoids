use super::ApiError;
use askama::Template;
use axum::response::Html;

#[derive(Template)]
#[template(path = "slangwords/store.html")]
pub struct StoreTemplate;

pub async fn go_to_store_page() -> Result<Html<String>, ApiError> {
    let template = StoreTemplate;

    let html = template.render()?;

    Ok(Html(html))
}
