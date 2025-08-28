#!/usr/bin/env bash
set -euo pipefail

# mkarchiso wrapper for Asenos-iso
# Produces: asenos-YYYY.MM-x86_64.iso

PROGNAME=$(basename "$0")
ROOT_DIR=$(cd "$( dirname "${BASH_SOURCE[0]}" )/.." && pwd)

# Defaults
ISO_DIR="$ROOT_DIR/iso"
WORK_DIR="$ROOT_DIR/work"
OUT_DIR="$ROOT_DIR/out"
ARCH="x86_64"
VERSION_OVERRIDE=""
KEEP_WORK=false

usage() {
	cat <<EOF
Usage: $PROGNAME [options]

Options:
	-h            Show this help
	-k            Keep work directory after successful build
	-v VERSION    Override version string (format: YYYY.MM)
	-w DIR        Work directory (default: $WORK_DIR)
	-o DIR        Output directory (default: $OUT_DIR)
	-i DIR        ISO directory (default: $ISO_DIR)

The created ISO will be named: asenos-<version>-${ARCH}.iso
If -v is not provided the script uses the current year.month (YYYY.MM).
EOF
}

while getopts ":hkv:w:o:i:" opt; do
	case $opt in
		h) usage; exit 0 ;;
		k) KEEP_WORK=true ;;
		v) VERSION_OVERRIDE="$OPTARG" ;;
		w) WORK_DIR="$OPTARG" ;;
		o) OUT_DIR="$OPTARG" ;;
		i) ISO_DIR="$OPTARG" ;;
		:) echo "Error: -$OPTARG requires an argument" >&2; usage; exit 2 ;;
		\?) echo "Unknown option: -$OPTARG" >&2; usage; exit 2 ;;
	esac
done

if [[ -z "${VERSION_OVERRIDE}" ]]; then
	VERSION=$(date +%Y.%m)
else
	VERSION="$VERSION_OVERRIDE"
fi

ISO_NAME="asenos-${VERSION}-${ARCH}.iso"

echo "ISO dir:    $ISO_DIR"
echo "Work dir:   $WORK_DIR"
echo "Out dir:    $OUT_DIR"
echo "Version:    $VERSION"
echo "ISO name:   $ISO_NAME"

mkdir -p "$WORK_DIR" "$OUT_DIR"

echo "Running mkarchiso..."
# run mkarchiso using the provided iso directory
scripts/mkarchiso.sh -v -w "$WORK_DIR" -o "$OUT_DIR" "$ISO_DIR"

echo "mkarchiso completed. locating produced ISO(s)..."

# find the first ISO in the output dir
shopt -s nullglob
isos=("$OUT_DIR"/*.iso)
shopt -u nullglob

if [[ ${#isos[@]} -eq 0 ]]; then
	echo "Error: no ISO found in $OUT_DIR" >&2
	exit 4
fi

if [[ ${#isos[@]} -gt 1 ]]; then
	echo "Warning: multiple ISO files found in $OUT_DIR, will pick the newest one." >&2
fi

# choose newest ISO by modification time (avoid parsing ls output)
# Use find to print modification time and path, sort by time and extract the path.
selected_iso=$(find "$OUT_DIR" -maxdepth 1 -type f -name '*.iso' -printf '%T@ %p\n' | sort -nr | head -n1 | cut -d' ' -f2-)

dest="$OUT_DIR/$ISO_NAME"

echo "Renaming $selected_iso -> $dest"
mv -f -- "$selected_iso" "$dest"

if [[ "$KEEP_WORK" == false ]]; then
	echo "Removing work directory $WORK_DIR"
	rm -rf "$WORK_DIR"
fi

echo "Build complete. ISO available at: $dest"
