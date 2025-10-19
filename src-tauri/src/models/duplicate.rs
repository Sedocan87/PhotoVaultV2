use crate::models::photo::Photo;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateGroup {
    pub hash: String,
    pub photos: Vec<Photo>,
    pub size: u64,
}
