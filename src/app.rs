use anyhow::Result;
use crate::services::digitalocean::DigitalOceanClient;
use crate::models::RegionOption;
use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub enum DeploymentMessage {
    Progress { step: usize, status: String },
    Complete { server_info: ServerInfo },
    Error { message: String },
}

#[derive(Debug, Clone)]
pub enum AppState {
    Welcome,
    Auth { token: String, cursor: usize },
    RegionSelect { selected_index: usize },
    TailscaleAuth { auth_key: String, cursor: usize },
    Loading { message: String },
    Deploy { progress: DeployProgress },
    Complete { server_info: ServerInfo },
    Error { message: String },
}

#[derive(Debug, Clone)]
pub struct DeployProgress {
    pub current_step: usize,
    pub total_steps: usize,
    pub status: String,
}


#[derive(Debug, Clone)]
pub struct ServerInfo {
    pub name: String,
    pub ip: String,
    pub cost: String,
}

pub struct App {
    pub state: AppState,
    pub should_quit: bool,
    do_client: Option<DigitalOceanClient>,
    tailscale_auth_key: Option<String>,
    pub selected_region: Option<RegionOption>,
    deployment_receiver: Option<mpsc::UnboundedReceiver<DeploymentMessage>>,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: AppState::Welcome,
            should_quit: false,
            do_client: None,
            tailscale_auth_key: None,
            selected_region: None,
            deployment_receiver: None,
        }
    }

    pub async fn handle_enter(&mut self) -> Result<()> {
        match &self.state {
            AppState::Welcome => {
                self.state = AppState::Auth {
                    token: String::new(),
                    cursor: 0,
                };
            }
            AppState::Auth { token, .. } => {
                if !token.is_empty() {
                    // Store DO client and move to region selection
                    self.do_client = Some(DigitalOceanClient::new(token.clone()));
                    self.state = AppState::RegionSelect {
                        selected_index: 0,
                    };
                }
            }
            AppState::RegionSelect { selected_index } => {
                let regions = RegionOption::available_regions();
                if let Some(region) = regions.get(*selected_index) {
                    self.selected_region = Some(region.clone());
                    self.state = AppState::TailscaleAuth {
                        auth_key: String::new(),
                        cursor: 0,
                    };
                }
            }
            AppState::TailscaleAuth { auth_key, .. } => {
                if !auth_key.is_empty() {
                    self.tailscale_auth_key = Some(auth_key.clone());
                    self.state = AppState::Loading {
                        message: "Deploying your VPN server...".to_string(),
                    };
                    self.start_deployment().await?;
                }
            }
            AppState::Complete { .. } => {
                self.should_quit = true;
            }
            AppState::Error { .. } => {
                self.state = AppState::Welcome;
            }
            _ => {}
        }
        Ok(())
    }

    pub fn handle_tab(&mut self) {
        // Handle tab navigation between UI elements
    }

    pub fn handle_back_tab(&mut self) {
        // Handle shift+tab navigation
    }

    pub fn handle_char(&mut self, c: char) {
        match &mut self.state {
            AppState::Auth { token, cursor } => {
                token.insert(*cursor, c);
                *cursor += 1;
            }
            AppState::RegionSelect { selected_index } => {
                let regions = RegionOption::available_regions();
                match c {
                    'j' | 's' => {
                        if *selected_index < regions.len() - 1 {
                            *selected_index += 1;
                        }
                    }
                    'k' | 'w' => {
                        if *selected_index > 0 {
                            *selected_index -= 1;
                        }
                    }
                    _ => {}
                }
            }
            AppState::TailscaleAuth { auth_key, cursor } => {
                auth_key.insert(*cursor, c);
                *cursor += 1;
            }
            _ => {}
        }
    }

    pub fn handle_backspace(&mut self) {
        match &mut self.state {
            AppState::Auth { token, cursor } => {
                if *cursor > 0 {
                    token.remove(*cursor - 1);
                    *cursor -= 1;
                }
            }
            AppState::TailscaleAuth { auth_key, cursor } => {
                if *cursor > 0 {
                    auth_key.remove(*cursor - 1);
                    *cursor -= 1;
                }
            }
            _ => {}
        }
    }

    pub fn handle_up(&mut self) {
        match &mut self.state {
            AppState::RegionSelect { selected_index } => {
                if *selected_index > 0 {
                    *selected_index -= 1;
                }
            }
            _ => {}
        }
    }

    pub fn handle_down(&mut self) {
        match &mut self.state {
            AppState::RegionSelect { selected_index } => {
                let regions = RegionOption::available_regions();
                if *selected_index < regions.len() - 1 {
                    *selected_index += 1;
                }
            }
            _ => {}
        }
    }

    pub async fn tick(&mut self) -> Result<()> {
        // Handle messages from deployment task
        if let Some(receiver) = &mut self.deployment_receiver {
            if let Ok(message) = receiver.try_recv() {
                match message {
                    DeploymentMessage::Progress { step, status } => {
                        self.state = AppState::Deploy {
                            progress: DeployProgress {
                                current_step: step,
                                total_steps: 6,
                                status,
                            },
                        };
                    }
                    DeploymentMessage::Complete { server_info } => {
                        self.state = AppState::Complete { server_info };
                        self.deployment_receiver = None;
                    }
                    DeploymentMessage::Error { message } => {
                        self.state = AppState::Error { message };
                        self.deployment_receiver = None;
                    }
                }
            }
        }
        Ok(())
    }

    async fn start_deployment(&mut self) -> Result<()> {
        let (tx, rx) = mpsc::unbounded_channel();
        self.deployment_receiver = Some(rx);

        let client = self.do_client.clone();
        let auth_key = self.tailscale_auth_key.clone();
        let region = self.selected_region.clone();
        
        tokio::spawn(async move {
            if let (Some(client), Some(auth_key)) = (client, auth_key) {
                let _ = Self::deploy_server_task(client, auth_key, region, tx).await;
            }
        });

        Ok(())
    }

    async fn deploy_server_task(
        client: DigitalOceanClient,
        auth_key: String,
        region: Option<RegionOption>,
        tx: mpsc::UnboundedSender<DeploymentMessage>,
    ) -> Result<()> {
        let send_progress = |step: usize, status: String| {
            let _ = tx.send(DeploymentMessage::Progress { step, status });
        };

        // Step 1: Validate credentials
        send_progress(1, "Validating credentials...".to_string());
        
        match client.validate_token().await {
            Ok(_) => {
                send_progress(2, "Creating server...".to_string());
                
                // Step 2: Create droplet
                match client.create_droplet(&auth_key, region).await {
                    Ok(droplet) => {
                        send_progress(3, "Waiting for server to be ready...".to_string());
                        
                        // Step 3: Wait for server
                        let _server_info = client.wait_for_droplet_ready(droplet.id).await?;
                        
                        send_progress(4, "Installing and configuring Tailscale...".to_string());
                        
                        // Step 4: Wait for Tailscale setup to complete
                        // The cloud-init script will handle the entire setup process
                        Self::wait_for_tailscale_setup(tx.clone()).await?;
                        
                        send_progress(5, "Finalizing server setup...".to_string());
                        
                        // Step 5: Get final server info
                        let server_info = client.wait_for_droplet_ready(droplet.id).await?;
                        
                        // Complete setup
                        let _ = tx.send(DeploymentMessage::Complete {
                            server_info: ServerInfo {
                                name: server_info.name,
                                ip: server_info.ip,
                                cost: "$4/month".to_string(),
                            },
                        });
                    }
                    Err(e) => {
                        let _ = tx.send(DeploymentMessage::Error {
                            message: format!("Failed to create server: {}", e),
                        });
                    }
                }
            }
            Err(e) => {
                let _ = tx.send(DeploymentMessage::Error {
                    message: format!("Invalid credentials: {}", e),
                });
            }
        }

        Ok(())
    }

    async fn wait_for_tailscale_setup(
        tx: mpsc::UnboundedSender<DeploymentMessage>,
    ) -> Result<()> {
        let send_progress = |step: usize, status: String| {
            let _ = tx.send(DeploymentMessage::Progress { step, status });
        };

        // Show detailed setup progress
        let setup_steps = vec![
            "Downloading and installing Tailscale...",
            "Configuring IP forwarding...",
            "Starting Tailscale daemon...",
            "Connecting to your Tailnet...",
            "Enabling SSH access...",
            "Configuring as exit node...",
            "Finalizing setup...",
        ];
        
        for (i, step_msg) in setup_steps.iter().enumerate() {
            send_progress(4, step_msg.to_string());
            
            // Wait time varies based on step complexity
            let wait_time = match i {
                0 => 15, // Installing takes longest
                1 => 5,  // IP forwarding config
                2 => 10, // Starting daemon
                3 => 15, // Connecting to Tailnet
                4 => 5,  // SSH setup
                5 => 5,  // Exit node config
                _ => 10, // Final steps
            };
            
            tokio::time::sleep(tokio::time::Duration::from_secs(wait_time)).await;
        }
        
        send_progress(4, "Tailscale setup completed successfully!".to_string());
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        
        Ok(())
    }

}