#!/usr/bin/env bash
# Frozen run (PROTOCOL.md). Usage: ECBENCH=<pinned binary> bash research/koblitz_n29_orbit_levers_20261118/run.sh
# The session goes to the scratchpad (SESS); only digests, the audit and tables land in git.
set -euo pipefail
ROOT="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)"
HERE="$ROOT/research/koblitz_n29_orbit_levers_20261118"
ECBENCH="${ECBENCH:?set ECBENCH to the pinned binary}"
LOCK="${CPU_LOCK:-/tmp/claude-0/-home-user/cba6917f-779a-544b-9ecf-84e512b45b33/scratchpad/cpu.lock}"
SESS="${SESS:-/tmp/claude-0/-home-user/cba6917f-779a-544b-9ecf-84e512b45b33/scratchpad/n29lv-sessions}"
ulimit -v 9000000
mkdir -p "$SESS" "$HERE/audits" "$HERE/logs"
sha256sum "$ECBENCH" > "$HERE/logs/binary.sha256"
out="$SESS/n29"
if [ ! -f "$out/session.json" ]; then
  ( TIMEFORMAT="wall_s=%R user_s=%U"; time flock "$LOCK" "$ECBENCH" run --wait --spec "$HERE/specs/n29.json" --out "$out" --cpus none --allow-busy > "$HERE/logs/run-n29.log" 2>&1 ) 2> "$HERE/logs/run-n29.time" || echo "run n29 failed"
fi
flock "$LOCK" "$ECBENCH" verify --dir "$out" --replay-all --out "$HERE/audits/audit-n29.json" > "$HERE/logs/verify-n29.log" 2>&1 || echo "verify n29 failed"
echo "done n29 $(date -u +%T)"
echo RUN-DONE
