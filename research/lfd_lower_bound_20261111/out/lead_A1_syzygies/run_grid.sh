#!/usr/bin/env bash
# FOLLOWUP-selector-extra-syzygies: run syz on the preregistered cells (all 2^k points).
# Usage (from research/lfd_lower_bound_20261111): out/lead_A1_syzygies/run_grid.sh "N:SEEDS:ES" ...
#   e.g. out/lead_A1_syzygies/run_grid.sh "15:1,5,6:2,3"   optional 4th field PERMILLE (default 1000 = all points)
set -euo pipefail
export RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1
od=out/lead_A1_syzygies
for cell in "$@"; do
  IFS=: read -r n seeds es pm <<< "$cell"; pm=${pm:-1000}
  for s in ${seeds//,/ }; do
    line=$(python3 emit_instance.py "$n" "$s")
    nz=$(echo "$line" | ./emin count)
    [ "$nz" = "0" ] || { echo "seed $s satisfiable" >&2; exit 1; }
    for e in ${es//,/ }; do
      res=$(echo "$line" | $od/syz "$e" "$pm" "$s" 2>> $od/run.log)
      echo "{\"seed\": $s, \"permille\": $pm, \"r\": $res}" | tee -a $od/grid.jsonl | cut -c1-200
    done
  done
done
