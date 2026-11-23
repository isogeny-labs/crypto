#!/bin/sh
set -eu
HERE=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
PINNED=/Volumes/SSD990/cryptanalysis/experiments/xedni-public-first-large-holdout-20261115/formal
cmp "$HERE/lean-toolchain" "$PINNED/lean-toolchain"
cmp "$HERE/lake-manifest.json" "$PINNED/lake-manifest.json"
cd "$PINNED"
lake env lean -R "$HERE" "$HERE/PairShifts.lean"
