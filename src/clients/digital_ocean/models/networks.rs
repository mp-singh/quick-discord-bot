use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Networks {
    pub v4: Vec<NetworkV4>,
    // Don't see the need for v6 right now
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NetworkV4 {
    pub gateway: Ipv4Addr,
    pub ip_address: Ipv4Addr,
    pub netmask: Ipv4Addr,
    #[serde(rename = "type")]
    pub kind: String,
}
