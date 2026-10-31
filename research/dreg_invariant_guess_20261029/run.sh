#!/usr/bin/env bash
# Invariant-guess study (PREREGISTRATION.md).  Thin orchestration only: every
# computation is examples/dreg_linear_guess.
#
#   research/dreg_invariant_guess_20261029/run.sh BIN [control|measure|sound]
#
# One process per run; a run whose output is non-empty is skipped, so a restart
# loses only the run in flight.  MAX_RUNS=N starts at most N runs per call.
set -u
BIN=$1
STEP=${2:-measure}
HERE=$(cd "$(dirname "$0")" && pwd)
REPO=$(cd "$HERE/../.." && pwd)
RUNS=$HERE/runs
mkdir -p "$RUNS"
export F4_F2_MAX_ROWS=50000000 F4_F2_MAX_COLS=50000000
export RAYON_NUM_THREADS=${RAYON_NUM_THREADS:-4}
F=research/dreg_invariant_guess_20261029/draws/m3-22-8.jsonl
STARTED=0
log() { echo "$(date -u +%FT%TZ) $*" | tee -a "$RUNS/queue.log"; }

one() { # name args...
  local name=$1
  shift
  local out=$RUNS/$name.jsonl
  if [ -s "$out" ]; then return; fi
  if [ -n "${MAX_RUNS:-}" ]; then
    [ "$STARTED" -ge "$MAX_RUNS" ] && exit 0
    STARTED=$((STARTED + 1))
  fi
  log "$name: start $*"
  local start; start=$(date +%s)
  (cd "$REPO" && prlimit --as=15000000000 timeout 7200 "$BIN" --to 4 --max-cols 140000 --check "$@" "$F") \
    > "$out" 2> "$RUNS/$name.log"
  log "$name: exit $? after $(( $(date +%s) - start )) s"
}

failed() { # arm draw: a branch saturated degree 4 without refuting
  grep -l '"degree":4,"xl_refuted":false,"closure_refuted":false,"saturated":true' \
    "$RUNS/$1.d$2".b*.jsonl >/dev/null 2>&1
}

case $STEP in
control)
  one N.d0 --draw 0 --fix 7=0,15=0,23=0
  for d in 0 3 5 6; do one "P.d$d" --draw "$d" --fix 7=0,15=0,23=0,24=0; done ;;
measure)
  for b in 0 1 2 3 4 5 6 7; do
    c=$((b & 1)); f=$((b >> 1 & 1)); g=$((b >> 2 & 1))
    for d in 0 3 5 6; do
      failed S3 "$d" || one "S3.d$d.b$b" --draw "$d" --sum "15=7+$c" --fix "23=$f,24=$g"
      failed X3 "$d" || one "X3.d$d.b$b" --draw "$d" --fix "7=$c,23=$f,24=$g"
    done
  done ;;
sound)
  for b in 0 1 2 3 4 5 6 7; do
    c=$((b & 1)); f=$((b >> 1 & 1)); g=$((b >> 2 & 1))
    one "S3.sat.d1.b$b" --draw 1 --satisfiable --sum "15=7+$c" --fix "23=$f,24=$g"
  done ;;
esac
