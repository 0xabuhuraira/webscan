# WebScan Changelog

## [1.0.0] - 2026-09-16

### Initial Release

#### Features
- High-performance asynchronous TCP scanning with Tokio
- Minecraft Java Edition Server List Ping detection
- Streaming target architecture (supports unlimited CIDR ranges)
- CIDR and IP address parsing (IPv4 and IPv6)
- Exclusion file support with CIDR/IP entries
- SOCKS5 proxy support (with optional authentication)
- HTTP CONNECT proxy support (with optional authentication)
- Proxy pooling with round-robin rotation and health tracking
- Rate limiting across all transport modes
- Multiple output formats: text, JSON, CSV, NDJSON
- Real-time progress display
- Checkpoint and resume functionality
- TOML-based configuration files
- Independent timeout handling (connect, Minecraft, proxy)
- Resource awareness (FD limits, memory)
- Fully unprivileged (no root or raw sockets)

#### Minecraft Protocol
- VarInt encoding/decoding
- VarLong support
- Minecraft framing implementation
- Server List Ping (Handshake, Status Request, Status Response)
- JSON status response parsing
- Metadata extraction (version, players, description)

#### Testing
- Unit tests for all core modules
- Integration tests for end-to-end scanning
- Proxy tests (SOCKS5, HTTP CONNECT)
- Output format tests
- Configuration parsing tests

#### Documentation
- Comprehensive README with examples
- CLI help text and usage documentation
- Example configuration files
- Example exclusion and proxy list files
- Contribution guidelines

#### Performance
- Tested concurrency: up to 50,000+ simultaneous connections
- Streaming generation: constant memory regardless of target range size
- Efficient CIDR iteration without expansion to individual IPs
- Minimal synchronization overhead with atomic operations
- Batched output writing

### Known Limitations
- Depends on unprivileged TCP connections (no raw sockets)
- Java Edition only (no Bedrock/RakNet/UDP)
- No player enumeration beyond aggregate counts
- DNS reverse lookup not yet implemented
- io_uring support not yet added

### Future Roadmap
- [ ] DNS reverse lookup
- [ ] io_uring backend for Linux
- [ ] Bedrock Edition support (separate module)
- [ ] Server vulnerability scanning
- [ ] Player statistics aggregation
- [ ] Geographic distribution mapping
- [ ] Export to standard formats (GeoJSON, etc.)
- [ ] Web UI for visualization
- [ ] Distributed scanning across multiple machines
