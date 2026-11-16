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
for variant in query-check query-compact-check; do
  for sanitizer in memcheck racecheck synccheck; do
    timeout 180 compute-sanitizer --tool "$sanitizer" --error-exitcode 86 "build/$variant" > "$out/$variant-$sanitizer.log" 2>&1
  done
done
nvcc -O3 -std=c++17 -arch=sm_120 -DER_CUDA -DER_TABLE_DIAGNOSTICS -x cu service.cpp -o build/explicit-relations-full
nvcc -O3 -std=c++17 -arch=sm_120 -DER_CUDA -DER_TABLE_DIAGNOSTICS -DER_COMPACT_ROWS -x cu service.cpp -o build/explicit-relations-compact
build/explicit-relations-full --self-test
build/paper-driver build/explicit-relations-full --self-test
build/explicit-relations-compact --self-test
build/paper-driver build/explicit-relations-compact --self-test
c++ -O3 -std=c++17 "$study/target.cpp" -o "$out/target"
rustc --edition=2021 -D warnings -O "$study/ladder-compact.rs" -o "$out/ladder"
rustc --edition=2021 -D warnings -O "$study/analyze-compact.rs" -o "$out/analyze"
timeout 180 gp -q "$study/cache-curves.gp" > "$out/extended-curves.txt" 2> "$out/curve-generation.log"
test "$(wc -l < "$out/extended-curves.txt")" = 4
cat "$out/extended-curves.txt" > "$out/curves.txt"
sha256sum "$out/target" "$out/ladder" "$out/analyze" build/paper-driver build/explicit-relations-compact build/explicit-relations-full > "$out/instruments.sha256"
"$out/ladder" "$out" "$PWD/build/paper-driver" "$PWD/build/explicit-relations-compact" "$PWD/build/explicit-relations-full" "$out/target" | tee "$out/ladder.log"
"$out/analyze" "$out/rows.csv" "$out" > "$out/analysis.log"
nvidia-smi -q > "$out/gpu-after.txt"
echo 'PASS complete bounded recovery ladder; timings remain L0 diagnostics'
