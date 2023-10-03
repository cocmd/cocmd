FROM ubuntu:18.04

RUN apt-get update \
    && apt-get install -y libssl-dev curl \
    libxdo-dev libxtst-dev libx11-dev \
    libxkbcommon-dev libwxgtk3.0-gtk3-dev libdbus-1-dev \
    wget git file build-essential pkg-config

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

# Install the latest Rust version
RUN set -eux; \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y; \
    rustup update; \
    rustup --version; \
    cargo --version; \
    rustc --version;

RUN mkdir cocmd && cargo install rust-script --version "0.7.0" && cargo install --force cargo-make --version 0.34.0

COPY . cocmd

RUN cd cocmd
