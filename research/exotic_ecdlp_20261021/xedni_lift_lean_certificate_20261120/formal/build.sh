#!/bin/sh
set -eu
FORMAL_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
PINNED=/Volumes/SSD990/cryptanalysis/experiments/xedni-public-first-large-holdout-20261115/formal
cmp "$FORMAL_DIR/lean-toolchain" "$PINNED/lean-toolchain"
cmp "$FORMAL_DIR/lake-manifest.json" "$PINNED/lake-manifest.json"
cd "$PINNED"
lake env lean "$FORMAL_DIR/BoundedLift64.lean"
lake env lean -R "$FORMAL_DIR" -o "$FORMAL_DIR/CandidateListProof.olean" "$FORMAL_DIR/CandidateListProof.lean"
lake env lean -R "$FORMAL_DIR" -o "$FORMAL_DIR/SquareModulo.olean" "$FORMAL_DIR/SquareModulo.lean"
lake env sh -c 'LEAN_PATH="$1:$LEAN_PATH" lean -R "$1" -o "$1/Sieve4096.olean" "$1/Sieve4096.lean"' sh "$FORMAL_DIR"
lake env sh -c 'LEAN_PATH="$1:$LEAN_PATH" lean -R "$1" "$1/FullLift4096.lean"' sh "$FORMAL_DIR"
