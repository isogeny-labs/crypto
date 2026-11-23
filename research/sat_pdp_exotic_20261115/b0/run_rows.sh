#!/usr/bin/env bash
# B0 rows, as frozen in protocol.json: one JSONL per rung into rows/l<ℓ>.jsonl.
# Usage: run_rows.sh <ℓ> [arms]   (default arms per the protocol's stopping rule 1)
# Counted metrics only (shared host, L0); wall time is descriptive. Run under
#   python3 tools/benchrec.py run $BENCHREC -- research/sat_pdp_exotic_20261115/b0/run_rows.sh 7
set -euo pipefail
L="$1"
case "$L" in
  # AM2: chain-p12 dropped at l = 7 after its one complete (censored) target
  7)  ARMS="${2:-oracle,native,lin,lin-nolex,native-aa,cms,cadical}"; BUDGET=4000000 ;;
  8|9) ARMS="${2:-oracle,native,lin,lin-nolex,native-aa,cms,cadical}"; BUDGET=4000000 ;;
  # AM3: native arms censored by construction at l = 10 (budget below C(2^l, 3)); not run
  10) ARMS="${2:-oracle,lin,lin-nolex}"; BUDGET=16000000 ;;
  *) echo "rung ℓ=$L is not in the frozen grid {7,8,9,10}" >&2; exit 2 ;;
esac
root="$(cd "$(dirname "$0")/../../.." && pwd)"
here="$root/research/sat_pdp_exotic_20261115/b0"
mkdir -p "$here/rows" "$here/inst/l$L"
bin="$root/target/release/examples/b0_pdp_sat_baseline"
ext="$root/tools/b0-ext-solvers/target/release/b0-ext-solvers"
sha256sum "$bin" "$ext" | tee -a "$here/rows/binaries.sha256"
exec "$bin" --l "$L" --uniform 16 --planted 4 --seed 20261115 --budget "$BUDGET" \
  --arms "$ARMS" --export "$here/inst/l$L" --ext-bin "$ext" --out "$here/rows/l$L.jsonl" > /dev/null
