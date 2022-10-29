use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::models::{image::Image, networks::Networks, region::Region};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Droplet {
    id: usize,
    name: String,
    memory: usize,
    vcpus: usize,
    disk: usize,
    locked: bool,
    created_at: DateTime<Utc>,
    status: String,
    backup_ids: Vec<usize>,
    snapshot_ids: Vec<usize>,
    features: Vec<String>,
    region: Region,
    image: Image,
    size: String,
    size_slug: String,
    networks: Networks,
    tags: Vec<String>,
    volume_ids: Vec<String>,
}

impl Droplet {
    pub fn create() -> Result<Droplet, String> {
        unimplemented!("work in progress")
    }

    pub fn delete(name: &str) -> Result<(), String> {
        unimplemented!("work in progress")
    }

    pub fn list_all(tags: Option<Vec<&str>>) -> Result<Vec<Droplet>, String> {
        unimplemented!("work in progress")
    }
    pub fn list_by_name(name: &str) {
        unimplemented!("work in progress")
    }
}
