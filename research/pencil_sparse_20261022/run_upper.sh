#!/usr/bin/env bash
set -u
root=research/pencil_sparse_20261022
date -u +%Y-%m-%dT%H:%M:%SZ > "$root/results/p186379-identity-t0.started"
timeout 3600s env PENCIL_MAX_TARGETS=1 PENCIL_NODE_ORDER=identity \
  "$root/target/release/pencil-sparse" --cohort 32 186379 \
  > "$root/results/p186379-identity-t0.jsonl" \
  2> "$root/results/p186379-identity-t0.stderr"
status=$?
printf '%s\n' "$status" > "$root/results/p186379-identity-t0.exit"
date -u +%Y-%m-%dT%H:%M:%SZ > "$root/results/p186379-identity-t0.ended"
exit "$status"
