#!/usr/bin/env bash
set -euo pipefail
branch_audit_root="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)"
cd "$branch_audit_root"
branch_audit_out=${1:?expected NEW audit output directory}
test ! -e "$branch_audit_out"
printf '%s  %s\n' '2b246aa058d7310484a85a22bef245d9ebe3b869331b790f2c6a476ba41296f2' '/workspace/curve25519-native-binaries/branch-affine-postcall-audit-v1' | sha256sum --check -
printf '%s  %s\n' '378d9e4b2acfb5b553196c9a125cd477c362120af12a7b4ed0aa8fbf0a1f0b53' 'research/curve25519_torus_native_20261115/branch-affine-v1/audit-v1.rs' | sha256sum --check -
'/workspace/curve25519-native-binaries/branch-affine-postcall-audit-v1' 'research/curve25519_torus_native_20261115/branch-affine-v1/native-v1' "$branch_audit_out"
