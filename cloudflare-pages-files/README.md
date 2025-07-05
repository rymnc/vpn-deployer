# VPN Deployer Install Page

This repository contains the static files for hosting the VPN Deployer install script on Cloudflare Pages.

## Files

- `index.html` - Landing page with install instructions
- `install.sh` - The installation script
- `_headers` - Cloudflare headers configuration
- `_redirects` - URL redirect rules

## Setup

1. Create a new repository called `vpn-deployer-install`
2. Copy these files to the repository
3. Push to GitHub
4. Connect to Cloudflare Pages
5. Configure custom domain `vpn-deployer.rymnc.com`

## Usage

After deployment, users can install with:

```bash
curl -fsSL vpn-deployer.rymnc.com | sh
```

The site will also be accessible via browser at https://vpn-deployer.rymnc.com