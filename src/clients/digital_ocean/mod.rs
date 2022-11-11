use std::str::FromStr;

use reqwest::redirect;
pub use reqwest::Client;

use crate::lazy_statics::DO_TOKEN;

use self::models::droplet::{Droplet, DropletCreate};

mod models;

#[derive(Debug)]
pub struct DigitalOcean {
    client: reqwest::Client,
    token: String,
}

impl DigitalOcean {
    pub fn builder() -> DigitalOceanClientBuiler {
        DigitalOceanClientBuiler::default()
    }

    pub async fn create_droplet(&self, new: DropletCreate) -> Result<Droplet, String> {
        println!("made it into create");
        self.client
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
            .map_err(|e| format!("unable to make request to create droplet {:?}", e))
    }

    pub async fn delete_droplet(&self, name: &str) -> Result<(), String> {
        let response = self
            .client
            .request(
                reqwest::Method::DELETE,
                reqwest::Url::from_str("https://api.digitalocean.com/v2/droplets").unwrap(),
            )
            .query(&[("tag_name", name)])
            .bearer_auth(DO_TOKEN.as_str())
            .send()
            .await;

        match response {
            Ok(r) => {
                if r.status() == 204 {
                    Ok(())
                } else {
                    Err(format!(
                        "failed to delete by tag. recieved code: {:?}",
                        r.status().as_str()
                    ))
                }
            }
            Err(e) => Err(e.to_string()),
        }
    }

    pub async fn list_droplets_by_tag_name(&self, name: &str) -> Result<Vec<Droplet>, String> {
        self.client
            .request(
                reqwest::Method::GET,
                reqwest::Url::from_str("https://api.digitalocean.com/v2/droplets").unwrap(),
            )
            .query(&[("tag_name", name)])
            .bearer_auth(DO_TOKEN.as_str())
            .send()
            .await
            .unwrap()
            .json::<Vec<Droplet>>()
            .await
            .map_err(|e| format!("unable to make request to create droplet {:?}", e))
    }
}

#[derive(Default)]
pub struct DigitalOceanClientBuiler {
    client: reqwest::Client,
    token: String,
}

impl DigitalOceanClientBuiler {
    pub fn new() -> DigitalOceanClientBuiler {
        DigitalOceanClientBuiler {
            client: reqwest::Client::builder()
                .redirect(redirect::Policy::none())
                .build()
                .unwrap(),
            ..Default::default()
        }
    }

    pub fn token(mut self, token: String) -> DigitalOceanClientBuiler {
        self.token = token;
        self
    }

    pub fn client(mut self, client: reqwest::Client) -> DigitalOceanClientBuiler {
        self.client = client;
        self
    }

    pub fn build(self) -> DigitalOcean {
        DigitalOcean {
            token: self.token,
            client: self.client,
        }
    }
}
