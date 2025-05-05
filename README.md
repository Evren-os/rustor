# Rustor

A blazing-fast, minimal CLI for your system info.

[![Preview](./assets/preview.png)](./assets/preview.png)

## Why Rustor?
- ⚡ <1 ms startup (smart 1 h cache)
- 🌈 Modern ANSI colors
- 🤝 Cross-distro OS-age detection (pacman, dpkg, installer logs, or fs fallback)

## Features
- 👤 User & Hostname  
- 🐧 Kernel version  
- ⏱️ Uptime (d h m)  
- 📆 OS age (from logs or filesystem)  
- 💾 Memory usage (GiB)  
- 🔄 Smart caching (TTL 1 h)

## Install
```bash
curl -fsSL https://raw.githubusercontent.com/Evren-os/rustor/main/install.sh | bash
```
or
```bash
git clone https://github.com/Evren-os/rustor.git
cd rustor
cargo build --release
mv target/release/rustor ~/.local/bin/
```

## Usage
```bash
rustor
```
