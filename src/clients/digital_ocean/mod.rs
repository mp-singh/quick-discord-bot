use reqwest::redirect;
pub use reqwest::Client;

mod droplets;
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
