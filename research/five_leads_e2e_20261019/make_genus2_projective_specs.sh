#!/usr/bin/env bash
set -euo pipefail
repo_root=$(git rev-parse --show-toplevel)
study_dir="$repo_root/research/five_leads_e2e_20261019"
output_dir="$study_dir/specs/genus2-projective-chart-v1"
mkdir -p "$output_dir"
for bits in 16 20 24 28 32 36 40; do
  input="$study_dir/specs/genus2-choice2-v1/order-genus-$bits.json"
  output="$output_dir/order-genus-$bits.json"
  jq '
    .label = ("five-leads-projective-chart-order-genus-" + (.label | capture("(?<bits>[0-9]+)$").bits))
    | .description = "Same frozen exact-order targets and cap; paired projective CRT chart, affine two-choice, and source negation rho; L0 operation counts"
    | .arms = ([.arms[] | select(.name == "rho_neg" or .name == "genus_choice2")]
      + [{"name":"genus_projective","role":"candidate","method":{"id":"ic.three_leads_genus2_projective_chart_v1","params":{"trial_cap":(.arms[] | select(.name == "genus_choice2") | .method.params.trial_cap)}}}])
  ' "$input" > "$output"
done
