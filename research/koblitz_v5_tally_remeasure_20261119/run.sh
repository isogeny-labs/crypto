#!/usr/bin/env bash
# Frozen run (PROTOCOL.md). Usage: ECBENCH=<pinned binary> PIN=<sha256 from PROTOCOL.md> bash research/koblitz_v5_tally_remeasure_20261119/run.sh
# Sessions go to the scratchpad (SESS); only digests, audits and tables land in git.
set -euo pipefail
ROOT="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)"
HERE="$ROOT/research/koblitz_v5_tally_remeasure_20261119"
ECBENCH="${ECBENCH:?set ECBENCH to the pinned binary}"
PIN="${PIN:?set PIN to the binary sha256 named in PROTOCOL.md}"
LOCK="${CPU_LOCK:-/tmp/claude-0/-home-user/cba6917f-779a-544b-9ecf-84e512b45b33/scratchpad/cpu.lock}"
SESS="${SESS:-/tmp/claude-0/-home-user/cba6917f-779a-544b-9ecf-84e512b45b33/scratchpad/v5t/sessions}"
ulimit -v 9000000
mkdir -p "$SESS" "$HERE/audits" "$HERE/logs"
got="$(sha256sum "$ECBENCH" | cut -d' ' -f1)"
[ "$got" = "$PIN" ] || { echo "binary sha256 $got is not the pinned $PIN"; exit 2; }
grep -q "$PIN" "$HERE/PROTOCOL.md" || { echo "PIN is not the one in PROTOCOL.md"; exit 2; }
echo "$got  $(basename "$ECBENCH")" > "$HERE/logs/binary.sha256"
for name in a0-n13 a1-n17 a0-n19 a0-n23 a1-n23 a1-n29 a0-n31 a0-n37; do
  out="$SESS/$name"
  if [ ! -f "$out/session.json" ]; then
    ( TIMEFORMAT="wall_s=%R user_s=%U"; time flock "$LOCK" "$ECBENCH" run --wait --spec "$HERE/specs/$name.json" --out "$out" --cpus none --allow-busy > "$SESS/run-$name.log" 2>&1 ) 2> "$HERE/logs/run-$name.time" || { echo "run $name failed"; exit 1; }
  fi
  flock "$LOCK" "$ECBENCH" verify --dir "$out" --replay-all --out "$SESS/audit-$name.json" > "$SESS/verify-$name.log" 2>&1 || { echo "verify $name failed"; exit 1; }
  gzip -9 -n -c "$SESS/audit-$name.json" > "$HERE/audits/audit-$name.json.gz"
  cp "$SESS/verify-$name.log" "$HERE/logs/verify-$name.log"
  echo "done $name $(date -u +%T)"
done
echo RUN-DONE
