use rust_embed::RustEmbed;

pub enum Deployment {
    HelloWorld,
}

impl ToString for Deployment {
    fn to_string(&self) -> String {
        use Deployment::*;
        match self {
            HelloWorld => "helloworld",
        }
        .to_string()
    }
}

impl Deployment {
    pub fn get_cloud_config(&self) -> String {
        std::str::from_utf8(
            Assets::get(&format!("{}.yaml", self.to_string()))
                .unwrap()
                .data
                .as_ref(),
        )
        .unwrap()
        .to_string()
    }
}

#[derive(RustEmbed)]
#[folder = "src/clients/digital_ocean/deployments/cloud-init"]
struct Assets;
