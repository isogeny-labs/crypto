#!/usr/bin/env bash
# PROTOCOL.md Amendment 2: stage-1 pruning. Reads stage1/paired-n{N}.json (ic_paired_band
# --objective-base base-k{Kb}) and writes survivors.json: per curve the 10 double-LP cells with
# the smallest minimax_vs_base among cells whose stage-1 runs all verified (ties: lowest K,
# then lowest c).
set -euo pipefail
cd "$(dirname "$0")"
out='{}'
for n in 37 41 43 47; do
  f=stage1/paired-n$n.json
  [ -f "$f" ] || continue
  s=$(jq -c '[.arms[] | select(.minimax_vs_base != null and (.arm|startswith("dlp-k")))
      | (.arm|capture("dlp-k(?<k>[0-9.]+)-c(?<c>[0-9.]+)")) as $p
      | {arm, k:($p.k|tonumber), c:($p.c|tonumber), o:.minimax_vs_base}]
      | sort_by([.o,.k,.c]) | .[0:10] | map(.arm)' "$f")
  out=$(jq -c --arg n $n --argjson s "$s" '. + {($n):$s}' <<<"$out")
done
jq -S . <<<"$out" > survivors.json
sha256sum survivors.json | tee SURVIVORS.sha256
