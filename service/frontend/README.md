# FlagDrive Frontend

FlagDrive is the frontend single-page application (SPA) for the "Hack the Government" Attack/Defense CTF service. It is built using [Leptos](https://leptos.dev/) (a Rust web framework) and styled with [Tailwind CSS v4](https://tailwindcss.com/).

## Prerequisites

To build and run this frontend, you will need the following tools installed on your system:

1. **Rust & Cargo**: The Rust toolchain.
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. **WebAssembly Target**: Leptos compiles to WebAssembly. Add the target via rustup:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```
3. **Trunk**: A WASM web application bundler for Rust.
   ```bash
   cargo install --locked trunk
   ```
4. **Node.js & npm**: Required to run the Tailwind CSS CLI.
   - Install from [nodejs.org](https://nodejs.org/) or via your system's package manager.

## Installation

1. Navigate to the frontend directory:
   ```bash
   cd service/frontend
   ```
2. Install the necessary node dependencies (Tailwind CSS CLI):
   ```bash
   npm install
   ```

## Development & Building

The project is configured to automatically compile Tailwind CSS via a hook in `Trunk.toml`. 

**To run the development server (with hot-reloading):**
```bash
trunk serve
```
The application will be available at `http://127.0.0.1:8080`.

**To build for production:**
```bash
trunk build --release
```
The built files will be located in the `dist/` directory. These files are ready to be served by the backend application.
