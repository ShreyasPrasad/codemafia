#!/bin/bash
curl https://sh.rustup.rs -sSf | sh -s -- -y
PATH=$PATH:/vercel/.cargo/bin

rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
cargo install --locked trunk