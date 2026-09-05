# battleisles

Nostalgia version of battle isles 

Using stable Rust (1.95 or newer, pinned to stable in rust-toolchain.toml)

clone and run 'cargo run'

If you want wasm support:

    - Install trunk with 'cargo install --locked trunk'

    - Install wasm target with 'rustup target add wasm32-unknown-unknown' 
    
    - Install wasm-bindg-clien with 'cargo install --locked wasm-bindgen-cli'
    
    - run 'trunk build'
    
    - run 'trunk serve'

If you want to use a codespace, everything should be ready in .devcontainer

