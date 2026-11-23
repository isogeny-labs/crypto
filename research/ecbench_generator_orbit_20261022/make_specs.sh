#!/usr/bin/env bash
# Writes the frozen claim specs (one session per curve). Thin orchestration only.
# Usage: bash research/ecbench_generator_orbit_20261022/make_specs.sh
set -euo pipefail
ROOT="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)"
OUT="$ROOT/docs/ecbench/specs"
# a n columns targets rounds target_seed   (columns: the per-curve choice of
# research/ecbench_frobenius_orbit_vs_rho_20261014/PROTOCOL.md §3, not retuned;
# n = 41 at 90 columns is left out, PROTOCOL.md §4)
PANEL="0 13 1 32 3 202610221001
1 17 1 32 3 202610221003
0 19 2 32 3 202610221005
0 23 3 32 3 202610221007
1 29 1 64 3 202610221009
0 31 2 32 3 202610221011
0 37 8 16 3 202610221013"
ic() { # name role oracle fb targets
  jq -n --arg name "$1" --arg role "$2" --arg or "$3" --arg fb "$4" --arg tg "$5" \
    '{name:$name, role:$role, method:{id:"ic.pipeline_full", params:{factor_base:$fb, oracle:$or, max_trials:"1000000000", targets:$tg}}}'
}
while read -r a n c t rounds seed; do
  orbit="compact-orbit-scan:columns=$c,raw_x_cap=100000000"
  arms=$(jq -n \
    --argjson b "$(ic "ic-lazy-c$c" baseline "mitm-frobenius-lazy-counted:m=2" "$orbit" stride-renew)" \
    --argjson l "$(ic "ic-layout-c$c" candidate "mitm-frobenius-layout-counted:m=2" "$orbit" stride-renew)" \
    --argjson g "$(ic "ic-gen-layout-c$c" candidate "mitm-frobenius-layout-counted:m=2" "$orbit,generator_orbit=1" stride-generator)" '[
    {name:"rho-frob", role:"reference", method:{id:"rho.signed_frobenius_full"}},
    {name:"rho-strong", role:"baseline", method:{id:"rho.signed_frobenius_strong_full"}},
    $b, $l, $g,
    {name:"rho-frob-aa", role:"control", method:{id:"rho.signed_frobenius_full"}}]')
  jq -n --argjson arms "$arms" --argjson a "$a" --argjson n "$n" --argjson seed "$seed" \
        --argjson t "$t" --argjson rounds "$rounds" '{
    schema:"ecbench.spec/v1",
    label:"Orbit-layout table and generator-orbit base, fully priced, vs the lazy orbit IC and rho: Koblitz a=\($a) n=\($n)",
    description:"Frozen in research/ecbench_generator_orbit_20261022/PROTOCOL.md before its data. Every arm is a *_full method. ic-layout differs from ic-lazy in the oracle id only; ic-gen-layout differs from ic-layout in generator_orbit=1 and targets=stride-generator. Engineering; no security claim.",
    workloads:{curves:[{kind:"koblitz", a:$a, n:$n}], targets_per_curve:$t, target_seed:$seed, target_kind:"public"},
    arms:$arms,
    measurement:{rounds:$rounds, warmup:1, order:"alternate", seed:($seed + 1000), isolation_required:"L0", timeout_seconds:3600}}' \
    > "$OUT/claim-koblitz-generator-orbit-a$a-n$n.json"
  echo "$OUT/claim-koblitz-generator-orbit-a$a-n$n.json"
done <<<"$PANEL"
