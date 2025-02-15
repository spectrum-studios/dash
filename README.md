# Dash

Full-stack development template in Rust

## Crates

**Dash** consists of the following crates:

- `backend`: Application backend using Axum
- `frontend`: Application frontend using Leptos
- `server`: Server-side API using Axum
- `tauri`: Tauri toolkit for building desktop application
- `types`: Common types between crates

All crates have prefix `dash_` to better distinguish with external crates.

## Development

### Setup

**Tauri** recommends to install the following system dependencies:

```sh
sudo apt update
sudo apt install libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

Install **Rust** if have not:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Install

Enable nightly version, then install tools and command-line interface:

```sh
rustup install nightly
rustup default nightly
rustup component add rustfmt clippy
rustup target add wasm32-unknown-unknown
cargo install just leptosfmt sqlx-cli tauri-cli trunk
```

### Operations

Custom commands are saved in `Justfile` in root directory, and they can be called by `just` command:

```sh
just
```

Typing `just` will show all available custom commands to use.

## Licensing

**Dash** is under _Apache-2.0 License_.

---

_Made by Spectrum Studios_
