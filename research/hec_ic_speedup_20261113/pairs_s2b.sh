#!/usr/bin/env bash
# One JSON line per (row, repeat, seed) pair of a run_s2b.sh directory:
# solve_ms of both solvers and their ratio fast/dense (jq; layout only).
#   research/hec_ic_speedup_20261113/pairs_s2b.sh OUT_DIR > OUT_DIR/pairs.jsonl
set -euo pipefail
out=${1:?usage: pairs_s2b.sh OUT_DIR}
for f in "$out"/rep*.txt; do
  rep=$(basename "$f" .txt)
  grep '^JSON' "$f" | sed 's/^JSON //' | jq -c --arg rep "$rep" '. + {rep: $rep}'
done | jq -c -s 'group_by([.genus, .p, .rep, .seed])[] |
  (map(select(.solver == "fast"))[0]) as $f | (map(select(.solver == "dense"))[0]) as $d |
  {genus: $f.genus, p: $f.p, rep: $f.rep, seed: $f.seed, fast_solve_ms: $f.solve_ms,
   dense_solve_ms: $d.solve_ms, ratio: ($f.solve_ms / $d.solve_ms)}'
