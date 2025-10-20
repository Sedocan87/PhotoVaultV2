use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use std::path::Path;
use log::error;

pub mod manager;

pub async fn init_db(db_path: &Path, migrations_path: &Path) -> Result<SqlitePool, sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(
            sqlx::sqlite::SqliteConnectOptions::new()
                .filename(db_path)
                .create_if_missing(true),
        )
        .await?;

    // Run migrations
    let migrator = sqlx::migrate::Migrator::new(migrations_path).await;

    match migrator {
        Ok(m) => {
            if let Err(e) = m.run(&pool).await {
                error!("Failed to run migrations: {}", e);
                return Err(e.into());
            }
        }
        Err(e) => {
            error!("Failed to read migrations: {}", e);
            return Err(sqlx::Error::from(e));
        }
    }

    Ok(pool)
}
