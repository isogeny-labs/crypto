#!/usr/bin/env bash
# Writes analysis/summary.json from the band reports and comparison files. jq only.
set -euo pipefail
D="$(cd "$(dirname "$0")/.." && pwd)"
cmps=$(for c in r24 r26 r28 c24 c28 c30; do for f in "$D"/sessions/$c/comparisons/*.json; do
  jq -c --arg c $c '{curve_tag: $c, a: .a.arm, b: .b.arm, ratio: .ops.ratio_b_over_a, ci95: .ops.ci95}' "$f"; done; done | jq -s .)
bands=$(for c in r24 r26 r28 c24 c28 c30; do jq -c --arg c $c '. + {curve_tag: $c}' "$D/analysis/band-$c.json"; done | jq -s .)
jq -n --argjson cmps "$cmps" --argjson bands "$bands" \
  --argjson curves '["r24","r26","r28","c24","c28","c30"]' \
  --argjson predicted '{"r24": 12.233956652458087, "r26": 14.453197377819107, "r28": 20.078173411100984, "c24": 11.461210709103137, "c28": 19.325980413591978, "c30": 24.54954651510073}' \
  -f "$D/scripts/summary.jq" > "$D/analysis/summary.json"
