#!/usr/bin/env bash
set -euo pipefail
worklist_root="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)"
cd "$worklist_root"
worklist_out=${1:?expected NEW output directory}
test ! -e "$worklist_out"
test ! -e "$worklist_out.control.log"
sha256sum --check research/curve25519_torus_native_20261115/worklist-solver-v1/SOURCES.sha256
printf '%s  %s\n' '0ae79df0ef371e82fac4b1cd5fea6418b8e9b5fd20dc0533d076774bcce8b01e' '/workspace/curve25519-native-binaries/worklist-solver-control-v1' | sha256sum --check -
'/workspace/curve25519-native-binaries/worklist-solver-control-v1' "$worklist_out" > "$worklist_out.control.log" 2>&1
