# WebScan Contribution Guide

Thank you for your interest in contributing to WebScan!

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/webscan.git`
3. Create a feature branch: `git checkout -b my-feature`
4. Set up development: `bash setup.sh`

## Code Style

- Follow standard Rust conventions
- Run `cargo fmt` before committing
- Run `cargo clippy` to check for common mistakes
- Add tests for new functionality

## Building and Testing

```bash
# Build
cargo build

# Test
cargo test

# Test with logging
RUST_LOG=debug cargo test -- --nocapture

# Release build
cargo build --release

# Check code style
cargo fmt --check
cargo clippy
```

## Pull Request Process

1. Ensure all tests pass: `cargo test`
2. Add tests for new functionality
3. Update README if needed
4. Ensure no `clippy` warnings: `cargo clippy`
5. Format code: `cargo fmt`
6. Submit PR with clear description

## Architecture

Key modules:

- `cli.rs` - Command-line argument parsing
- `config.rs` - Configuration management
- `targets.rs` - Streaming target generation
- `cidr.rs` - CIDR/IP parsing
- `scheduler.rs` - Async task scheduling
- `minecraft*.rs` - Minecraft protocol implementation
- `proxy.rs`, `socks5.rs`, `http_connect.rs` - Proxy support
- `output.rs` - Result formatting and writing
- `progress.rs` - Progress display

## Testing

Add tests in `tests/` directory. Each test file focuses on a specific module:

- `minecraft_varint_tests.rs` - VarInt encoding/decoding
- `target_parsing_tests.rs` - CIDR/port parsing
- `exclusion_tests.rs` - Exclusion matching
- `proxy_tests.rs` - Proxy configuration
- `output_tests.rs` - Output formatting

## Performance Considerations

- Avoid allocations in hot paths
- Use bounded queues for concurrency
- Prefer byte-oriented parsing
- Minimize synchronization overhead
- Stream results instead of buffering

## Security

- Never log credentials or sensitive data
- Sanitize all untrusted input
- Validate packet lengths against reasonable maximums
- Use timeouts to prevent blocking
- Follow principle of least privilege

## Questions?

Open an issue with the `[question]` tag or start a discussion.

## License

By contributing, you agree your work will be licensed under MIT.
