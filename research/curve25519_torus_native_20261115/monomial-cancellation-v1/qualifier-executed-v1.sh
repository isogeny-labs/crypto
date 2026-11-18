#!/usr/bin/env bash
set -euo pipefail
monomial_root="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)"
cd "$monomial_root"
monomial_out=${1:?expected NEW output directory}
test ! -e "$monomial_out"
test ! -e "$monomial_out.control.log"
sha256sum --check research/curve25519_torus_native_20261115/monomial-cancellation-v1/SOURCES.sha256
printf '%s  %s\n' '7c3f639c461a4cfaa7d480f0e26d044708f2246357e36d274e3737b84fa4c6b5' '/workspace/curve25519-native-binaries/monomial-cancellation-control-v1' | sha256sum --check -
'/workspace/curve25519-native-binaries/monomial-cancellation-control-v1' "$monomial_out" > "$monomial_out.control.log" 2>&1
