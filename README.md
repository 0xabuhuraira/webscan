# WebScan - Production-Quality Minecraft Java Edition Discovery Scanner

**WebScan** is a high-performance, unprivileged, asynchronous TCP-based Minecraft Java Edition server discovery scanner written in Rust. It is inspired by Masscan's streaming architecture and high-throughput philosophy but implements independent unprivileged scanning without raw packet injection.

## Features

- ✅ **High-Performance Scanning**: Asynchronous TCP connections with configurable concurrency (tested up to 50,000+ concurrent connections)
- ✅ **Minecraft Java Edition Detection**: Lightweight Server List Ping protocol implementation
- ✅ **Streaming Target Architecture**: Never materializes full target set in memory (supports `0.0.0.0/0`)
- ✅ **CIDR/IP Parsing**: Efficient IPv4 and IPv6 CIDR range generation
- ✅ **Exclusion Filtering**: Support for exclusion files with CIDR and IP entries
- ✅ **Proxy Support**: SOCKS5, SOCKS5 with auth, HTTP CONNECT proxies
- ✅ **Proxy Pooling**: Round-robin proxy rotation with health tracking and quarantine
- ✅ **Rate Limiting**: Configurable rate control across all transport modes
- ✅ **Multiple Output Formats**: Text, JSON, CSV, NDJSON (streaming-friendly)
- ✅ **Progress Display**: Real-time statistics without throughput penalty
- ✅ **Checkpoint/Resume**: Resume large scans without restarting
- ✅ **Configuration Files**: TOML-based configuration with CLI override
- ✅ **Timeout Handling**: Independent timeouts for connections, Minecraft protocol, proxies
- ✅ **Resource Awareness**: Automatic FD limit detection and adjustment
- ✅ **Fully Unprivileged**: No root, CAP_NET_RAW, or raw sockets required

## Quick Start

### Build

```bash
cargo build --release
```

The binary will be at `target/release/webscan`.

### Basic Usage

```bash
# Scan a single network
./webscan 192.168.1.0/24 -p25565

# Multiple common Minecraft ports
./webscan 10.0.0.0/8 -p25565,25566,25567

# Port range
./webscan 10.0.0.0/16 -p25560-25600

# Large range with exclusions
./webscan 0.0.0.0/0 -p0-65535 --excludefile exclude.txt --concurrency 50000

# With SOCKS5 proxy
./webscan targets.txt -p25565 --proxy socks5://127.0.0.1:1080

# Proxy pool
./webscan targets.txt -p25565 --proxy-list proxies.txt
```

## Command-Line Options

```
Usage: webscan [OPTIONS] [TARGET]

Arguments:
  [TARGET]  Target specification (CIDR, IP, or file with targets)

Options:
  -p, --ports <PORTS>
          Ports to scan (e.g., 25565 or 25565,25566 or 25565-25600)

  --excludefile <FILE>
          Exclusion file with CIDRs or IPs to exclude

  --concurrency <NUM>
          Maximum concurrent connections [default: 10000]

  --rate <NUM>
          Rate limiting (0 = best effort) [default: 0]

  --connect-timeout <SECS>
          Connection timeout in seconds [default: 2]

  --minecraft-timeout <SECS>
          Minecraft protocol timeout in seconds [default: 3]

  --proxy-timeout <SECS>
          Proxy timeout in seconds [default: 3]

  --retries <NUM>
          Number of retries on failure [default: 1]

  --proxy <PROXY>
          SOCKS5 or HTTP proxy (socks5://host:port or http://host:port)

  --proxy-list <FILE>
          Proxy list file (one proxy per line)

  --per-proxy-concurrency <NUM>
          Per-proxy concurrency limit

  --protocol-version <VERSION>
          Minecraft protocol version [default: 770]

  --host-header <HOST>
          Host header for Minecraft handshake

  --minecraft-ping
          Enable Minecraft ping for latency measurement

  --show-open
          Show all TCP open ports (not just Minecraft)

  --output <FORMAT>
          Output format (text, json, csv, ndjson) [default: text]

  -o, --output-file <FILE>
          Output file path

  --no-progress
          Disable progress display

  --randomize-targets
          Randomize target order

  --seed <SEED>
          Seed for randomization

  --checkpoint <FILE>
          Checkpoint file path

  --resume
          Resume from checkpoint

  --config <FILE>
          Configuration file path (~/.webscan.toml by default)

  --resolve
          Enable DNS resolution of results

  --benchmark
          Run benchmark

  -v, --verbose
          Verbosity level

  -h, --help
          Print help

  -V, --version
          Print version
```

