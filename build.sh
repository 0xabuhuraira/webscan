#!/bin/bash
set -e

echo "Building WebScan..."
cargo build --release

echo "Running tests..."
cargo test

echo ""
echo "Build complete!"
echo "Binary: target/release/webscan"
echo ""
echo "Quick start:"
echo "  ./target/release/webscan 192.168.1.0/24 -p25565"
echo ""
