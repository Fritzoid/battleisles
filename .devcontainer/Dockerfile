FROM mcr.microsoft.com/devcontainers/rust:latest
RUN sudo apt-get update && sudo apt-get install librust-alsa-sys-dev -y && sudo apt-get install libudev-dev -y && cargo install --locked trunk
#&& rustup target add wasm32-unknown-unknown && cargo install --locked wasm-bindgen-cli