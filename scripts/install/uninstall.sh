#!/usr/bin/env bash
set -euo pipefail

BIN_TARGET=${BIN_TARGET:-/usr/local/bin}
APP_DIR=${APP_DIR:-/opt/qbit}

sudo rm -f "$BIN_TARGET/qbit"
sudo rm -f "$APP_DIR/qbit"

if [ -d "$APP_DIR" ] && [ -z "$(ls -A "$APP_DIR" 2>/dev/null)" ]; then
  sudo rmdir "$APP_DIR"
fi

echo "qbit removed from $APP_DIR and $BIN_TARGET/qbit"
