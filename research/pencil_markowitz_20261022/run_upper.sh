#!/usr/bin/env bash
set -u
root=research/pencil_markowitz_20261022
stem="$root/results/p186379-markowitz-t0"
mkdir -p "$root/results"
date -u +%Y-%m-%dT%H:%M:%SZ > "$stem.started"
(
  ulimit -v 8388608
  timeout 3600s env PENCIL_MAX_TARGETS=1 PENCIL_NODE_ORDER=identity \
    PENCIL_SOLVER=markowitz PENCIL_MATRIX_CAP=25000000 \
    research/pencil_sparse_20261022/target/release/pencil-sparse --cohort 32 186379
) > "$stem.jsonl" 2> "$stem.stderr"
status=$?
printf '%s\n' "$status" > "$stem.exit"
date -u +%Y-%m-%dT%H:%M:%SZ > "$stem.ended"
research/pencil_sparse_20261022/target/release/cap-process-status "$status" "$stem"
exit "$status"
