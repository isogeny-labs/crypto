#!/usr/bin/env bash
set -euo pipefail
D=/var/tmp/pencil-nonmonic-division-20261101-v1/native-false-control
cd "$D"
export LEAN_PATH=/var/tmp/pencil-mumford-validity-20261031-v1/dependency-union:/var/tmp/pencil-sameu-global-smooth-20261031-v1/dependency-union
set +e
timeout 120 /var/tmp/pencil-same-support-lean-local-v1-wco_em6r/toolchain/bin/lean -j1 -M4096 FalseControl.lean > stdout 2> stderr
code=$?
set -e
printf '{"native_exit":%s,"gate_exit":%s}\n' "$code" "$code" > receipt.json
exit "$code"
