#!/bin/bash
# XP-20261015-shape-g1-frozen-cells, PROTOCOL.md (card directory). One process per
# (cell, shape), one thread, prlimit --as cap and a KILL timeout; each writes
# runs/g1-<n>-<l>-d<dmax>-<shape>.jsonl (.err, .meta). progress.txt records starts
# and ends; a finished (cell, shape) is not repeated.
#   run.sh stage A            # (13,5) d_max 6, all five shapes in parallel
#   run.sh stage B [shapes]   # (13,5) d_max 7, the named shapes (default all), launched together: keep <= 2 at a time
#   run.sh stage C <n:l:d> [shapes]
#   run.sh score              # factor_base_shape_ladder score runs/*.jsonl > summary.md
set -u
here=$(cd "$(dirname "$0")" && pwd)
bin=${BIN:-$here/../../target/release/examples/factor_base_shape_ladder}
export RAYON_NUM_THREADS=1 F4_F2_MAX_ROWS=50000000 F4_F2_MAX_COLS=50000000 KIC_SPARSE_DENSE_FINISH=1 KIC_SPARSE_F5=1 KIC_SPARSE_DENSE_BUDGET_MB=1200
mkdir -p "$here/runs"
SEED=20261015

launch() { # cell shape cap_gb wall_s
  local cell=$1 shape=$2 cap_gb=$3 wall=$4
  local n=${cell%%:*} rest=${cell#*:} l d; l=${rest%%:*}; d=${rest#*:}
  local name="g1-$n-$l-d$d-$shape"
  if grep -q " end $name " "$here/progress.txt" 2>/dev/null; then echo "$name done, skipped"; return; fi
  echo "$(date -u +%FT%TZ) start $name cap=${cap_gb}GB wall=${wall}s: g1 --cell $cell --shapes $shape --instances 2 --targets 2 --seed $SEED" >> "$here/progress.txt"
  (
    start=$(date +%s)
    ( prlimit --as=$((cap_gb*1024*1024*1024)) -- timeout --signal=KILL "$wall" "$bin" g1 --cell "$cell" --shapes "$shape" --instances 2 --targets 2 --seed $SEED > "$here/runs/$name.jsonl" 2> "$here/runs/$name.err" ) &
    pid=$!; peak=0
    while kill -0 $pid 2>/dev/null; do
      rss=$(ps -o rss= --ppid $pid 2>/dev/null | sort -n | tail -1); rss=${rss:-0}
      [ "$rss" -gt "$peak" ] && peak=$rss
      sleep 10
    done
    wait $pid; rc=$?; end=$(date +%s)
    echo "{\"args\":\"g1 --cell $cell --shapes $shape --instances 2 --targets 2 --seed $SEED\",\"exit\":$rc,\"wall_s\":$((end-start)),\"peak_rss_kB_sampled\":$peak,\"cap_gb\":$cap_gb,\"wall_cap_s\":$wall}" > "$here/runs/$name.meta"
    echo "$(date -u +%FT%TZ) end $name exit $rc wall=$((end-start))s peak_rss_kB=$peak" >> "$here/progress.txt"
  ) &
}

case "${1:-}" in
  stage)
    case "$2" in
      A) for s in random random2 gp fp coord; do launch 13:5:6 "$s" 16 86400; done ;;
      B) shift 2; for s in "${@:-random random2 gp fp coord}"; do launch 13:5:7 "$s" 60 1209600; done ;;
      C) cell=$3; shift 3; d=${cell##*:}; cap=24; [ "$d" = 7 ] && cap=60; for s in "${@:-random random2 gp fp coord}"; do launch "$cell" "$s" $cap 1209600; done ;;
    esac
    wait ;;
  score) "$bin" score "$here"/runs/*.jsonl > "$here/summary.md"; cat "$here/summary.md" ;;
  *) echo "usage: $0 stage A|B|C … | score" >&2; exit 2 ;;
esac
