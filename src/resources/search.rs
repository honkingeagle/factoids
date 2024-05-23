use crate::SharedState;
use askama::Template;
use axum::{
    extract::{Form, State},
    response::Html,
};
use serde::Deserialize;
use super::ApiError;

#[derive(Deserialize)]
pub struct SlangWord {
    pub word: String,
}

#[derive(sqlx::FromRow)]
pub struct SearchedWords {
    pub id: i32,
    pub word: String,
}

#[derive(Template)]
#[template(path = "slangwords/search.html")]
pub struct SearchTemplate {
    pub searched_word: String,
    pub words: Vec<SearchedWords>,
}

impl SearchTemplate {
    pub fn new(words: Vec<SearchedWords>, searched_word: String) -> SearchTemplate {
        SearchTemplate {
            words,
            searched_word,
        }
    }
}

pub async fn search_slang_word(
    State(state): State<SharedState>,
    Form(slangword): Form<SlangWord>,
) -> Result<Html<String>, ApiError> {
    let slangwords = sqlx::query_as::<_, SearchedWords>(
        r#"
        select id, word
        from slangwords
        where to_tsvector(word) @@ to_tsquery($1)
        "#,
    )
    .bind(&slangword.word)
    .fetch_all(&state.pool)
    .await?;

    let template = SearchTemplate::new(slangwords, slangword.word);

    let html = template.render()?;

    Ok(Html(html))
}
