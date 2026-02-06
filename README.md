# hyperfetch

A fast, configurable system info tool written in Rust. It prints a neofetch‑style summary with ASCII art, distro‑aware colors, and a wide range of hardware/OS details.

## Highlights
- Wide system info coverage: OS, kernel, uptime, packages, shell, display, CPU/GPU, memory, disks, network, battery, audio, locale, and more
- ASCII logos loaded from the repo’s `src/ascii` folder (or a custom folder)
- Neofetch‑style `$1..$9` and `$R` color tokens supported in ASCII assets
- Distro‑aware palette (e.g. Gentoo purple, macOS yellow)
- Multi‑battery aggregation (combined percent + time)
- JSON output for scripting
- Public IP lookup (feature‑gated)
- Linux + macOS support (macOS uses targeted fallbacks)

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
By default, ASCII art is loaded from the repo’s `src/ascii` folder. You can override it:
```bash
export HYPERFETCH_ASCII_DIR=/path/to/ascii
```

ASCII assets support neofetch‑style `$1..$9` and `$R` color tokens. The palette is selected by distro.

## macOS Notes
macOS uses fallbacks for:
- OS name/version (`sw_vers`)
- CPU model (`sysctl`)
- Uptime/boot time (`sysctl kern.boottime`)
- Battery (`pmset -g batt`)

## Public IP
Public IP lookup is behind the `network` feature and uses external services.

Enable it at build time:
```bash
cargo build --release --features network
```

## Development
Run in debug:
```bash
cargo run --
```

## License
No license file is currently included. Add one if you plan to distribute.
