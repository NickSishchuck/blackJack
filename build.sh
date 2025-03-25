#!/bin/bash

echo "Building for multiple platforms..."

# Create builds directory if it doesn't exist
mkdir -p builds

# Build for Windows
echo "Building Windows target..."
cargo build --target x86_64-pc-windows-gnu --release
cp target/x86_64-pc-windows-gnu/release/black-jak.exe builds/

# Build for Linux
echo "Building Linux target..."
cargo build --target x86_64-unknown-linux-gnu --release
cp target/x86_64-unknown-linux-gnu/release/black-jak builds/black-jak-linux


echo "Build complete! Binaries are in the 'builds' directory"