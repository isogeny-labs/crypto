#!/bin/sh
# PROTOCOL.md section 7, thin orchestration only (every figure is native `ecbench`):
# the section 5a band with pairwise verdicts, the paired comparisons for every
# registered pair at w = 0, 0.1 and 1, the ratio table, and the figures.
set -eu
cd "$(dirname "$0")"
E=${ECBENCH:-../../target/release/ecbench}
mkdir -p analysis
"$E" band --dir sessions/P20 sessions/P24 sessions/P28 sessions/P32 --weights 0,0.1,1 --json analysis/band.json --ranks > analysis/band.md
for b in 20 24 28 32; do
  for pair in inc:ksum5 inc:ksum4 ksum4:ksum5 rho-neg:ksum5 bsgs-neg:ksum5; do
    a=${pair%%:*}; c=${pair#*:}
    for w in 0 0.1 1; do
      "$E" compare --dir "sessions/P$b" --a "$a" --b "$c" --weight "$w" --json > "analysis/compare-P$b-${a}__${c}-w$w.json"
    done
  done
done
{
  echo "| bits | pair (B / A) | w = 0 | w = 0.1 | w = 1 | band order |"
  echo "|--:|:--|:--|:--|:--|:--|"
  for b in 20 24 28 32; do
    for pair in inc:ksum5 inc:ksum4 ksum4:ksum5 rho-neg:ksum5 bsgs-neg:ksum5; do
      a=${pair%%:*}; c=${pair#*:}; row="| $b | $c / $a |"
      for w in 0 0.1 1; do
        row="$row $(jq -r '.ops | "\(.ratio_b_over_a*1000|round/1000) [\(.ci95_bootstrap[0]*1000|round/1000), \(.ci95_bootstrap[1]*1000|round/1000)]"' "analysis/compare-P$b-${a}__${c}-w$w.json") |"
      done
      echo "$row $(jq -r '.ops.band.order' "analysis/compare-P$b-${a}__${c}-w0.json") |"
    done
  done
} > analysis/ratios.md
node plot.mjs
for f in ratio rho_multiple; do rsvg-convert -z 2 "$f.svg" -o "$f.png"; done
