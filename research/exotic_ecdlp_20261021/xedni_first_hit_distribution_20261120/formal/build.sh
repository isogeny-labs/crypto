#!/bin/sh
set -eu
FORMAL_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
PINNED=/Volumes/SSD990/cryptanalysis/experiments/xedni-public-first-large-holdout-20261115/formal
cmp "$FORMAL_DIR/lean-toolchain" "$PINNED/lean-toolchain"
cmp "$FORMAL_DIR/lake-manifest.json" "$PINNED/lake-manifest.json"
cd "$PINNED"
lake env lean "$FORMAL_DIR/FirstHitBound.lean"
