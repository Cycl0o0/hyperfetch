# hyperfetch

A fast, configurable system info tool written in Rust. It prints a neofetch-style summary with ASCII art, colorized labels, and a large set of hardware/OS details.

## Features
- Wide set of system info: OS, kernel, uptime, packages, shell, display, CPU/GPU, memory, disks, network, battery, and more
- ASCII art logos with distro-aware color palettes
- Multi-battery aggregation (combined percent and time)
- Optional public IP lookup (feature-gated)
- JSON output for scripting
- Supports Linux and macOS (with macOS-specific fallbacks)

## Install
Build from source:
```bash
cargo build --release
```
Run:
```bash
./target/release/hyperfetch
```

## Usage
```bash
hyperfetch [OPTIONS]
```

Options:
- `-c, --config <FILE>`: Use custom config file
- `-a, --ascii <DISTRO>`: Use specific distro ASCII art
- `--no-ascii`: Disable ASCII art
- `--no-colors`: Disable colored output
- `-s, --small`: Use small ASCII art
- `-l, --logo-only`: Print only the ASCII logo
- `-j, --json`: Output as JSON
- `--public-ip`: Fetch and display public IP with geolocation
- `--list-logos`: List available ASCII logos

Examples:
```bash
hyperfetch
hyperfetch --small
hyperfetch --no-ascii
hyperfetch --logo-only
hyperfetch --json
hyperfetch --ascii gentoo
hyperfetch --public-ip
```

## Configuration
Default config path:
- Linux: `~/.config/hyperfetch/config.toml`
- macOS: `~/Library/Application Support/hyperfetch/config.toml`

Example config:
```toml
[display]
show_ascii = true
show_colors = true
small_ascii = false
ascii_distro = "gentoo"

[colors]
# "auto" or "distro" uses the ASCII palette for label colors
primary = "auto"
secondary = "white"

[info]
os = true
kernel = true
hostname = true
uptime = true
packages = true
shell = true
resolution = true
de = true
wm = true
theme = true
icons = true
terminal = true
cpu = true
gpu = true
memory = true
disk = true
network = true
battery = true
public_ip = false
```

## ASCII Art
By default, ASCII art is loaded from the repo’s `src/ascii` folder. You can override the folder using:
```bash
export HYPERFETCH_ASCII_DIR=/path/to/ascii
```

The ASCII assets support neofetch-style `$1..$9` and `$R` color tokens. The palette is chosen by distro.

## macOS Support is experimental currently
macOS is supported with fallbacks for:
- OS name/version
- CPU model
- Uptime and boot time

Battery info on macOS is read via `pmset -g batt`.

## Public IP
Public IP lookup is gated behind the `network` feature. If you enable it, it queries an external service.

## Development
Run in debug:
```bash
cargo run --
```

## License
AGPLv3
