use crate::SharedState;
use axum::{
    extract::{Form, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use serde::Deserialize;
use sqlx::Row;

#[derive(Deserialize)]
pub struct NewSlangWord {
    pub word: String,
    pub synonym: String,
    pub description: String,
}

pub async fn create_slang_word(
    State(state): State<SharedState>,
    Form(slangword): Form<NewSlangWord>,
) -> impl IntoResponse {
    let query = sqlx::query(
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
    .await;

    match query {
        Ok(pg_row) => {
            let id: i32 = pg_row.get("id");
            let uri = format!("/slangword/{}", id);

            Redirect::to(&uri).into_response()
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "failed to add slangword, please try again!".to_string(),
        )
            .into_response(),
    }
}
