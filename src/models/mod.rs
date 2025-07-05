use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropletRequest {
    pub name: String,
    pub region: String,
    pub size: String,
    pub image: String,
    pub ssh_keys: Vec<String>,
    pub monitoring: bool,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Droplet {
    pub id: u64,
    pub name: String,
    pub status: String,
    pub networks: Networks,
    pub region: Region,
    pub size: Size,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Networks {
    pub v4: Vec<NetworkV4>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkV4 {
    pub ip_address: String,
    pub netmask: String,
    #[serde(rename = "type")]
    pub network_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Region {
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Size {
    pub slug: String,
    pub memory: u32,
    pub vcpus: u32,
    pub disk: u32,
    pub price_monthly: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropletResponse {
    pub droplet: Droplet,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub email: String,
    pub uuid: String,
    pub email_verified: bool,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountResponse {
    pub account: Account,
}

#[derive(Debug, Clone)]
pub struct ServerInfo {
    pub name: String,
    pub ip: String,
}

impl Default for DropletRequest {
    fn default() -> Self {
        Self {
            name: format!("tailscale-vpn-{}", uuid::Uuid::new_v4().to_string()[..8].to_string()),
            region: "nyc1".to_string(),
            size: "s-1vcpu-512mb-10gb".to_string(), // Cheapest option
            image: "ubuntu-22-04-x64".to_string(),
            ssh_keys: vec![],
            monitoring: true,
            tags: vec!["tailscale-vpn".to_string()],
        }
    }
}