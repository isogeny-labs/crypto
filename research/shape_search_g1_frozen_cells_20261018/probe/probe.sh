#!/bin/bash
# Timed sizing probe for XP-20261015-shape-g1-frozen-cells (disclosed; not a result).
# One random draw per cell at the card's d_max = 7, seed 1 (not the measured seed), 1 thread,
# address-space cap 64 GB, 4 h wall cap. Output: <cell>.jsonl, .err, .meta (exit, wall, sampled peak RSS).
set -u
here=$(cd "$(dirname "$0")" && pwd)
bin=$here/../../../target/release/examples/factor_base_shape_ladder
export RAYON_NUM_THREADS=${4:-1} F4_F2_MAX_ROWS=50000000 F4_F2_MAX_COLS=50000000 KIC_SPARSE_DENSE_FINISH=1 KIC_SPARSE_F5=1 KIC_SPARSE_DENSE_BUDGET_MB=1200
cell=$1; cap_gb=${2:-64}; wall=${3:-14400}; threads=${4:-1}
name=probe-${cell//:/-}-th$threads
echo "$(date -u +%FT%TZ) start $name cap=${cap_gb}GB wall=${wall}s" >> "$here/progress.txt"
start=$(date +%s)
( prlimit --as=$((cap_gb*1024*1024*1024)) -- timeout --signal=KILL "$wall" "$bin" g1 --cell "$cell" --shapes random --instances 1 --targets 1 --seed 1 > "$here/$name.jsonl" 2> "$here/$name.err" ) &
pid=$!; peak=0
while kill -0 $pid 2>/dev/null; do
  rss=$(ps -o rss= --ppid $pid 2>/dev/null | sort -n | tail -1); rss=${rss:-0}
  [ "$rss" -gt "$peak" ] && peak=$rss
  sleep 5
done
wait $pid; rc=$?
end=$(date +%s)
echo "{\"cell\":\"$cell\",\"exit\":$rc,\"wall_s\":$((end-start)),\"peak_rss_kB_sampled\":$peak,\"cap_gb\":$cap_gb}" > "$here/$name.meta"
echo "$(date -u +%FT%TZ) end $name exit $rc wall=$((end-start))s peak_rss_kB=$peak" >> "$here/progress.txt"
