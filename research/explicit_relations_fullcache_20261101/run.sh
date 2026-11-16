#!/usr/bin/env bash
set -euo pipefail
root=$(git rev-parse --show-toplevel)
study="$root/research/explicit_relations_fullcache_20261101"
prior="$root/research/explicit_relations_e2e_20261030"
out=${EPHEMERAL_OUT:?}
mkdir -p "$out"
export RAYON_NUM_THREADS=1
nvidia-smi -q > "$out/gpu-before.txt"
cat /proc/cpuinfo > "$out/cpuinfo.txt"
cat /proc/self/status > "$out/process-status.txt"
git rev-parse HEAD > "$out/source-commit.txt"
[[ $(nvidia-smi --query-gpu=name --format=csv,noheader) == 'NVIDIA GeForce RTX 4090' ]]
cd "$root/gpu/explicit-relations"
mkdir -p build
flags=(-O3 -lineinfo -Xptxas=-v -std=c++17 -arch=sm_89 -DER_TABLE_DIAGNOSTICS -DER_COMPACT_ROWS -DER_WIDE_INDEX -DER_DENSE_ROWS -DER_CUB_SORT -DER_RIGHT_FULL_CACHE -DER_NEGATIVE_FILTER -DER_QUERY_CACHED -DER_AFFINE_QUERY -DER_QUERY_PERSISTENT)
make build/paper-driver build/affine-host-check
cargo test --locked --manifest-path Cargo.toml --target-dir build/cargo
cargo clippy --locked --manifest-path Cargo.toml --target-dir build/cargo -- -D warnings
build/affine-host-check > "$out/host-arithmetic.log"
nvcc "${flags[@]}" affine-device-check.cu -o build/affine-device-check 2> "$out/arithmetic-ptxas.log"
build/affine-device-check > "$out/gpu-arithmetic.log"
nvcc "${flags[@]}" -DER_CUDA -x cu service.cpp -o build/explicit-relations-affine 2> "$out/baseline-ptxas.log"
nvcc "${flags[@]}" -DER_QUERY_FULL_CACHE -DER_CUDA -x cu service.cpp -o build/explicit-relations-fullcache 2> "$out/candidate-ptxas.log"
nvcc "${flags[@]}" -DER_QUERY_FULL_CACHE query-fullcache-check.cu -o build/query-fullcache-check 2> "$out/differential-ptxas.log"
build/query-fullcache-check > "$out/differential.log" 2>&1
# The public backend differential includes forced full-cache refusal and
# partial-cache fallback, plus retained storage, infinity and tail cases.
nvcc "${flags[@]}" -DER_QUERY_FULL_CACHE -DER_PERSISTENT_BLOCK_LIMIT=2 query-check.cu -o build/query-fullcache-backend-check 2> "$out/backend-ptxas.log"
build/query-fullcache-backend-check > "$out/backend-check.log" 2>&1
for name in query-fullcache-check query-fullcache-backend-check; do
  for sanitizer in memcheck racecheck synccheck; do
    timeout 180 compute-sanitizer --tool "$sanitizer" --error-exitcode 86 "build/$name" > "$out/$name-$sanitizer.log" 2>&1
  done
done
for name in explicit-relations-affine explicit-relations-fullcache; do
  "build/$name" --self-test > "$out/$name-service-check.log" 2>&1
  build/paper-driver "build/$name" --self-test > "$out/$name-driver-check.log" 2>&1
  printf 'curve 65521 0 7 4294967296 1048576\nquit\n' | "build/$name" > "$out/$name-cap-header.log"
  test "$(cat "$out/$name-cap-header.log")" = 'READY 1'
done
# Verify no timing or diagnostic-stage kernel is present in the candidate.
cuobjdump --dump-ptx build/explicit-relations-fullcache > "$out/candidate.ptx"
grep -q 'search_right_fullcache' "$out/candidate.ptx"
if grep -Eq '%clock|search_right_profile' "$out/candidate.ptx"; then
  echo 'Unexpected profile instrumentation in candidate' >&2; exit 86
fi
c++ -O3 -std=c++17 "$prior/target.cpp" -o "$out/target"
rustc --edition=2021 -D warnings -O "$study/ladder.rs" -o "$out/ladder"
rustc --edition=2021 -D warnings -O "$study/analyze.rs" -o "$out/analyze"
timeout 180 gp -q "$prior/field63-curves.gp" > "$out/curves.txt" 2> "$out/curve-generation.log"
cmp "$study/curves.txt" "$out/curves.txt"
sha256sum build/paper-driver build/explicit-relations-affine build/explicit-relations-fullcache "$out/target" "$out/ladder" "$out/analyze" > "$out/instruments.sha256"
printf 'PREFLIGHT_COMPLETE\n'
"$out/ladder" "$out" "$PWD/build/paper-driver" "$PWD/build/explicit-relations-fullcache" "$PWD/build/explicit-relations-affine" "$out/target" | tee "$out/ladder.log"
"$out/analyze" "$out" > "$out/analysis.log"
nvidia-smi -q > "$out/gpu-after.txt"
printf '0\n' > "$out/validation.exit"
printf 'PASS matched full-cache complete recovery panel\n'
