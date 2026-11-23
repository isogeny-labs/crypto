#!/usr/bin/env bash
set -euo pipefail
build=$(realpath "${1:?qualified build}");public=$(realpath "${2:?public}");out=${3:?new output};test ! -e "$out";mkdir "$out"
for f in "$public"/*.json;do
 seed=$(jq -r .public_seed "$f")
 for mode in balanced quarter;do
  for variant in combined combined-native;do
   TRACE_PAIR_KERNEL=barrett "$build/target-hot-$variant" --check-only "$mode" "$f" "$seed" >> "$out/$variant.jsonl"
  done
 done
done
for variant in combined combined-native;do jq -s '{calls:length,expected:384,passed:(length==384 and all(.timings_collected==false))}' "$out/$variant.jsonl" > "$out/$variant-summary.json";done
