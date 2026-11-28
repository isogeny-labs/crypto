#!/usr/bin/env bash
set -euo pipefail
indexed_root="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)"
cd "$indexed_root"
indexed_out=${1:?expected NEW output directory}
test ! -e "$indexed_out"
test ! -e "$indexed_out.control.log"
sha256sum --check research/curve25519_torus_native_20261115/indexed-circuit-v1/SOURCES.sha256
printf '%s  %s\n' '1bd10ec12e3ae459dde65cd14c5f69772edd1c6e00b8fe137dc85a0763b39bfc' '/workspace/curve25519-native-binaries/indexed-circuit-control-v1' | sha256sum --check -
'/workspace/curve25519-native-binaries/indexed-circuit-control-v1' "$indexed_out" > "$indexed_out.control.log" 2>&1
