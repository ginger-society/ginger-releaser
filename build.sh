#!/bin/bash

# Exit immediately if a command exits with a non-zero status
set -e

# List of all target platforms
targets=(
    # "x86_64-unknown-linux-gnu"
    # "x86_64-apple-darwin"
    "x86_64-pc-windows-gnu"
    # "x86_64-unknown-linux-musl"
    "aarch64-unknown-linux-gnu"
    "aarch64-apple-darwin"
    # "aarch64-pc-windows-msvc"
    # "aarch64-unknown-linux-musl"
)


# Function to build for a specific target
build_target() {
    local target=$1
    echo "Building for $target..."
    cargo build --release --target "$target"
}

# Loop over all targets and build
for target in "${targets[@]}"; do
    build_target "$target"
done

