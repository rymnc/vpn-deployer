#!/bin/bash
# Tailscale VPN Setup Script (Alternative to cloud-init)
# This script is kept for reference but cloud-init is preferred

set -e

echo "Installing Tailscale..."

# Install Tailscale
curl -fsSL https://pkgs.tailscale.com/stable/ubuntu/jammy.noarmor.gpg | sudo tee /usr/share/keyrings/tailscale-archive-keyring.gpg >/dev/null
curl -fsSL https://pkgs.tailscale.com/stable/ubuntu/jammy.sources | sudo tee /etc/apt/sources.list.d/tailscale.list

sudo apt-get update
sudo apt-get install -y tailscale

# Enable IP forwarding for subnet routing
echo 'net.ipv4.ip_forward = 1' | sudo tee -a /etc/sysctl.conf
echo 'net.ipv6.conf.all.forwarding = 1' | sudo tee -a /etc/sysctl.conf
sudo sysctl -p

# Enable Tailscale service
sudo systemctl enable tailscaled
sudo systemctl start tailscaled

echo "Tailscale installed successfully!"
echo "To connect this server to your tailnet, run:"
echo "sudo tailscale up --advertise-routes=10.0.0.0/8,172.16.0.0/12,192.168.0.0/16 --accept-routes"