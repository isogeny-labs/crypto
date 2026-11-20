#!/usr/bin/env bash
# Frozen run (PROTOCOL.md). Usage: bash research/koblitz_known_log_columns_20261118/a1/run.sh [n ...]
# Sessions go to the scratchpad (SESS); only records digests, audits, tables land in git.
set -euo pipefail
ROOT="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)"
HERE="$ROOT/research/koblitz_known_log_columns_20261118/a1"
ECBENCH="${ECBENCH:-$ROOT/target/release/ecbench}"
LOCK="${CPU_LOCK:-/tmp/claude-0/-home-user/cba6917f-779a-544b-9ecf-84e512b45b33/scratchpad/cpu.lock}"
SESS="${SESS:-/tmp/claude-0/-home-user/cba6917f-779a-544b-9ecf-84e512b45b33/scratchpad/klc-sessions-a1}"
ulimit -v 9000000
mkdir -p "$SESS" "$HERE/audits" "$HERE/logs"
NS="${*:-23 37}"
for n in $NS; do
  spec="$HERE/specs/n$n.json"; name="n$n"; out="$SESS/$name"
  [ -f "$out/session.json" ] && { echo "skip $name"; continue; }
  ( TIMEFORMAT="wall_s=%R user_s=%U"; time flock "$LOCK" "$ECBENCH" run --wait --spec "$spec" --out "$out" --cpus none --allow-busy > "$HERE/logs/run-$name.log" 2>&1 ) 2> "$HERE/logs/run-$name.time" || echo "run $name failed"
  "$ECBENCH" verify --dir "$out" --replay-all --out "$HERE/audits/audit-$name.json" > "$HERE/logs/verify-$name.log" 2>&1 || echo "verify $name failed"
  echo "done $name $(date -u +%T)"
done
echo RUN-DONE
