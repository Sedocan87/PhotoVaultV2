use crate::models::album::Album;
use sqlx::SqlitePool;

pub struct AlbumService {
    pool: SqlitePool,
}

impl AlbumService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_album(&self, name: String) -> Result<Album, sqlx::Error> {
        let mut conn = self.pool.acquire().await?;
        let id = sqlx::query("INSERT INTO albums (name) VALUES (?)")
            .bind(&name)
            .execute(&mut *conn)
            .await?
            .last_insert_rowid();

        let album = sqlx::query_as::<_, Album>("SELECT * FROM albums WHERE id = ?")
            .bind(id)
            .fetch_one(&mut *conn)
            .await?;
        Ok(album)
    }

    pub async fn add_photos_to_album(
        &self,
        photo_ids: Vec<i64>,
        album_id: i64,
    ) -> Result<(), sqlx::Error> {
        let mut conn = self.pool.acquire().await?;
        for photo_id in photo_ids {
            sqlx::query("INSERT INTO photo_album (photo_id, album_id) VALUES (?, ?)")
                .bind(photo_id)
                .bind(album_id)
                .execute(&mut *conn)
                .await?;
        }
        Ok(())
    }

    pub async fn remove_photos_from_album(
        &self,
        photo_ids: Vec<i64>,
        album_id: i64,
    ) -> Result<(), sqlx::Error> {
        let mut conn = self.pool.acquire().await?;
        for photo_id in photo_ids {
            sqlx::query("DELETE FROM photo_album WHERE photo_id = ? AND album_id = ?")
                .bind(photo_id)
                .bind(album_id)
                .execute(&mut *conn)
                .await?;
        }
        Ok(())
    }

    pub async fn delete_album(&self, album_id: i64) -> Result<(), sqlx::Error> {
        let mut conn = self.pool.acquire().await?;
        sqlx::query("DELETE FROM photo_album WHERE album_id = ?")
            .bind(album_id)
            .execute(&mut *conn)
            .await?;
        sqlx::query("DELETE FROM albums WHERE id = ?")
            .bind(album_id)
            .execute(&mut *conn)
            .await?;
        Ok(())
    }
}
