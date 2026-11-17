#!/usr/bin/env bash
set -euo pipefail
out=${EPHEMERAL_OUT:?ephemeral output directory required}
study=research/f4_f5_boolean_runtime_20261019
mkdir -p "$out/paired" "$out/controls" "$out/comparison" "$out/reference-controls"
cp "$study/FINAL_PROTOCOL.md" "$study/HYPOTHESES.md" "$study/run-final.sh" "$out/"
git rev-parse HEAD > "$out/revision.txt"
git diff --binary > "$out/local.patch"
rustc --version > "$out/compiler.txt"
lscpu > "$out/cpu.txt"
free -b > "$out/memory.txt"
cargo test --locked --release --lib folded_callback_batches_preserve_every_key_witness_and_group_charge > "$out/batch-map-test.log" 2>&1
cargo test --locked --release --lib cryptanalysis::ic_framework::rootfind::tests > "$out/rootfind-tests.log" 2>&1
cargo build --locked --release --example f4_f5_boolean_perf --bin isolated_bench > "$out/build.log" 2>&1
bin=target/release/examples/f4_f5_boolean_perf
iso=target/release/isolated_bench
cp "$bin" "$out/candidate-benchmark"
sha256sum "$bin" "$iso" > "$out/binaries.sha256"
sha256sum src/cryptanalysis/gf2_elim.rs src/cryptanalysis/ic_boundary.rs src/cryptanalysis/ic_framework/rootfind.rs tools/generalized-rootfinder/src/solver.rs examples/f4_f5_boolean_perf.rs > "$out/source.sha256"
cpu=$(lscpu -p=CPU | sed '/^#/d' | tail -1)
cpus=$(cat "/sys/devices/system/cpu/cpu$cpu/topology/thread_siblings_list")
printf '%s\n' "$cpus" > "$out/reserved-cpus.txt"
sleep 60
sample() {
  local prefix=$1 batch=$2 arm=$3
  shift 3
  for attempt in 0 1 2; do
    local name="$prefix-$attempt"
    printf '%q ' "$iso" run --cpus "$cpus" --out "$name.isolation.jsonl" -- env RAYON_NUM_THREADS=1 KIC_GF2_WRITE_TABLE_ONCE=1 KIC_GF2_VECTOR_TABLES=1 KIC_ROOTFIND_BATCH_SETUP="$batch" timeout 75 "$bin" --arm "$arm" "$@" > "$name.command"
    set +e
    "$iso" run --cpus "$cpus" --out "$name.isolation.jsonl" --label "$(basename "$prefix")" -- \
      env RAYON_NUM_THREADS=1 KIC_GF2_WRITE_TABLE_ONCE=1 KIC_GF2_VECTOR_TABLES=1 KIC_ROOTFIND_BATCH_SETUP="$batch" timeout 75 "$bin" --arm "$arm" "$@" > "$name.jsonl" 2> "$name.stderr"
    code=$?
    set -e
    printf '%s\n' "$code" > "$name.exit"
    if [ "$code" = 0 ] && jq -e '.run.exit_status==0 and .run.contended==false and (.left_on_reserved.user_threads|length)==0' "$name.isolation.jsonl" >/dev/null 2>&1; then break; fi
  done
}
for n in 17 23; do
  for random in 0 1; do
    for round in 0 1 2 3 4; do
      prefix="rootfind-mitm-n$n-r$random-$round"
      sample "$out/controls/$prefix-a" 0 rootfind-mitm --n "$n" --random "$random"
      sample "$out/controls/$prefix-b" 0 rootfind-mitm --n "$n" --random "$random"
      sample "$out/paired/$prefix-a" 0 rootfind-mitm --n "$n" --random "$random"
      sample "$out/paired/$prefix-b" 1 rootfind-mitm --n "$n" --random "$random"
      for side in a b; do
        sample "$out/reference-controls/solve-f4-n$n-r$random-$round-$side" 1 solve-f4 --n "$n" --random "$random"
        sample "$out/reference-controls/decomp-f6-n$n-r$random-$round-$side" 1 decomp-f6 --n "$n" --random "$random"
      done
      for arm in roots-gray roots-wide solve-f4 decomp-f4 solve-f5 decomp-f5 solve-inherited decomp-f6 generalized-native decomp-generalized roots-moebius rootfind-mitm roots-monica rootfind-table generic-gb decomp-f6-pairs; do
        sample "$out/comparison/$arm-n$n-r$random-$round" 1 "$arm" --n "$n" --random "$random"
      done
    done
  done
done
printf '%s\n' complete > "$out/final-phase.status"
