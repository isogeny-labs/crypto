#!/usr/bin/env bash
# Thin orchestration for PREREGISTRATION.md.  One process at a time.
#   research/dreg_xl_degree5_20261018/run.sh [controls|registered]
set -u
HERE=$(cd "$(dirname "$0")" && pwd)
REPO=$(cd "$HERE/../.." && pwd)
BIN=$REPO/target/release/examples/xl_dense_degree
RUNS=$HERE/runs
S=research/dreg_closure_scale_20261016/draws
G=research/dreg_ell_grid_20260925/runs
export RAYON_NUM_THREADS=2
log() { echo "$(date -u +%FT%TZ) $*" | tee -a "$RUNS/queue.log"; }
one() { # out-name, then arguments to the engine
  local name=$1; shift
  [ -s "$RUNS/$name.jsonl" ] && { log "$name: done already, skipped"; return 0; }
  log "$name: start"
  local t0; t0=$(date +%s)
  (cd "$REPO" && prlimit --as=8000000000 timeout 10800 "$BIN" run --threads 2 --max-bytes 6300000000 "$@") \
    > "$RUNS/$name.jsonl" 2> "$RUNS/$name.log"
  local rc=$?
  log "$name: exit $rc after $(( $(date +%s) - t0 )) s"
}
case ${1:-registered} in
controls)
  for d in 4 5; do   # positive: m3 l=2, XL degree exactly 5
    one ctl-pos-n12l2.D$d --degree $d --role positive --expect $([ $d = 5 ] && echo refuted || echo open) --unsat $G/cell-12-2-7.jsonl
    one ctl-pos-n10l2.D$d --degree $d --role positive --expect $([ $d = 5 ] && echo refuted || echo open) --unsat $G/cell-10-2-7.jsonl
  done
  for d in 5 6; do   # positive: m3 l=3, XL degree exactly 6 (draws 9, 14)
    one ctl-pos-n5l3.D$d --degree $d --role positive --expect $([ $d = 6 ] && echo refuted || echo open) --draw 9,14 $G/cell-5-3-7.jsonl
  done
  for d in 5 6; do   # soundness: satisfiable draws never refute
    one ctl-sound-n4l3.D$d --degree $d --role soundness --expect open --draw 0,1,2,3 $G/cell-4-3-7.jsonl
  done ;;
registered)
  one registered-n19l7.D5 --degree 5 --role registered --draw 0,3,5,7 $S/m3-19-7.jsonl
  one registered-n21l7.D5 --degree 5 --role registered --draw 0,2,3,6 $S/m3-21-7.jsonl
  one soundness-n19l7.D5 --degree 5 --role soundness-registered-cell --expect open --draw 1,2,4,6 $S/m3-19-7.jsonl
  one soundness-n21l7.D5 --degree 5 --role soundness-registered-cell --expect open --draw 1,4,5 $S/m3-21-7.jsonl ;;
esac
