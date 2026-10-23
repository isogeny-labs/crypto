#!/usr/bin/env bash
# PROTOCOL.md section 5 and Amendment 2: the frozen selection. Reads stage2/paired-n{N}.json (the output of
#   ic_paired_band --objective-base base-k{Kb} --out stage2/paired-n{N}.json sessions/stage2-n{N})
# and writes selection.json: per curve the dlp cell and the baseline cell with the smallest
# minimax_vs_base (ties: lowest K, then lowest c). Cells with an unverified run have a null
# objective and are skipped.
set -euo pipefail
cd "$(dirname "$0")"
declare -A KB=([37]=8 [41]=128 [43]=16 [47]=64)
out='{}'
for n in 37 41 43 47; do
  f=stage2/paired-n$n.json
  [ -f "$f" ] || continue
  sel=$(jq -c --argjson kb ${KB[$n]} '
    [.arms[] | select(.minimax_vs_base != null)] as $a
    | { dlp: ([$a[] | select(.arm|startswith("dlp-k"))
               | (.arm|capture("dlp-k(?<k>[0-9.]+)-c(?<c>[0-9.]+)")) as $p
               | {k:($p.k|tonumber), c:($p.c|tonumber), objective:.minimax_vs_base}]
              | sort_by([.objective,.k,.c]) | .[0]),
        base_retuned: ([$a[] | select(.arm|startswith("base-k"))
               | {k:(.arm|ltrimstr("base-k")|tonumber), objective:.minimax_vs_base}]
              | sort_by([.objective, (if .k==$kb then 0 else 1 end), .k]) | .[0]),
        cells_ranked: ([$a[] | select(.arm|startswith("dlp-k"))] | length) }' "$f")
  out=$(jq -c --arg n $n --argjson s "$sel" '. + {($n):$s}' <<<"$out")
done
jq -S . <<<"$out" > selection.json
sha256sum selection.json | tee SELECTION.sha256
