#!/usr/bin/env bash
# Specs of PROTOCOL.md: per Koblitz curve, rho.signed_frobenius_strong (reference),
# the ic plan (ic-planner/2) compact-orbit ic.pipeline arm, and ic.koblitz_descent
# with its rule_v8 defaults. Usage: scripts/make_specs.sh (from the study directory)
set -euo pipefail
cd "$(dirname "$0")/.."
IC=../../target/release/ic
gen() { # tag a n
  local tag=$1 a=$2 n=$3 plan
  plan=$($IC plan --koblitz "$a:$n" --ecbench-arm pipeline --json | jq -c '.ecbench_arm | .role = "baseline"')
  jq -n --argjson a "$a" --argjson n "$n" --argjson plan "$plan" --arg tag "$tag" '{
    schema: "ecbench.spec/v1",
    label: "Koblitz routing, curve \($tag): the ledger §23 descent against the planner recipe and the strong rho",
    description: "research/ic_koblitz_descent_routing_20261026/PROTOCOL.md",
    workloads: {curves: [{kind: "koblitz", a: $a, n: $n}], targets_per_curve: 8, target_seed: 20261103},
    arms: [{name: "rho", role: "reference", method: {id: "rho.signed_frobenius_strong"}},
           $plan,
           {name: "descent", role: "candidate", method: {id: "ic.koblitz_descent"}}],
    measurement: {rounds: 3, warmup: 1, order: "alternate_measured", seed: 20261104, isolation_required: "L2", timeout_seconds: 1800}
  }' > "specs/$tag.json"
}
gen k47 1 47
gen k41 0 41
gen k53 0 53
