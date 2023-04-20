#!/bin/bash
curl https://sh.rustup.rs -sSf | sh -s -- -y
PATH=$PATH:/vercel/.cargo/bin

cd app && rustup target add wasm32-unknown-unknown
cd app && cargo install wasm-bindgen-cli
cd app && cargo install --locked trunk