use crate::models::rename::{RenamePreview, RenameResult};

pub struct RenameService;

impl RenameService {
    pub async fn bulk_rename(
        photo_ids: Vec<i64>,
        pattern: String,
    ) -> Result<Vec<RenameResult>, String> {
        // Logic to bulk rename photos
        Ok(vec![])
    }

    pub async fn preview_bulk_rename(
        photo_ids: Vec<i64>,
        pattern: String,
    ) -> Result<Vec<RenamePreview>, String> {
        // Logic to preview bulk rename
        Ok(vec![])
    }
}
