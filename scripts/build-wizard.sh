#!/usr/bin/env zsh
set -euo pipefail

# Build the setupwizard Rust project and copy the release binary into the iso airootfs

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
WIZARD_DIR="$ROOT_DIR/setupwizard"
MANIFEST="$WIZARD_DIR/Cargo.toml"
BIN_NAME="setupwizard"
RELEASE_BIN="$WIZARD_DIR/target/release/$BIN_NAME"
DEST_DIR="$ROOT_DIR/iso/airootfs/usr/bin"

echo "root: $ROOT_DIR"
echo "building $BIN_NAME from $MANIFEST"

if [ ! -f "$MANIFEST" ]; then
	echo "Cargo manifest not found at $MANIFEST" >&2
	exit 1
fi

if ! command -v cargo >/dev/null 2>&1; then
	echo "cargo not found in PATH. Please install Rust toolchain (cargo)." >&2
	exit 1
fi

pushd "$WIZARD_DIR" >/dev/null
echo "Running: cargo build --release --locked"
# use --locked when Cargo.lock is present; will work either way
cargo build --release
popd >/dev/null

if [ ! -f "$RELEASE_BIN" ]; then
	echo "Expected release binary not found: $RELEASE_BIN" >&2
	exit 1
fi

echo "Creating destination directory: $DEST_DIR"
mkdir -p "$DEST_DIR"

echo "Copying $RELEASE_BIN -> $DEST_DIR/$(basename "$RELEASE_BIN")"
cp -f "$RELEASE_BIN" "$DEST_DIR/"
chmod 755 "$DEST_DIR/$(basename "$RELEASE_BIN")"

echo "Build and install complete. Installed binary: $DEST_DIR/$(basename "$RELEASE_BIN")"

exit 0

