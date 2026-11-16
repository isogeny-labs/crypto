#!/usr/bin/env bash
# E12c (Amendment 11): build hky_window with --features wide-vars, gate it on that
# exact binary, then `newcells --policies w1x` on the given cells.
#   research/fall_degree_bounds_20261008/e12/run_window.sh OUTDIR CELL [CELL...]
set -uo pipefail
OUT=$1; shift
S=research/fall_degree_bounds_20261008
mkdir -p "$OUT"
log() { echo "$(date -u +%FT%TZ) $*" | tee -a "$OUT/run.log"; }
fail() { log "GATE FAILED: $*"; echo "gate_failed: $*" >"$OUT/status"; exit 1; }
cargo build --release --features wide-vars --example hky_window >"$OUT/build.log" 2>&1 || { echo build_failed >"$OUT/status"; exit 1; }
B=$PWD/target/release/examples/hky_window
sha256sum "$B" | cut -d' ' -f1 >"$OUT/sha256_hky_window"
nproc >"$OUT/nproc"; free -g >"$OUT/free.txt"
log "gate 0"
cargo test --release --features wide-vars --lib -- fast_chained_count_matches_the_reference chained_count \
  >"$OUT/gate0.log" 2>&1 || fail "gate 0 unit tests"
grep -q "test result: ok. [1-9]" "$OUT/gate0.log" || fail "gate 0 ran no test"
key='{draw,d_f_le3,d_f_window,cols:.window.w1x.cols,dim:.window.w1x.dim,d2:.per_degree["2"].dim,d3:.per_degree["3"].dim}'
log "gate 1"
for name in B_3-18-6 B_4-13-4 E7_6-8-2 E8_7-8-2; do
  HKY_WINDOW="$B" OUT="$OUT/gate1" POLICIES=w1x bash "$S/e11/run_validate.sh" "$name" >>"$OUT/gate1.log" 2>&1 || fail "gate 1 $name exit"
  diff <(jq -c "$key" "$S/e11/validate_final/$name.jsonl") <(jq -c "$key" "$OUT/gate1/$name.jsonl") >"$OUT/gate1_$name.diff" \
    || fail "gate 1 $name differs from e11/validate_final"
  log "gate 1 $name reproduces e11/validate_final"
done
log "gate 2"
"$B" newcells --cells 4:16:4 --seed 20261012 --keep 4 --policies w1x >"$OUT/gate2_4-16-4.jsonl" 2>"$OUT/gate2_4-16-4.err" || fail "gate 2 exit"
k2='{draw,v_basis,x_r,d_f_le3,d_f_window,cols:.window.w1x.cols,dim:.window.w1x.dim,d2:.per_degree["2"].dim,d3:.per_degree["3"].dim}'
diff <(jq -c "$k2" "$S/e11/scaling/window_4-16-4.jsonl") <(jq -c "$k2" "$OUT/gate2_4-16-4.jsonl") >"$OUT/gate2.diff" \
  || fail "gate 2 differs from e11/scaling/window_4-16-4.jsonl"
log "gate passed"
echo gate_passed >"$OUT/status"
for cell in "$@"; do
  o="$OUT/W_${cell//:/-}.jsonl"
  log "start $cell"
  "$B" newcells --cells "$cell" --seed 20261012 --keep 4 --policies w1x >"$o" 2>"${o%.jsonl}.err"
  rc=$?
  log "done $cell exit=$rc draws=$(wc -l <"$o")"
  echo "$cell exit=$rc" >>"$OUT/cells_status"
done
echo done >>"$OUT/status"
