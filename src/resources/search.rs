use crate::SharedState;
use askama::Template;
use axum::{
    extract::{Form, State},
    http::StatusCode,
    response::{Html, IntoResponse},
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SlangWord {
    pub word: String,
}

#[derive(sqlx::FromRow)]
pub struct SearchedWords {
    pub id: i32,
    pub word: String
}

#[derive(Template)]
#[template(path = "slangwords/search.html")]
pub struct SearchTemplate {
    pub searched_word: String,
    pub words: Vec<SearchedWords>
}



impl SearchTemplate {
    pub fn new(words: Vec<SearchedWords>, searched_word: String) -> SearchTemplate {
        SearchTemplate { words, searched_word }
    }
}

pub async fn search_slang_word(
    State(state): State<SharedState>,
    Form(slangword): Form<SlangWord>,
) -> impl IntoResponse {
    let query = sqlx::query_as::<_, SearchedWords>(
        r#"
        select id, word
        from slangwords
        where to_tsvector(word) @@ to_tsquery($1)
        "#,
    )
    .bind(&slangword.word)
    .fetch_all(&state.pool)
    .await;

    match query {
        Ok(slangwords) => {
            let template = SearchTemplate::new(slangwords, slangword.word);

            match template.render() {
                Ok(html) => Html(html).into_response(),
                Err(err) => (
                    StatusCode::NOT_FOUND,
                    format!("Failed to render template! {err}"),
                )
                    .into_response(),
            }
        }
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{err}")).into_response(),
    }
}
