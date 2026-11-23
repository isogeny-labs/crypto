#!/usr/bin/env bash
set -euo pipefail
contract_root="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)"
cd "$contract_root"
contract_out=${1:?expected NEW output directory}
test ! -e "$contract_out"
test ! -e "$contract_out.control.log"
sha256sum --check research/curve25519_torus_native_20261115/monomial-cancellation-v1/ENGINE_ROLLBACK_SOURCES.sha256
printf '%s  %s\n' 'a33cf3b2053489e8632f8a094f5d39a64fbc9fb20dae8db63c915f84a4c13625' '/workspace/curve25519-native-binaries/monomial-engine-rollback-v1' | sha256sum --check -
'/workspace/curve25519-native-binaries/monomial-engine-rollback-v1' "$contract_out" > "$contract_out.control.log" 2>&1
