#!/usr/bin/env bash
# Writes the frozen claim specs (one session per curve). Thin orchestration only.
# Usage: bash research/ecbench_lazy_orbit_members_20261018/make_specs.sh
set -euo pipefail
ROOT="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)"
OUT="$ROOT/docs/ecbench/specs"
# a n columns targets rounds target_seed   (columns: the per-curve choice of
# research/ecbench_frobenius_orbit_vs_rho_20261014/PROTOCOL.md §3, not retuned)
PANEL="0 13 1 32 3 202610181001
1 17 1 32 3 202610181003
0 19 2 32 3 202610181005
0 23 3 32 3 202610181007
1 29 1 32 3 202610181009
0 31 2 32 3 202610181011
0 37 8 16 3 202610181013
0 41 90 8 2 202610181015"
ic() { # name role oracle fb
  jq -n --arg name "$1" --arg role "$2" --arg or "$3" --arg fb "$4" \
    '{name:$name, role:$role, method:{id:"ic.pipeline_full", params:{factor_base:$fb, oracle:$or, max_trials:"1000000000", targets:"stride-renew"}}}'
}
while read -r a n c t rounds seed; do
  orbit="compact-orbit-scan:columns=$c,raw_x_cap=100000000"
  arms=$(jq -n --argjson b "$(ic "ic-orbit-c$c-renew" baseline "mitm-frobenius-counted:m=2" "$orbit")" \
                --argjson l "$(ic "ic-orbit-c$c-renew-lazy" candidate "mitm-frobenius-lazy-counted:m=2" "$orbit")" '[
    {name:"rho-frob", role:"reference", method:{id:"rho.signed_frobenius_full"}},
    {name:"rho-strong", role:"baseline", method:{id:"rho.signed_frobenius_strong_full"}},
    $b, $l,
    {name:"rho-frob-aa", role:"control", method:{id:"rho.signed_frobenius_full"}}]')
  jq -n --argjson arms "$arms" --argjson a "$a" --argjson n "$n" --argjson seed "$seed" \
        --argjson t "$t" --argjson rounds "$rounds" '{
    schema:"ecbench.spec/v1",
    label:"Lazy orbit-member index, fully priced, vs the frozen orbit IC and rho: Koblitz a=\($a) n=\($n)",
    description:"Frozen in research/ecbench_lazy_orbit_members_20261018/PROTOCOL.md before its data. Every arm is a *_full method. Only the oracle differs between the two IC arms. Engineering; no security claim.",
    workloads:{curves:[{kind:"koblitz", a:$a, n:$n}], targets_per_curve:$t, target_seed:$seed, target_kind:"public"},
    arms:$arms,
    measurement:{rounds:$rounds, warmup:1, order:"alternate", seed:($seed + 1000), isolation_required:"L0", timeout_seconds:3600}}' \
    > "$OUT/claim-koblitz-lazy-members-a$a-n$n.json"
  echo "$OUT/claim-koblitz-lazy-members-a$a-n$n.json"
done <<<"$PANEL"
