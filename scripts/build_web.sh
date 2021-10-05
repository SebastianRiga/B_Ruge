#!/usr/bin/env bash

# We exit on any error, since this script is also run in the ci/cd
set -e

echo "Building project for wasm32-unknown-unknown target to prepare web assembly..."

cargo build --release --target wasm32-unknown-unknown 

echo "Creating web assembly..."

wasm-bindgen target/wasm32-unknown-unknown/release/b_ruge.wasm --out-dir web/out --no-modules --no-typescript

echo "Copying wrapper files..."

cp -a web/templates/. web/out/
mkdir -p web/out/resources
cp -r resources/ambiance resources/music resources/sfx web/out/resources/

echo "Done!"