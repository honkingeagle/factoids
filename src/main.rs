use factoids::AppStateBuilder;
use sqlx::PgPool;
use std::{process, sync::Arc};

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!().run(&pool).await.unwrap_or_else(|err| {
        eprintln!("Unable to migrate sql files: {err}");
        process::exit(1);
    });

    let state = Arc::new(AppStateBuilder::new().pool(pool).build());

    let router = factoids::create_router(state);

    Ok(router.into())
}
