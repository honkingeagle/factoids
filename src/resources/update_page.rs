use super::{ApiError, SlangWord};
use crate::SharedState;
use askama::Template;
use axum::extract::{Path, State};

#[derive(Template)]
#[template(path = "slangwords/update.html")]
struct UpdateTemplate {
    slangword: SlangWord,
}

impl UpdateTemplate {
    pub fn new(slangword: SlangWord) -> Self {
        Self { slangword }
    }
}

pub async fn go_to_update_page(
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

    let template = UpdateTemplate::new(slang_word);

    let html = template.render()?;

    Ok(html)
}
