use crate::SharedState;
use axum::{
    extract::{Form, State, Path},
    response::Redirect,
};
use super::ApiError;


#[derive(sqlx::FromRow, serde::Deserialize)]
pub struct SlangWord {
    pub word: String,
    pub synonym: String,
    pub description: String,
}

pub async fn update_slang_word(
    State(state): State<SharedState>,
    Path(id): Path<i32>,
    Form(slangword): Form<SlangWord>,
) -> Result<Redirect, ApiError> {
    sqlx::query(
        r#"
            update slangwords 
            set word = $1, synonym = $2, description = $3
            where id = $4
        "#,
    )
    .bind(slangword.word)
    .bind(slangword.synonym)
    .bind(slangword.description)
    .bind(id)
    .execute(&state.pool)
    .await?;

    let uri = format!("/slangword/{}", id);

    Ok(Redirect::to(&uri))
}
