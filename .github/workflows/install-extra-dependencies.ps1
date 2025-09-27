Set-StrictMode -Version 'Latest'
$ErrorActionPreference = 'Stop'

# Install npm modules
npm install

# Install tauri CLI
cargo install tauri-cli --version "^2.0.0" --locked
