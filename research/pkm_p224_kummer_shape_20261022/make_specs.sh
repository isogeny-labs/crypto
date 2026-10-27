#!/bin/bash
# Writes specs/SPEC-s{20,24,26,28}.json (PROTOCOL.md 4.2-4.3) from curves.jsonl
# (the four Solinas curves, in size order) and the four prior-ladder Proth
# curves of the same bit size. Thin orchestration over jq.
set -eu
here=$(cd "$(dirname "$0")" && pwd)
prior=$here/../pkm_tower_ecbench_20261013/curves.jsonl
mkdir -p "$here/specs"
i=0
for bits in 20 24 26 28; do
  i=$((i + 1))
  case $bits in 20) t=8; pl=1; tf4=10;; 24) t=9; pl=3; tf4=12;; 26) t=10; pl=4; tf4=0;; 28) t=11; pl=5; tf4=0;; esac
  sol=$(sed -n "${i}p" "$here/curves.jsonl")
  pro=$(sed -n "${pl}p" "$prior")
  jq -n --argjson sol "$sol" --argjson pro "$pro" --arg bits "$bits" --argjson t "$t" --argjson tf4 "$tf4" \
     --argjson seed "$((202610220000 + bits))" '
  def ic(name; role; fb; oracle): {name: name, role: role, method: {id: "ic.pipeline",
      params: {factor_base: fb, oracle: oracle, max_trials: "200000"}}};
  {
    schema: "ecbench.spec/v1",
    label: ("PKM Kummer-tower IC on Solinas vs Proth curves, " + $bits + "-bit p, t=" + ($t|tostring)),
    description: "Preregistered (PROTOCOL.md 4): ic.pipeline with the Kummer tower base on a P-224-topology Solinas curve (p = 2^n - 2^k + 1) and on the prior ladder Proth curve of the same bit size, the harness mitm table oracle and the root-finding oracle, against rho with the negation map, eight one-target workloads per curve. All IC arms charge every group operation.",
    workloads: {curves: [$sol, $pro], targets_per_curve: 8, target_seed: $seed},
    arms: ([
      {name: "rho-neg", role: "reference", method: {id: "rho.negation"}},
      ic("ic-mitm"; "baseline"; "prime-kummer:t=\($t)"; "mitm:m=3,negation_folded=1"),
      ic("ic-rootfind"; "candidate"; "prime-kummer:t=\($t)"; "rootfind-mitm:m=3")
    ] + (if $tf4 > 0 then [ic("ic-mitm-tf4"; "control"; "prime-kummer:t=\($tf4)"; "mitm:m=3,negation_folded=1")] else [] end)
    + [{name: "rho-neg-aa", role: "control", method: {id: "rho.negation"}}]),
    measurement: {rounds: 3, warmup: 1, order: "alternate", seed: ($seed + 1000000000), isolation_required: "L2", timeout_seconds: 3600}
  }' > "$here/specs/SPEC-s$bits.json"
done
