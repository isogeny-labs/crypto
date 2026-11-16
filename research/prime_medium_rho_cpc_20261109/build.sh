#!/usr/bin/env bash
# PROTOCOL.md sections 3-4: seeds, the five measured ecbench specs (PC, M1-M4)
# and their isolab.eks.request/v1 envelopes. Curves, arms, rounds and the image
# are the parent studies' (research/prime_medium_tier_rho_20261107 and
# research/prime_medium_rho_p4_confirm_20261108); only the seeds are new.
# Run from the repository root. Deterministic: rerunning writes the same bytes.
set -euo pipefail
D=research/prime_medium_rho_cpc_20261109
P=research/prime_medium_tier_rho_20261107
PC0=research/prime_medium_rho_p4_confirm_20261108
PREFIX="prime_medium_rho_cpc_20261109/v1/"
seed() { printf '%s' "$PREFIX$1" | sha256sum | cut -c1-12 | xargs -I{} printf '%d' 0x{}; }
tags="target_seed/PC target_seed/M measurement_seed/PC measurement_seed/M1 measurement_seed/M2 measurement_seed/M3 measurement_seed/M4"
{ printf '{"schema":"prime_medium_rho_cpc.seeds/v1","rule":"seed(tag) = int(SHA-256(UTF-8(prefix + tag)).hexdigest()[0:12], 16)","prefix":"%s","seeds":[' "$PREFIX"
  first=1; for t in $tags; do [ $first = 1 ] || printf ','; first=0; printf '{"tag":"%s","seed":%s}' "$t" "$(seed "$t")"; done; printf ']}\n'; } | jq . > "$D/seeds.json"
s() { jq -er --arg t "$1" '.seeds[] | select(.tag == $t) | .seed' "$D/seeds.json"; }

rho_arms='[
 {"name":"rho-neg","role":"reference","method":{"id":"rho.negation"}},
 {"name":"rho-neg-aa","role":"control","method":{"id":"rho.negation"}},
 {"name":"rho-plain","role":"baseline","method":{"id":"rho.plain"}},
 {"name":"bsgs-neg","role":"baseline","method":{"id":"bsgs.negation"}}
]'
# spec NAME LABEL ARMS CURVE_TAG TARGETS TARGET_SEED MEASUREMENT_SEED
spec() {
  jq -n --slurpfile c "$P/curves.json" --argjson arms "$3" --arg label "$2" --arg tag "$4" \
    --argjson t "$5" --argjson ts "$6" --argjson ms "$7" '
    {schema: "ecbench.spec/v1", label: $label,
     description: "research/prime_medium_rho_cpc_20261109/PROTOCOL.md. Curves from research/prime_medium_tier_rho_20261107/curves.json; seeds from seeds.json.",
     workloads: {curves: [$c[0].curves[] | select(.job_curve_tags | index($tag)) | .curve_spec],
                 targets_per_curve: $t, target_seed: $ts, target_kind: "planted"},
     arms: $arms,
     measurement: {rounds: 6, warmup: 1, order: "alternate", seed: $ms,
                   isolation_required: "L2", timeout_seconds: 300}}' > "$D/specs/$1.json"
}
spec PC "prime medium rho CPC: positive control bsgs.textbook / bsgs.negation, 33-41-bit curves, fresh target block" \
  "$(jq -c .arms "$PC0/specs/PC.json")" J1 16 "$(s target_seed/PC)" "$(s measurement_seed/PC)"
spec M1 "prime medium rho CPC M1: 33-, 37- and 41-bit curves" "$rho_arms" J1 8 "$(s target_seed/M)" "$(s measurement_seed/M1)"
spec M2 "prime medium rho CPC M2: 45-bit curves" "$rho_arms" J2 8 "$(s target_seed/M)" "$(s measurement_seed/M2)"
spec M3 "prime medium rho CPC M3: 49-bit curve 1" "$rho_arms" J3 4 "$(s target_seed/M)" "$(s measurement_seed/M3)"
spec M4 "prime medium rho CPC M4: 49-bit curve 2" "$rho_arms" J5 4 "$(s target_seed/M)" "$(s measurement_seed/M4)"

build=$(jq -c .job.build "$PC0/requests/PC.request.json")
for j in PC M1 M2 M3 M4; do
  jq -n --slurpfile s "$D/specs/$j.json" --arg name "prime-medium-rho-cpc-20261109-$(tr 'A-Z' 'a-z' <<<"$j")" --argjson b "$build" '
    {schema: "isolab.eks.request/v1",
     job: {job_name: $name, instance_type: "c8a.2xlarge", seconds: 3600, memory: "4Gi", spec: $s[0], build: $b},
     provenance: {harness: "claude-code: research/prime_medium_rho_cpc_20261109 (PROTOCOL.md)", source_revision: $b.source_revision}}' \
    > "$D/requests/$j.request.json"
done
