# FlagDrive Backend

The backend service of FlagDrive is for the ENOWARS10 "Hack the Government" Attack/Defense CTF. It is built using the [Axum](https://github.com/tokio-rs/axum) web framework, powered by [Tokio](https://tokio.rs/), and uses [SQLx](https://github.com/launchbadge/sqlx) with an [SQLite](https://sqlite.org/) database.

It serves both the JSON API endpoints for user/file operations and the static compiled files of the frontend single-page application (SPA).

## Prerequisites

To build and run this backend locally, you will need the following tools:

1. **Rust & Cargo**: The Rust toolchain.
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

## Configuration & CLI Arguments

The backend can be configured using command-line arguments:

```bash
cargo run -- --help
```

### Available Arguments:
* `-d`, `--database <PATH>`: The path to the SQLite database file (defaults to `flagdrive.db`).
* `-a`, `--addr <IP:PORT>`: The socket address to bind the server to (defaults to `0.0.0.0:4859`).
* `--dist <PATH>`: The path to serve the frontend static files from (defaults to `./dist`).

## Development & Running

The backend utilizes built-in SQLx migrations to automatically set up the database schema on startup, so no manual database initialization is required.

**To run the backend in development mode:**
```bash
cargo run -- --database flagdrive.db --addr 127.0.0.1:4859
```
The backend API server will start on `http://127.0.0.1:4859`.

**To run in release (production) mode:**
```bash
cargo run --release -- --database flagdrive.db --addr 0.0.0.0:4859
```

## Project Structure

* `src/main.rs`: Entrypoint of the Axum server that parses arguments, configures CORS, sets up static file routing, and spawns the tokio runtime.
* `src/api_routes/`: API endpoint handlers (Auth, Files, GDPR export, User profiles).
* `src/database.rs`: SQLite database interaction and schema helpers.
* `migrations/`: Database SQL migration files.
