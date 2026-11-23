#!/usr/bin/env bash
# Frozen run (PROTOCOL.md). Usage: bash research/koblitz_rho_reference_20261115/run.sh [n ...]
set -euo pipefail
ROOT="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)"
HERE="$ROOT/research/koblitz_rho_reference_20261115"
ECBENCH="${ECBENCH:-$ROOT/target/release/ecbench}"
LOCK="${CPU_LOCK:-/tmp/claude-0/-home-user/cba6917f-779a-544b-9ecf-84e512b45b33/scratchpad/cpu.lock}"
ulimit -v 9000000
mkdir -p "$HERE/sessions" "$HERE/audits" "$HERE/logs"
NS="${*:-17 23 29 31 37}"
for n in $NS; do
  spec="$HERE/specs/n$n.json"; name="n$n"; out="$HERE/sessions/$name"
  [ -f "$out/session.json" ] && { echo "skip $name"; continue; }
  ( TIMEFORMAT="wall_s=%R user_s=%U"; time flock "$LOCK" "$ECBENCH" run --wait --spec "$spec" --out "$out" --cpus none --allow-busy > "$HERE/logs/run-$name.log" 2>&1 ) 2> "$HERE/logs/run-$name.time" || echo "run $name failed"
  "$ECBENCH" verify --dir "$out" --replay-all --out "$HERE/audits/audit-$name.json" > "$HERE/logs/verify-$name.log" 2>&1 || echo "verify $name failed"
  echo "done $name $(date -u +%T)"
done
echo RUN-DONE
