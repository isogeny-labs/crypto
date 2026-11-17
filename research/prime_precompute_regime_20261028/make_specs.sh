#!/bin/sh
# Specs for PROTOCOL.md, from the frontier campaign's frozen 32/36/40-bit prime
# specs (base-specs/, copied from research/prime_ksum_table_20261028/
# frontier-specs/).  Thin orchestration only.
#   ./make_specs.sh train        rho-neg and kl-ksum4 on the train targets
#   ./make_specs.sh test         test panel; needs params.json (S_P frozen)
set -eu
cd "$(dirname "$0")"
mode=${1:?train or test}
# #2808's frozen k = 4 base size, and the k-sum table's nominal entries 2^3·C(A,4).
a4() { jq -r --arg b "$1" '.prime[] | select(.bits == ($b|tonumber)) | .ksum4' ../prime_ksum_table_20261028/params.json; }
entries() { awk -v a="$1" 'BEGIN { printf "%.0f", 8 * a * (a - 1) * (a - 2) * (a - 3) / 24 }'; }
kl() { # A
  jq -n --arg a "$1" '{name:"kl-ksum4", role:"candidate", method:{id:"ic.known_log_ksum_v1",
    params:{size:$a, k:"4", base_seed:"20261028", max_sums:"100000000", max_trials:"1000000000000"}}}'
}
ic() { # A
  jq -n --arg a "$1" '{name:"ic-ksum4", role:"candidate", method:{id:"ic.pipeline",
    params:{factor_base:("prime-abscissa:size="+$a), oracle:"ksum:m=4", max_trials:"13528399621"}}}'
}
pre() { # NAME ENTRIES
  jq -n --arg n "$1" --arg e "$2" '{name:$n, role:"baseline", method:{id:"rho.preprocessing",
    params:{entries:$e}}}'
}
mkdir -p "$mode"
for b in 32 36 40; do
  src=base-specs/spec-P$b.json
  a=$(a4 "$b")
  if [ "$mode" = train ]; then
    arms=$(kl "$a" | jq -s .)
    jq --argjson arms "$arms" --arg b "$b" '
      .label = ("prime precomputation regime: train panel, " + $b + "-bit a=-3 curve")
      | .description = "PROTOCOL.md selection: kl-ksum4 target-blind cost, read to freeze pre-eqP entries. Counts only. Public synthetic; no security claim."
      | .workloads.targets_per_curve = 4
      | .workloads.target_seed = 20261015100
      | .arms = ([.arms[] | select(.name == "rho-neg")] + $arms)
      | .measurement.rounds = 1 | .measurement.warmup = 0' "$src" > "$mode/spec-P$b.json"
  else
    sp=$(jq -r --arg b "$b" '.prime[] | select(.bits == ($b|tonumber)) | .pre_eqP_entries' params.json)
    rounds=2; [ "$b" = 40 ] && rounds=1
    arms=$( { ic "$a"; kl "$a"; pre pre-eqS "$(entries "$a")"; pre pre-eqP "$sp"; } | jq -s .)
    jq --argjson arms "$arms" --arg b "$b" --argjson rounds "$rounds" '
      .label = ("prime precomputation regime: test panel, " + $b + "-bit a=-3 curve")
      | .description = "PROTOCOL.md test panel: rho-neg reference, A/A control, cold ic-ksum4, known-log k = 4 sum table, preprocessing rho at equal entries and at equal precomputation. Counts only; AGENTS.md 5a band. Public synthetic; no security claim."
      | .arms = ([.arms[] | select(.name == "rho-neg" or .name == "rho-neg-aa")] + $arms)
      | .measurement.rounds = $rounds' "$src" > "$mode/spec-P$b.json"
  fi
done
ls "$mode"
