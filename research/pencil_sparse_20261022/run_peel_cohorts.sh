#!/usr/bin/env bash
set -u
root=research/pencil_sparse_20261022
run_curve() {
  local bits="$1" p="$2" stem="$root/results/p${2}-peel"
  date -u +%Y-%m-%dT%H:%M:%SZ > "$stem.started"
  timeout 3600s env PENCIL_SOLVER=peel PENCIL_NODE_ORDER=identity \
    "$root/target/release/pencil-sparse" --cohort "$bits" "$p" \
    > "$stem.jsonl" 2> "$stem.stderr"
  local status=$?
  printf '%s\n' "$status" > "$stem.exit"
  date -u +%Y-%m-%dT%H:%M:%SZ > "$stem.ended"
}
for entry in 16:1151 16:1187 20:3371 20:3527 20:3931 24:11731 24:11887 24:12143; do
  bits="${entry%%:*}"
  p="${entry#*:}"
  run_curve "$bits" "$p" &
done
wait
