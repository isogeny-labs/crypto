#!/usr/bin/env bash
set -uo pipefail
root=research/vieta_coset_rust_20261115/speedup-20261117
binary=${1:?immutable worker}
failed=0
for config in "$root"/configs/primary-budget8b/*.json; do
 name=$(basename "$config" .json)
 code=0
 timeout --signal=TERM --kill-after=3s 600 "$binary" panel "$config" "$root/runs/primary-budget8b/$name" || code=$?
 printf '{"configuration":"%s","exit":%s}\n' "$name" "$code" >> "$root/evidence/budget-classes.jsonl"
 if [[ "$code" -eq 101 ]]; then exit 101; fi
 if [[ "$code" -ne 0 ]]; then failed=4; fi
done
exit "$failed"
