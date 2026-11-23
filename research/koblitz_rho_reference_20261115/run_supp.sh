#!/usr/bin/env bash
set -euo pipefail
ROOT="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)"; HERE="$ROOT/research/koblitz_rho_reference_20261115"
ECBENCH="${ECBENCH:-$ROOT/target/release/ecbench}"; LOCK=/tmp/claude-0/-home-user/cba6917f-779a-544b-9ecf-84e512b45b33/scratchpad/cpu.lock
ulimit -v 9000000; mkdir -p "$HERE/sessions_supp" "$HERE/logs"
for n in ${*:-17 23 29 31 37}; do out="$HERE/sessions_supp/n$n"; [ -f "$out/session.json" ] && continue
  flock "$LOCK" "$ECBENCH" run --wait --spec "$HERE/specs_supp/n$n.json" --out "$out" --cpus none --allow-busy > "$HERE/logs/supp-run-n$n.log" 2>&1
  "$ECBENCH" verify --dir "$out" --replay-all --out "$HERE/audits/supp-audit-n$n.json" > "$HERE/logs/supp-verify-n$n.log" 2>&1; echo "done $n"; done; echo RUN-DONE
