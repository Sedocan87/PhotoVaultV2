use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use std::path::Path;

pub mod manager;

pub async fn init_db(db_path: &Path) -> Result<SqlitePool, sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(
            sqlx::sqlite::SqliteConnectOptions::new()
                .filename(db_path)
                .create_if_missing(true),
        )
        .await?;

    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}