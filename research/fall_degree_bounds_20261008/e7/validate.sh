#!/usr/bin/env bash
# E7 gate (Amendment 7): examples/hky_par.rs must reproduce the validated
# engine's d_F and every per-degree closure dimension on the committed draws.
# Usage: run from the repo root; writes e7/validate/*.jsonl and prints one
# line per draw plus a final AGREE/DISAGREE count.
set -euo pipefail
S=research/fall_degree_bounds_20261008
B=target/release/examples/hky_par
O=$S/e7/validate
mkdir -p "$O"
cat research/dreg_m4_chain_20261006/runs/cell-9-3-6.u{0,1,2,3}.jsonl >"$O/inputs_m4_9-3.jsonl"
# reference records | ladder inputs | m
pairs=(
  "e4/m3_5-2.jsonl|research/dreg_fixed_surplus_20260923/runs/cell-5-2-7.jsonl|3"
  "e4/m3_7-3.jsonl|research/dreg_fixed_surplus_20260923/runs/cell-7-3-7.jsonl|3"
  "e4/x_m3_11-4.jsonl|research/dreg_fixed_surplus_20260923/runs/cell-11-4-6.jsonl|3"
  "e4/x_m3_13-4.jsonl|research/dreg_fixed_surplus_20260923/runs/cell-13-4-6.jsonl|3"
  "e6/A_m3_13-5.jsonl|$S/e6/inputs_13-5.jsonl|3"
  "e4/x_m4_9-3.jsonl|$O/inputs_m4_9-3.jsonl|4"
)
agree=0; dis=0
for p in "${pairs[@]}"; do
  IFS='|' read -r ref inp m <<<"$p"
  out=$O/$(basename "$ref")
  [ -s "$out" ] || "$B" ladder "$inp" --m "$m" --cmax 4 >"$out"
  while read -r line; do
    cell=$(jq -r .cell <<<"$line"); draw=$(jq -r .draw <<<"$line")
    mine=$(jq -c '{d_f, dims: (.per_degree | map_values(.dim))}' <<<"$line")
    theirs=$(jq -c --arg c "$cell" --argjson d "$draw" \
      'select(.cell==$c and .draw==$d) | {d_f, dims: .closure_dims}' "$S/$ref")
    if [ -n "$theirs" ] && [ "$mine" = "$theirs" ]; then agree=$((agree+1)); v=AGREE
    else dis=$((dis+1)); v=DISAGREE; fi
    echo "$v $ref $cell draw=$draw par=$mine ref=$theirs secs=$(jq .secs <<<"$line")"
  done <"$out"
done
echo "AGREE $agree DISAGREE $dis"
