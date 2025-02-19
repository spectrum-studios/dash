# View all available commands
default:
  just --list

# Generate Tauri icons
icon:
  cargo-tauri icon tauri/icons/icon.png

# Start development on application
dev:
  cargo tauri dev

# Build application
build:
  cargo tauri build

# Start server
server:
  cargo run --bin dash_server

# Build crates
crate:
  cargo build -p dash_types
  cargo build -p dash_server
  cargo build -p dash_backend
  cargo build -p dash_frontend
  cargo build -p dash_tauri

# Format files
format:
  cargo +nightly fmt --all