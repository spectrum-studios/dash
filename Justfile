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

# Format files
format:
  cargo fmt
