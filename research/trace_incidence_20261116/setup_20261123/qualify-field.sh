#!/usr/bin/env bash
set -euo pipefail
build=$(realpath "${1:?build}");public=$(realpath "${2:?public}");out=${3:?new output};test ! -e "$out";mkdir "$out"
for f in "$public"/*.json;do
 seed=$(jq -r .public_seed "$f")
 for mode in ap4 ap8;do
  TRACE_PAIR_KERNEL=barrett "$build/target-field-bloom" --check-only "$mode" "$f" "$seed" >> "$out/bloom.jsonl"
  TRACE_PAIR_KERNEL=barrett "$build/target-field-direct" --check-only "$mode" "$f" "$seed" >> "$out/direct.jsonl"
 done
done
for arm in bloom direct;do jq -s '{calls:length,expected:960,passed:(length==960 and all(.timings_collected==false))}' "$out/$arm.jsonl" > "$out/$arm-summary.json";done
