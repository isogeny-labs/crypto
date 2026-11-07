#!/usr/bin/env bash
# PROTOCOL.md sections 2-4: write the six measured ecbench specs (J1..J6), the
# development smoke spec and the six isolab.eks.request/v1 envelopes from
# curves.json and seeds.json. Thin orchestration only (jq); every curve fact
# comes from `ecbench plan` (curves.json), every seed from seeds.json.
# Run from the repository root. Deterministic: rerunning writes the same bytes.
set -euo pipefail
D=research/prime_medium_tier_rho_20261107
seed() { jq -er --arg t "$1" '.seeds[] | select(.tag == $t) | .seed' "$D/seeds.json"; }
TA=$(seed target_seed/block=A)
TB=$(seed target_seed/block=B)

arms='[
 {"name":"rho-neg","role":"reference","method":{"id":"rho.negation"}},
 {"name":"rho-neg-aa","role":"control","method":{"id":"rho.negation"}},
 {"name":"rho-plain","role":"baseline","method":{"id":"rho.plain"}},
 {"name":"bsgs-neg","role":"baseline","method":{"id":"bsgs.negation"}},
 {"name":"rho-neg-v2","role":"candidate","method":{"id":"rho.negation_v2"}}
]'

# spec NAME LABEL CURVE_FILTER TARGETS TARGET_SEED MEASUREMENT_SEED ROUNDS WARMUP
spec() {
  jq -n --slurpfile c "$D/curves.json" --argjson arms "$arms" \
    --arg label "$2" --arg filter "$3" --argjson t "$4" --argjson ts "$5" \
    --argjson ms "$6" --argjson rounds "$7" --argjson warmup "$8" '
    {schema: "ecbench.spec/v1",
     label: $label,
     description: "research/prime_medium_tier_rho_20261107/PROTOCOL.md. Generic prime-order curves from prime_orbit_index_calculus::generate_instance(generic, bits, seed), given explicitly (curves.json); seeds from seeds.json. Operation counts are the result; wall time only at L2 against the A/A arm.",
     workloads: {
       curves: [$c[0].curves[] | select(.job_curve_tags | index($filter)) | .curve_spec],
       targets_per_curve: $t, target_seed: $ts, target_kind: "planted"},
     arms: $arms,
     measurement: {rounds: $rounds, warmup: $warmup, order: "alternate", seed: $ms,
       isolation_required: "L2", timeout_seconds: 300}}' > "$D/specs/$1.json"
}

spec J1 "prime medium-tier rho bound J1: 33-, 37- and 41-bit curves, target block A" J1 8 "$TA" "$(seed measurement_seed/job=J1)" 5 1
spec J2 "prime medium-tier rho bound J2: 45-bit curves, target block A" J2 8 "$TA" "$(seed measurement_seed/job=J2)" 5 1
spec J3 "prime medium-tier rho bound J3: 49-bit curve 1, target block A" J3 4 "$TA" "$(seed measurement_seed/job=J3)" 5 1
spec J4 "prime medium-tier rho bound J4: 49-bit curve 1, target block B" J4 4 "$TB" "$(seed measurement_seed/job=J4)" 5 1
spec J5 "prime medium-tier rho bound J5: 49-bit curve 2, target block A" J5 4 "$TA" "$(seed measurement_seed/job=J5)" 5 1
spec J6 "prime medium-tier rho bound J6: 49-bit curve 2, target block B" J6 4 "$TB" "$(seed measurement_seed/job=J6)" 5 1
# Development smoke only (README "Development checks"): the first 33-bit curve,
# 2 targets, 1 round, no warm-up. Never a result; not an EKS request.
spec smoke-dev "DEVELOPMENT SMOKE ONLY: one 33-bit curve, 2 targets, 1 round (not a measurement)" SMOKE 2 "$TA" "$(seed measurement_seed/job=SMOKE)" 1 0

mkdir -p "$D/requests"
for j in J1 J2 J3 J4 J5 J6; do
  name="prime-medium-rho-20261107-$(tr 'J' 'j' <<<"$j")"
  jq -n --slurpfile s "$D/specs/$j.json" --arg name "$name" '
    {schema: "isolab.eks.request/v1",
     job: {job_name: $name, instance_type: "c8a.2xlarge", seconds: 3600, memory: "4Gi", spec: $s[0]},
     provenance: {harness: "claude-code: research/prime_medium_tier_rho_20261107 (PROTOCOL.md)",
                  source_revision: "FILL: 40-hex commit that lands this protocol"}}' \
    > "$D/requests/$j.request.json"
done
