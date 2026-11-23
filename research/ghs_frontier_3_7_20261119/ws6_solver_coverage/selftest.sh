#!/usr/bin/env bash
set -euo pipefail

repo_root=$(git rev-parse --show-toplevel)
study=$repo_root/research/ghs_frontier_3_7_20261119/ws6_solver_coverage
tmp_dir=$(mktemp -d /tmp/ws6-solver-census.XXXXXX)
trap 'rm -rf "$tmp_dir"' EXIT

ECBENCH_BIN=${ECBENCH_BIN:-$repo_root/target/release/ecbench} \
  "$study/validate.sh" >"$tmp_dir/control.out"

jq 'del(.nested_algorithms[-1]) | .nested_algorithm_count = 36' \
  "$study/receipts/inventory.json" >"$tmp_dir/inventory-missing.json"
if ECBENCH_BIN=${ECBENCH_BIN:-$repo_root/target/release/ecbench} \
  "$study/validate.sh" "$tmp_dir/inventory-missing.json" \
  "$study/receipts/legacy-92-require-all.json" >"$tmp_dir/missing.out" 2>&1
then
  echo 'validator accepted a missing nested engine' >&2
  exit 1
fi

jq '.admissible = true' "$study/receipts/legacy-92-require-all.json" \
  >"$tmp_dir/audit-promoted.json"
if ECBENCH_BIN=${ECBENCH_BIN:-$repo_root/target/release/ecbench} \
  "$study/validate.sh" "$study/receipts/inventory.json" \
  "$tmp_dir/audit-promoted.json" >"$tmp_dir/promoted.out" 2>&1
then
  echo 'validator accepted an inadmissible audit relabelled admissible' >&2
  exit 1
fi

if ECBENCH_BIN=${ECBENCH_BIN:-$repo_root/target/release/ecbench} \
  "$study/validate.sh" "$study/receipts/inventory-pre-fix.json" \
  "$study/receipts/legacy-92-require-all-pre-fix.json" >"$tmp_dir/pre-fix.out" 2>&1
then
  echo 'validator accepted the incomplete pre-fix inventory' >&2
  exit 1
fi

echo 'WS6 SOLVER CENSUS SELFTEST PASS'
echo 'corruptions_rejected=3/3'
