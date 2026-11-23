#!/usr/bin/env bash
set -euo pipefail
root=$(git rev-parse --show-toplevel)
out=${EPHEMERAL_OUT:?output directory required}
study="$root/research/explicit_relations_e2e_20261030"
mkdir -p "$out"
nvidia-smi -q > "$out/gpu-before.txt"
cat /proc/cpuinfo > "$out/cpuinfo.txt"
cat /proc/self/status > "$out/process-status.txt"
git rev-parse HEAD > "$out/source-commit.txt"
nvcc --version > "$out/nvcc.txt"
bash "$root/gpu/explicit-relations/validate-rtx.sh"
cd "$root/gpu/explicit-relations"
make ARCH=sm_120 build/query-wide-check build/query-dense-check
build/query-wide-check
build/query-dense-check
for variant in query-check query-compact-check query-wide-check query-dense-check; do
  for sanitizer in memcheck racecheck synccheck; do
    timeout 180 compute-sanitizer --tool "$sanitizer" --error-exitcode 86 "build/$variant" > "$out/$variant-$sanitizer.log" 2>&1
  done
done
nvcc -O3 -std=c++17 -arch=sm_120 -DER_CUDA -DER_TABLE_DIAGNOSTICS -DER_COMPACT_ROWS -DER_WIDE_INDEX -x cu service.cpp -o build/explicit-relations-wide16
nvcc -O3 -std=c++17 -arch=sm_120 -DER_CUDA -DER_TABLE_DIAGNOSTICS -DER_COMPACT_ROWS -DER_WIDE_INDEX -DER_DENSE_ROWS -x cu service.cpp -o build/explicit-relations-dense
build/explicit-relations-wide16 --self-test
build/paper-driver build/explicit-relations-wide16 --self-test
build/explicit-relations-dense --self-test
build/paper-driver build/explicit-relations-dense --self-test
timeout 600 compute-sanitizer --tool memcheck --error-exitcode 86 build/explicit-relations-dense --self-test > "$out/dense-service-memcheck.log" 2>&1
# Exercise the actual line protocol with the advertised maximum before data.
for variant in explicit-relations-wide16 explicit-relations-dense; do
  printf 'curve 65521 0 7 4294967296 1048576\nquit\n' | "build/$variant" > "$out/$variant-cap-header.log"
  test "$(cat "$out/$variant-cap-header.log")" = 'READY 1'
done
c++ -O3 -std=c++17 "$study/target.cpp" -o "$out/target"
rustc --edition=2021 -D warnings -O "$study/ladder-boundary.rs" -o "$out/ladder"
rustc --edition=2021 -D warnings -O "$study/analyze-dense.rs" -o "$out/analyze"
timeout 180 gp -q "$study/boundary-curves.gp" > "$out/extended-curves.txt" 2> "$out/curve-generation.log"
test "$(wc -l < "$out/extended-curves.txt")" = 3
cat "$out/extended-curves.txt" > "$out/curves.txt"
sha256sum "$out/target" "$out/ladder" "$out/analyze" build/paper-driver build/explicit-relations-dense build/explicit-relations-wide16 > "$out/instruments.sha256"
"$out/ladder" "$out" "$PWD/build/paper-driver" "$PWD/build/explicit-relations-dense" "$PWD/build/explicit-relations-wide16" "$out/target" | tee "$out/ladder.log"
"$out/analyze" "$out/rows.csv" "$out" > "$out/analysis.log"
nvidia-smi -q > "$out/gpu-after.txt"
echo 'PASS complete bounded recovery ladder; timings remain L0 diagnostics'
