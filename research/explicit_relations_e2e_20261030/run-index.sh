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
timeout 120 compute-sanitizer --tool memcheck --error-exitcode 86 build/query-check > "$out/index-memcheck.log" 2>&1
timeout 120 compute-sanitizer --tool racecheck --error-exitcode 86 build/query-check > "$out/racecheck.log" 2>&1
timeout 120 compute-sanitizer --tool synccheck --error-exitcode 86 build/query-check > "$out/synccheck.log" 2>&1
nvcc -O3 -std=c++17 -arch=sm_120 -DER_CUDA -DER_TABLE_DIAGNOSTICS -x cu service.cpp -o build/explicit-relations-cuda
nvcc -O3 -std=c++17 -arch=sm_120 -DER_CUDA -DER_TABLE_DIAGNOSTICS -DER_QUERY_BINARY_REFERENCE -x cu service.cpp -o build/explicit-relations-binary
nvcc -O3 -std=c++17 -arch=sm_120 -DER_QUERY_BINARY_REFERENCE query-check.cu -o build/query-binary-check
build/explicit-relations-binary --self-test
build/query-binary-check
build/paper-driver build/explicit-relations-binary --self-test
c++ -O3 -std=c++17 "$study/target.cpp" -o "$out/target"
rustc --edition=2021 -D warnings -O "$study/ladder-index.rs" -o "$out/ladder"
rustc --edition=2021 -D warnings -O "$study/analyze-index.rs" -o "$out/analyze"
timeout 180 gp -q "$study/index-curves.gp" > "$out/extended-curves.txt" 2> "$out/curve-generation.log"
test "$(wc -l < "$out/extended-curves.txt")" = 5
cat "$out/extended-curves.txt" > "$out/curves.txt"
sha256sum "$out/target" "$out/ladder" "$out/analyze" build/paper-driver build/explicit-relations-cuda build/explicit-relations-binary > "$out/instruments.sha256"
"$out/ladder" "$out" "$PWD/build/paper-driver" "$PWD/build/explicit-relations-cuda" "$PWD/build/explicit-relations-binary" "$out/target" | tee "$out/ladder.log"
"$out/analyze" "$out/rows.csv" "$out" > "$out/analysis.log"
nvidia-smi -q > "$out/gpu-after.txt"
echo 'PASS complete bounded recovery ladder; timings remain L0 diagnostics'
