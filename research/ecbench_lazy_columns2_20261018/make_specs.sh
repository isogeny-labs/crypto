#!/usr/bin/env bash
# Writes the frozen specs (one session per curve). Thin orchestration only.
# Usage: bash research/ecbench_lazy_columns2_20261018/make_specs.sh
set -euo pipefail
ROOT="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)"
OUT="$ROOT/docs/ecbench/specs"
# a n c0 targets rounds target_seed   (c0: the column count the lazy baseline
# keeps, i.e. the 2026-10-14 protocol's per-curve choice; the candidate is c = 2)
PANEL="0 13 1 32 3 202610182001
1 17 1 32 3 202610182003
0 23 3 32 3 202610182005
1 29 1 32 3 202610182007
0 37 8 16 3 202610182009"
ic() { # name role oracle columns
  jq -n --arg name "$1" --arg role "$2" --arg or "$3" --arg fb "compact-orbit-scan:columns=$4,raw_x_cap=100000000" \
    '{name:$name, role:$role, method:{id:"ic.pipeline_full", params:{factor_base:$fb, oracle:$or, max_trials:"1000000000", targets:"stride-renew"}}}'
}
while read -r a n c0 t rounds seed; do
  arms=$(jq -n --argjson b "$(ic "ic-lazy-c$c0" baseline "mitm-frobenius-lazy-counted:m=2" "$c0")" \
                --argjson k "$(ic "ic-lazy-c2" candidate "mitm-frobenius-lazy-counted:m=2" 2)" '[
    {name:"rho-frob", role:"reference", method:{id:"rho.signed_frobenius_full"}},
    {name:"rho-strong", role:"baseline", method:{id:"rho.signed_frobenius_strong_full"}},
    $b, $k,
    {name:"rho-frob-aa", role:"control", method:{id:"rho.signed_frobenius_full"}}]')
  jq -n --argjson arms "$arms" --argjson a "$a" --argjson n "$n" --argjson seed "$seed" \
        --argjson t "$t" --argjson rounds "$rounds" '{
    schema:"ecbench.spec/v1",
    label:"Two orbit columns under the lazy member index, fully priced: Koblitz a=\($a) n=\($n)",
    description:"Frozen in research/ecbench_lazy_columns2_20261018/PROTOCOL.md before its data. Every arm is a *_full method. Only the column count differs between the two IC arms. Engineering; no security claim.",
    workloads:{curves:[{kind:"koblitz", a:$a, n:$n}], targets_per_curve:$t, target_seed:$seed, target_kind:"public"},
    arms:$arms,
    measurement:{rounds:$rounds, warmup:1, order:"alternate", seed:($seed + 1000), isolation_required:"L0", timeout_seconds:3600}}' \
    > "$OUT/claim-koblitz-lazy-c2-a$a-n$n.json"
  echo "$OUT/claim-koblitz-lazy-c2-a$a-n$n.json"
done <<<"$PANEL"
