use crate::models::tag::Tag;
use sqlx::SqlitePool;

pub struct TagService {
    pool: SqlitePool,
}

impl TagService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn add_tag(&self, photo_id: i64, tag_name: String) -> Result<(), String> {
        // Logic to add a tag to a photo
        Ok(())
    }

    pub async fn remove_tag(&self, photo_id: i64, tag_id: i64) -> Result<(), String> {
        // Logic to remove a tag from a photo
        Ok(())
    }

    pub async fn get_photo_tags(&self, photo_id: i64) -> Result<Vec<Tag>, String> {
        // Logic to get all tags for a photo
        Ok(vec![])
    }
}