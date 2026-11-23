#!/usr/bin/env bash
# Frozen run (PROTOCOL.md). Usage: ECBENCH=<pinned binary> bash research/koblitz_lean_sib_confirm_20261118/run.sh
# Sessions go to the scratchpad (SESS); only digests, audits and tables land in git.
set -euo pipefail
ROOT="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)"
HERE="$ROOT/research/koblitz_lean_sib_confirm_20261118"
ECBENCH="${ECBENCH:?set ECBENCH to the pinned binary}"
PIN=43f44fec44b902f22753c2b44fd10d0567ad327cf73c9298c3558de3ff5c3e65
LOCK="${CPU_LOCK:-/tmp/claude-0/-home-user/cba6917f-779a-544b-9ecf-84e512b45b33/scratchpad/cpu.lock}"
SESS="${SESS:-/tmp/claude-0/-home-user/cba6917f-779a-544b-9ecf-84e512b45b33/scratchpad/lsc-sessions}"
ulimit -v 9000000
mkdir -p "$SESS" "$HERE/audits" "$HERE/logs"
got="$(sha256sum "$ECBENCH" | cut -d' ' -f1)"
[ "$got" = "$PIN" ] || { echo "binary sha256 $got is not the pinned $PIN"; exit 2; }
echo "$got  $(basename "$ECBENCH")" > "$HERE/logs/binary.sha256"
# Interleaved session order: replicate-major, curves rotated.
for name in n23-r1 n29-r1 n37-r1 n29-r2 n37-r2 n23-r2 n37-r3 n23-r3 n29-r3; do
  out="$SESS/$name"
  if [ ! -f "$out/session.json" ]; then
    ( TIMEFORMAT="wall_s=%R user_s=%U"; time flock "$LOCK" "$ECBENCH" run --wait --spec "$HERE/specs/$name.json" --out "$out" --cpus none --allow-busy > "$SESS/run-$name.log" 2>&1 ) 2> "$HERE/logs/run-$name.time" || { echo "run $name failed"; exit 1; }
  fi
  flock "$LOCK" "$ECBENCH" verify --dir "$out" --replay-all --out "$SESS/audit-$name.json" > "$SESS/verify-$name.log" 2>&1 || { echo "verify $name failed"; exit 1; }
  gzip -9 -c "$SESS/audit-$name.json" > "$HERE/audits/audit-$name.json.gz"
  echo "done $name $(date -u +%T)"
done
echo RUN-DONE
