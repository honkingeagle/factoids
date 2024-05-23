use super::{ApiError, SlangWord};
use crate::SharedState;
use askama::Template;
use axum::extract::{Query, State};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Pagination {
    pub page: i32,
    pub limit: i32,
}

impl Default for Pagination {
    fn default() -> Pagination {
        Pagination { page: 1, limit: 5 }
    }
}

#[derive(Template)]
#[template(path = "slangwords/index.html")]
struct IndexTemplate {
    slang_words: Vec<SlangWord>,
    page: i32,
}

impl IndexTemplate {
    pub fn new(slang_words: Vec<SlangWord>, page: i32) -> IndexTemplate {
        IndexTemplate { slang_words, page }
    }
}

pub async fn get_slang_words(
    State(state): State<SharedState>,
    paginator: Option<Query<Pagination>>,
) -> Result<String, ApiError> {
    let Query(paginator) = paginator.unwrap_or_default();

    let offset = (paginator.page - 1) * paginator.limit;

    let slang_words = sqlx::query_as::<_, SlangWord>(
        r#"
        select * 
        from slangwords
        order by word
        limit $1 offset $2
        "#,
    )
    .bind(paginator.limit)
    .bind(offset)
    .fetch_all(&state.pool)
    .await?;

    let template = IndexTemplate::new(slang_words, paginator.page + 1);

    let html = template.render()?;

    Ok(html)
}
