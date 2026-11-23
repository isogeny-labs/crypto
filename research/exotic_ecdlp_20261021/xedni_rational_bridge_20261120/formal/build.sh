#!/bin/sh
set -eu
HERE=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
BASE=$(CDPATH= cd -- "$HERE/../../xedni_lift_lean_certificate_20261120/formal" && pwd)
PINNED=/Volumes/SSD990/cryptanalysis/experiments/xedni-public-first-large-holdout-20261115/formal
cmp "$HERE/lean-toolchain" "$PINNED/lean-toolchain"
cmp "$HERE/lake-manifest.json" "$PINNED/lake-manifest.json"
sh "$BASE/build.sh"
cd "$PINNED"
lake env sh -c 'LEAN_PATH="$1:$LEAN_PATH" lean -R "$1" -o "$1/FullLift4096.olean" "$1/FullLift4096.lean"' sh "$BASE"
lake env sh -c 'LEAN_PATH="$1:$2:$LEAN_PATH" lean -R "$2" "$2/RationalBridge.lean"' sh "$BASE" "$HERE"
