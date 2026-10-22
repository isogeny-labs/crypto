#!/usr/bin/env bash
# Guess-count study at ℓ = 8 (PREREGISTRATION.md).  Thin orchestration only:
# every computation is examples/dreg_ladder (draws) and examples/dreg_closure --fix.
#
#   research/dreg_closure_guess_20261022/run.sh LADDER_BIN CLOSURE_BIN [gen|measure]
#
# One process per (draw, arm); a run whose output is non-empty is skipped, so a
# restart loses only the run in flight.  A draw's G4 branches stop at its first
# branch that does not refute.
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
NAME=m3-22-8
F=research/dreg_closure_guess_20261022/draws/$NAME.jsonl
log() { echo "$(date -u +%FT%TZ) $*" | tee -a "$RUNS/queue.log"; }

one() { # draw arm fix
  local k=$1 a=$2 fix=$3 out=$RUNS/$NAME.d$1.$2.jsonl
  if [ -s "$out" ]; then log "$NAME d$k $a: done already, skipped"; return; fi
  log "$NAME d$k $a: start fix=$fix"
  local start; start=$(date +%s)
  (cd "$REPO" && prlimit --as=15000000000 timeout 7200 "$CLOSURE" --to 4 --max-cols 130000 \
     --draw "$k" --fix "$fix" "$F") > "$out" 2> "$RUNS/$NAME.d$k.$a.log"
  log "$NAME d$k $a: exit $? after $(( $(date +%s) - start )) s"
}

failed() { # draw: has a G4 branch that saturated degree 4 without refuting
  grep -l '"degree":4,"xl_refuted":false,"closure_refuted":false,"saturated":true' \
    "$RUNS/$NAME.d$1".G4b*.jsonl >/dev/null 2>&1
}

case $STEP in
gen)
  (cd "$REPO" && "$LADDER" --cells 22:8:3 --m 3 --unsat 4 --ffd-max 3 --seed 20261022) \
    > "$DRAWS/$NAME.jsonl" 2> "$DRAWS/$NAME.log" ;;
gen-shards) # Amendment 1: draws from 3 on, four shards in parallel
  for i in 0 1 2 3; do
    (cd "$REPO" && "$LADDER" --cells 22:8:3 --m 3 --ffd-max 3 --seed 20261022 \
       --shard "$i/4" --first-draw 3 --max-draws 16) \
      > "$DRAWS/shard-$i.jsonl" 2> "$DRAWS/shard-$i.log" &
  done
  wait ;;
measure)
  draws=$(grep -v '"satisfiable"' "$REPO/$F" | grep -o '"draw":[0-9]*' | cut -d: -f2)
  for k in $draws; do one "$k" G6 6=0,7=0,14=0,15=0,22=0,23=0; done
  for b in $(seq 0 15); do
    for k in $draws; do
      if failed "$k"; then log "$NAME d$k G4b$b: draw already fails, skipped"; continue; fi
      one "$k" "G4b$b" "7=$((b & 1)),15=$((b >> 1 & 1)),23=$((b >> 2 & 1)),24=$((b >> 3 & 1))"
    done
  done ;;
esac
