#!/usr/bin/env bash
# One m = 9 process: build every depth-2..5 set (warmup lines), then the 8 queries.
# Usage: run_rep.sh <binary> <out_dir> <tag> <arm>
set -euo pipefail
bin=$1; out=$2; tag=$3; arm=$4
mkdir -p "$out"
RAYON_NUM_THREADS=4 "$bin" 9 2 3 5 300000 20261031 "$arm" 120 \
  > "$out/$tag.jsonl" 2> "$out/$tag.err" || echo "$tag exit $?" >> "$out/$tag.err"
