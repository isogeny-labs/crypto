#!/usr/bin/env bash
set -euo pipefail
affine_root="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)"
cd "$affine_root"
affine_out=${1:?expected NEW output directory}
test ! -e "$affine_out"
test ! -e "$affine_out.control.log"
sha256sum --check research/curve25519_torus_native_20261115/affine-closure-v1/SOURCES.sha256
printf '%s  %s\n' '54ad26145290b1228e4f45c6669176ae4b79ca17c7f187441bc9b6c2b7d1de7d' '/workspace/curve25519-native-binaries/affine-closure-control-v1' | sha256sum --check -
'/workspace/curve25519-native-binaries/affine-closure-control-v1' "$affine_out" > "$affine_out.control.log" 2>&1
