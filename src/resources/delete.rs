use crate::SharedState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
pub async fn delete_slang_word(
    State(state): State<SharedState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let query = sqlx::query(
        r#"
            delete 
            from slangwords
            where id = $1
        "#,
    )
    .bind(id)
    .execute(&state.pool)
    .await;

    match query {
        Ok(_) => Redirect::to("/").into_response(),
        Err(err) => (StatusCode::NOT_FOUND, format!("{err}")).into_response(),
    }
}
