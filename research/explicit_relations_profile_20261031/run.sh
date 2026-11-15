#!/usr/bin/env bash
set -euo pipefail
root=$(git rev-parse --show-toplevel)
study="$root/research/explicit_relations_profile_20261031"
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
build/affine-host-check
nvcc "${flags[@]}" affine-device-check.cu -o build/affine-device-check 2> "$out/affine-ptxas.log"
build/affine-device-check
nvcc "${flags[@]}" -DER_CUDA -x cu service.cpp -o build/explicit-relations-affine 2> "$out/stock-ptxas.log"
nvcc "${flags[@]}" -DER_QUERY_PROFILE -DER_CUDA -x cu service.cpp -o build/explicit-relations-profile 2> "$out/profile-ptxas.log"
nvcc "${flags[@]}" -DER_QUERY_PROFILE query-profile-check.cu -o build/query-profile-check 2> "$out/check-ptxas.log"
build/query-profile-check > "$out/profile-check.log" 2>&1
for sanitizer in memcheck racecheck synccheck; do
  timeout 180 compute-sanitizer --tool "$sanitizer" --error-exitcode 86 build/query-profile-check > "$out/profile-$sanitizer.log" 2>&1
done
# Existing differentials also cover the normalization refactor and fallback paths.
nvcc "${flags[@]}" -DER_PERSISTENT_BLOCK_LIMIT=2 query-check.cu -o build/query-affine-check 2> "$out/query-ptxas.log"
build/query-affine-check > "$out/query-check.log" 2>&1
build/explicit-relations-affine --self-test > "$out/service-check.log" 2>&1
build/paper-driver build/explicit-relations-affine --self-test > "$out/driver-check.log" 2>&1
rustc --edition=2021 -D warnings -O "$study/capture.rs" -o "$out/capture"
rustc --edition=2021 -D warnings -O "$study/analyze.rs" -o "$out/analyze"
sha256sum build/explicit-relations-affine build/explicit-relations-profile build/paper-driver "$out/capture" "$out/analyze" > "$out/instruments.sha256"
for seed in 101 202; do
  read -r -a params < "$study/63-$seed.input"
  set +e
  ER_CAPTURE="$out/$seed.commands" ER_SERVICE="$PWD/build/explicit-relations-affine" timeout 600 build/paper-driver "$out/capture" "${params[@]}" > "$out/$seed.capture.stdout" 2> "$out/$seed.capture.stderr"
  rc=$?
  set -e
  printf '%s\n' "$rc" > "$out/$seed.capture.exit"
  test "$rc" = 1
  test "$(cat "$out/$seed.commands.complete")" = 8
  test "$(grep -c '^find ' "$out/$seed.commands")" = 8
  grep -Eq 'relation service (write failed|exited without a result)' "$out/$seed.capture.stderr"
  for round in 0 1 2 3 4; do
    arms=(b s x)
    if (( round % 2 )); then arms=(x s b); fi
    for arm in "${arms[@]}"; do
      mode=stock
      [[ "$arm" != s ]] || mode=stages
      id="$seed-$round-$arm"
      set +e
      ER_PROFILE_MODE="$mode" timeout 600 build/explicit-relations-profile < "$out/$seed.commands" > "$out/$id.stdout" 2> "$out/$id.stderr"
      rc=$?
      set -e
      printf '%s\n' "$rc" > "$out/$id.exit"
      test "$rc" = 0
      cmp "$out/$seed.commands.answers" "$out/$id.stdout"
      printf 'PROFILE_CELL_COMPLETE %s\n' "$id"
    done
  done
done
"$out/analyze" "$out" > "$out/analysis.log"
# Repository profiler wrapper records unsupported facilities rather than estimates.
cd "$root"
set +e
timeout 180 bash tools/profile.sh probe --gpu-control --out "$out/probe" > "$out/probe.log" 2>&1
printf '%s\n' "$?" > "$out/probe.exit"
for seed in 101 202; do
  timeout 600 bash tools/profile.sh nsys --out "$out/nsys-$seed" -- bash "$study/replay.sh" "$root/gpu/explicit-relations/build/explicit-relations-affine" "$out/$seed.commands" > "$out/nsys-$seed.log" 2>&1
  printf '%s\n' "$?" > "$out/nsys-$seed.exit"
  timeout 600 bash tools/profile.sh ncu --out "$out/ncu-$seed" --kernel search_right_persistent --launch-count 1 --source -- bash "$study/replay.sh" "$root/gpu/explicit-relations/build/explicit-relations-affine" "$out/$seed.commands" > "$out/ncu-$seed.log" 2>&1
  printf '%s\n' "$?" > "$out/ncu-$seed.exit"
done
set -e
nvidia-smi -q > "$out/gpu-after.txt"
printf '0\n' > "$out/validation.exit"
printf 'PASS bounded profile: 30 cells, 240 queries; exact protocol replies match\n'
