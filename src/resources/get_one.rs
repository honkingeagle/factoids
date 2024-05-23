use super::{ApiError, SlangWord};
use crate::SharedState;
use askama::Template;
use axum::extract::{Path, State};

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
) -> Result<String, ApiError> {
    let slang_word = sqlx::query_as::<_, SlangWord>(
        r#"
            select * 
            from slangwords 
            where id = $1
        "#,
    )
    .bind(id)
    .fetch_one(&state.pool)
    .await?;

    let template = SlangWordTemplate::new(slang_word);

    let html = template.render()?;

    Ok(html)
}
