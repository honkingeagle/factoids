use super::ApiError;
use crate::SharedState;
use axum::{
    extract::{Path, State},
    response::Redirect,
};

pub async fn delete_slang_word(
    State(state): State<SharedState>,
    Path(id): Path<i32>,
) -> Result<Redirect, ApiError> {
    sqlx::query(
        r#"
            delete 
            from slangwords
            where id = $1
        "#,
    )
    .bind(id)
    .execute(&state.pool)
    .await?;

    Ok(Redirect::to("/"))
}
