#!/usr/bin/env bash
# Frozen run (PROTOCOL.md section 6). Usage: bash research/koblitz_ksum_fold_study_20261115/run.sh
set -euo pipefail
ROOT="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)"
HERE="$ROOT/research/koblitz_ksum_fold_study_20261115"
ECBENCH="${ECBENCH:-$ROOT/target/release/ecbench}"
LOCK="${CPU_LOCK:-/tmp/claude-0/-home-user/cba6917f-779a-544b-9ecf-84e512b45b33/scratchpad/cpu.lock}"
mkdir -p "$HERE/sessions" "$HERE/audits" "$HERE/logs"
for spec in "$HERE"/specs/n*.json; do
  name=$(basename "$spec" .json); out="$HERE/sessions/$name"
  [ -f "$out/session.json" ] && { echo "skip $name"; continue; }
  flock "$LOCK" "$ECBENCH" run --wait --spec "$spec" --out "$out" --cpus none --allow-busy > "$HERE/logs/run-$name.log" 2>&1
  "$ECBENCH" verify --dir "$out" --replay-all --out "$HERE/audits/audit-$name.json" > "$HERE/logs/verify-$name.log" 2>&1
  echo "done $name"
done
echo RUN-DONE
