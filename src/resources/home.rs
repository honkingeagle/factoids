use super::ApiError;
use askama::Template;

#[derive(Template)]
#[template(path = "base.html")]
struct HomeTemplate;

pub async fn home() -> Result<String, ApiError> {
    let template = HomeTemplate;

    let html = template.render()?;

    Ok(html)
}
