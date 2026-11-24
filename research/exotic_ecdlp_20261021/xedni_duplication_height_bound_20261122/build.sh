#!/bin/sh
set -eu
HERE=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
PINNED=/Volumes/SSD990/cryptanalysis/experiments/xedni-public-first-large-holdout-20261115/formal
CANONICAL=$(CDPATH= cd -- "$HERE/../xedni_canonical_height_limit_20261122/formal" && pwd)
cmp "$HERE/lean-toolchain" "$PINNED/lean-toolchain"
cmp "$HERE/lake-manifest.json" "$PINNED/lake-manifest.json"
cmp "$CANONICAL/lean-toolchain" "$PINNED/lean-toolchain"
cmp "$CANONICAL/lake-manifest.json" "$PINNED/lake-manifest.json"
python3 "$HERE/generate_lean_cells.py" --check
cd "$PINNED"
lake env lean -R "$HERE" -o "$HERE/Bounds.olean" "$HERE/Bounds.lean"
lake env sh -c 'LEAN_PATH="$1:$LEAN_PATH" lean -R "$1" -o "$1/Cells.olean" "$1/Cells.lean"' sh "$HERE"
lake env sh -c 'LEAN_PATH="$1:$LEAN_PATH" lean -R "$1" -o "$1/Global.olean" "$1/Global.lean"' sh "$HERE"
lake env sh -c 'LEAN_PATH="$1:$LEAN_PATH" lean -R "$1" -o "$1/PointBridge.olean" "$1/PointBridge.lean"' sh "$HERE"
lake env sh -c 'LEAN_PATH="$1:$LEAN_PATH" lean -R "$1" -o "$1/IntegerBridge.olean" "$1/IntegerBridge.lean"' sh "$HERE"
lake env sh -c 'LEAN_PATH="$1:$LEAN_PATH" lean -R "$1" -o "$1/RationalBridge.olean" "$1/RationalBridge.lean"' sh "$HERE"
lake env sh -c 'LEAN_PATH="$1:$LEAN_PATH" lean -R "$1" -o "$1/OutputBridge.olean" "$1/OutputBridge.lean"' sh "$HERE"
lake env sh -c 'LEAN_PATH="$1:$LEAN_PATH" lean -R "$1" -o "$1/UpperBridge.olean" "$1/UpperBridge.lean"' sh "$HERE"
lake env sh -c 'LEAN_PATH="$1:$LEAN_PATH" lean -R "$1" -o "$1/HeightBridge.olean" "$1/HeightBridge.lean"' sh "$HERE"
lake env sh -c 'LEAN_PATH="$1:$LEAN_PATH" lean -R "$1" -o "$1/TorsionBridge.olean" "$1/TorsionBridge.lean"' sh "$HERE"
lake env lean -R "$CANONICAL" -o "$HERE/CanonicalLimit.olean" "$CANONICAL/CanonicalLimit.lean"
lake env sh -c 'LEAN_PATH="$1:$LEAN_PATH" lean -R "$1" "$1/CanonicalBridge.lean"' sh "$HERE"
