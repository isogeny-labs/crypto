#!/bin/sh
set -eu
HERE=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
ROOT=$(CDPATH= cd -- "$HERE/../.." && pwd)
PINNED=/Volumes/SSD990/cryptanalysis/experiments/xedni-public-first-large-holdout-20261115/formal
TANGENT="$ROOT/xedni_collision_tangent_obstruction_20261120/formal"
cmp "$HERE/lean-toolchain" "$PINNED/lean-toolchain"
cmp "$HERE/lake-manifest.json" "$PINNED/lake-manifest.json"
cd "$PINNED"
lake env lean -R "$TANGENT" -o "$HERE/Tangent.olean" "$TANGENT/Tangent.lean"
lake env sh -c 'LEAN_PATH="$1:$LEAN_PATH" lean -R "$1" -o "$1/Shift.olean" "$1/Shift.lean"' sh "$HERE"
lake env sh -c 'LEAN_PATH="$1:$LEAN_PATH" lean -R "$1" "$1/Cases.lean"' sh "$HERE"
