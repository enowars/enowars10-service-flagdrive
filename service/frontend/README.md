# FlagDrive Frontend

The frontend of FlagDrive is a single-page application (SPA) for the ENOWARS10 "Hack the Government" Attack/Defense CTF. It is built using [Leptos](https://leptos.dev/) (a Rust web framework) and styled with [Tailwind CSS v4](https://tailwindcss.com/).

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
4. **Tailwind CSS CLI Standalone Binary**: The standalone Tailwind CSS binary is used to compile styles.
   - Download the standalone binary for your OS from the [Tailwind CSS Releases](https://github.com/tailwindlabs/tailwindcss/releases) page.
   - Rename it to `tailwindcss` and make it executable: `chmod +x tailwindcss`
   - Place it in your system's `PATH` (e.g., `/usr/local/bin/tailwindcss` or equivalent).

## Development & Building

The project is configured to automatically compile Tailwind CSS via a hook in `Trunk.toml`. 

**To run the development server (with hot-reloading):**
```bash
trunk serve
```
The application will be available at `http://127.0.0.1:4859`.

**To build for production:**
```bash
trunk build --release
```
The built files will be located in the `dist/` directory. These files are ready to be served by the backend application.

## Project Structure

* `index.html`: Entry-point HTML file utilized by Trunk for injecting the compiled WebAssembly asset.
* `Trunk.toml`: Configuration file for the Trunk bundler, detailing building hooks (like compiling Tailwind styles).
* `src/main.rs`: The WebAssembly startup entry point that mounts the Leptos application onto the DOM.
* `src/app.rs`: Main Leptos layout, defining pages, app state, and routing logic.
* `src/components/`: Reusable components (e.g., `UploadModal`, `DownloadModal`, `Navbar`, etc.).
* `src/pages/`: Specific page components linked with routes (`Login`, `Register`, `Dashboard`, `Profile`).
* `src/styles/`: Source Tailwind stylesheet (`tailwind.css`) that styles our application.
