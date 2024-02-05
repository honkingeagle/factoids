use sqlx::PgPool;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] _pool: PgPool
) -> shuttle_axum::ShuttleAxum {
    let router = factoids::create_router();

    Ok(router.into())
}
