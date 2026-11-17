#!/bin/sh
# Collect the recorded values the report plots, from the bound records and
# the two frontier documents; no figure is computed here (jq selects fields).
# Usage (repository root): sh research/ecbench_frontier_v2_20261023/extract.sh OLD_FRONTIER.json
# OLD_FRONTIER.json is docs/bounds/frontier.json at main 3a1ea541d
# (`git show 3a1ea541d:docs/bounds/frontier.json`).
set -eu
OLD=$1
D=research/ecbench_frontier_v2_20261023
for f in docs/bounds/records/*.json; do
  jq -c --arg file "$(basename "$f" .json)" '{bound_id, file: $file, fit_version: (.fit_options.fit_version // 1),
     method: .method.id, params: .method.params, family: .domain.family, tier: .domain.tier,
     sizes: [.sizes[] | select(.verified > 0) | .log2_r], admissible: (.admissibility.status == "admissible"),
     bounded: .admissibility.bounded,
     ops: .dimensions.ops.value, ops_ci95: .dimensions.ops.ci95,
     alpha: .fit.alpha, alpha_ci95: .fit.alpha_ci95, tail: .tail}' "$f"
done | jq -s '.' > "$D/records.json"
jq '{schema, frontier_id, domains: [.domains[] | {family: .domain.family, tier: .domain.tier,
     ops_leader, memory_leader,
     frontier: [.entries[] | select(.is_frontier) | .bound_id],
     dominated: [.entries[] | select(.is_frontier | not) | .bound_id]}]}' "$OLD" > "$D/frontier-v1.json"
jq '{schema, frontier_id, domains: [.domains[] | {family: .domain.family, tier: .domain.tier,
     ops_leader, memory_leader, ops_tie: (.ops_tie // []), memory_tie: (.memory_tie // []),
     frontier: [.entries[] | select(.is_frontier) | .bound_id],
     dominated: [.entries[] | select(.is_frontier | not) | .bound_id],
     unranked: [(.unranked // [])[] | .bound_id],
     price_checks: [.entries[] | . as $e | (.price_checks // [])[] | {dominated: $e.bound_id, dominator, holds_across_band, break_even_price}]}]}' \
  docs/bounds/frontier.json > "$D/frontier-v2.json"
