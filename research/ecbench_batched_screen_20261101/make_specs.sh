#!/usr/bin/env bash
# Writes specs/<cell>.json: the frozen panel of PROTOCOL.md.
set -euo pipefail
cd "$(dirname "$0")"
mkdir -p specs
cells=("n13a0 0 13 78 202611011001" "n17a1 1 17 102 202611011002" "n19a0 0 19 114 202611011003" "n23a0 0 23 138 202611011004")
batches=(1 2 4 8 16 32 64 128 256)
for c in "${cells[@]}"; do
  read -r name a n points tseed <<<"$c"
  arms=""
  for b in "${batches[@]}"; do
    role=candidate; [ "$b" = 64 ] && role=baseline
    arms+=$(printf '{"name":"ic-b%s","role":"%s","method":{"id":"ic.koblitz_batched","params":{"factor_base_points":"%s","batch_trials":"%s"}}},' "$b" "$role" "$points" "$b")
  done
  arms+=$(printf '{"name":"ic-b64-aa","role":"control","method":{"id":"ic.koblitz_batched","params":{"factor_base_points":"%s","batch_trials":"64"}}}' "$points")
  jq -n --arg label "batch_trials development screen: Koblitz a=$a n=$n" \
        --arg desc "Frozen in research/ecbench_batched_screen_20261101/PROTOCOL.md before its data. Only batch_trials differs between arms. Counted unit, L0, diagnostic; no security claim." \
        --argjson a "$a" --argjson n "$n" --argjson tseed "$tseed" --argjson arms "[$arms]" '{
    schema: "ecbench.spec/v1", label: $label, description: $desc,
    workloads: {curves: [{kind: "koblitz", a: $a, n: $n}], targets_per_curve: 32, target_seed: $tseed, target_kind: "public"},
    arms: $arms,
    measurement: {rounds: 4, warmup: 1, order: "alternate_measured", seed: 202611012001, isolation_required: "L0", timeout_seconds: 3600}
  }' > "specs/$name.json"
done
ls specs