## Configuration File

Create `~/.webscan.toml` or specify with `--config`:

```toml
# Global settings
concurrency = 10000
rate = 100000
connect_timeout = 2
minecraft_timeout = 3
proxy_timeout = 3
retries = 1

# Minecraft protocol
protocol_version = 770

# Output
output_format = "ndjson"
no_progress = false

# Proxy
per_proxy_concurrency = 500
```

CLI arguments override configuration file values.

## Exclusion File Format

`exclude.txt`:

```text
# RFC1918 private ranges
10.0.0.0/8
172.16.0.0/12
192.168.0.0/16

# Specific IPs
8.8.8.8
1.1.1.1

# Comments are supported
```

## Proxy List Format

`proxies.txt`:

```text
socks5://127.0.0.1:1080
socks5://user:pass@10.0.0.5:1080
http://proxy.example.com:8080
http://user:pass@proxy.example.com:8080
```

**Note:** Credentials are never logged or written to output.

## Output Formats

### Text (default)

```
1.2.3.4:25565  MINECRAFT  "Paper 1.21.5"  12/100  latency=18ms
1.2.3.5:25565  MINECRAFT  "1.20.4"  3/20  latency=25ms
```

### JSON

```json
[
  {
    "ip": "1.2.3.4",
    "port": 25565,
    "protocol": "minecraft",
    "version": "Paper 1.21.5",
    "protocol_version": 770,
    "players_online": 12,
    "players_max": 100,
    "description": "A test server",
    "latency_ms": 18
  }
]
```

### NDJSON (streaming)

```json
{"ip":"1.2.3.4","port":25565,"protocol":"minecraft","version":"Paper 1.21.5","protocol_version":770,"players_online":12,"players_max":100,"latency_ms":18}
{"ip":"1.2.3.5","port":25565,"protocol":"minecraft","version":"1.20.4","protocol_version":758,"players_online":3,"players_max":20,"latency_ms":25}
```

### CSV

```csv
ip,port,protocol,version,protocol_version,players_online,players_max,description,latency_ms
1.2.3.4,25565,minecraft,"Paper 1.21.5",770,12,100,"A test server",18
```

## Examples

### Scan a local network

```bash
./webscan 192.168.1.0/24 -p25565
```

### Scan with high concurrency

```bash
./webscan 10.0.0.0/8 -p25565 --concurrency 50000
```

### Scan with exclusions

```bash
./webscan 10.0.0.0/8 -p25565 --excludefile exclude.txt
```

### Output to NDJSON file

```bash
./webscan 10.0.0.0/8 -p25565 --output ndjson -o results.ndjson
```

### Resume a scan

```bash
./webscan 10.0.0.0/8 -p25565 --checkpoint scan.ckpt --resume
```

### Use proxy pool

```bash
./webscan targets.txt -p25565 --proxy-list proxies.txt --concurrency 5000
```

### Scan all ports, find all open TCP ports

```bash
./webscan 192.168.1.0/24 -p0-65535 --show-open
```

## Architecture

WebScan uses a streaming pipeline architecture:

```
    Target Generator (CIDR/IP stream)
           ↓
    Exclusion Filter
           ↓
    Bounded Work Queue
           ↓
    Async TCP Scheduler
           ↓
    ┌─────────────────┐
    │ Direct TCP      │
    │ or Proxy Pool   │
    └────────┬────────┘
             ↓
    TCP Connection Success?
             ↓ yes
    Minecraft Status Probe
             ↓
    JSON Status Parser
             ↓
    Result Output (text/json/csv/ndjson)
```

## Performance Characteristics

- **Memory Usage**: Approximately bounded by concurrency level, not target count
- **CPU Usage**: Minimal per-connection (packet framing + JSON parsing)
- **Network**: Unprivileged TCP only (no raw packets)
- **Throughput**: Dependent on network, proxy, and target responsiveness
  - Direct: ~100k-500k attempts/sec (typical broadband)
  - Via SOCKS5: ~10k-50k attempts/sec
  - Via HTTP CONNECT: ~5k-20k attempts/sec

