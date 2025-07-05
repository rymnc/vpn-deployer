#!/bin/bash

# VPN Deployer Install Script
# Usage: curl -fsSL vpn-deployer.rymnc.com | sh

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# GitHub repository
REPO="rymnc/vpn-deployer"
BINARY_NAME="vpn-deployer"

# Detect OS and architecture
detect_platform() {
    local os=$(uname -s | tr '[:upper:]' '[:lower:]')
    local arch=$(uname -m)
    
    case $os in
        linux*)
            os="linux"
            ;;
        darwin*)
            os="macos"
            ;;
        msys*|mingw*|cygwin*)
            os="windows"
            ;;
        *)
            echo -e "${RED}Error: Unsupported operating system: $os${NC}"
            exit 1
            ;;
    esac
    
    case $arch in
        x86_64|amd64)
            arch="amd64"
            ;;
        aarch64|arm64)
            arch="arm64"
            ;;
        *)
            echo -e "${RED}Error: Unsupported architecture: $arch${NC}"
            exit 1
            ;;
    esac
    
    echo "${os}-${arch}"
}

# Get the latest release version from GitHub
get_latest_version() {
    curl -s "https://api.github.com/repos/$REPO/releases/latest" | \
        grep '"tag_name":' | \
        sed -E 's/.*"([^"]+)".*/\1/'
}

# Download and install the binary
install_binary() {
    local platform=$1
    local version=$2
    local ext="tar.gz"
    
    if [[ $platform == *"windows"* ]]; then
        ext="zip"
        BINARY_NAME="${BINARY_NAME}.exe"
    fi
    
    local filename="${BINARY_NAME}-${platform}.${ext}"
    local url="https://github.com/$REPO/releases/download/$version/$filename"
    local tmpdir=$(mktemp -d)
    
    echo -e "${BLUE}Downloading $filename...${NC}"
    
    if ! curl -fsSL "$url" -o "$tmpdir/$filename"; then
        echo -e "${RED}Error: Failed to download $filename${NC}"
        echo -e "${RED}Please check if the release exists at: $url${NC}"
        exit 1
    fi
    
    echo -e "${BLUE}Extracting binary...${NC}"
    cd "$tmpdir"
    
    if [[ $ext == "tar.gz" ]]; then
        tar -xzf "$filename"
    else
        unzip -q "$filename"
    fi
    
    # Find install directory
    local install_dir="/usr/local/bin"
    if [[ ! -w "$install_dir" ]]; then
        install_dir="$HOME/.local/bin"
        mkdir -p "$install_dir"
    fi
    
    echo -e "${BLUE}Installing to $install_dir...${NC}"
    
    # Move binary to install directory
    if [[ -w "$install_dir" ]]; then
        mv "$BINARY_NAME" "$install_dir/"
        chmod +x "$install_dir/$BINARY_NAME"
    else
        echo -e "${YELLOW}Need sudo privileges to install to $install_dir${NC}"
        sudo mv "$BINARY_NAME" "$install_dir/"
        sudo chmod +x "$install_dir/$BINARY_NAME"
    fi
    
    # Clean up
    cd - > /dev/null
    rm -rf "$tmpdir"
    
    echo -e "${GREEN}✓ VPN Deployer installed successfully!${NC}"
    echo -e "${GREEN}✓ Binary location: $install_dir/$BINARY_NAME${NC}"
    
    # Check if install directory is in PATH
    if [[ ":$PATH:" != *":$install_dir:"* ]]; then
        echo -e "${YELLOW}Warning: $install_dir is not in your PATH${NC}"
        echo -e "${YELLOW}Add this to your shell profile (~/.bashrc, ~/.zshrc, etc.):${NC}"
        echo -e "${YELLOW}export PATH=\"$install_dir:\$PATH\"${NC}"
        echo ""
        echo -e "${BLUE}Or run directly with: $install_dir/$BINARY_NAME${NC}"
    else
        echo -e "${GREEN}✓ You can now run: $BINARY_NAME${NC}"
    fi
}

# Main installation function
main() {
    echo -e "${BLUE}VPN Deployer Installation Script${NC}"
    echo -e "${BLUE}=================================${NC}"
    echo ""
    
    # Check for required tools
    for tool in curl tar; do
        if ! command -v $tool &> /dev/null; then
            echo -e "${RED}Error: $tool is required but not installed${NC}"
            exit 1
        fi
    done
    
    if [[ "$(detect_platform)" == *"windows"* ]] && ! command -v unzip &> /dev/null; then
        echo -e "${RED}Error: unzip is required but not installed${NC}"
        exit 1
    fi
    
    local platform=$(detect_platform)
    local version=$(get_latest_version)
    
    if [[ -z "$version" ]]; then
        echo -e "${RED}Error: Could not determine latest version${NC}"
        exit 1
    fi
    
    echo -e "${BLUE}Platform: $platform${NC}"
    echo -e "${BLUE}Latest version: $version${NC}"
    echo ""
    
    install_binary "$platform" "$version"
    
    echo ""
    echo -e "${GREEN}Installation complete!${NC}"
    echo -e "${BLUE}Run 'vpn-deployer --help' to get started${NC}"
}

# Run main function
main "$@"