# FlagDrive Backend Architecture

The backend service of FlagDrive is built using the [Axum](https://github.com/tokio-rs/axum) web framework, powered by [Tokio](https://tokio.rs/), and uses [SQLx](https://github.com/launchbadge/sqlx) with a [PostgreSQL](https://www.postgresql.org/) database.

It serves both the JSON API endpoints for user/file operations and the static compiled files of the frontend single-page application (SPA).

## Prerequisites

To build and run the backend locally, you will need the following tools:

1. **Rust & Cargo**: The Rust toolchain.
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. **PostgreSQL**: A running PostgreSQL instance (or via Docker Compose).

## Configuration & CLI Arguments

The backend can be configured using command-line arguments or the `DATABASE_URL` environment variable:

```bash
cargo run -- --help
```

### Available Arguments:
* `-d`, `--database <URL>`: Database connection string / URI.
* `--pg-host <HOST>`: PostgreSQL host address (e.g. `localhost`).
* `--pg-port <PORT>`: PostgreSQL port (defaults to `5432`).
* `--pg-user <USER>`: PostgreSQL user (defaults to `flagdrive`).
* `--pg-password <PASSWORD>`: PostgreSQL password (defaults to `flagdrivepassword`).
* `--pg-dbname <DBNAME>`: PostgreSQL database name (defaults to `flagdrive`).
* `-a`, `--addr <IP:PORT>`: The socket address to bind the server to (defaults to `0.0.0.0:4859`).
* `--dist <PATH>`: The path to serve the frontend static files from (defaults to `./dist`).

## Development & Running

The backend utilizes built-in SQLx migrations to automatically set up the database schema on startup, so no manual database initialization is required.

**To run the backend in development mode:**
```bash
DATABASE_URL=postgres://flagdrive:flagdrivepassword@localhost:5432/flagdrive cargo run -- --addr 127.0.0.1:4859
```
The backend API server will start on `http://127.0.0.1:4859`.

**To run in release mode:**
```bash
DATABASE_URL=postgres://flagdrive:flagdrivepassword@localhost:5432/flagdrive cargo run --release -- --addr 0.0.0.0:4859
```

## Project Structure

* `src/main.rs`: Entrypoint of the Axum server parsing command-line arguments, connecting to the PostgreSQL database, configuring routes, and starting the Tokio runtime.
* `src/cli.rs`: Command-line interface argument definitions.
* `src/auth.rs`: Session token validation, password hashing, and authentication utilities.
* `src/crypto.rs`: Cryptographic functions for file and metadata encryption.
* `src/api_routes/`: Folder containing endpoint handlers for auth, files, GDPR, and user management.
* `src/database/`: Folder containing PostgreSQL database handlers and core SQL queries (`core`, `files`, `gdpr`, `relations`, `users`).
* `migrations/`: SQL migration files for PostgreSQL (`migrations/postgres`).

