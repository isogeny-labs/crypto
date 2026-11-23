#!/usr/bin/env bash
set -euo pipefail
out=${EPHEMERAL_OUT:?ephemeral output directory required}
study=research/f4_f5_boolean_runtime_20261019
mkdir -p "$out/paired" "$out/controls" "$out/references" "$out/ablation" "$out/holdouts" "$out/source"
cp "$study/OPTIMIZATION_PROTOCOL.md" "$study/HYPOTHESES.md" "$study/run-optimization.sh" "$out/"
git rev-parse HEAD > "$out/revision.txt"
git diff --binary > "$out/local.patch"
rustc --version > "$out/compiler.txt"
lscpu > "$out/cpu.txt"
free -b > "$out/memory.txt"
files=(src/cryptanalysis/gf2_elim.rs examples/f4_f5_boolean_perf.rs tools/generalized-rootfinder/src/solver.rs src/cryptanalysis/ic_boundary.rs src/cryptanalysis/ic_framework/rootfind.rs)
for file in "${files[@]}"; do
  mkdir -p "$out/source/$(dirname "$file")"
  cp "$file" "$out/source/$file"
  cp "$study/frozen-baseline/$file" "$file"
done
cargo build --locked --release --example f4_f5_boolean_perf --bin isolated_bench > "$out/baseline-build.log" 2>&1
cp target/release/examples/f4_f5_boolean_perf "$out/baseline-benchmark"
for file in "${files[@]}"; do cp "$out/source/$file" "$file"; done
cargo build --locked --release --example f4_f5_boolean_perf --bin isolated_bench > "$out/candidate-build.log" 2>&1
cp target/release/examples/f4_f5_boolean_perf "$out/candidate-benchmark"
sha256sum "$out/baseline-benchmark" "$out/candidate-benchmark" > "$out/binaries.sha256"
sha256sum "${files[@]}" > "$out/source.sha256"
iso=target/release/isolated_bench
cpu=$(lscpu -p=CPU | sed '/^#/d' | tail -1)
cpus=$(cat "/sys/devices/system/cpu/cpu$cpu/topology/thread_siblings_list")
printf '%s\n' "$cpus" > "$out/reserved-cpus.txt"
# Let the build's CPU-pressure moving average decay before the A/A controls.
sleep 60
sample() {
  local prefix=$1 variant=$2 arm=$3
  shift 3
  local bin="$out/candidate-benchmark" vector=1 scalar=0 single=1
  case "$variant" in
    baseline) bin="$out/baseline-benchmark"; vector=0 ;;
    legacy) bin="$out/baseline-benchmark"; vector=0; single=0 ;;
    portable) vector=0 ;;
    scalar) scalar=1 ;;
  esac
  for attempt in 0 1 2; do
    local name="$prefix-$attempt"
    printf '%q ' "$iso" run --cpus "$cpus" --out "$name.isolation.jsonl" -- env RAYON_NUM_THREADS=1 KIC_GF2_WRITE_TABLE_ONCE="$single" KIC_GF2_VECTOR_TABLES="$vector" KIC_GENERALIZED_SCALAR_GRAY="$scalar" timeout 75 "$bin" --arm "$arm" "$@" > "$name.command"
    set +e
    "$iso" run --cpus "$cpus" --out "$name.isolation.jsonl" --label "$(basename "$prefix")" -- \
      env RAYON_NUM_THREADS=1 KIC_GF2_WRITE_TABLE_ONCE="$single" KIC_GF2_VECTOR_TABLES="$vector" KIC_GENERALIZED_SCALAR_GRAY="$scalar" timeout 75 "$bin" --arm "$arm" "$@" > "$name.jsonl" 2> "$name.stderr"
    code=$?
    set -e
    printf '%s\n' "$code" > "$name.exit"
    if [ "$code" = 0 ] && jq -e '.run.exit_status==0 and .run.contended==false and (.left_on_reserved.user_threads|length)==0' "$name.isolation.jsonl" >/dev/null 2>&1; then break; fi
  done
}
for n in 17 23; do
  for random in 0 1; do
    for round in 0 1 2 3 4; do
      for arm in solve-f4 solve-f5 solve-inherited generalized-native decomp-f4 decomp-f5 decomp-f6 decomp-generalized; do
        prefix="$arm-n$n-r$random-$round"
        sample "$out/controls/$prefix-a" baseline "$arm" --n "$n" --random "$random"
        sample "$out/controls/$prefix-b" baseline "$arm" --n "$n" --random "$random"
        sample "$out/paired/$prefix-a" baseline "$arm" --n "$n" --random "$random"
        sample "$out/paired/$prefix-b" candidate "$arm" --n "$n" --random "$random"
        case "$arm" in
          generalized-native|decomp-generalized) sample "$out/ablation/$prefix-scalar" scalar "$arm" --n "$n" --random "$random" ;;
        esac
      done
      for arm in roots-gray roots-wide roots-moebius roots-monica generic-gb decomp-f6-pairs rootfind-mitm rootfind-table; do
        sample "$out/references/$arm-n$n-r$random-$round" candidate "$arm" --n "$n" --random "$random"
      done
    done
  done
done
for n in 17 23; do
  for arm in matrix-f4 matrix-f5; do
    for round in 0 1 2 3 4; do
      prefix="$arm-n$n-r0-$round"
      sample "$out/controls/$prefix-a" baseline "$arm" --n "$n"
      sample "$out/controls/$prefix-b" baseline "$arm" --n "$n"
      sample "$out/paired/$prefix-a" baseline "$arm" --n "$n"
      sample "$out/paired/$prefix-b" candidate "$arm" --n "$n"
      sample "$out/ablation/$prefix-portable" portable "$arm" --n "$n"
      sample "$out/ablation/$prefix-legacy" legacy "$arm" --n "$n"
    done
  done
done
for seed in 231 232; do
  for arm in matrix-f4 matrix-f5; do
    for round in 0 1 2 3 4; do
      prefix="$arm-n23-s$seed-$round"
      sample "$out/holdouts/$prefix-a" baseline "$arm" --seed "$seed"
      sample "$out/holdouts/$prefix-b" candidate "$arm" --seed "$seed"
    done
  done
done
printf '%s\n' complete > "$out/optimization-phase.status"
