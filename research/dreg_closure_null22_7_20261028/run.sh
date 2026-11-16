#!/usr/bin/env bash
# Matched-null study at (22, 7) (PREREGISTRATION.md).  Thin orchestration only:
# every computation is examples/dreg_ladder (draws) and examples/dreg_closure.
#
#   research/dreg_closure_null22_7_20261028/run.sh LADDER_BIN CLOSURE_BIN [gen|gen-shards|control|measure]
#
# One process per run; a run whose output is non-empty is skipped, so a restart
# loses only the run in flight.  MAX_RUNS=N starts at most N runs this
# invocation, prints BATCH_LIMIT and exits; ALL_DONE is printed when the step has
# nothing left to run.
set -u
LADDER=$1
CLOSURE=$2
STEP=${3:-measure}
HERE=$(cd "$(dirname "$0")" && pwd)
REPO=$(cd "$HERE/../.." && pwd)
DRAWS=$HERE/draws
RUNS=$HERE/runs
mkdir -p "$DRAWS" "$RUNS"
export F4_F2_MAX_ROWS=50000000 F4_F2_MAX_COLS=50000000
export RAYON_NUM_THREADS=${RAYON_NUM_THREADS:-4}
NAME=m3-22-7
F=research/dreg_closure_null22_7_20261028/draws/$NAME.jsonl
SEED=20261028
STARTED=0
log() { echo "$(date -u +%FT%TZ) $*" | tee -a "$RUNS/queue.log"; }

limit() { # exit when the MAX_RUNS cap is reached
  if [ -n "${MAX_RUNS:-}" ]; then
    if [ "$STARTED" -ge "$MAX_RUNS" ]; then echo BATCH_LIMIT; exit 0; fi
    STARTED=$((STARTED + 1))
  fi
}

unsat() { grep -v '"satisfiable"' "$REPO/$F" | grep -o '"draw":[0-9]*' | cut -d: -f2 | head -4; }
sat()   { grep '"satisfiable"' "$REPO/$F" | grep -o '"draw":[0-9]*' | cut -d: -f2 | head -2; }

one() { # label draw cap-cols file-suffix extra-flags...
  local a=$1 k=$2 cap=$3; shift 3
  local out=$RUNS/$NAME.d$k.$a.jsonl
  if [ -s "$out" ]; then return; fi
  limit
  log "$NAME d$k $a: start $*"
  local start; start=$(date +%s)
  (cd "$REPO" && prlimit --as=15000000000 timeout 7200 "$CLOSURE" --to 4 --max-cols "$cap" \
     --draw "$k" "$@" "$F") > "$out" 2> "$RUNS/$NAME.d$k.$a.log"
  log "$NAME d$k $a: exit $? after $(( $(date +%s) - start )) s"
}

n7() { # instrument-identity control: unspecialised l = 7, n = 19, draw 0
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

case $STEP in
gen)
  (cd "$REPO" && "$LADDER" --cells 22:7:3 --m 3 --unsat 4 --ffd-max 3 --seed $SEED) \
    > "$DRAWS/$NAME.jsonl" 2> "$DRAWS/$NAME.log" ;;
gen-shards) # four shards in parallel, same seeded sequence; merge by draw order afterwards
  for i in 0 1 2 3; do
    (cd "$REPO" && "$LADDER" --cells 22:7:3 --m 3 --ffd-max 3 --seed $SEED \
       --shard "$i/4" --max-draws 24) > "$DRAWS/shard-$i.jsonl" 2> "$DRAWS/shard-$i.log" &
  done
  wait ;;
control)
  n7
  for k in $(unsat); do one B "$k" 140000 --fix 6=0,13=0,20=0; done
  for k in $(sat);   do one S "$k" 140000 --satisfiable; done
  echo ALL_DONE ;;
measure)
  for k in $(unsat); do one N0 "$k" 140000; done
  echo ALL_DONE ;;
*) echo "usage: run.sh LADDER_BIN CLOSURE_BIN [gen|gen-shards|control|measure]" >&2; exit 2 ;;
esac
