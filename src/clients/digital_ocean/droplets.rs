use std::str::FromStr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::lazy_statics::{DO_TOKEN, REQUEST};

use super::models::{
    droplet_create::DropletCreate, image::Image, networks::Networks, region::Region,
};

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
    pub async fn create(new: DropletCreate) -> Result<Droplet, String> {
        REQUEST
            .request(
                reqwest::Method::POST,
                reqwest::Url::from_str("https://api.digitalocean.com/v2/droplets").unwrap(),
            )
            .bearer_auth(DO_TOKEN.as_str())
            .json(&new)
            .send()
            .await
            .unwrap()
            .json::<Droplet>()
            .await
            .map_err(|e| format!("unable to makke request to create droplet {:?}", e))
    }

    pub async fn delete(name: &str) -> Result<(), String> {
        unimplemented!("work in progress")
    }

    pub async fn list_all(tags: Option<Vec<&str>>) -> Result<Vec<Droplet>, String> {
        unimplemented!("work in progress")
    }
    pub async fn list_by_name(name: &str) {
        unimplemented!("work in progress")
    }
}
