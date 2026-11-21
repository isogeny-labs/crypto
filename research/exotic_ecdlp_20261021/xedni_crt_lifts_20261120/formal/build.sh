#!/bin/sh
set -eu
FORMAL_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
PINNED=/Volumes/SSD990/cryptanalysis/experiments/xedni-public-first-large-holdout-20261115/formal
test "$(shasum -a 256 "$FORMAL_DIR/lean-toolchain" | cut -d ' ' -f 1)" = 8f89aa44fccdb1a0b6cc768c7c9f54b94cd40e47a08d582f82d1235f13075cf7
test "$(shasum -a 256 "$FORMAL_DIR/lake-manifest.json" | cut -d ' ' -f 1)" = 08e342212c26f112b3bf40d51cbfab0950c47a997a8d473dd6f193dc920ede10
cmp "$FORMAL_DIR/lean-toolchain" "$PINNED/lean-toolchain"
cmp "$FORMAL_DIR/lake-manifest.json" "$PINNED/lake-manifest.json"
cd "$PINNED"
lake env lean "$FORMAL_DIR/Witness.lean"
