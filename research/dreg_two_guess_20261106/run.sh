#!/usr/bin/env bash
# Two-guess study (PREREGISTRATION.md).  Thin orchestration only: every
# computation is examples/dreg_linear_guess.
#
#   research/dreg_two_guess_20261106/run.sh BIN
#
# One process per run; a run whose output is non-empty is skipped.  An empty
# earlier attempt is moved to runs/attempts/ before a retry.  MAX_RUNS=N starts
# at most N runs per call.  Exits non-zero if any run it started did.
set -u
BIN=$1
HERE=$(cd "$(dirname "$0")" && pwd)
REPO=$(cd "$HERE/../.." && pwd)
RUNS=$HERE/runs
mkdir -p "$RUNS/attempts"
export F4_F2_MAX_ROWS=50000000 F4_F2_MAX_COLS=50000000
export RAYON_NUM_THREADS=${RAYON_NUM_THREADS:-4}
FIRST=research/dreg_invariant_guess_20261029/draws/m3-22-8.jsonl
MORE=research/dreg_invariant_guess_more_20261029/draws/m3-22-8-more.jsonl
STARTED=0
STATUS=0
log() { echo "$(date -u +%FT%TZ) $*" | tee -a "$RUNS/queue.log"; }
file_of() { case $1 in 0|3|5|6) echo "$FIRST" ;; *) echo "$MORE" ;; esac; }

one() { # name draw maxcols args...
  local name=$1 draw=$2 cols=$3
  shift 3
  local out=$RUNS/$name.jsonl
  if [ -s "$out" ]; then return; fi
  if [ -e "$out" ]; then
    local stamp; stamp=$(date -u +%Y%m%dT%H%M%SZ)
    mv "$out" "$RUNS/attempts/$name.$stamp.jsonl"
    [ -e "$RUNS/$name.log" ] && mv "$RUNS/$name.log" "$RUNS/attempts/$name.$stamp.log"
    log "$name: earlier empty attempt kept as attempts/$name.$stamp.*"
  fi
  if [ -n "${MAX_RUNS:-}" ] && [ "$STARTED" -ge "$MAX_RUNS" ]; then
    exit $STATUS
  fi
  STARTED=$((STARTED + 1))
  log "$name: start --draw $draw $*"
  local start; start=$(date +%s)
  (cd "$REPO" && prlimit --as=15000000000 timeout 7200 "$BIN" --to 4 --max-cols "$cols" --check \
     --draw "$draw" "$@" "$(file_of "$draw")") > "$out" 2> "$RUNS/$name.log"
  local rc=$?
  log "$name: exit $rc after $(( $(date +%s) - start )) s"
  [ $rc -ne 0 ] && STATUS=$rc
}

failed() { # arm draw
  grep -l '"degree":4,"xl_refuted":false,"closure_refuted":false,"saturated":true' \
    "$RUNS/$1.d$2".b*.jsonl >/dev/null 2>&1
}

one N.d5 5 140000 --sum 15=7+0 --fix 23=0,24=0
for b in 0 1 2 3; do
  c=$((b & 1)); h=$((b >> 1 & 1))
  for d in 0 3 5 6 8 11 14 15 17 24 25 27; do
    failed K2L "$d" || one "K2L.d$d.b$b" "$d" 150000 --sum "15=7+$c" --link2 "20=$h"
    failed K2S "$d" || one "K2S.d$d.b$b" "$d" 150000 --sum "15=7+$c" --fix "23=$h"
  done
done
exit $STATUS
