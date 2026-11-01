#!/usr/bin/env bash
set -euo pipefail
base=/var/tmp/pencil-graph-forest-review-20261101/native-verification
out=$(mktemp -d "$base/false-XXXXXXXX")
printf 'example : False := by\n  trivial\n' > "$out/FalseControl.lean"
cd "$out"
export LEAN_PATH=/var/tmp/pencil-mumford-validity-20261031-v1/dependency-union:/var/tmp/pencil-sameu-global-smooth-20261031-v1/dependency-union
set +e
timeout 120 /var/tmp/pencil-same-support-lean-local-v1-wco_em6r/toolchain/bin/lean -j1 -M4096 FalseControl.lean > stdout 2> stderr
code=$?
set -e
printf '{"native_exit":%s,"gate_exit":%s}\n' "$code" "$code" > receipt.json
printf '%s\n' "$out"
exit "$code"
