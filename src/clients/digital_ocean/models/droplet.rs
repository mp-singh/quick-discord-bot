use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{image::Image, networks::Networks, region::Region};

#[derive(Debug, Serialize, Deserialize)]
pub struct DropletCreate {
    name: String,
    region: String,
    size: String,
    image: Image,
    tags: Vec<String>,
    ssh_key: Option<Vec<String>>,
    backup: Option<bool>,
}

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
