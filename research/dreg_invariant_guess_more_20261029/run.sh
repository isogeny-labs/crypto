#!/usr/bin/env bash
# Invariant-guess extension (PREREGISTRATION.md).  Thin orchestration only:
# draws from examples/dreg_ladder, every computation from examples/dreg_linear_guess.
#
#   research/dreg_invariant_guess_more_20261029/run.sh LADDER BIN [gen|replay|measure]
#
# One process per run; a run whose output is non-empty is skipped.  Each
# attempt's output is kept: a non-empty earlier attempt is never overwritten,
# and an empty one is moved to runs/attempts/ before a retry.  MAX_RUNS=N
# starts at most N runs per call.  The script exits non-zero if any run it
# started exited non-zero.
set -u
LADDER=$1
BIN=$2
STEP=${3:-measure}
HERE=$(cd "$(dirname "$0")" && pwd)
REPO=$(cd "$HERE/../.." && pwd)
RUNS=$HERE/runs
DRAWS=$HERE/draws
mkdir -p "$RUNS/attempts" "$DRAWS"
export F4_F2_MAX_ROWS=50000000 F4_F2_MAX_COLS=50000000
export RAYON_NUM_THREADS=${RAYON_NUM_THREADS:-4}
F=research/dreg_invariant_guess_more_20261029/draws/m3-22-8-more.jsonl
STARTED=0
STATUS=0
log() { echo "$(date -u +%FT%TZ) $*" | tee -a "$RUNS/queue.log"; }

one() { # name args...
  local name=$1
  shift
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
  log "$name: start $*"
  local start; start=$(date +%s)
  (cd "$REPO" && prlimit --as=15000000000 timeout 7200 "$BIN" --to 4 --max-cols 140000 --check "$@" "$F") \
    > "$out" 2> "$RUNS/$name.log"
  local rc=$?
  log "$name: exit $rc after $(( $(date +%s) - start )) s"
  [ $rc -ne 0 ] && STATUS=$rc
}

failed() { # arm draw
  grep -l '"degree":4,"xl_refuted":false,"closure_refuted":false,"saturated":true' \
    "$RUNS/$1.d$2".b*.jsonl >/dev/null 2>&1
}

registered() { # the first 8 unsatisfiable draws, in draw order
  grep -v '"satisfiable"' "$REPO/$F" | grep -o '"draw":[0-9]*' | cut -d: -f2 | head -8
}

case $STEP in
gen)
  (cd "$REPO" && "$LADDER" --cells 22:8:3 --m 3 --ffd-max 3 --seed 20261022 \
     --shard 0/1 --first-draw 7 --max-draws 40) > "$DRAWS/m3-22-8-more.jsonl" 2> "$DRAWS/gen.log" ;;
replay)
  (cd "$REPO" && "$LADDER" --cells 22:8:3 --m 3 --ffd-max 3 --seed 20261022 \
     --shard 0/1 --first-draw 0 --max-draws 7) > "$DRAWS/replay-0-6.jsonl" 2> "$DRAWS/replay.log" ;;
measure)
  for d in $(registered); do one "P.d$d" --draw "$d" --fix 7=0,15=0,23=0,24=0; done
  for b in 0 1 2 3 4 5 6 7; do
    c=$((b & 1)); f=$((b >> 1 & 1)); g=$((b >> 2 & 1))
    for d in $(registered); do
      failed S3 "$d" || one "S3.d$d.b$b" --draw "$d" --sum "15=7+$c" --fix "23=$f,24=$g"
      failed X3 "$d" || one "X3.d$d.b$b" --draw "$d" --fix "7=$c,23=$f,24=$g"
    done
  done ;;
esac
exit $STATUS
