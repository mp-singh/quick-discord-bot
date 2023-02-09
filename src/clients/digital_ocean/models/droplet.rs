use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{image::Image, networks::Networks, region::Region};

#[derive(Debug, Serialize, Deserialize)]
pub struct DropletCreate {
    pub name: String,
    pub region: String,
    pub size: String,
    pub image: String,
    pub tags: Vec<String>,
    pub user_data: Option<String>,
    pub ssh_key: Option<Vec<String>>,
    pub backup: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DropletCreateResponse {
    pub droplet: Droplet,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DropletListResponse {
    pub droplets: Vec<Droplet>,
    pub meta: Meta,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Meta {
    pub total: usize,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename = "droplet")]
pub struct Droplet {
    pub id: usize,
    pub name: String,
    pub memory: usize,
    pub vcpus: usize,
    pub disk: usize,
    pub locked: bool,
    pub created_at: DateTime<Utc>,
    pub status: String,
    pub backup_ids: Vec<usize>,
    pub snapshot_ids: Vec<usize>,
    pub features: Vec<String>,
    pub region: Region,
    pub image: Image,
    pub size_slug: String,
    pub networks: Networks,
    pub tags: Vec<String>,
    pub volume_ids: Vec<String>,
}
