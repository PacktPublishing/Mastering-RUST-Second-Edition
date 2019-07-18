#!/bin/sh

set -ex

# Prerequisites
rustup target add wasm32-unknown-unknown || true
# Update the dependency in Cargo.toml to match with the latest wasm-bindgen-cli tool
cargo install -f wasm-bindgen-cli || true
cargo build --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/debug/livemd.wasm --out-dir app
cd app
yarn install
yarn run serve
