pub mod delete;
pub mod get_one;
pub mod home;
pub mod store;
pub mod store_page;
pub mod update;
pub mod update_page;

pub use home::home;
pub use get_one::get_slang_word;
pub use store::create_slang_word;
pub use update::update_slang_word;
pub use store_page::go_to_store_page;
pub use update_page::go_to_update_page;
pub use delete::delete_slang_word;

#[derive(sqlx::FromRow, serde::Deserialize)]
pub struct SlangWord {
    pub id: i32,
    pub word: String,
    pub synonym: String,
    pub description: String,
}
