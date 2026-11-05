#!/usr/bin/env bash
# Thin orchestration: run the frozen bench for each m of a shard.
# Usage: run_shard.sh <binary> <out_dir> <m,m,...>
set -euo pipefail
bin=$1; out=$2; ms=$3
mkdir -p "$out"
for m in ${ms//,/ }; do
  RAYON_NUM_THREADS=1 "$bin" "$m" 2 3 2 300000 20261031 \
    unordered,all-summands,all-summands+t2.2,all-summands+affine+t2.2 120 \
    > "$out/m$m.jsonl" 2> "$out/m$m.err" || echo "m=$m exit $?" >> "$out/m$m.err"
done
