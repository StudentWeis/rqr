<h2 align="center"><em>
  <strong>rqr：Rust QR</strong>
</em></h2>

<p align="center">
A fast and lightweight QR code CLI tool written in Rust
</p>

<p align="center">
  <a href="https://www.rust-lang.org"><img src="https://img.shields.io/badge/language-Rust-orange.svg" alt="Rust"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License"></a>
  <a href="https://github.com/StudentWeis/rqr/releases"><img src="https://img.shields.io/github/v/release/StudentWeis/rqr" alt="Release"></a>
</p>

## Overview

`rqr` is a command-line tool for encoding and decoding QR codes. It supports generating QR codes as PNG images or displaying them directly in the terminal, as well as decoding QR codes from local files or network URLs.

## Features

- **Encode & Decode**: Full support for QR code generation and recognition
- **Terminal Display**: Render QR codes directly in the terminal using ASCII art
- **Network Support**: Decode QR codes from remote URLs
- **Configurable**: Customize size, error correction level, and margin
- **Fast & Lightweight**: Built with Rust for optimal performance

## Installation

### From Source

```bash
git clone https://github.com/StudentWeis/rqr.git
cd rqr
cargo build --release
cargo install --path .
```

### Pre-built Binaries

Download pre-built binaries from the [Releases](https://github.com/StudentWeis/rqr/releases) page.

## Usage

### Generate QR Code

```bash
# Save as PNG file
rqr encode "Hello, World!" --output hello.png

# Display in terminal
rqr encode "Terminal test" --terminal

# With custom parameters
rqr encode "https://rust-lang.org" \
  --output rust.png \
  --size 300 \
  --error-correction H \
  --margin 5
```

### Decode QR Code

```bash
# From local file
rqr decode qr-image.png

# From network URL
rqr decode "https://example.com/qr-code.png"
```

## Command Reference

### `encode` Options

| Option | Short | Default | Description |
|--------|-------|---------|-------------|
| `--output` | `-o` | `qr.png` | Output file path |
| `--size` | `-s` | `200` | Image size in pixels |
| `--error-correction` | `-e` | `M` | Error correction level (L/M/Q/H) |
| `--margin` | `-m` | `10` | Margin size in modules |
| `--terminal` | `-t` | - | Display in terminal instead of saving |

### Error Correction Levels

| Level | Recovery Capacity |
|-------|-------------------|
| L (Low) | ~7% |
| M (Medium) | ~15% (default) |
| Q (Quartile) | ~25% |
| H (High) | ~30% |

## Examples

### URL QR Code

```bash
rqr encode "https://github.com/rust-lang/rust" --output github.png --size 400
```

### Contact Information

```bash
rqr encode "Name: John Doe
Phone: 138-0000-0000
Email: john@example.com" --output contact.png
```

### WiFi Configuration

```bash
rqr encode "WIFI:T:WPA;S:MyNetwork;P:password123;;" --output wifi.png
```

### High-Reliability QR Code

```bash
rqr encode "Critical data" --error-correction H --output important.png
```

## Development

```bash
# Build
cargo build

# Run tests
cargo test

# Run with development build
cargo run -- encode "Test" --terminal
```

### Project Structure

See [Architecture.md](doc/Architecture.md) for detailed design documentation.

## Roadmap

- [ ] Custom color themes
- [ ] Additional output formats (SVG, JPEG)
- [ ] Batch processing
- [ ] WebAssembly support

## License

MIT License - see [LICENSE](LICENSE) for details.

## Acknowledgments

- [clap](https://crates.io/crates/clap) - Command-line argument parsing
- [qrcode](https://crates.io/crates/qrcode) - QR code generation
- [rqrr](https://crates.io/crates/rqrr) - QR code recognition
- [image](https://crates.io/crates/image) - Image processing
- [reqwest](https://crates.io/crates/reqwest) - HTTP client
