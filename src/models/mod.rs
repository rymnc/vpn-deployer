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
pub struct RegionOption {
    pub name: String,
    pub slug: String,
    pub description: String,
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
            image: "ubuntu-24-04-x64".to_string(),
            ssh_keys: vec![],
            monitoring: true,
            tags: vec!["tailscale-vpn".to_string()],
        }
    }
}

impl RegionOption {
    pub fn available_regions() -> Vec<RegionOption> {
        vec![
            RegionOption {
                name: "New York".to_string(),
                slug: "nyc1".to_string(),
                description: "NYC - United States".to_string(),
            },
            RegionOption {
                name: "San Francisco".to_string(),
                slug: "sfo3".to_string(),
                description: "SF - United States".to_string(),
            },
            RegionOption {
                name: "Amsterdam".to_string(),
                slug: "ams3".to_string(),
                description: "AMS - Netherlands".to_string(),
            },
            RegionOption {
                name: "Singapore".to_string(),
                slug: "sgp1".to_string(),
                description: "SG - Singapore".to_string(),
            },
            RegionOption {
                name: "Sydney".to_string(),
                slug: "syd1".to_string(),
                description: "SYD - Australia".to_string(),
            },
        ]
    }
}