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
sudo apt install libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  file \
  libxdo-dev \
  libssl-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev
```

Install **Rust** if have not:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Make sure there is an [**SQLite**](https://www.sqlite.org/) or [**PostgreSQL**](https://www.postgresql.org/) database
ready for the server to manage data. Replace `DATABASE_URL` using `sqlite://db_name.sqlite` or
`postgres://username:password@host/db_name` respectively during configuration in later stage.

### Install

Enable nightly version, then install tools and command-line interface:

```sh
rustup install nightly
rustup default nightly
rustup component add rustfmt clippy
rustup target add wasm32-unknown-unknown
cargo install just leptosfmt sqlx-cli tauri-cli trunk
```

### Configuration

Edit environment variables in `.env` in root directory:

```properties
# Authentication token expiry in seconds
AUTH_TOKEN_EXPIRY=1

# Database URL
DATABASE_URL="database_url"

# JWT audience
JWT_AUDIENCE="spectrumstudios.com"

# JWT issuer
JWT_ISSUER="Spectrum Studios"

# JWT secret
JWT_SECRET="yourjwtsecret"

# 16-byte password salt
PASSWORD_SALT="yourpasswordsalt"
```

### Operations

Custom commands are saved in `Justfile` in root directory, and they can be called by `just` command. Typing `just` will
show all available custom commands to use.

## Licensing

**Dash** is under _Apache-2.0 License_.

---

_Made by Spectrum Studios_
