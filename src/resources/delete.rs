use axum::{
    extract::Path
};

pub async fn delete_slang_word(Path(_id): Path<i32>) {}
