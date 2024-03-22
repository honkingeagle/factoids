use super::SlangWord;
use crate::SharedState;
use askama::Template;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{Html, IntoResponse},
};
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
struct HomeTemplate {
    slang_words: Vec<SlangWord>,
}

impl HomeTemplate {
    pub fn new(slang_words: Vec<SlangWord>) -> HomeTemplate {
        HomeTemplate { slang_words }
    }
}

pub async fn home(
    State(state): State<SharedState>,
    paginator: Option<Query<Pagination>>,
) -> impl IntoResponse {
    let Query(paginator) = paginator.unwrap_or_default();

    let offset = (paginator.page - 1) * paginator.limit;

    let query = sqlx::query_as::<_, SlangWord>(
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
    .await;

    match query {
        Ok(slang_words) => {
            let template = HomeTemplate::new(slang_words);

            match template.render() {
                Ok(html) => Html(html).into_response(),
                Err(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to render template!"),
                )
                    .into_response(),
            }
        }
        Err(_) => (
            StatusCode::NOT_FOUND,
            format!("No other slangwords available!"),
        )
            .into_response(),
    }
}
