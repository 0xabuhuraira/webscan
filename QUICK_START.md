# Quick Start Guide

## Installation

### Prerequisites
- Rust 1.70+ (install from https://rustup.rs/)
- Linux/macOS/Windows (Unix preferred)

### Build

```bash
git clone https://github.com/0xabuhuraira/webscan.git
cd webscan
cargo build --release
```

Binary: `target/release/webscan`

## Common Commands

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

### Output to NDJSON
```bash
./webscan 10.0.0.0/8 -p25565 --output ndjson -o results.ndjson
```

### Use SOCKS5 proxy
```bash
./webscan targets.txt -p25565 --proxy socks5://127.0.0.1:1080
```

### Proxy pool
```bash
./webscan targets.txt -p25565 --proxy-list proxies.txt
```

### Resume scan
```bash
./webscan 10.0.0.0/8 -p25565 --checkpoint scan.ckpt --resume
```

## Configuration File

Create `~/.webscan.toml`:

```toml
concurrency = 10000
rate = 100000
connect_timeout = 2
minecraft_timeout = 3
protocol_version = 770
```

## Output Examples

### Text (default)
```
1.2.3.4:25565  MINECRAFT  "Paper 1.21.5"  12/100  latency=18ms
```

### NDJSON (streaming)
```json
{"ip":"1.2.3.4","port":25565,"protocol":"minecraft","version":"Paper 1.21.5","players_online":12,"players_max":100,"latency_ms":18}
```

## Troubleshooting

### "Too many open files"
```bash
ulimit -n 65536
```

### Slow performance
- Reduce concurrency: `--concurrency 5000`
- Enable rate limiting: `--rate 100000`
- Use NDJSON output

### Debug output
```bash
RUST_LOG=debug ./webscan 192.168.1.0/24 -p25565
```

## Documentation

- Full guide: `README.md`
- Examples: `README.md` (Examples section)
- Config: `webscan.toml.example`
- Exclusions: `exclude.txt.example`
- Proxies: `proxies.txt.example`

## Next Steps

1. Read the main README
2. Set up exclusion file
3. Configure desired options
4. Run your first scan
5. Check results in preferred format

For more details, see `README.md`.
