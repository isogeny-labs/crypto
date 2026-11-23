#!/usr/bin/env bash
set -euo pipefail

project_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
build_dir="${project_dir}/build"
results_dir="${project_dir}/results"
mkdir -p "${build_dir}" "${results_dir}/corpus" "${results_dir}/raw"

if command -v cmake >/dev/null 2>&1 && command -v ninja >/dev/null 2>&1; then
  cmake -S "${project_dir}" -B "${build_dir}" -G Ninja -DCMAKE_BUILD_TYPE=Release
  cmake --build "${build_dir}"
  ctest --test-dir "${build_dir}" --output-on-failure | tee "${results_dir}/raw/ctest.log"
else
  make -C "${project_dir}" all
  "${build_dir}/semaev_lab" self-test | tee "${results_dir}/raw/ctest.log"
fi

seed=6000285944033488934
samples_per_arity="${SAMPLES_PER_ARITY:-16667}"
bench_cases="${BENCH_CASES:-200}"
bench_repeats="${BENCH_REPEATS:-20}"

for prime in 1009 1000003 2147483647; do
  for curve in "2 3" "0 7" "5 11"; do
    read -r curve_a curve_b <<< "${curve}"
    corpus="${results_dir}/corpus/p${prime}_a${curve_a}_b${curve_b}.jsonl"
    "${build_dir}/semaev_lab" generate \
      --prime "${prime}" --a "${curve_a}" --b "${curve_b}" --seed "${seed}" \
      --min-arity 3 --max-arity 8 --samples "${samples_per_arity}" \
      --output "${corpus}" | tee "${results_dir}/raw/generate_p${prime}_a${curve_a}_b${curve_b}.log"
    gzip -9 -f "${corpus}"
  done

  "${build_dir}/semaev_lab" benchmark \
    --prime "${prime}" --a 2 --b 3 --seed "${seed}" \
    --min-arity 3 --max-arity 9 --bench-cases "${bench_cases}" --bench-repeats "${bench_repeats}" \
    --output "${results_dir}/benchmark_p${prime}.csv" | tee "${results_dir}/raw/benchmark_p${prime}.log"
done

python3 "${project_dir}/scripts/analyze_summaries.py" "${results_dir}"
sha256sum "${results_dir}"/corpus/*.gz "${results_dir}"/corpus/*.summary.csv \
  "${results_dir}"/*.csv > "${results_dir}/SHA256SUMS"

{
  echo "timestamp_utc=$(date -u +%Y-%m-%dT%H:%M:%SZ)"
  echo "uname=$(uname -a)"
  echo "compiler=$(g++ --version | head -n 1)"
  echo "samples_per_arity=${samples_per_arity}"
  echo "bench_cases=${bench_cases}"
  echo "bench_repeats=${bench_repeats}"
  echo "seed=${seed}"
  echo "git_revision=$(git -C "${project_dir}" rev-parse HEAD 2>/dev/null || echo unversioned)"
} > "${results_dir}/environment.txt"

echo "campaign complete: ${results_dir}"
