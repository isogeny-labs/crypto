#!/usr/bin/env bash
# Version-2 supplement: preserve v1 and rerun only the external profilers.
set -euo pipefail
root=$(git rev-parse --show-toplevel)
study="$root/research/explicit_relations_profile_20261031"
out=${EPHEMERAL_OUT:?}
mkdir -p "$out"
nvidia-smi -q > "$out/gpu-before.txt"
git rev-parse HEAD > "$out/source-commit.txt"
command -v gawk > "$out/gawk-path.txt"
[[ $(nvidia-smi --query-gpu=name --format=csv,noheader) == 'NVIDIA GeForce RTX 4090' ]]
cd "$root/gpu/explicit-relations"
mkdir -p build
nvcc -O3 -lineinfo -Xptxas=-v -std=c++17 -arch=sm_89 -DER_TABLE_DIAGNOSTICS -DER_COMPACT_ROWS -DER_WIDE_INDEX -DER_DENSE_ROWS -DER_CUB_SORT -DER_RIGHT_FULL_CACHE -DER_NEGATIVE_FILTER -DER_QUERY_CACHED -DER_AFFINE_QUERY -DER_QUERY_PERSISTENT -DER_CUDA -x cu service.cpp -o build/explicit-relations-affine 2> "$out/stock-ptxas.log"
sha256sum build/explicit-relations-affine > "$out/instruments.sha256"
cd "$root"
# Prerequisite/tool capabilities are recorded even if the provider refuses counters.
set +e
timeout 180 bash tools/profile.sh probe --gpu-control --out "$out/probe" > "$out/probe.log" 2>&1
printf '%s\n' "$?" > "$out/probe.exit"
set -e
for seed in 101 202; do
  # Verify the copied transcript and current unprofiled service before attribution.
  timeout 600 "$root/gpu/explicit-relations/build/explicit-relations-affine" < "$study/results-v1/$seed.commands" > "$out/$seed.stdout" 2> "$out/$seed.stderr"
  cmp "$out/$seed.stdout" "$study/results-v1/$seed.commands.answers"
  set +e
  timeout 600 bash tools/profile.sh nsys --out "$out/nsys-$seed" -- bash "$study/replay.sh" "$root/gpu/explicit-relations/build/explicit-relations-affine" "$study/results-v1/$seed.commands" > "$out/nsys-$seed.log" 2>&1
  printf '%s\n' "$?" > "$out/nsys-$seed.exit"
  timeout 600 bash tools/profile.sh ncu --out "$out/ncu-$seed" --kernel search_right_persistent --launch-count 1 --source -- bash "$study/replay.sh" "$root/gpu/explicit-relations/build/explicit-relations-affine" "$study/results-v1/$seed.commands" > "$out/ncu-$seed.log" 2>&1
  printf '%s\n' "$?" > "$out/ncu-$seed.exit"
  set -e
done
nvidia-smi -q > "$out/gpu-after.txt"
printf '0\n' > "$out/validation.exit"
printf 'PASS profiler supplement; inspect individual profiler statuses\n'
