#!/usr/bin/env bash
# Builds a .deb package from the release binary.
#
# Usage:
#   ./build-linux-deb.sh <version> <arch> <path-to-qbit-binary> <output-dir>
#
# Example:
#   ./build-linux-deb.sh 1.0.0 amd64 target/release/qbit dist

set -euo pipefail

VERSION="${1:?version required, e.g. 1.0.0}"
ARCH="${2:?arch required, e.g. amd64}"
BINARY_PATH="${3:?path to built qbit binary required}"
OUT_DIR="${4:-dist}"

if [ ! -f "$BINARY_PATH" ]; then
  echo "error: binary not found at $BINARY_PATH" >&2
  exit 1
fi

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CONTROL_TEMPLATE="$SCRIPT_DIR/../linux/debian/control.template"

STAGE_DIR="$(mktemp -d)"
trap 'rm -rf "$STAGE_DIR"' EXIT

PKG_NAME="cli-qbit_${VERSION}_${ARCH}"
PKG_ROOT="$STAGE_DIR/$PKG_NAME"

# Payload: the binary must land at /usr/bin/qbit (checklist requirement)
mkdir -p "$PKG_ROOT/usr/bin"
mkdir -p "$PKG_ROOT/DEBIAN"

cp "$BINARY_PATH" "$PKG_ROOT/usr/bin/qbit"
chmod 755 "$PKG_ROOT/usr/bin/qbit"

# Render control file with real version/arch
sed -e "s/__VERSION__/$VERSION/" -e "s/__ARCH__/$ARCH/" \
  "$CONTROL_TEMPLATE" > "$PKG_ROOT/DEBIAN/control"

mkdir -p "$OUT_DIR"
OUT_FILE="$OUT_DIR/${PKG_NAME}.deb"

dpkg-deb --build --root-owner-group "$PKG_ROOT" "$OUT_FILE"

echo "Built: $OUT_FILE"

# Sanity checks matching the checklist's acceptance criteria
echo "--- dpkg-deb info ---"
dpkg-deb --info "$OUT_FILE"
echo "--- payload contents ---"
dpkg-deb --contents "$OUT_FILE"
