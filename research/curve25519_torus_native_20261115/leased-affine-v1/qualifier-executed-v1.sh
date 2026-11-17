#!/usr/bin/env bash
set -euo pipefail
leased_root="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)"
cd "$leased_root"
leased_out=${1:?expected NEW output directory}
test ! -e "$leased_out"
test ! -e "$leased_out.control.log"
sha256sum --check research/curve25519_torus_native_20261115/leased-affine-v1/SOURCES.sha256
printf '%s  %s\n' '939476d96d8111aef1cb651d4d529f998d96c87c5a78d6b7dcbb1668022a8eba' '/workspace/curve25519-native-binaries/leased-affine-control-v1' | sha256sum --check -
'/workspace/curve25519-native-binaries/leased-affine-control-v1' "$leased_out" > "$leased_out.control.log" 2>&1
