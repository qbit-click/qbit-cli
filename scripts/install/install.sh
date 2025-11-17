#!/usr/bin/env bash
set -euo pipefail

BIN_TARGET=${BIN_TARGET:-/usr/local/bin}
APP_DIR=${APP_DIR:-/opt/qbit}

sudo mkdir -p "$APP_DIR"
sudo cp qbit-cli "$APP_DIR/qbit"
sudo chmod +x "$APP_DIR/qbit"
sudo ln -sf "$APP_DIR/qbit" "$BIN_TARGET/qbit"

echo "qbit installed to $APP_DIR and symlinked at $BIN_TARGET/qbit"
