#!/usr/bin/env bash
# E7 local tier (Amendment 7). Gate first, on the same binary; then the cells.
# Usage (repo root): HKY_PAR=<binary> research/fall_degree_bounds_20261008/e7/run_local.sh
set -euo pipefail
S=research/fall_degree_bounds_20261008
E=$S/e7
B=${HKY_PAR:?set HKY_PAR to the hky_par binary}
sha256sum "$B" | cut -d' ' -f1 >"$E/sha256_hky_par"
log() { echo "$(date -u +%FT%TZ) $*" >>"$E/run_local.log"; }
log "gate 1: validate.sh"
HKY_PAR="$B" "$S/e7/validate.sh" >"$E/validate.log" 2>&1
tail -1 "$E/validate.log" | grep -qx 'AGREE 24 DISAGREE 0' || { log "gate 1 FAILED"; exit 1; }
log "gate 2: m=5 (10,2) reproduction"
"$B" newcells --cells 5:10:2 --seed 20261012 --keep 4 --cmax 4 >"$E/gate_B_5-10-2.jsonl"
diff <(jq -c '{draw,d_f,v_basis,x_r,dims:.closure_dims}' "$S/e6/B_5-10-2.rerun.jsonl") \
     <(jq -c '{draw,d_f,v_basis,x_r,dims:(.per_degree|map_values(.dim))}' "$E/gate_B_5-10-2.jsonl") \
  || { log "gate 2 FAILED"; exit 1; }
log "gate passed"
for cell in 6:8:2 5:11:3 6:9:2; do
  out=$E/E7_${cell//:/-}.jsonl
  log "start $cell"
  "$B" newcells --cells "$cell" --seed 20261012 --keep 4 --cmax 4 >"$out" 2>"${out%.jsonl}.err"
  log "done $cell exit=$?"
done
