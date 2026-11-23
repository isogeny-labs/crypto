#!/usr/bin/env bash
set -euo pipefail
branch_root="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)"
cd "$branch_root"
branch_out=${1:?expected NEW output directory}
test ! -e "$branch_out"
test ! -e "$branch_out.control.log"
sha256sum --check research/curve25519_torus_native_20261115/branch-affine-v1/SOURCES.sha256
printf '%s  %s\n' '87e5872e4f5743884237b56b740a27be6f03c2e1127adde1ccf60c149ebcfb4f' '/workspace/curve25519-native-binaries/branch-affine-control-v1' | sha256sum --check -
'/workspace/curve25519-native-binaries/branch-affine-control-v1' "$branch_out" > "$branch_out.control.log" 2>&1
