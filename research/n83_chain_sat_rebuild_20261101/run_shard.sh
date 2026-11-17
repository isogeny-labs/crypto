#!/usr/bin/env bash
# Thin orchestration: run the frozen bench for one m.
# Usage: run_shard.sh <binary> <out_dir> <m> <meet_depth>
set -euo pipefail
bin=$1; out=$2; m=$3; d=$4
mkdir -p "$out"
RAYON_NUM_THREADS=1 "$bin" "$m" 2 3 5 300000 20261031 \
  "all-summands+t2.2+meet$d,all-summands+t2.2+meet$d+rebuild" 120 \
  > "$out/m$m.jsonl" 2> "$out/m$m.err" || echo "m=$m exit $?" >> "$out/m$m.err"
