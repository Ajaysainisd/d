#!/bin/bash
set -euo pipefail

REPO="Ajaysainisd/d"
BIN="d"
INSTALL_DIR="/usr/local/bin"

main() {
    local platform arch archive_url tmpdir

    platform=$(uname -s | tr '[:upper:]' '[:lower:]')
    arch=$(uname -m)

    case "$arch" in
        x86_64|amd64) arch="x86_64" ;;
        arm64|aarch64) arch="arm64" ;;
        *) echo "Unsupported architecture: $arch"; exit 1 ;;
    esac

    if [[ "$platform" == "darwin" ]]; then
        archive="d-macos-${arch}.tar.gz"
    elif [[ "$platform" == "linux" ]]; then
        archive="d-linux-x86_64.tar.gz"
    else
        echo "Unsupported platform: $platform"
        exit 1
    fi

    archive_url="https://github.com/${REPO}/releases/latest/download/${archive}"

    tmpdir=$(mktemp -d)
    trap "rm -rf $tmpdir" EXIT

    echo "Downloading d ${archive}..."
    curl -fsSL "$archive_url" -o "$tmpdir/$archive"

    echo "Extracting..."
    tar -xzf "$tmpdir/$archive" -C "$tmpdir"

    echo "Installing to ${INSTALL_DIR}/${BIN}..."
    sudo cp "$tmpdir/d-cli" "${INSTALL_DIR}/${BIN}"
    sudo chmod +x "${INSTALL_DIR}/${BIN}"

    echo "d installed successfully!"
    echo "Run 'd --help' to get started."
}

main
