#!/usr/bin/env bash
# E11 scaling: fresh newcells draws (hky_par's draw rule, seed 20261012) on the
# balanced-diagonal cells, window policy w1x, one process per cell (so VmHWM is
# that cell's), then the dense engine hky_par on the same draws for reference.
# Peak RSS of each dense draw: `fall_degree_e11_tools maxrss` (examples/fall_degree_e11_tools.rs).
# Local cells only (N <= 48); N >= 58 is not run here.
# Usage (repo root): bash research/fall_degree_bounds_20261008/e11/run_scaling.sh
set -euo pipefail
S=research/fall_degree_bounds_20261008/e11
W=${HKY_WINDOW:-target/release/examples/hky_window}
P=${HKY_PAR:-target/release/examples/hky_par}
T=${E11_TOOLS:-target/release/examples/fall_degree_e11_tools}
SEED=20261012
KEEP=${KEEP:-4}
mkdir -p "$S/scaling"
for cell in ${CELLS:-4:12:3 3:21:7 4:16:4}; do
  tag=${cell//:/-}
  out=$S/scaling/window_$tag.jsonl
  if [ ! -s "$out" ]; then
    echo "== window $cell ($(date -u +%H:%M:%S))" >&2
    "$W" newcells --cells "$cell" --seed "$SEED" --keep "$KEEP" --policies w1x >"$out.part" 2>"$S/scaling/window_$tag.err"
    mv "$out.part" "$out"
  fi
  ref=$S/scaling/dense_$tag.jsonl
  if [ ! -s "$ref" ] && [ -z "${NO_DENSE:-}" ]; then
    m=${cell%%:*}
    echo "== dense $cell ($(date -u +%H:%M:%S))" >&2
    # one process per draw, so each VmHWM is that draw's
    : >"$ref.part"
    while read -r line; do
      d=$(jq -c '{draw, n, v_basis, x_r, solutions, m}' <<<"$line")
      tmp=$(mktemp)
      echo "$d" >"$tmp"
      "$T" maxrss "$tmp.rss" "$P" ladder "$tmp" --m "$m" --cmax 4 >"$tmp.out"
      jq -c --argjson rss "$(cat "$tmp.rss")" --argjson draw "$(jq .draw <<<"$d")" \
        '. + {maxrss_kb: $rss, draw: $draw}' "$tmp.out" >>"$ref.part"
      rm -f "$tmp" "$tmp.rss" "$tmp.out"
    done <"$out"
    mv "$ref.part" "$ref"
  fi
done
"$T" scaling-table >"$S/scaling_table.json"
