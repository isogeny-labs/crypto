#!/usr/bin/env bash
# Pilot (pilot/SELECTION_RULE.md). Usage: ECBENCH=<binary> bash research/koblitz_k1_sibwalk_20261118/run_pilot.sh
set -euo pipefail
ROOT="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)"
HERE="$ROOT/research/koblitz_k1_sibwalk_20261118"
ECBENCH="${ECBENCH:?}"
LOCK="${CPU_LOCK:-/tmp/claude-0/-home-user/cba6917f-779a-544b-9ecf-84e512b45b33/scratchpad/cpu.lock}"
SESS="${SESS:-/tmp/claude-0/-home-user/cba6917f-779a-544b-9ecf-84e512b45b33/scratchpad/sw-pilot}"
ulimit -v 9000000
mkdir -p "$SESS" "$HERE/pilot/logs"
sha256sum "$ECBENCH" | sed "s#  .*#  $(basename "$ECBENCH")#" > "$HERE/pilot/logs/binary.sha256"
for n in 23 29 37; do
  out="$SESS/pilot-n$n"
  [ -f "$out/session.json" ] || flock "$LOCK" "$ECBENCH" run --wait --spec "$HERE/pilot/pilot-n$n.json" --out "$out" --cpus none --allow-busy > "$SESS/run-n$n.log" 2>&1
  flock "$LOCK" "$ECBENCH" verify --dir "$out" --replay-all --out "$SESS/audit-n$n.json" > "$SESS/verify-n$n.log" 2>&1
  gzip -9 -c "$SESS/audit-n$n.json" > "$HERE/pilot/logs/audit-pilot-n$n.json.gz"
  echo "pilot n$n done"
done
