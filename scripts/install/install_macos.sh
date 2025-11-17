#!/usr/bin/env bash
set -euo pipefail

BIN_TARGET=${BIN_TARGET:-/usr/local/bin}
APP_DIR=${APP_DIR:-/Applications/QbitCLI}

sudo mkdir -p "$APP_DIR/Contents/MacOS"
sudo mkdir -p "$APP_DIR/Contents/Resources"

sudo cp qbit-cli "$APP_DIR/Contents/MacOS/qbit"
sudo chmod +x "$APP_DIR/Contents/MacOS/qbit"

cat <<'APP' | sudo tee "$APP_DIR/Contents/Info.plist" >/dev/null
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleExecutable</key>
  <string>qbit</string>
  <key>CFBundleIdentifier</key>
  <string>com.example.qbit</string>
  <key>CFBundleName</key>
  <string>Qbit CLI</string>
  <key>CFBundleIconFile</key>
  <string>icon.icns</string>
</dict>
</plist>
APP

# 📝 یک فایل icon.icns واقعی در این مسیر قرار دهید.

sudo ln -sf "$APP_DIR/Contents/MacOS/qbit" "$BIN_TARGET/qbit"
echo "Qbit CLI app installed. You may run 'qbit' globally."
