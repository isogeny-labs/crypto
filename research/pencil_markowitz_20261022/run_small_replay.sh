#!/usr/bin/env bash
set -u
root=research/pencil_markowitz_20261022
stem="$root/results/replay/p739-markowitz"
mkdir -p "$root/results/replay"
date -u +%Y-%m-%dT%H:%M:%SZ > "$stem.started"
timeout 3600s env PENCIL_MAX_TARGETS=60 PENCIL_NODE_ORDER=identity \
  PENCIL_SOLVER=markowitz PENCIL_MATRIX_CAP=10000000 \
  research/pencil_sparse_20261022/target/release/pencil-sparse --cohort 16 739 \
  > "$stem.jsonl" 2> "$stem.stderr"
status=$?
printf '%s\n' "$status" > "$stem.exit"
date -u +%Y-%m-%dT%H:%M:%SZ > "$stem.ended"
exit "$status"
