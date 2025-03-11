#!/bin/bash

# Build WASM
cargo build --target wasm32-unknown-unknown --release

# Create build_cargo directory if it doesn't exist
mkdir -p build_cargo

# Optimize WASM and output directly to build_cargo directory
wasm-opt -Oz -o build_cargo/rugfactory_factory.wasm target/wasm32-unknown-unknown/release/rugfactory_factory.wasm

echo "Build completed successfully!"