use sqlx::SqlitePool;
use std::path::{Path, PathBuf};
use crate::db;

pub struct DatabaseManager {
    pub primary_db: SqlitePool,
    pub backup_db: Option<SqlitePool>,
}

impl DatabaseManager {
    pub async fn initialize(primary_path: PathBuf, backup_path: Option<PathBuf>, migrations_path: &Path) -> Result<Self, sqlx::Error> {
        let primary_db = db::init_db(&primary_path, migrations_path).await?;
        let backup_db = if let Some(backup_path) = backup_path {
            Some(db::init_db(&backup_path, migrations_path).await?)
        } else {
            None
        };

        Ok(DatabaseManager {
            primary_db,
            backup_db,
        })
    }

    pub async fn execute_on_both(&mut self, query: &str) -> Result<(), sqlx::Error> {
        sqlx::query(query).execute(&self.primary_db).await?;
        if let Some(backup_db) = &self.backup_db {
            sqlx::query(query).execute(backup_db).await?;
        }
        Ok(())
    }
}
