#!/usr/bin/env bash
set -euo pipefail

if [[ "$(uname -s)" != "Linux" ]]; then
    echo "Error: This script is intended for Linux systems only." >&2
    exit 1
fi

for cmd in git cargo; do
    if ! command -v "$cmd" > /dev/null 2>&1; then
        echo "Error: '$cmd' is not installed. Please install it and try again." >&2
        exit 1
    fi
done

REPO_URL="https://github.com/Evren-os/rustor.git"
TEMP_DIR="$(mktemp -d)"
INSTALL_PATH="/usr/local/bin/rustor"

echo "Cloning rustor repository..."
git clone "$REPO_URL" "$TEMP_DIR"

cd "$TEMP_DIR"

echo "Building rustor with Cargo..."
cargo build --release

echo "Installing rustor to ${INSTALL_PATH}..."
if [ "$EUID" -ne 0 ]; then
    sudo mv target/release/rustor "${INSTALL_PATH}"
else
    mv target/release/rustor "${INSTALL_PATH}"
fi

echo "Cleaning up..."
rm -rf "$TEMP_DIR"

echo "Installation complete. You can now run 'rustor' to view your system info."
