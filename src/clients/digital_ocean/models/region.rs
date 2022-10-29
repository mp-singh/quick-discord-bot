use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Region {
    name: String,
    slug: String,
    sizes: Vec<String>,
    available: bool,
    features: Vec<String>,
}
