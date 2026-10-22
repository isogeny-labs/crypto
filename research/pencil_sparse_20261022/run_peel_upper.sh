#!/usr/bin/env bash
set -u
root=research/pencil_sparse_20261022
stem="$root/results/p186379-peel-t0"
date -u +%Y-%m-%dT%H:%M:%SZ > "$stem.started"
timeout 3600s env PENCIL_MAX_TARGETS=1 PENCIL_NODE_ORDER=identity PENCIL_SOLVER=peel \
  "$root/target/release/pencil-sparse" --cohort 32 186379 \
  > "$stem.jsonl" 2> "$stem.stderr"
status=$?
printf '%s\n' "$status" > "$stem.exit"
date -u +%Y-%m-%dT%H:%M:%SZ > "$stem.ended"
exit "$status"
