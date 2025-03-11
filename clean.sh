#!/bin/bash

# Remove build directories
rm -rf build_near
rm -rf build_cargo

# Clean specific target directories that are safe to remove
# These are intermediate build artifacts that can be rebuilt quickly
rm -rf target/wasm32-unknown-unknown/release/deps/*.wasm
rm -rf target/wasm32-unknown-unknown/release/*.wasm
rm -rf target/near/*.wasm
rm -rf target/near/*.json

# Keep the following directories to maintain faster rebuilds:
# - target/debug (contains debug build artifacts)
# - target/release (contains release build artifacts)
# - target/wasm32-unknown-unknown/release/deps/*.rlib (compiled libraries)
# - target/wasm32-unknown-unknown/release/deps/*.d (dependency info)

echo "Clean completed successfully!"