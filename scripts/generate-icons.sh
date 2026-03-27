#!/bin/bash
# Generate PNG icons from SVG
# Requires: Inkscape or ImageMagick

SVG="src-tauri/icons/icon.svg"
OUT_DIR="src-tauri/icons"

if command -v convert &> /dev/null; then
    # ImageMagick
    convert -background none -resize 32x32 "$SVG" "$OUT_DIR/32x32.png"
    convert -background none -resize 128x128 "$SVG" "$OUT_DIR/128x128.png"
    convert -background none -resize 256x256 "$SVG" "$OUT_DIR/128x128@2x.png"
    convert -background none -resize 512x512 "$SVG" "$OUT_DIR/icon.png"
    echo "Icons generated with ImageMagick"
elif command -v inkscape &> /dev/null; then
    # Inkscape
    inkscape "$SVG" -w 32 -h 32 -o "$OUT_DIR/32x32.png"
    inkscape "$SVG" -w 128 -h 128 -o "$OUT_DIR/128x128.png"
    inkscape "$SVG" -w 256 -h 256 -o "$OUT_DIR/128x128@2x.png"
    inkscape "$SVG" -w 512 -h 512 -o "$OUT_DIR/icon.png"
    echo "Icons generated with Inkscape"
else
    echo "Please install ImageMagick or Inkscape to generate icons"
    echo "  macOS: brew install imagemagick"
    echo "  Linux: sudo apt install imagemagick"
    echo "  Windows: choco install imagemagick"
fi

# For macOS .icns and Windows .ico, use:
# macOS: png2icns or create via Xcode
# Windows: convert icon.png icon.ico