#!/usr/bin/env bash

# We exit on any error, since this script is also run in the ci/cd
set -e

echo "Installing web assembly prerequisites..."

echo "Adding wasm dependencies..."

rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli

echo "Done!"