#!/usr/bin/env bash
# E12a/E12b (Amendment 11): build hky_par, run Amendment 10's gates 0–3 on that
# exact binary, then `newcells` on the given cells with KEEP and CMAX.
#   KEEP=4 CMAX=4 [LEAN=1] research/fall_degree_bounds_20261008/e12/run_par.sh OUTDIR CELL [CELL...]
# LEAN=1 first builds the Lean scoring (lean/) and records the log (no gate).
set -uo pipefail
OUT=$1; shift
S=research/fall_degree_bounds_20261008
KEEP=${KEEP:?}; CMAX=${CMAX:?}
mkdir -p "$OUT"
log() { echo "$(date -u +%FT%TZ) $*" | tee -a "$OUT/run.log"; }
if [ "${LEAN:-0}" = 1 ]; then
  log "lean"
  bash "$S/e12/run_lean.sh" "$OUT/lean" || log "lean exit=$?"
fi
cargo build --release --example hky_par >"$OUT/build.log" 2>&1 || { echo build_failed >"$OUT/status"; exit 1; }
B=$PWD/target/release/examples/hky_par
sha256sum "$B" | cut -d' ' -f1 >"$OUT/sha256_hky_par"
nproc >"$OUT/nproc"; free -g >"$OUT/free.txt"
fail() { log "GATE FAILED: $*"; echo "gate_failed: $*" >"$OUT/status"; exit 1; }
log "gate 0"
cargo test --release --lib -- fast_chained_count_matches_the_reference chained_count \
  >"$OUT/gate0.log" 2>&1 || fail "gate 0 unit tests"
grep -q "test result: ok. [1-9]" "$OUT/gate0.log" || fail "gate 0 ran no test"
log "gate 1"
rm -rf "$S/e7/validate"
HKY_PAR="$B" "$S/e7/validate.sh" >"$OUT/gate1.log" 2>&1 || fail "validate.sh exit"
tail -1 "$OUT/gate1.log" | grep -qx 'AGREE 24 DISAGREE 0' || fail "gate 1 $(tail -1 "$OUT/gate1.log")"
key='{draw,d_f,v_basis,x_r,dims:(.per_degree|map_values(.dim))}'
gate_cell() {
  local cell=$1 ref=$2 rkey=$3 o="$OUT/gate_${1//:/-}.jsonl"
  "$B" newcells --cells "$cell" --seed 20261012 --keep 4 --cmax 4 >"$o" 2>"${o%.jsonl}.err" || fail "$cell exit"
  diff <(jq -c "$rkey" "$S/$ref") <(jq -c "$key" "$o") >"${o%.jsonl}.diff" || fail "$cell differs from $ref"
  log "gate $cell reproduces $ref"
}
log "gates 2-3"
gate_cell 5:10:2 e6/B_5-10-2.rerun.jsonl '{draw,d_f,v_basis,x_r,dims:.closure_dims}'
gate_cell 3:18:6 e6/B_3-18-6.jsonl '{draw,d_f,v_basis,x_r,dims:.closure_dims}'
gate_cell 4:8:2 e8/E8_4-8-2.jsonl "$key"
gate_cell 5:8:2 e8/E8_5-8-2.jsonl "$key"
log "gate passed"
echo gate_passed >"$OUT/status"
for cell in "$@"; do
  o="$OUT/E_${cell//:/-}.jsonl"
  log "start $cell keep=$KEEP cmax=$CMAX"
  "$B" newcells --cells "$cell" --seed 20261012 --keep "$KEEP" --cmax "$CMAX" >"$o" 2>"${o%.jsonl}.err"
  rc=$?
  log "done $cell exit=$rc draws=$(wc -l <"$o")"
  echo "$cell exit=$rc" >>"$OUT/cells_status"
done
echo done >>"$OUT/status"
