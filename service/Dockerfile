# --- TOOLCHAIN ---
FROM rust:1.95.0-slim AS toolchain

# Install system dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    pkg-config \
    libssl-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Install Rust target for Wasm
RUN rustup target add wasm32-unknown-unknown

# Install bin-installed tools for faster tool installs 
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall -y trunk@0.21.14 cargo-chef@0.1.77 wasm-bindgen-cli@0.2.121

RUN curl -Lfo /usr/local/bin/tailwindcss https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64 && \
    chmod +x /usr/local/bin/tailwindcss


# --- CHEF ---
FROM toolchain AS chef
WORKDIR /ctf/service
COPY . .

# Prepare recipe file
RUN cargo chef prepare --recipe-path /ctf/recipe.json


# --- COOK-FRONTEND ---
FROM toolchain AS cook-frontend
WORKDIR /ctf/service

# Cook recipe file
COPY --from=chef /ctf/recipe.json /ctf/recipe.json
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo chef cook --release --package flagdrive-frontend --target wasm32-unknown-unknown --recipe-path /ctf/recipe.json


# --- COOK-BACKEND ---
FROM toolchain AS cook-backend
WORKDIR /ctf/service

# Cook recipe file
COPY --from=chef /ctf/recipe.json /ctf/recipe.json
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo chef cook --release --package flagdrive-backend --recipe-path /ctf/recipe.json


# --- BUILDER-FRONTEND ---
FROM toolchain AS builder-frontend
WORKDIR /ctf/service

# Build frontend
COPY --from=cook-frontend /ctf/service/target /ctf/service/target
COPY --from=cook-frontend /ctf/service/backend /ctf/service/backend
COPY Cargo.toml Cargo.lock ./
COPY backend/Cargo.toml ./backend/
COPY shared ./shared
COPY frontend ./frontend
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cd frontend && trunk build --release && cd ..


# --- BUILDER-BACKEND ---
FROM toolchain AS builder-backend
WORKDIR /ctf/service

# Build backend
COPY --from=cook-backend /ctf/service/target /ctf/service/target
COPY --from=cook-backend /ctf/service/frontend /ctf/service/frontend
COPY Cargo.toml Cargo.lock ./
COPY frontend/Cargo.toml ./frontend/
COPY shared ./shared
COPY backend ./backend
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cd backend && cargo build --release && cd ..


# --- DATABASE ---
FROM toolchain AS database
WORKDIR /ctf/data


# --- FINAL IMAGE ---
FROM gcr.io/distroless/cc-debian13 AS runtime

COPY --from=builder-frontend /ctf/service/frontend/dist /ctf/service/dist
COPY --from=builder-backend /ctf/service/target/release/flagdrive-backend /ctf/service/flagdrive
COPY --from=database /ctf/data /ctf/data

ENTRYPOINT ["/ctf/service/flagdrive", "--dist", "/ctf/service/dist", "--database", "/ctf/data/flagdrive.db", "--addr", "0.0.0.0:4859"]