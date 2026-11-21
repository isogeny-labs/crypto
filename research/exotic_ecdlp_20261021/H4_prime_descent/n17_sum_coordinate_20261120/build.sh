#!/bin/sh
set -eu
FORMAL_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
PINNED_PROJECT=/Volumes/SSD990/cryptanalysis/experiments/xedni-public-first-large-holdout-20261115/formal
test "$(shasum -a 256 "$FORMAL_DIR/TripleSum.lean" | cut -d ' ' -f 1)" = f94535738082a51e5c813a1b7f97c96776e50af4ffab24b30a989831e9617675
cmp "$FORMAL_DIR/lean-toolchain" "$PINNED_PROJECT/lean-toolchain"
cmp "$FORMAL_DIR/lake-manifest.json" "$PINNED_PROJECT/lake-manifest.json"
test "$(git -C "$PINNED_PROJECT/.lake/packages/mathlib" rev-parse HEAD)" = 021c46bdd042f67b538b9f9871d556a93713a9a9
cd "$PINNED_PROJECT"
lake env lean "$FORMAL_DIR/TripleSum.lean"
