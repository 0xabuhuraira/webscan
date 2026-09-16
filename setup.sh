#!/bin/bash
# WebScan development environment setup

echo "WebScan Development Setup"
echo ""

# Check Rust
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust/Cargo not found. Install from https://rustup.rs/"
    exit 1
fi

echo "✓ Rust version: $(rustc --version)"
echo "✓ Cargo version: $(cargo --version)"

# Check system FD limit
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    LIMIT=$(ulimit -n)
    echo "✓ FD limit: $LIMIT"
    if [ "$LIMIT" -lt 4096 ]; then
        echo "⚠️  Consider increasing FD limit: ulimit -n 65536"
    fi
fi

echo ""
echo "Setup complete! Run 'cargo build --release' to build."
