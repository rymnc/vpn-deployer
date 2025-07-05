# VPN Deployer

A simple, interactive tool to deploy your own VPN server on DigitalOcean using Tailscale.

## Quick Install

```bash
curl -fsSL vpn-deployer.rymnc.com | sh
```

## What You Need

- A [DigitalOcean](https://digitalocean.com) account and API token
- A [Tailscale](https://tailscale.com) account and auth key

## Features

- 🚀 **One-Line Install**: Simple curl command installation
- 🎯 **Non-Technical Friendly**: Interactive terminal interface
- 💰 **Cost-Effective**: Uses $4/month DigitalOcean droplet
- 🔒 **Secure**: Automatic Tailscale setup with exit node
- 📱 **Cross-Platform**: Works on macOS, Linux, and Windows
- ⚡ **Real-time Progress**: Live deployment tracking

## Manual Installation

Download the latest binary for your platform from the [releases page](https://github.com/rymnc/vpn-deployer/releases).

### Linux/macOS
```bash
# Download and extract
wget https://github.com/rymnc/vpn-deployer/releases/latest/download/vpn-deployer-linux-amd64.tar.gz
tar -xzf vpn-deployer-linux-amd64.tar.gz

# Move to PATH
sudo mv vpn-deployer /usr/local/bin/
```

### Windows
Download `vpn-deployer-windows-amd64.zip` from releases and extract the .exe file.

## Development

```bash
# Clone and build
git clone https://github.com/rymnc/vpn-deployer.git
cd vpn-deployer
cargo run
```

## Usage

1. Run the application:
   ```bash
   cargo run
   ```

2. Follow the step-by-step prompts:
   - Enter your DigitalOcean API token
   - Wait for server creation and configuration
   - Connect your devices to the VPN

## What It Does

1. **Validates** your DigitalOcean API credentials
2. **Creates** a $4/month Ubuntu server in NYC region
3. **Installs** Tailscale using cloud-init
4. **Configures** the server as a VPN exit node
5. **Provides** connection instructions

## Getting Your API Token

1. Go to [DigitalOcean API page](https://cloud.digitalocean.com/account/api)
2. Click "Generate New Token"
3. Give it a name and select "Read" and "Write" permissions
4. Copy the token and paste it in the application

## Architecture

- **Rust** with async/await for performance
- **Ratatui** for beautiful terminal UI
- **Reqwest** for DigitalOcean API calls
- **Cloud-init** for automated server setup
- **Tailscale** for secure VPN networking

## Development

```bash
# Check code
cargo check

# Run in development
cargo run

# Build for release
cargo build --release
```

## Security

- API tokens are not stored permanently
- All communication uses HTTPS
- Server uses latest Ubuntu LTS with automatic updates
- Tailscale provides end-to-end encryption

## Cost

- **Server**: $4/month (DigitalOcean s-1vcpu-512mb-10gb)
- **Tailscale**: Free for personal use
- **Total**: ~$4/month (~$0.006/hour)

## License

MIT License - see LICENSE file for details