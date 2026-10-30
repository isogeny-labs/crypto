#!/usr/bin/env bash
set -euo pipefail
out=${EPHEMERAL_OUT:?ephemeral output directory required}
study=research/f4_f5_ecbench_20261019
mkdir -p "$out/source" "$out/controls" "$out/paired" "$out/comparison" "$out/four-worker"
cp "$study/PROTOCOL.md" "$study/run-native.sh" "$out/"
git rev-parse HEAD > "$out/revision.txt"
git diff --binary > "$out/local.patch"
rustc --version > "$out/compiler.txt"
lscpu > "$out/cpu.txt"
free -b > "$out/memory.txt"
files=(src/cryptanalysis/koblitz_groebner.rs src/cryptanalysis/gf2_elim.rs examples/f4_f5_boolean_perf.rs)
for file in "${files[@]}"; do
  mkdir -p "$out/source/$(dirname "$file")"
  cp "$file" "$out/source/$file"
  cp "$study/frozen-baseline/$(basename "$file")" "$file"
done
cargo build --locked --release --example f4_f5_boolean_perf --bin isolated_bench > "$out/baseline-build.log" 2>&1
cp target/release/examples/f4_f5_boolean_perf "$out/baseline-benchmark"
for file in "${files[@]}"; do cp "$out/source/$file" "$file"; done
cargo test --locked --release --lib matrix_runtime_ -- --test-threads=1 > "$out/runtime-tests.log" 2>&1
for workers in 1 4; do
  RAYON_NUM_THREADS="$workers" KIC_GF2_WRITE_TABLE_ONCE=1 KIC_GF2_VECTOR_TABLES=1 KIC_GF2_SERIAL_CLEAR=1 \
    cargo test --locked --release --lib cryptanalysis::gf2_elim::tests -- --test-threads=1 > "$out/kernel-tests-$workers.log" 2>&1
done
cargo build --locked --release --example f4_f5_boolean_perf --bin isolated_bench > "$out/candidate-build.log" 2>&1
cp target/release/examples/f4_f5_boolean_perf "$out/candidate-benchmark"
sha256sum "$out/baseline-benchmark" "$out/candidate-benchmark" > "$out/binaries.sha256"
sha256sum "${files[@]}" tools/generalized-rootfinder/src/solver.rs src/cryptanalysis/ic_framework/rootfind.rs src/cryptanalysis/ic_boundary.rs > "$out/source.sha256"
iso=target/release/isolated_bench
mapfile -t core_sets < <(lscpu -p=CPU | sed '/^#/d' | while read -r cpu; do cat "/sys/devices/system/cpu/cpu$cpu/topology/thread_siblings_list"; done | sort -u)
[ "${#core_sets[@]}" -ge 3 ]
single=${core_sets[-1]}
multi=$single,${core_sets[-2]}
printf 'one=%s\nfour=%s\n' "$single" "$multi" > "$out/reserved-cpus.txt"
sleep 60
sample() {
  local prefix=$1 variant=$2 arm=$3 n=$4 seed=$5 random=$6 workers=${7:-1}
  local bin="$out/candidate-benchmark" policy=$variant cpus=$single
  if [ "$workers" = 4 ]; then cpus=$multi; fi
  if [ "$variant" = baseline ]; then bin="$out/baseline-benchmark"; policy=default; fi
  for attempt in 0 1 2; do
    local name="$prefix-$attempt"
    printf '%q ' "$iso" run --cpus "$cpus" --out "$name.isolation.jsonl" -- env \
      RAYON_NUM_THREADS="$workers" KIC_GF2_WRITE_TABLE_ONCE=1 KIC_GF2_VECTOR_TABLES=1 \
      KIC_GF2_SERIAL_CLEAR=1 KIC_ROOTFIND_BATCH_SETUP=1 timeout 75 "$bin" \
      --matrix-runtime "$policy" --arm "$arm" --n "$n" --seed "$seed" --random "$random" > "$name.command"
    set +e
    "$iso" run --cpus "$cpus" --out "$name.isolation.jsonl" --label "$(basename "$prefix")" -- env \
      RAYON_NUM_THREADS="$workers" KIC_GF2_WRITE_TABLE_ONCE=1 KIC_GF2_VECTOR_TABLES=1 \
      KIC_GF2_SERIAL_CLEAR=1 KIC_ROOTFIND_BATCH_SETUP=1 timeout 75 "$bin" \
      --matrix-runtime "$policy" --arm "$arm" --n "$n" --seed "$seed" --random "$random" > "$name.jsonl" 2> "$name.stderr"
    code=$?
    set -e
    printf '%s\n' "$code" > "$name.exit"
    if [ "$code" = 0 ] && jq -e '.run.exit_status==0 and .run.contended==false and (.left_on_reserved.user_threads|length)==0' "$name.isolation.jsonl" >/dev/null 2>&1; then break; fi
  done
}
for n in 17 23; do
  for case in 230-0 230-1 231-0 232-0; do
    seed=${case%-*}; random=${case#*-}
    for round in 0 1 2 3 4; do
      for arm in solve-f4 solve-f5 decomp-f4 decomp-f5; do
        prefix="$arm-n$n-s$seed-r$random-$round"
        sample "$out/controls/$prefix-a" baseline "$arm" "$n" "$seed" "$random"
        sample "$out/controls/$prefix-b" baseline "$arm" "$n" "$seed" "$random"
        sample "$out/paired/$prefix-a" baseline "$arm" "$n" "$seed" "$random"
        sample "$out/paired/$prefix-prepared" prepared "$arm" "$n" "$seed" "$random"
        sample "$out/paired/$prefix-tail" quadratic-tail "$arm" "$n" "$seed" "$random"
      done
      if [ "$seed" = 230 ]; then
        for arm in roots-gray roots-wide solve-f4 decomp-f4 solve-f5 decomp-f5 solve-inherited decomp-f6 \
          generalized-native generalized-typed decomp-generalized decomp-generalized-typed \
          roots-moebius rootfind-mitm roots-monica rootfind-table generic-gb decomp-f6-pairs; do
          sample "$out/comparison/$arm-n$n-r$random-$round" quadratic-tail "$arm" "$n" "$seed" "$random"
        done
      fi
    done
  done
  for arm in matrix-f4 matrix-f5; do
    for round in 0 1 2 3 4; do
      prefix="$arm-n$n-$round"
      sample "$out/four-worker/$prefix-aa" baseline "$arm" "$n" 230 0 4
      sample "$out/four-worker/$prefix-ab" baseline "$arm" "$n" 230 0 4
      sample "$out/four-worker/$prefix-a" baseline "$arm" "$n" 230 0 4
      sample "$out/four-worker/$prefix-b" quadratic-tail "$arm" "$n" 230 0 4
    done
  done
done
printf '%s\n' complete > "$out/native-phase.status"
