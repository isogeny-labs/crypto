#!/usr/bin/env bash
# E13 under Amendment 18: local, sequential, one process per draw, 13 GiB address-space cap.
# Resumable: a draw whose .jsonl exists is skipped. Run from anywhere; SRC is the extracted archive.
set -u
SRC=${SRC:-/home/user/e13src}; OUT=${OUT:-/home/user/fdf/research/fall_degree_bounds_20261008/e13/local}
mkdir -p "$OUT"; cd "$SRC" || exit 1
S=research/fall_degree_bounds_20261008
export RAYON_NUM_THREADS=4
CAP_KB=$((13*1024*1024))
log() { echo "$(date -u +%FT%TZ) $*" >>"$OUT/run.log"; }
P=$SRC/target/release/examples/dreg_linear_guess
if [ ! -s "$OUT/gates_passed" ]; then
  cargo build -j3 --release --example dreg_linear_guess >"$OUT/build_pe.log" 2>&1 || { log "build_pe_failed"; exit 1; }
  sha256sum "$P" >"$OUT/sha256_dreg_linear_guess"
  log "G0"
  cargo test -j3 --release --lib -- fast_chained_count_matches_the_reference chained_count partial_enlargement >"$OUT/g0.log" 2>&1 \
    && grep -q "test result: ok. [1-9]" "$OUT/g0.log" || { log "G0 failed"; exit 1; }
  log "G1"
  ( ulimit -v $CAP_KB; "$P" --engine pe --to 4 --max-cols 140000 --draw 0 --sum 15=7+0 --fix 23=0,24=1 \
      research/dreg_invariant_guess_20261029/draws/m3-22-8.jsonl >"$OUT/g1.jsonl" 2>"$OUT/g1.err" ) || { log "G1 exit"; exit 1; }
  k='[.scan[]|{degree,closure_refuted,saturated,closure_rank}]'
  diff <(jq -c "$k" research/dreg_invariant_guess_20261029/runs/S3.d0.b4.jsonl) <(jq -c "$k" "$OUT/g1.jsonl") >"$OUT/g1.diff" || { log "G1 differs"; exit 1; }
  log "gates passed"; echo ok >"$OUT/gates_passed"
fi
draw() { # name to maxcols file draw
  local name=$1 to=$2 mc=$3 f=$4 d=$5 o="$OUT/$1.jsonl"
  [ -s "$o" ] && return 0
  local n=1; while [ -e "$OUT/$name.err.attempt$n" ]; do n=$((n+1)); done
  [ -e "$OUT/$name.err" ] && mv "$OUT/$name.err" "$OUT/$name.err.attempt$n"
  log "start $name"; local s=$(date +%s)
  ( ulimit -v $CAP_KB; "$P" --engine pe --batch 64 --to "$to" --max-cols "$mc" --draw "$d" "$f" >"$o.part" 2>"$OUT/$name.err" )
  local rc=$?
  if [ $rc -eq 0 ] && [ -s "$o.part" ]; then mv "$o.part" "$o"; else mv "$o.part" "$OUT/$name.failed" 2>/dev/null; echo "exit=$rc" >>"$OUT/$name.err"; fi
  log "done $name exit=$rc $(( $(date +%s) - s ))s"
}
E10B=$S/e10/run-r3-B/out
for d in 0 1 3 4; do draw B_3-21-7_d$d 4 400000 $E10B/E_3-21-7.jsonl $d; done
for d in 0 1 2 3; do draw B_3-24-8_d$d 4 400000 $E10B/E_3-24-8.jsonl $d; done
for d in 1 2 3 4; do draw B_3-27-9_d$d 4 400000 $E10B/E_3-27-9.jsonl $d; done
for d in 0 1 2 3; do draw B_ctl_4-12-3_d$d 4 400000 $S/e10/run-r3-C/out/E_4-12-3.jsonl $d; done
for d in 1 2 3 4; do draw B_ctl_6-10-3_d$d 4 600000 $S/e9/run-r3/out/E_6-10-3.jsonl $d; done
log "E13b finished"
for d in 0 1 3 4; do draw A_3-21-7_d$d 5 1000000 $E10B/E_3-21-7.jsonl $d; done
log "E13a finished"
if [ ! -s "$OUT/C/status" ]; then
  ( ulimit -v $CAP_KB; bash $S/e12/run_window.sh "$OUT/C" 4:28:7 >"$OUT/C.log" 2>&1 ); log "E13c exit=$?"
fi
log "ALL DONE"; echo done >"$OUT/all_done"
