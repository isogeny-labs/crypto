#!/bin/sh
set -eu
HERE=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
ROOT=$(CDPATH= cd -- "$HERE/../.." && pwd)
PINNED=/Volumes/SSD990/cryptanalysis/experiments/xedni-public-first-large-holdout-20261115/formal
TANGENT="$ROOT/xedni_collision_tangent_obstruction_20261120/formal"
SAMEX="$ROOT/xedni_same_x_obstruction_20261120/formal"
cmp "$HERE/lean-toolchain" "$PINNED/lean-toolchain"
cmp "$HERE/lake-manifest.json" "$PINNED/lake-manifest.json"
cd "$PINNED"
lake env lean -R "$TANGENT" -o "$HERE/Tangent.olean" "$TANGENT/Tangent.lean"
lake env lean -R "$SAMEX" -o "$HERE/SameX.olean" "$SAMEX/SameX.lean"
lake env sh -c 'LEAN_PATH="$1:$LEAN_PATH" lean -R "$1" "$1/FullSource.lean"' sh "$HERE"
