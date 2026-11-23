#!/usr/bin/env bash
set -euo pipefail
build=$(realpath "${1:?build}");public=$(realpath "${2:?public}");out=${3:?new output};test ! -e "$out";mkdir "$out"
for f in "$public"/*.json;do
 seed=$(jq -r .public_seed "$f")
 for mode in balanced quarter half one two;do
  TRACE_PAIR_KERNEL=barrett "$build/target-scaling-bloom" --check-only "$mode" "$f" "$seed" >> "$out/field.jsonl"
 done
 TRACE_PAIR_KERNEL=barrett "$build/target-scaling-direct" --check-only balanced "$f" "$seed" >> "$out/direct-balanced.jsonl"
 TRACE_PAIR_KERNEL=barrett "$build/target-scaling-baseline" --check-only balanced "$f" "$seed" >> "$out/baseline.jsonl"
done
jq -s '{calls:length,expected:960,passed:(length==960 and all(.timings_collected==false))}' "$out/field.jsonl" > "$out/field-summary.json"
jq -s '{calls:length,expected:192,passed:(length==192 and all(.timings_collected==false))}' "$out/baseline.jsonl" > "$out/baseline-summary.json"
jq -s '{calls:length,expected:192,passed:(length==192 and all(.timings_collected==false))}' "$out/direct-balanced.jsonl" > "$out/direct-balanced-summary.json"
