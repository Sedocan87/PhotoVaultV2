use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreSummary {
    pub missing_files: i64,
    pub different_files: i64,
}
