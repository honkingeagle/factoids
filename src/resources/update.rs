use crate::SharedState;
use axum::{
    extract::{Form, Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};

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
) -> impl IntoResponse {
    let query = sqlx::query(
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
    .await;

    match query {
        Ok(_) => {
            let uri = format!("/slangword/{}", id);

            Redirect::to(&uri).into_response()
        }
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Err {err}")).into_response(),
    }
}
