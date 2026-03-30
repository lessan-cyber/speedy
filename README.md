# speedy

A fast, terminal-based internet speed test CLI built in Rust.

## Features

- **Ping & Jitter** - Measures latency to Google DNS (8.8.8.8)
- **Download Speed** - 10-second timed download test
- **Upload Speed** - 10-second timed upload test
- **Table Output** - Clean ASCII table display
- **JSON Output** - Machine-readable results for scripting
- **Simple Mode** - Disable progress bars for minimal output

## Installation

```bash
cargo build --release
```

The binary will be at `target/release/speedy`.

## Usage

```bash
# Run full speed test
speedy

# Download only
speedy --download-only

# Upload only
speedy --upload-only

# Simple mode (no progress bars)
speedy --simple

# JSON output
speedy --json

# Combine flags
speedy --download-only --json
```

## Options

| Flag | Short | Description |
|------|-------|-------------|
| `--simple` | `-s` | Disable progress bar animations |
| `--download-only` | `-d` | Test download speed only |
| `--upload-only` | `-u` | Test upload speed only |
| `--json` | `-j` | Output results as JSON |

## Example Output

```plaintext
+-----------+-------------+-----------------+---------------+
| Ping (ms) | Jitter (ms) | Download (Mbps) | Upload (Mbps) |
+-----------+-------------+-----------------+---------------+
| 15.23     | 1.45        | 95.12           | 42.67         |
+-----------+-------------+-----------------+---------------+
```

JSON mode:
```json
{
  "ping_ms": 15.23,
  "jitter_ms": 1.45,
  "download_mbps": 95.12,
  "upload_mbps": 42.67
}
```

## Dependencies

- [clap](https://crates.io/crates/clap) - CLI argument parsing
- [reqwest](https://crates.io/crates/reqwest) - HTTP client (blocking mode)
- [indicatif](https://crates.io/crates/indicatif) - Progress bars
- [tabled](https://crates.io/crates/tabled) - Table formatting
- [colored](https://crates.io/crates/colored) - Terminal colors
- [serde](https://crates.io/crates/serde) / [serde_json](https://crates.io/crates/serde_json) - JSON serialization

## License

MIT
