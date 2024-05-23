use super::ApiError;
use askama::Template;

#[derive(Template)]
#[template(path = "slangwords/store.html")]
pub struct StoreTemplate;

pub async fn go_to_store_page() -> Result<String, ApiError> {
    let template = StoreTemplate;

    let html = template.render()?;

    Ok(html)
}
