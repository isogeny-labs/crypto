#!/usr/bin/env bash
set -euo pipefail

repo_root=$(git rev-parse --show-toplevel)
study_rel=research/ghs_frontier_3_7_20261119/ws6_solver_coverage
inventory=${1:-$repo_root/$study_rel/receipts/inventory.json}
audit=${2:-$repo_root/$study_rel/receipts/legacy-92-require-all.json}
ecbench_bin=${ECBENCH_BIN:-$repo_root/target/release/ecbench}

cd "$repo_root"

sha256sum -c "$study_rel/SOURCE_SHA256SUMS"
if [[ "$inventory" == "$repo_root/$study_rel/receipts/inventory.json" \
   && "$audit" == "$repo_root/$study_rel/receipts/legacy-92-require-all.json" ]]; then
  sha256sum -c "$study_rel/RECEIPT_SHA256SUMS"
fi

test -x "$ecbench_bin"
test "$(sha256sum "$ecbench_bin" | awk '{print $1}')" \
  = dcac1f24cd12b81a0729794aebe8c736592cbf20fe6cfd92e86da549df303476
grep -qx 'status=PASS' "$study_rel/receipts/unit-test.txt"

jq -e '
  .schema == "ecbench.optimization-inventory/v1" and
  .method_count == 111 and
  .nested_algorithm_count == 37 and
  (.methods | length) == 111 and
  (.methods | map(.id) | unique | length) == 111 and
  (.nested_algorithms | length) == 37 and
  (.nested_algorithms | map(.key) | unique | length) == 37 and
  ([.nested_algorithms[] | select(.kind == "system_solver")] | length) == 21 and
  ([.nested_algorithms[] | select(.kind == "pdp3_engine")] | length) == 12 and
  ([.nested_algorithms[] | select(.kind == "pkm_tower_engine")] | length) == 4
' "$inventory" >/dev/null

for required_key in \
  system_solver/f4-f2 \
  system_solver/matrix-f4 \
  system_solver/matrix-f5 \
  system_solver/sat-cdcl \
  system_solver/fes-f2 \
  system_solver/fes-f2-gpu \
  system_solver/fes-f2-wide \
  pdp3_engine/f6-ic \
  pdp3_engine/f6-ic-pair \
  pdp3_engine/rootfind-negfold-batch \
  pdp3_engine/rootfind-negfold-batch-harvest \
  pdp3_engine/rootfind-negfold-batch-harvest-torsion \
  pdp3_engine/rootfind-window-negfold \
  pkm_tower_engine/sig-f5 \
  pkm_tower_engine/f6
do
  jq -e --arg key "$required_key" \
    'any(.nested_algorithms[]; .key == $key)' "$inventory" >/dev/null
done

jq -e '
  .schema == "ecbench.optimization-audit/v1" and
  .admissible == false and
  .require_all_development_coverage == true and
  (.development.specs | length) == 92 and
  .development.method_count == 20 and
  .development.nested_algorithm_count == 17 and
  ([.development.specs[].errors[]] | length) == 0 and
  .coverage.required_method_count == 111 and
  .coverage.required_nested_algorithm_count == 37 and
  (.coverage.missing_development_methods | length) == 91 and
  (.coverage.missing_development_nested_algorithms | length) == 20 and
  (.leakage.reused_target_seeds | length) == 0 and
  (.leakage.reused_measurement_seeds | length) == 0
' "$audit" >/dev/null

jq -e '
  .schema == "ghs.frontier.solver-catalog/v1" and
  .built_in_inventory.top_level_methods == 111 and
  .built_in_inventory.nested_total == 37 and
  (.relation_matrices | length) == 15 and
  (.relation_matrices | unique | length) == 15 and
  (.target_sources | length) == 20 and
  (.target_sources | unique | length) == 20 and
  (.root_and_table_engines | length) == 12 and
  (.root_and_table_engines | unique | length) == 12 and
  (.technique_dispositions.wdsat | contains("false-UNSAT"))
' "$study_rel/catalog.json" >/dev/null

jq -e '
  .schema == "ghs.frontier.missing-coverage/v1" and
  .required.methods == 111 and
  .required.nested == 37 and
  .covered.methods == 20 and
  .covered.nested == 17 and
  (.missing.methods | length) == 91 and
  (.missing.nested | length) == 20 and
  .admissible == false
' "$study_rel/missing-coverage.json" >/dev/null

test "$(jq -r '.nested_algorithm_count' "$study_rel/receipts/inventory-pre-fix.json")" = 34
test "$(jq '[.nested_algorithms[] | select(.kind == "pdp3_engine")] | length' \
  "$study_rel/receipts/inventory-pre-fix.json")" = 9

echo 'WS6 SOLVER CENSUS VALIDATION PASS'
echo 'methods=111 system_solvers=21 pdp3_engines=12 pkm_tower_engines=4 nested=37'
echo 'legacy_92_specs=92 covered_methods=20 covered_nested=17 missing_methods=91 missing_nested=20 admissible=false'
echo 'timing_observations=0 performance_claims=0 leaderboard_updates=0'
