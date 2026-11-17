#!/usr/bin/env bash
# Writes the frozen ecbench specs for protocol.json (one session per curve).
# Thin orchestration only; mirrors research/ecbench_lazy_orbit_members_20261018/make_specs.sh
# with the protocol's changes: c in {1,2,3} with eager and lazy arms at each c,
# 64 public targets, fresh seeds, 5 measured rounds + 1 warmup (L2 admission minimum), L2.
# Usage: bash research/ic_lead_chase_20261116/make_specs.sh
set -euo pipefail
ROOT="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)"
OUT="$ROOT/research/ic_lead_chase_20261116/specs"
mkdir -p "$OUT"
# a n targets rounds target_seed          (protocol.json randomization.seed_derivation)
PANEL="1 17 64 5 202611160017
1 29 64 5 202611160029
0 31 64 5 202611160031"
ic() { # name role oracle fb
  jq -n --arg name "$1" --arg role "$2" --arg or "$3" --arg fb "$4" \
    '{name:$name, role:$role, method:{id:"ic.pipeline_full", params:{factor_base:$fb, oracle:$or, max_trials:"1000000000", targets:"stride-renew"}}}'
}
while read -r a n t rounds seed; do
  arms=$(jq -n '[
    {name:"rho-frob", role:"reference", method:{id:"rho.signed_frobenius_full"}},
    {name:"rho-strong", role:"baseline", method:{id:"rho.signed_frobenius_strong_full"}}]')
  for c in 1 2 3; do
    orbit="compact-orbit-scan:columns=$c,raw_x_cap=100000000"
    arms=$(jq -n --argjson arms "$arms" \
      --argjson b "$(ic "ic-orbit-c$c-renew" baseline "mitm-frobenius-counted:m=2" "$orbit")" \
      --argjson l "$(ic "ic-orbit-c$c-renew-lazy" candidate "mitm-frobenius-lazy-counted:m=2" "$orbit")" \
      '$arms + [$b, $l]')
  done
  arms=$(jq -n --argjson arms "$arms" '$arms + [{name:"rho-frob-aa", role:"control", method:{id:"rho.signed_frobenius_full"}}]')
  jq -n --argjson arms "$arms" --argjson a "$a" --argjson n "$n" --argjson seed "$seed" \
        --argjson t "$t" --argjson rounds "$rounds" '{
    schema:"ecbench.spec/v1",
    label:"Lazy orbit-member column sweep vs strong rho: Koblitz a=\($a) n=\($n)",
    description:"Frozen by research/ic_lead_chase_20261116/protocol.json before its data. Every arm is a *_full method. Within each c only the oracle differs between the two IC arms. Engineering; no security claim.",
    workloads:{curves:[{kind:"koblitz", a:$a, n:$n}], targets_per_curve:$t, target_seed:$seed, target_kind:"public"},
    arms:$arms,
    measurement:{rounds:$rounds, warmup:1, order:"alternate", seed:(202611162000 + $n), isolation_required:"L2", timeout_seconds:3600}}' \
    > "$OUT/lazycol-a$a-n$n.json"
  echo "$OUT/lazycol-a$a-n$n.json"
done <<<"$PANEL"
