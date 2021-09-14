#!/usr/bin/env bash

set -e

echo "Installing web assembly prerequisites..."

echo "Adding wasm dependencies..."

rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli

echo "Done!"