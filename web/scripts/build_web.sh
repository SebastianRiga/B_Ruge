#!/usr/bin/env bash

echo "Adding wasm dependencies..."

rustup target add wasm32-unknown-unknown

echo "Building project for wasm32-unknown-unknown target to prepare web assembly..."

cargo build --release --target wasm32-unknown-unknown 

echo "Creating web assembly..."

wasm-bindgen target/wasm32-unknown-unknown/release/b_ruge.wasm --out-dir web/out --no-modules --no-typescript

echo "Copying wrapper files..."

cp -a web/templates/. web/out/

echo "Done!"