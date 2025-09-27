#!/bin/bash
set -euo pipefail

# Install dependencies
sudo apt update
sudo apt install -y \
    build-essential \
    curl \
    file \
    librsvg2-dev \
    libssl-dev \
    libwebkit2gtk-4.1-dev \
    libxdo-dev \
    libayatana-appindicator3-dev \
    wget

# Install npm modules
npm install

# Install tauri CLI
cargo install tauri-cli --version "^2.0.0" --locked
