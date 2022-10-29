use serde::{Deserialize, Serialize};

use super::image::Image;

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
