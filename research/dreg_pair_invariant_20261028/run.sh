#!/usr/bin/env bash
# Pair-invariant study (PREREGISTRATION.md).  Thin orchestration only: every
# computation is examples/dreg_invariant.
#
#   research/dreg_pair_invariant_20261028/run.sh BIN [control|primary|sound|secondary]
#
# One process per (arm, cell, draw, step); a run whose output is non-empty is
# skipped, so a restart loses only the run in flight.  MAX_RUNS=N starts at
# most N runs per invocation.
set -u
BIN=$1
STEP=${2:-primary}
HERE=$(cd "$(dirname "$0")" && pwd)
REPO=$(cd "$HERE/../.." && pwd)
RUNS=$HERE/runs
mkdir -p "$RUNS"
export F4_F2_MAX_ROWS=50000000 F4_F2_MAX_COLS=50000000
export RAYON_NUM_THREADS=${RAYON_NUM_THREADS:-4}
C6=research/dreg_closure_driver_20261016/draws/m3-22-6.jsonl
C7=research/dreg_closure_scale_20261016/draws/m3-21-7.jsonl
STARTED=0
log() { echo "$(date -u +%FT%TZ) $*" | tee -a "$RUNS/queue.log"; }

one() { # name file draw args...
  local name=$1 file=$2 draw=$3
  shift 3
  local out=$RUNS/$name.jsonl
  if [ -s "$out" ]; then return; fi
  if [ -n "${MAX_RUNS:-}" ]; then
    [ "$STARTED" -ge "$MAX_RUNS" ] && exit 0
    STARTED=$((STARTED + 1))
  fi
  log "$name: start $*"
  local start; start=$(date +%s)
  (cd "$REPO" && prlimit --as=15000000000 timeout 7200 "$BIN" --check --draw "$draw" "$@" "$file") \
    > "$out" 2> "$RUNS/$name.log"
  log "$name: exit $? after $(( $(date +%s) - start )) s"
}

refuted3() { # name: the dense degree-3 run refuted
  grep -q '"closure_degree":3' "$RUNS/$1.jsonl" 2>/dev/null
}

case $STEP in
control)
  for d in 0 1 2 3; do one "B.22-6.d$d" $C6 $d --arm B --to 4 --max-cols 130000; done
  for d in 0 2 3 6; do one "B.21-7.d$d" $C7 $d --arm B --to 4 --max-cols 130000; done ;;
primary)
  for a in Q R; do
    for d in 0 1 2 3; do one "$a.22-6.d$d" $C6 $d --arm $a --to 3 --max-cols 130000; done
    for d in 0 2 3 6; do one "$a.21-7.d$d" $C7 $d --arm $a --to 3 --max-cols 130000; done
  done ;;
sound)
  for a in Q R; do
    for d in 1 4 5; do one "$a.21-7.sat.d$d" $C7 $d --arm $a --to 3 --max-cols 130000 --satisfiable; done
  done ;;
secondary)
  for a in Q R; do
    for d in 0 1 2 3; do
      refuted3 "$a.22-6.d$d" || one "$a.22-6.d$d.sparse4" $C6 $d --arm $a --sparse 4 --max-entries 1000000000
    done
    ran7=0
    for d in 0 2 3 6; do
      if ! refuted3 "$a.21-7.d$d"; then
        ran7=1
        one "$a.21-7.d$d.sparse4" $C7 $d --arm $a --sparse 4 --max-entries 1000000000
      fi
    done
    if [ $ran7 = 1 ]; then
      for d in 1 4 5; do
        one "$a.21-7.sat.d$d.sparse4" $C7 $d --arm $a --sparse 4 --max-entries 1000000000 --satisfiable
      done
    fi
  done ;;
esac
