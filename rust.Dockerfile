FROM rustlang/rust:nightly

RUN rustup target add wasm32-unknown-unknown
RUN cargo install cargo-watch wasm-pack wasm-bindgen-cli
