#!/usr/bin/env bash
set -euo pipefail

# Repository to install from
readonly REPO_URL='https://github.com/Evren-os/rustor.git'

# Install root defaults to ~/.local so that binaries land in ~/.local/bin
readonly INSTALL_ROOT="${INSTALL_ROOT:-$HOME/.local}"
readonly INSTALL_BIN="${INSTALL_ROOT}/bin"

# Ensure the target bin directory exists
mkdir -p "$INSTALL_BIN"

# Check for Cargo
if ! command -v cargo >/dev/null 2>&1; then
  echo "Error: cargo not found. Please install the Rust toolchain first." >&2
  exit 1
fi

echo "Installing rustor from ${REPO_URL} into ${INSTALL_BIN}…"
cargo install \
  --git "$REPO_URL" \
  --locked \
  --force \
  --root "$INSTALL_ROOT" \
  --bin rustor

echo "✔️  Installation complete!"
echo "   rustor is now at ${INSTALL_BIN}/rustor"
