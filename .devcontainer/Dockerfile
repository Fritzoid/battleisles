FROM mcr.microsoft.com/devcontainers/rust:latest
USER vscode
RUN apt-get update && apt-get install librust-alsa-sys-dev -y && apt-get install libudev-dev -y && cargo install trunk
#&& rustup target add wasm32-unknown-unknown && cargo install --locked wasm-bindgen-cli