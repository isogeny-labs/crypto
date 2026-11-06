#!/usr/bin/env bash
# Three-guess study at l = 8 (PREREGISTRATION.md).  Thin orchestration only:
# every computation is examples/dreg_closure (--fix); draws are the frozen
# research/dreg_closure_guess_20261022/draws/m3-22-8.jsonl.
#
#   research/dreg_closure_guess3_20261023/run.sh CLOSURE_BIN [control|measure]
#
# One process per run; a run whose output is non-empty is skipped, so a restart
# loses only the run in flight.  A draw's G3 branches stop at its first branch
# that saturates degree 4 without refuting.  MAX_RUNS=N starts at most N runs
# this invocation, prints BATCH_LIMIT and exits; ALL_DONE is printed when the
# step has nothing left to run.
set -u
CLOSURE=$1
STEP=${2:-measure}
HERE=$(cd "$(dirname "$0")" && pwd)
REPO=$(cd "$HERE/../.." && pwd)
RUNS=$HERE/runs
mkdir -p "$RUNS"
export F4_F2_MAX_ROWS=50000000 F4_F2_MAX_COLS=50000000
export RAYON_NUM_THREADS=${RAYON_NUM_THREADS:-4}
NAME=m3-22-8
F=research/dreg_closure_guess_20261022/draws/$NAME.jsonl
DRAWS="0 3 5 6"   # the frozen unsatisfiable draws
STARTED=0
log() { echo "$(date -u +%FT%TZ) $*" | tee -a "$RUNS/queue.log"; }

limit() { # exit when the MAX_RUNS cap is reached
  if [ -n "${MAX_RUNS:-}" ]; then
    if [ "$STARTED" -ge "$MAX_RUNS" ]; then echo BATCH_LIMIT; exit 0; fi
    STARTED=$((STARTED + 1))
  fi
}

one() { # draw arm fix
  local k=$1 a=$2 fix=$3 out=$RUNS/$NAME.d$1.$2.jsonl
  if [ -s "$out" ]; then return; fi
  limit
  log "$NAME d$k $a: start fix=$fix"
  local start; start=$(date +%s)
  (cd "$REPO" && prlimit --as=15000000000 timeout 7200 "$CLOSURE" --to 4 --max-cols 140000 \
     --draw "$k" --fix "$fix" "$F") > "$out" 2> "$RUNS/$NAME.d$k.$a.log"
  log "$NAME d$k $a: exit $? after $(( $(date +%s) - start )) s"
}

n7() { # instrument-identity and negative control: unspecialised l = 7, draw 0
  local out=$RUNS/fresh-m3-19-7.d0.jsonl
  if [ -s "$out" ]; then return; fi
  limit
  log "N7: start"
  local start; start=$(date +%s)
  (cd "$REPO" && prlimit --as=15000000000 timeout 7200 "$CLOSURE" --to 4 --max-cols 130000 \
     --draw 0 research/dreg_closure_scale_20261016/draws/m3-19-7.jsonl) \
     > "$out" 2> "$RUNS/fresh-m3-19-7.d0.log"
  log "N7: exit $? after $(( $(date +%s) - start )) s"
}

failed() { # draw: a G3 branch saturated degree 4 without refuting
  grep -l '"degree":4,"xl_refuted":false,"closure_refuted":false,"saturated":true' \
    "$RUNS/$NAME.d$1".G3b*.jsonl >/dev/null 2>&1
}

case $STEP in
control)
  n7
  for k in $DRAWS; do one "$k" G6 6=0,7=0,14=0,15=0,22=0,23=0; done
  echo ALL_DONE ;;
measure)
  g3() { one "$1" "G3b$2" "7=$(($2 & 1)),15=$(($2 >> 1 & 1)),23=$(($2 >> 2 & 1))"; }
  g3 0 0   # memory pilot: G3b0 on draw 0 first
  for b in $(seq 0 7); do
    for k in $DRAWS; do
      if failed "$k"; then continue; fi
      g3 "$k" "$b"
    done
  done
  echo ALL_DONE ;;
*) echo "usage: run.sh CLOSURE_BIN [control|measure]" >&2; exit 2 ;;
esac
