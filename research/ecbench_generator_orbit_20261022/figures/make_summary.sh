#!/usr/bin/env bash
# Collects recorded values into figures/summary.json: the §5a band rows
# (tables/band-*.json, written by `ecbench band`), the saved paired
# comparisons (sessions/*/comparisons/*.json, `ecbench compare --save`) and
# per-arm phase means over non-warm-up records. jq only; no new result.
set -euo pipefail
HERE="$(cd "$(dirname "$0")/.." && pwd)"
out='[]'
for d in "$HERE"/sessions/*/; do
  name=$(basename "$d")
  band=$(jq '.rows' "$HERE/tables/band-$name.json")
  cmps=$(jq -s 'map({a: .a.arm, b: .b.arm, ratio: .ops.ratio_b_over_a, ci95: .ops.ci95, pairs: .ops.pairs})' "$d"/comparisons/*.json)
  phases=$(jq -s 'map(select(.warmup|not)) | group_by(.arm) | map({arm: .[0].arm, runs: length,
      gae: (map(.cost.total_gae)|add/length),
      trials: (map(.phases[] | select(.name=="relations") | .native.trials // empty) | if length>0 then add/length else null end),
      phases: (map(.phases | map({(.name): .gae}) | add) | (map(keys)|add|unique) as $k | [$k[] as $p | {($p): (map(.[$p] // 0)|add/length)}] | add)})' "$d/records.jsonl")
  out=$(jq --arg name "$name" --argjson band "$band" --argjson cmps "$cmps" --argjson ph "$phases" \
    '. + [{session: $name, a: ($name[1:2]|tonumber), n: ($name|split("-n")[1]|tonumber),
           log2r: $band[0].log2_r, band: $band, comparisons: $cmps, arms: $ph}]' <<<"$out")
done
jq '{schema: "crypto.generator_orbit_study/v1", weights: [0, 0.1, 1], curves: (sort_by(.log2r))}' <<<"$out" > "$HERE/figures/summary.json"
echo "$HERE/figures/summary.json"
