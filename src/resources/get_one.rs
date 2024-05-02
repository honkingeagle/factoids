use super::SlangWord;
use crate::SharedState;
use askama::Template;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
};

#[derive(Template)]
#[template(path = "slangwords/show.html")]
struct SlangWordTemplate {
    pub slang_word: SlangWord,
}

impl SlangWordTemplate {
    pub fn new(slang_word: SlangWord) -> Self {
        Self { slang_word }
    }
}

pub async fn get_slang_word(
    State(state): State<SharedState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let query = sqlx::query_as::<_, SlangWord>(
        r#"
            select * 
            from slangwords 
            where id = $1
        "#,
    )
    .bind(id)
    .fetch_one(&state.pool)
    .await;

    match query {
        Ok(slang_word) => {
            let template = SlangWordTemplate::new(slang_word);

            match template.render() {
                Ok(html) => Html(html).into_response(),
                Err(err) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to render template! {err}"),
                )
                    .into_response(),
            }
        }
        Err(err) => (StatusCode::NOT_FOUND, format!("{err}")).into_response(),
    }
}
