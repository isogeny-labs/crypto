#!/usr/bin/env bash
# The closure-slope study (PREREGISTRATION.md).  Thin orchestration only:
# every computation is examples/dreg_ladder (draws) and examples/dreg_closure
# --sparse (closure at one degree on the sparse engine).
#
#   research/dreg_closure_slope_20261018/run.sh LADDER_BIN CLOSURE_BIN [gen|check|measure|control]
#
# gen:     write draws/<cell>.jsonl from the frozen seed (exact solution count,
#          XL to degree 3 only).
# check:   P4, the instrument check: sparse degree-4 closure on committed dense
#          degree-4 refutations (two draws each of m3 (18,6) and m4 (13,4)).
# measure: per unsatisfiable draw, degrees in turn (4,5,6 fresh; 5,6 for the
#          committed m·ℓ = 21 cells), stopping at the first refutation;
#          cheapest cell first; at most $JOBS processes at a time.  A
#          (draw, degree) whose runs/<cell>.d<K>.D<d>.jsonl is non-empty is
#          skipped, so a restart loses only the runs in flight.
# control: P5, the first satisfiable draw of each fresh cell at degrees 4, 5.
set -u
LADDER=$1
CLOSURE=$2
STEP=${3:-measure}
JOBS=${JOBS:-2}
HERE=$(cd "$(dirname "$0")" && pwd)
REPO=$(cd "$HERE/../.." && pwd)
DRAWS=$HERE/draws
RUNS=$HERE/runs
mkdir -p "$DRAWS" "$RUNS"
SEED=20261018
MAX_ENTRIES=350000000
AS_LIMIT=7000000000
TIME_LIMIT=10800
# m:n:ell, cheapest (smallest N) first
FRESH="3:22:8 3:24:8 3:25:9 3:27:9 4:18:5 3:28:10 3:30:10 4:20:5 4:24:6"
SCALE=research/dreg_closure_scale_20261016/draws
COMMITTED21="3:19:7:$SCALE/m3-19-7.jsonl 3:21:7:$SCALE/m3-21-7.jsonl"
log() { echo "$(date -u +%FT%TZ) $*" | tee -a "$RUNS/queue.log"; }

one() { # name file draw degree [extra flags]
  local name=$1 file=$2 k=$3 d=$4; shift 4
  local out=$RUNS/$name.d$k.D$d.jsonl
  if [ -s "$out" ]; then log "$name d$k D$d: done already, skipped"; return 0; fi
  log "$name d$k D$d: start"
  local start; start=$(date +%s)
  (cd "$REPO" && prlimit --as=$AS_LIMIT timeout $TIME_LIMIT \
     "$CLOSURE" --sparse "$d" --max-entries $MAX_ENTRIES --draw "$k" "$@" "$file") > "$out" 2> "$RUNS/$name.d$k.D$d.log"
  local rc=$?
  log "$name d$k D$d: exit $rc after $(( $(date +%s) - start )) s"
  return 0
}

# Scan one draw over the degrees given, stopping at the first refutation.
scan() { # name file draw degrees...
  local name=$1 file=$2 k=$3; shift 3
  for d in "$@"; do
    one "$name" "$file" "$k" "$d"
    local out=$RUNS/$name.d$k.D$d.jsonl
    if [ -s "$out" ] && grep -q '"closure_refuted":true' "$out"; then
      log "$name d$k: refuted at degree $d"
      return
    fi
  done
}

unsat_draws() { grep -v '"satisfiable"' "$1" | grep -o '"draw":[0-9]*' | cut -d: -f2; }
sat_draws() { grep '"satisfiable"' "$1" | grep -o '"draw":[0-9]*' | cut -d: -f2; }

case $STEP in
gen)
  for c in $FRESH; do
    IFS=: read -r m n ell <<< "$c"
    f=$DRAWS/m$m-$n-$ell.jsonl
    if [ -s "$f" ]; then log "gen m$m-$n-$ell: exists, skipped"; continue; fi
    log "gen m$m-$n-$ell: start"
    (cd "$REPO" && "$LADDER" --cells "$n:$ell:3" --m "$m" --unsat 4 --ffd-max 3 --seed $SEED) \
      > "$f" 2> "$DRAWS/m$m-$n-$ell.log"
    log "gen m$m-$n-$ell: exit $? ($(grep -vc satisfiable "$f") unsat, $(grep -c satisfiable "$f") sat)"
  done ;;
check)
  for k in 0 1; do
    one "check-m3-18-6" "$SCALE/m3-18-6.jsonl" "$(unsat_draws "$REPO/$SCALE/m3-18-6.jsonl" | sed -n "$((k+1))p")" 4
    one "check-m4-13-4" "$SCALE/m4-13-4.jsonl" "$(unsat_draws "$REPO/$SCALE/m4-13-4.jsonl" | sed -n "$((k+1))p")" 4
  done ;;
measure)
  # Build the job list: committed m·ℓ = 21 cells at degrees 5, 6; fresh cells at 4, 5, 6.
  jobs=()
  for c in $COMMITTED21; do
    IFS=: read -r m n ell file <<< "$c"
    for k in $(unsat_draws "$REPO/$file"); do jobs+=("committed-m$m-$n-$ell|$file|$k|5 6"); done
  done
  for c in $FRESH; do
    IFS=: read -r m n ell <<< "$c"
    f=research/dreg_closure_slope_20261018/draws/m$m-$n-$ell.jsonl
    [ -s "$REPO/$f" ] || continue
    for k in $(unsat_draws "$REPO/$f"); do jobs+=("fresh-m$m-$n-$ell|$f|$k|4 5 6"); done
  done
  for j in "${jobs[@]}"; do
    IFS='|' read -r name file k degrees <<< "$j"
    # shellcheck disable=SC2086
    scan "$name" "$file" "$k" $degrees &
    while [ "$(jobs -rp | wc -l)" -ge "$JOBS" ]; do sleep 5; done
  done
  wait ;;
control)
  for c in $FRESH; do
    IFS=: read -r m n ell <<< "$c"
    f=research/dreg_closure_slope_20261018/draws/m$m-$n-$ell.jsonl
    [ -s "$REPO/$f" ] || continue
    k=$(sat_draws "$REPO/$f" | head -1)
    [ -n "$k" ] || { log "control m$m-$n-$ell: no satisfiable draw recorded"; continue; }
    for d in 4 5; do one "control-m$m-$n-$ell" "$f" "$k" "$d" --satisfiable; done
  done ;;
esac
