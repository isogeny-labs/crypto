#!/usr/bin/env bash
set -euo pipefail
build=$(realpath "${1:?build}");public=$(realpath "${2:?public}");out=${3:?new output};test ! -e "$out";mkdir "$out"
for f in "$public"/*.json;do
 seed=$(jq -r .public_seed "$f")
 for mode in ap2 ap4 ap8;do
  TRACE_PAIR_KERNEL=barrett "$build/target-flat-full" --check-only "$mode" "$f" "$seed" >> "$out/full.jsonl"
 done
 TRACE_PAIR_KERNEL=barrett "$build/target-flat-gen" --check-only ap4 "$f" "$seed" >> "$out/gen.jsonl"
done
jq -s '{calls:length,expected:1440,passed:(length==1440 and all(.timings_collected==false))}' "$out/full.jsonl" > "$out/full-summary.json"
jq -s '{calls:length,expected:480,passed:(length==480 and all(.timings_collected==false))}' "$out/gen.jsonl" > "$out/gen-summary.json"
