# FROM rust:1.95.0-slim AS builder
# SHELL ["/bin/bash", "--login", "-c"]

# # Add cargo bin to path
# RUN echo 'export PATH="/usr/local/cargo/bin:$PATH"' >> /etc/profile \
#     && echo 'export PATH="/usr/local/cargo/bin:$PATH"' >> /root/.bashrc

# # Install system dependencies
# RUN apt-get update && apt-get install -y --no-install-recommends \
#     ca-certificates \
#     pkg-config \
#     libssl-dev \
#     curl \
#     && rm -rf /var/lib/apt/lists/*

# # Install Rust target for Wasm
# RUN rustup target add wasm32-unknown-unknown

# # Install bin-installed tools for faster tool installs 
# RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
# RUN cargo binstall trunk
# RUN cargo binstall -y wasm-bindgen-cli@0.2.121

# # Install nodejs & tailwindcss
# RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.4/install.sh | bash
# RUN nvm install 26
# RUN npm install -g tailwindcss @tailwindcss/cli

# # Set work dir and copy files
# WORKDIR /ctf/service
# COPY . .

# # Prebuild 
# RUN cargo fetch
# RUN cd frontend && npm install && cd ..
# RUN cd frontend && trunk build --release && cd ..

# ENTRYPOINT ["/usr/bin/bash"]

# #RUN cd backend && cargo build --release && cd ..

# Install nodejs & tailwindcss
# RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.4/install.sh | bash
# RUN nvm install 26
# RUN npm install -g tailwindcss @tailwindcss/cli

# Build frontend
# RUN cd frontend && npm install && cd ..
# RUN cd frontend && trunk build --release && cd ..

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
RUN cargo binstall trunk
RUN cargo binstall cargo-chef
RUN cargo binstall wasm-bindgen-cli

RUN curl -Lfo /usr/local/bin/tailwindcss https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64 && \
    chmod +x /usr/local/bin/tailwindcss

# --- CHEF ---
FROM toolchain AS chef
WORKDIR /ctf/service
COPY . .

# Prepare recipe file
RUN cargo chef prepare --recipe-path /ctf/recipe.json

# --- BUILDER ---
FROM toolchain AS builder
WORKDIR /ctf/service

# Cook recipe file
COPY --from=chef /ctf/recipe.json /ctf/recipe.json
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo chef cook --release --package flagdrive-backend --recipe-path /ctf/recipe.json
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo chef cook --release --package flagdrive-frontend --target wasm32-unknown-unknown --recipe-path /ctf/recipe.json

# Build frontend and backend
COPY . .
RUN cd frontend && trunk build --release && cd ..
RUN cd backend && cargo build --release && cd ..

ENTRYPOINT ["/usr/bin/bash"]
