use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenameResult {
    pub photo_id: i64,
    pub old_name: String,
    pub new_name: String,
    pub success: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenamePreview {
    pub photo_id: i64,
    pub old_name: String,
    pub new_name: String,
}
