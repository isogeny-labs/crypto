#!/bin/sh
set -eu
HERE=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
BASE=$(CDPATH= cd -- "$HERE/../../xedni_lift_lean_certificate_20261120/formal" && pwd)
RATIONAL=$(CDPATH= cd -- "$HERE/../../xedni_rational_bridge_20261120/formal" && pwd)
SQUARE=$(CDPATH= cd -- "$HERE/../../xedni_square_denominator_20261120/formal" && pwd)
PINNED=/Volumes/SSD990/cryptanalysis/experiments/xedni-public-first-large-holdout-20261115/formal
cmp "$HERE/lean-toolchain" "$PINNED/lean-toolchain"
cmp "$HERE/lake-manifest.json" "$PINNED/lake-manifest.json"
sh "$SQUARE/build.sh"
cd "$PINNED"
lake env sh -c 'LEAN_PATH="$1:$2:$3:$LEAN_PATH" lean -R "$3" -o "$3/FullAffine4096.olean" "$3/FullAffine4096.lean"' sh "$BASE" "$RATIONAL" "$SQUARE"
lake env sh -c 'LEAN_PATH="$1:$2:$3:$4:$LEAN_PATH" lean -R "$4" "$4/PointBridge.lean"' sh "$BASE" "$RATIONAL" "$SQUARE" "$HERE"
