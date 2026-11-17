#!/usr/bin/env bash
# LEADS.md L1: the two cells research/dreg_closure_scale_20261016/RESULTS.md names as
# deciding the "set by m·ℓ" reading: m = 4, ℓ = 5 (m·ℓ = 20) and ℓ = 6 (24), at n = 13
# (N = m·ℓ + (m−2)·n = 46 and 48). Same engine, seed and flags as the scale study
# (examples/dreg_ladder for the draws, examples/dreg_closure --to 4), with the memory
# cap raised to 14 GB and --max-cols to 200000 because the (4,5) cell is predicted at
# about 179k columns and 15 GB.  Usage: run.sh gen|measure
set -u
HERE=$(cd "$(dirname "$0")" && pwd); REPO=$(cd "$HERE/../.." && pwd)
LADDER=$REPO/target/release/examples/dreg_ladder; CLOSURE=$REPO/target/release/examples/dreg_closure
DRAWS=$HERE/draws; RUNS=$HERE/runs; mkdir -p "$DRAWS" "$RUNS"
export F4_F2_MAX_ROWS=50000000 F4_F2_MAX_COLS=50000000 RAYON_NUM_THREADS=${RAYON_NUM_THREADS:-4}
SEED=20261016
CELLS="4:13:5 4:13:6"
log() { echo "$(date -u +%FT%TZ) $*" | tee -a "$RUNS/queue.log"; }
case ${1:-measure} in
gen)
  for c in $CELLS; do IFS=: read -r m n ell <<< "$c"
    (cd "$REPO" && "$LADDER" --cells "$n:$ell:3" --m "$m" --unsat 4 --ffd-max 3 --seed $SEED) > "$DRAWS/m$m-$n-$ell.jsonl" 2> "$DRAWS/m$m-$n-$ell.log"
    log "gen m$m-$n-$ell: $(wc -l < "$DRAWS/m$m-$n-$ell.jsonl") draws, exit $?"
  done ;;
measure)
  for c in $CELLS; do IFS=: read -r m n ell <<< "$c"; f="$DRAWS/m$m-$n-$ell.jsonl"
    for k in $(grep -v '"satisfiable"' "$f" | grep -o '"draw":[0-9]*' | cut -d: -f2); do out="$RUNS/m$m-$n-$ell.d$k.jsonl"
      if [ -s "$out" ]; then log "m$m-$n-$ell d$k: done already"; continue; fi
      log "m$m-$n-$ell d$k: start"; start=$(date +%s)
      (cd "$REPO" && prlimit --as=13000000000 timeout 10800 "$CLOSURE" --to 4 --max-cols 200000 --draw "$k" "$f") > "$out" 2> "$RUNS/m$m-$n-$ell.d$k.log"
      log "m$m-$n-$ell d$k: exit $? after $(( $(date +%s) - start )) s"
    done
  done ;;
esac
