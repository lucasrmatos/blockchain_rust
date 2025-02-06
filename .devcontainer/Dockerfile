FROM mcr.microsoft.com/devcontainers/rust:latest

# Instalar dependências adicionais (opcional)
RUN apt-get update && apt-get install -y \
    git \
    vim \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Instalar ferramentas adicionais para Rust
RUN rustup toolchain install stable && \
    rustup default stable && \
    rustup component add clippy rustfmt