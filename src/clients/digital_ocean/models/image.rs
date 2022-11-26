use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Image {
    id: usize,
    name: String,
    #[serde(rename = "type")]
    kind: String,
    distribution: String,
    slug: Option<String>,
    public: bool,
    regions: Vec<String>,
    min_disk_size: usize,
    size_gigabytes: Option<f32>,
    created_at: DateTime<Utc>,
}
