#!/usr/bin/env bash
set -euo pipefail

BIN_TARGET=${BIN_TARGET:-/usr/local/bin}
APP_DIR=${APP_DIR:-/Applications/QbitCLI}

sudo rm -f "$BIN_TARGET/qbit"
sudo rm -rf "$APP_DIR"

echo "qbit removed from $APP_DIR and $BIN_TARGET/qbit"
