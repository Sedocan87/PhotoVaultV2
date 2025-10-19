use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FilterCriteria {
    pub date_from: Option<DateTime<Utc>>,
    pub date_to: Option<DateTime<Utc>>,
    pub min_width: Option<u32>,
    pub min_height: Option<u32>,
    pub tags: Option<Vec<i64>>,
    pub albums: Option<Vec<i64>>,
    pub query: Option<String>,
}
