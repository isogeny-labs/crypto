#!/usr/bin/env bash
# Writes specs/<cell>.json: the panel of PROTOCOL.md (PROPOSED, not frozen).
# Same arms and methods as research/ecbench_batched_screen_20261101, with fresh
# seeds, the round-0026 holdout cell added, and isolation_required L2.
set -euo pipefail
cd "$(dirname "$0")"
mkdir -p specs
# name a n factor_base_points target_seed   (target seeds are fresh: not 202611011001..04)
cells=("n13a0 0 13 78 202611031001" "n17a1 1 17 102 202611031002" "n19a0 0 19 114 202611031003" "n23a0 0 23 138 202611031004" "n19a1 1 19 114 202611031005")
batches=(1 2 4 8 16 64)
for c in "${cells[@]}"; do
  read -r name a n points tseed <<<"$c"
  arms=""
  for b in "${batches[@]}"; do
    role=candidate; [ "$b" = 64 ] && role=baseline
    arms+=$(printf '{"name":"ic-b%s","role":"%s","method":{"id":"ic.koblitz_batched","params":{"factor_base_points":"%s","batch_trials":"%s"}}},' "$b" "$role" "$points" "$b")
  done
  arms+=$(printf '{"name":"ic-b64-aa","role":"control","method":{"id":"ic.koblitz_batched","params":{"factor_base_points":"%s","batch_trials":"64"}}}' "$points")
  jq -n --arg label "batch_trials isolated confirmation: Koblitz a=$a n=$n" \
        --arg desc "PROPOSED in research/ecbench_batch16_isolated_20261103/PROTOCOL.md; not frozen until the owner approves it. Only batch_trials differs between arms. L2 wall time and counted GAE; diagnostic, no security claim." \
        --argjson a "$a" --argjson n "$n" --argjson tseed "$tseed" --argjson arms "[$arms]" '{
    schema: "ecbench.spec/v1", label: $label, description: $desc,
    workloads: {curves: [{kind: "koblitz", a: $a, n: $n}], targets_per_curve: 32, target_seed: $tseed, target_kind: "public"},
    arms: $arms,
    measurement: {rounds: 4, warmup: 1, order: "alternate_measured", seed: 202611032001, isolation_required: "L2", timeout_seconds: 3600}
  }' > "specs/$name.json"
done
ls specs
