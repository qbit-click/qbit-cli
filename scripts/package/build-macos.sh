#!/usr/bin/env bash
# Builds a .pkg installer from the release binary.
#
# Usage:
#   ./build-macos.sh <version> <arch> <path-to-qbit-binary> <output-dir>
#
# Example:
#   ./build-macos.sh 1.0.0 arm64 target/release/qbit dist

set -euo pipefail

VERSION="${1:?version required, e.g. 1.0.0}"
ARCH="${2:?arch required, e.g. arm64 or x86_64}"
BINARY_PATH="${3:?path to built qbit binary required}"
OUT_DIR="${4:-dist}"

if [ ! -f "$BINARY_PATH" ]; then
  echo "error: binary not found at $BINARY_PATH" >&2
  exit 1
fi

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
MACOS_PKG_DIR="$REPO_ROOT/packaging/macos"

STAGE_DIR="$(mktemp -d)"
trap 'rm -rf "$STAGE_DIR"' EXIT

PAYLOAD_DIR="$STAGE_DIR/payload"
mkdir -p "$PAYLOAD_DIR/usr/local/bin"

cp "$BINARY_PATH" "$PAYLOAD_DIR/usr/local/bin/qbit"
chmod 755 "$PAYLOAD_DIR/usr/local/bin/qbit"

# Render distribution.xml with real version
sed "s/__VERSION__/$VERSION/" "$MACOS_PKG_DIR/distribution.xml" > "$STAGE_DIR/distribution.xml"

mkdir -p "$OUT_DIR"

# Step 1: build the component package (the actual payload)
# Note: no --component-plist here. That option is for .app bundles
# (it declares bundle relocation/versioning behavior); our payload is
# a plain binary at usr/local/bin/qbit, so --root + --install-location
# is sufficient and correct.
pkgbuild \
  --root "$PAYLOAD_DIR" \
  --identifier "com.qbit-click.qbit-cli" \
  --version "$VERSION" \
  --install-location "/" \
  "$STAGE_DIR/component.pkg"

# Step 2: wrap it in a product archive using the distribution.xml
PKG_NAME="qbit-cli-${VERSION}-macos-${ARCH}.pkg"
OUT_FILE="$OUT_DIR/$PKG_NAME"

productbuild \
  --distribution "$STAGE_DIR/distribution.xml" \
  --package-path "$STAGE_DIR" \
  "$OUT_FILE"

echo "Built: $OUT_FILE"

# Sanity checks
echo "--- pkgutil payload check ---"
pkgutil --payload-files "$OUT_FILE" 2>/dev/null || pkgutil --expand "$OUT_FILE" "$STAGE_DIR/expanded" && find "$STAGE_DIR/expanded" -maxdepth 2
