#!/bin/sh
set -eu
FORMAL_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
PRIOR_DIR=$(CDPATH= cd -- "$FORMAL_DIR/../../xedni_fixed_curve_lean_bridge_20261121/formal" && pwd)
PINNED=/Volumes/SSD990/cryptanalysis/experiments/xedni-public-first-large-holdout-20261115/formal
cmp "$FORMAL_DIR/lean-toolchain" "$PINNED/lean-toolchain"
cmp "$FORMAL_DIR/lake-manifest.json" "$PINNED/lake-manifest.json"
cd "$PINNED"
lake env lean -R "$PRIOR_DIR" -o "$FORMAL_DIR/FixedCurveSquareDenominator.olean" "$PRIOR_DIR/FixedCurveSquareDenominator.lean"
lake env sh -c 'LEAN_PATH="$1:$LEAN_PATH" lean -R "$1" "$1/NaiveHeight.lean"' sh "$FORMAL_DIR"