## Building from Source

### Requirements

- Rust 1.70+
- Linux/macOS/Windows (Unix preferred)

### Build

```bash
git clone https://github.com/0xabuhuraira/webscan.git
cd webscan
cargo build --release
```

The binary will be at `target/release/webscan`.

### Strip Binary (optional)

```bash
strip target/release/webscan
```

## Testing

Run tests:

```bash
cargo test
```

Run with logging:

```bash
RUST_LOG=debug cargo test -- --nocapture
```

## Security & Authorization

⚠️ **IMPORTANT**: WebScan is intended for scanning systems and networks where you have **explicit authorization**. Unauthorized network scanning may be illegal in your jurisdiction.

WebScan does NOT:
- Exploit vulnerabilities
- Guess credentials
- Perform login attacks
- Download player data
- Modify server state
- Bypass authentication
- Evade firewalls or IDS

WebScan performs only:
- TCP reachability testing
- Minecraft Server List Ping (lightweight protocol handshake)
- Minecraft metadata collection (version, player count, description)

## Limitations

- **Unprivileged TCP only**: Does not use raw sockets or packet injection
- **Java Edition only**: Does not support Bedrock Edition (UDP/RakNet)
- **No player enumeration**: Only reports aggregate player counts
- **No server commands**: Read-only, does not join or authenticate
- **Dependent on network**: Throughput limited by ISP, proxy, and target responsiveness

## Troubleshooting

### "Too many open files"

Increase your system's file descriptor limit:

```bash
ulimit -n 65536
```

Then retry with appropriate concurrency:

```bash
./webscan 10.0.0.0/8 -p25565 --concurrency 50000
```

### Slow performance

- Reduce concurrency: `--concurrency 5000`
- Enable rate limiting: `--rate 100000`
- Check network/proxy latency
- Use NDJSON output for large scans

### Proxy errors

- Verify proxy URL format: `socks5://host:port` or `http://host:port`
- Check proxy credentials: Credentials in URLs are URL-encoded
- Verify proxy supports target protocol
- Check timeout values: `--proxy-timeout 5`

## Project Structure

```
webscan/
├── Cargo.toml              # Project manifest
├── Cargo.lock              # Locked dependencies
├── README.md               # This file
├── LICENSE                 # MIT License
├── src/
│   ├── main.rs             # Entry point
│   ├── cli.rs              # CLI argument parsing
│   ├── config.rs           # Configuration management
│   ├── error.rs            # Error types
│   ├── scanner.rs          # Main scanner orchestration
│   ├── scheduler.rs        # Async task scheduler
│   ├── targets.rs          # Target streaming generation
│   ├── cidr.rs             # CIDR/IP parsing
│   ├── ports.rs            # Port range parsing
│   ├── exclude.rs          # Exclusion list matching
│   ├── transport.rs        # Transport abstractions
│   ├── proxy.rs            # Proxy pool management
│   ├── socks5.rs           # SOCKS5 proxy implementation
│   ├── http_connect.rs     # HTTP CONNECT proxy implementation
│   ├── minecraft.rs        # Minecraft protocol probing
│   ├── minecraft_varint.rs # Minecraft VarInt codec
│   ├── minecraft_status.rs # Minecraft Server List Ping
│   ├── dns.rs              # DNS resolution
│   ├── output.rs           # Output formatting
│   ├── progress.rs         # Progress display
│   ├── checkpoint.rs       # Checkpoint/resume
│   └── benchmark.rs        # Benchmark functionality
└── tests/
    ├── minecraft_status.rs # Minecraft protocol tests
    ├── target_streaming.rs # Target generation tests
    ├── proxies.rs          # Proxy tests
    └── integration.rs      # Integration tests
```

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Add tests
4. Submit a pull request

## License

MIT License - see LICENSE file for details.

## Acknowledgments

- Inspired by Masscan's streaming architecture and high-throughput design philosophy
- Minecraft Server List Ping protocol documentation from the official wiki
- Built with Rust, Tokio, and excellent open-source libraries

## Disclaimer

This tool is provided for authorized security testing and network administration only. The authors assume no liability for misuse or damage caused by this software. Users are responsible for ensuring they have proper authorization before scanning any network or system.
