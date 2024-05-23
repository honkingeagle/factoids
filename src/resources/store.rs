use crate::SharedState;
use axum::{
    extract::{Form, State},
    response::Redirect,
};
use serde::Deserialize;
use sqlx::Row;
use super::ApiError;

#[derive(Deserialize)]
pub struct NewSlangWord {
    pub word: String,
    pub synonym: String,
    pub description: String,
}

pub async fn create_slang_word(
    State(state): State<SharedState>,
    Form(slangword): Form<NewSlangWord>,
) -> Result<Redirect, ApiError> {
    let pg_row = sqlx::query(
        r#"
            insert into slangwords 
            (word, synonym, description) values ($1, $2, $3)
            returning id
        "#,
    )
    .bind(slangword.word)
    .bind(slangword.synonym)
    .bind(slangword.description)
    .fetch_one(&state.pool)
    .await?;

    let id: i32 = pg_row.get("id");
    let uri = format!("/slangword/{}", id);

    Ok(Redirect::to(&uri))
}
