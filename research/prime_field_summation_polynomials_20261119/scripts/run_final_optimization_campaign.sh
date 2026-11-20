#!/usr/bin/env bash
set -euo pipefail

project_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
results_dir="${project_dir}/results/final"
mkdir -p "${results_dir}"
make -C "${project_dir}" test | tee "${results_dir}/integrated_self_test.log"

seed=6000285944033488934
for prime in 1009 1000003 2147483647; do
  "${project_dir}/build/semaev_lab" benchmark \
    --prime "${prime}" --a 2 --b 3 --seed "${seed}" \
    --min-arity 3 --max-arity 9 --bench-cases 200 --bench-repeats 20 \
    --reduction auto --output "${results_dir}/root_auto_p${prime}.csv" \
    2>&1 | tee "${results_dir}/root_auto_p${prime}.log"
  "${project_dir}/build/semaev_lab" benchmark \
    --prime "${prime}" --a 2 --b 3 --seed "${seed}" \
    --min-arity 9 --max-arity 9 --bench-cases 200 --bench-repeats 20 \
    --reduction generic --output "${results_dir}/root_generic_p${prime}.csv" \
    2>&1 | tee "${results_dir}/root_generic_p${prime}.log"
  "${project_dir}/build/semaev_lab" coeff-benchmark \
    --prime "${prime}" --a 2 --b 3 --seed "${seed}" \
    --min-degree 2 --max-degree 4096 --bench-cases 5 --bench-repeats 3 \
    --reduction auto --output "${results_dir}/coeff_p${prime}.csv" \
    2>&1 | tee "${results_dir}/coeff_p${prime}.log"
done

"${project_dir}/build/semaev_lab" benchmark-target \
  --prime 1000003 --a 2 --b 3 --seed "${seed}" \
  --min-arity 9 --max-arity 14 --bench-cases 20 --bench-repeats 5 \
  --reduction auto --output "${results_dir}/target_p1000003.csv" \
  2>&1 | tee "${results_dir}/target_p1000003.log"

for prime in 1009 1000003 2147483647; do
  "${project_dir}/build/semaev_lab" automorphism-benchmark \
    --prime "${prime}" --a 0 --b 7 --seed "${seed}" \
    --min-arity 3 --max-arity 12 --bench-cases 200 --bench-repeats 20 \
    --reduction auto --output "${results_dir}/automorphism_j0_p${prime}.csv" \
    2>&1 | tee "${results_dir}/automorphism_j0_p${prime}.log"
done
"${project_dir}/build/semaev_lab" automorphism-benchmark \
  --prime 1009 --a 5 --b 0 --seed "${seed}" \
  --min-arity 3 --max-arity 12 --bench-cases 200 --bench-repeats 20 \
  --reduction auto --output "${results_dir}/automorphism_j1728_p1009.csv" \
  2>&1 | tee "${results_dir}/automorphism_j1728_p1009.log"

sha256sum "${results_dir}"/*.csv "${results_dir}"/*.log > "${results_dir}/SHA256SUMS"
