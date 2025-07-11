use anyhow::{anyhow, Result};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde_json::json;
use std::time::Duration;
use tokio::time::sleep;

use crate::models::*;

const DO_API_BASE: &str = "https://api.digitalocean.com/v2";

#[derive(Clone)]
pub struct DigitalOceanClient {
    client: reqwest::Client,
}

impl DigitalOceanClient {
    pub fn new(token: String) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(30))
            .build()
            .unwrap();

        Self { client }
    }

    pub async fn validate_token(&self) -> Result<()> {
        let response = self
            .client
            .get(&format!("{}/account", DO_API_BASE))
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(anyhow!("Invalid token or insufficient permissions"))
        }
    }

    pub async fn create_droplet(
        &self,
        auth_key: &str,
        region: Option<crate::models::RegionOption>,
    ) -> Result<Droplet> {
        let cloud_init_script = self.generate_cloud_init_script(auth_key);
        let mut droplet_request = DropletRequest::default();

        // Set region if provided
        if let Some(region_option) = region {
            droplet_request.region = region_option.slug;
        }

        // Add cloud-init script
        let payload = json!({
            "name": droplet_request.name,
            "region": droplet_request.region,
            "size": droplet_request.size,
            "image": droplet_request.image,
            "monitoring": droplet_request.monitoring,
            "tags": droplet_request.tags,
            "user_data": cloud_init_script
        });

        let response = self
            .client
            .post(&format!("{}/droplets", DO_API_BASE))
            .json(&payload)
            .send()
            .await?;

        if response.status().is_success() {
            let droplet_response: DropletResponse = response.json().await?;
            Ok(droplet_response.droplet)
        } else {
            let error_text = response.text().await?;
            Err(anyhow!("Failed to create droplet: {}", error_text))
        }
    }

    pub async fn wait_for_droplet_ready(&self, droplet_id: u64) -> Result<ServerInfo> {
        let mut attempts = 0;
        let max_attempts = 60; // 5 minutes with 5-second intervals

        loop {
            let response = self
                .client
                .get(&format!("{}/droplets/{}", DO_API_BASE, droplet_id))
                .send()
                .await?;

            if response.status().is_success() {
                let droplet_response: DropletResponse = response.json().await?;
                let droplet = droplet_response.droplet;

                if droplet.status == "active" {
                    // Find the public IP
                    let public_ip = droplet
                        .networks
                        .v4
                        .iter()
                        .find(|net| net.network_type == "public")
                        .map(|net| net.ip_address.clone())
                        .ok_or_else(|| anyhow!("No public IP found"))?;

                    return Ok(ServerInfo {
                        name: droplet.name,
                        ip: public_ip,
                    });
                }
            }

            attempts += 1;
            if attempts >= max_attempts {
                return Err(anyhow!("Timeout waiting for droplet to be ready"));
            }

            sleep(Duration::from_secs(5)).await;
        }
    }

    fn generate_cloud_init_script(&self, auth_key: &str) -> String {
        format!(
            r#"#cloud-config
    packages:
      - curl
      - wget

    runcmd:
      # Install Tailscale
      - ['sh', '-c', 'curl -fsSL https://tailscale.com/install.sh | sh']

      # Configure IP forwarding
      - ['sh', '-c', 'echo "net.ipv4.ip_forward = 1" | tee -a /etc/sysctl.d/99-tailscale.conf']
      - ['sh', '-c', 'echo "net.ipv6.conf.all.forwarding = 1" | tee -a /etc/sysctl.d/99-tailscale.conf']
      - ['sysctl', '-p', '/etc/sysctl.d/99-tailscale.conf']

      # Clean up any existing machine identity
      # See: https://github.com/tailscale/tailscale/issues/9382
      - ['systemctl', 'stop', 'tailscaled']
      - ['sh', '-c', 'rm -rf /var/lib/tailscale/* || true']

      # Start Tailscale daemon with clean state
      - ['systemctl', 'enable', 'tailscaled']
      - ['systemctl', 'start', 'tailscaled']

      # Wait for daemon to be ready
      - ['sleep', '10']

      # Connect to Tailscale with auth key (with retry logic)
      - ['sh', '-c', 'for i in {{1..10}}; do if tailscale up --reset --force-reauth --auth-key={} --accept-routes --advertise-exit-node; then echo "Tailscale connected successfully on attempt $i"; break; else echo "Attempt $i failed, retrying in 1 second..."; sleep 1; fi; done']

      # Enable SSH access
      - ['tailscale', 'set', '--ssh']

      # Log success
      - ['sh', '-c', 'echo "SUCCESS: Tailscale connected at $(date)" > /var/log/tailscale-success.log']
      - ['sh', '-c', 'tailscale status >> /var/log/tailscale-success.log 2>&1']

    final_message: "Cloud-init complete. Tailscale setup finished."
    "#,
            auth_key
        )
    }
}
