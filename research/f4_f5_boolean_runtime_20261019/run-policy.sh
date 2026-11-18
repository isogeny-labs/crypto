#!/usr/bin/env bash
set -euo pipefail
out=${EPHEMERAL_OUT:?ephemeral output directory required}
study=research/f4_f5_boolean_runtime_20261019
mkdir -p "$out/policy" "$out/policy-controls" "$out/comparison" "$out/reference-controls" "$out/source"
cp "$study/POLICY_PROTOCOL.md" "$study/run-policy.sh" "$out/"
git rev-parse HEAD > "$out/revision.txt"
git diff --binary > "$out/local.patch"
rustc --version > "$out/compiler.txt"
lscpu > "$out/cpu.txt"
free -b > "$out/memory.txt"
files=(examples/f4_f5_boolean_perf.rs tools/generalized-rootfinder/src/solver.rs)
for file in "${files[@]}"; do
  mkdir -p "$out/source/$(dirname "$file")"
  cp "$file" "$out/source/$file"
  cp "$study/frozen-iteration1/$file" "$file"
done
cargo build --locked --release --example f4_f5_boolean_perf --bin isolated_bench > "$out/baseline-build.log" 2>&1
cp target/release/examples/f4_f5_boolean_perf "$out/baseline-benchmark"
for file in "${files[@]}"; do cp "$out/source/$file" "$file"; done
cargo test --locked --release --manifest-path tools/generalized-rootfinder/Cargo.toml > "$out/typed-tests.log" 2>&1
cargo build --locked --release --example f4_f5_boolean_perf --bin isolated_bench > "$out/candidate-build.log" 2>&1
cp target/release/examples/f4_f5_boolean_perf "$out/candidate-benchmark"
sha256sum "$out/baseline-benchmark" "$out/candidate-benchmark" > "$out/binaries.sha256"
sha256sum "${files[@]}" src/cryptanalysis/gf2_elim.rs src/cryptanalysis/ic_boundary.rs src/cryptanalysis/ic_framework/rootfind.rs > "$out/source.sha256"
iso=target/release/isolated_bench
cpu=$(lscpu -p=CPU | sed '/^#/d' | tail -1)
cpus=$(cat "/sys/devices/system/cpu/cpu$cpu/topology/thread_siblings_list")
printf '%s\n' "$cpus" > "$out/reserved-cpus.txt"
sleep 60
sample() {
  local prefix=$1 variant=$2 arm=$3 n=$4 seed=$5 random=$6
  local bin="$out/candidate-benchmark" explicit=0
  if [ "$variant" = baseline ]; then bin="$out/baseline-benchmark"; fi
  if [ "$variant" = wide ]; then explicit=1; fi
  for attempt in 0 1 2; do
    local name="$prefix-$attempt"
    printf '%q ' "$iso" run --cpus "$cpus" --out "$name.isolation.jsonl" -- env RAYON_NUM_THREADS=1 KIC_GF2_WRITE_TABLE_ONCE=1 KIC_GF2_VECTOR_TABLES=1 KIC_ROOTFIND_BATCH_SETUP=1 timeout 75 "$bin" --arm "$arm" --n "$n" --seed "$seed" --random "$random" --explicit-wide "$explicit" > "$name.command"
    set +e
    "$iso" run --cpus "$cpus" --out "$name.isolation.jsonl" --label "$(basename "$prefix")" -- \
      env RAYON_NUM_THREADS=1 KIC_GF2_WRITE_TABLE_ONCE=1 KIC_GF2_VECTOR_TABLES=1 KIC_ROOTFIND_BATCH_SETUP=1 timeout 75 "$bin" --arm "$arm" --n "$n" --seed "$seed" --random "$random" --explicit-wide "$explicit" > "$name.jsonl" 2> "$name.stderr"
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
      for arm in generalized-native decomp-generalized; do
        prefix="$arm-n$n-s$seed-r$random-$round"
        sample "$out/policy-controls/$prefix-a" baseline "$arm" "$n" "$seed" "$random"
        sample "$out/policy-controls/$prefix-b" baseline "$arm" "$n" "$seed" "$random"
        sample "$out/policy/$prefix-a" baseline "$arm" "$n" "$seed" "$random"
        sample "$out/policy/$prefix-b" candidate "$arm" "$n" "$seed" "$random"
        sample "$out/policy/$prefix-wide" wide "$arm" "$n" "$seed" "$random"
        typed=generalized-typed
        if [ "$arm" = decomp-generalized ]; then typed=decomp-generalized-typed; fi
        sample "$out/policy/$prefix-typed" candidate "$typed" "$n" "$seed" "$random"
      done
      if [ "$seed" = 230 ]; then
        for side in a b; do
          sample "$out/reference-controls/solve-f4-n$n-r$random-$round-$side" candidate solve-f4 "$n" "$seed" "$random"
          sample "$out/reference-controls/decomp-f6-n$n-r$random-$round-$side" candidate decomp-f6 "$n" "$seed" "$random"
        done
        for arm in roots-gray roots-wide solve-f4 decomp-f4 solve-f5 decomp-f5 solve-inherited decomp-f6 generalized-native generalized-typed decomp-generalized decomp-generalized-typed roots-moebius rootfind-mitm roots-monica rootfind-table generic-gb decomp-f6-pairs; do
          sample "$out/comparison/$arm-n$n-r$random-$round" candidate "$arm" "$n" "$seed" "$random"
        done
      fi
    done
  done
done
printf '%s\n' complete > "$out/policy-phase.status"
