use anyhow::Result;
use crate::services::digitalocean::DigitalOceanClient;
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
    deployment_receiver: Option<mpsc::UnboundedReceiver<DeploymentMessage>>,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: AppState::Welcome,
            should_quit: false,
            do_client: None,
            tailscale_auth_key: None,
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
                    // Store DO client and move to Tailscale auth step
                    self.do_client = Some(DigitalOceanClient::new(token.clone()));
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
        
        tokio::spawn(async move {
            if let (Some(client), Some(auth_key)) = (client, auth_key) {
                let _ = Self::deploy_server_task(client, auth_key, tx).await;
            }
        });

        Ok(())
    }

    async fn deploy_server_task(
        client: DigitalOceanClient,
        auth_key: String,
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
                match client.create_droplet(&auth_key).await {
                    Ok(droplet) => {
                        send_progress(3, "Waiting for server to be ready...".to_string());
                        
                        // Step 3: Wait for server
                        let _server_info = client.wait_for_droplet_ready(droplet.id).await?;
                        
                        send_progress(4, "Installing and configuring Tailscale...".to_string());
                        
                        // Step 4: Tailscale is installed via cloud-init
                        // Wait a bit for the service to start
                        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                        
                        send_progress(5, "Restarting server...".to_string());
                        
                        // Step 5: Restart the droplet
                        client.restart_droplet(droplet.id).await?;
                        
                        send_progress(6, "Waiting for server to come back online...".to_string());
                        
                        // Step 6: Wait for server to be ready after restart
                        let server_info = client.wait_for_droplet_ready(droplet.id).await?;
                        
                        // Track restart progress in real-time
                        Self::track_restart_progress_task(client, droplet.id, tx.clone()).await?;
                        
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

    async fn track_restart_progress_task(
        client: DigitalOceanClient,
        droplet_id: u64,
        tx: mpsc::UnboundedSender<DeploymentMessage>,
    ) -> Result<()> {
        let send_progress = |step: usize, status: String| {
            let _ = tx.send(DeploymentMessage::Progress { step, status });
        };

        // Show detailed restart progress
        send_progress(6, "Initiating server restart...".to_string());
        
        // Wait a moment for the restart to begin
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        
        send_progress(6, "Server is restarting...".to_string());
        
        // Monitor the restart process
        let mut attempts = 0;
        let max_attempts = 60; // 5 minutes maximum
        
        loop {
            attempts += 1;
            
            // Update progress message based on attempts
            let progress_msg = match attempts {
                1..=10 => "Shutting down services...",
                11..=20 => "Restarting server...",
                21..=40 => "Booting up system...",
                41..=50 => "Starting services...",
                _ => "Finalizing restart...",
            };
            
            send_progress(6, progress_msg.to_string());
            
            // Check if droplet is back online
            match client.get_droplet_status(droplet_id).await {
                Ok(status) => {
                    if status == "active" {
                        send_progress(6, "Server restart completed successfully!".to_string());
                        break;
                    }
                }
                Err(_) => {
                    // Droplet might be temporarily unreachable during restart
                    if attempts > max_attempts {
                        return Err(anyhow::anyhow!("Server restart timed out after 5 minutes"));
                    }
                }
            }
            
            if attempts > max_attempts {
                return Err(anyhow::anyhow!("Server restart timed out after 5 minutes"));
            }
            
            // Wait before next check
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
        
        // Additional wait for services to fully initialize
        send_progress(6, "Waiting for services to initialize...".to_string());
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        
        Ok(())
    }

}