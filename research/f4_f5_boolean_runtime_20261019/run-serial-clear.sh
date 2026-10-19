#!/usr/bin/env bash
set -euo pipefail
out=${EPHEMERAL_OUT:?}
study=research/f4_f5_boolean_runtime_20261019
mkdir -p "$out/paired" "$out/controls" "$out/source/src/cryptanalysis"
cp "$study/SERIAL_CLEAR_PROTOCOL.md" "$study/run-serial-clear.sh" "$out/"
git rev-parse HEAD > "$out/revision.txt"
git diff --binary > "$out/local.patch"
rustc --version > "$out/compiler.txt"
lscpu > "$out/cpu.txt"
free -b > "$out/memory.txt"
cp src/cryptanalysis/gf2_elim.rs "$out/source/src/cryptanalysis/"
for state in 0 1; do
  RAYON_NUM_THREADS=1 KIC_GF2_WRITE_TABLE_ONCE=1 KIC_GF2_VECTOR_TABLES=1 KIC_GF2_SERIAL_CLEAR=$state cargo test --locked --release --lib cryptanalysis::gf2_elim::tests -- --test-threads=1 > "$out/kernel-tests-$state.log" 2>&1
done
cargo build --locked --release --example f4_f5_boolean_perf --bin isolated_bench > "$out/build.log" 2>&1
bin=target/release/examples/f4_f5_boolean_perf
iso=target/release/isolated_bench
cp "$bin" "$out/candidate-benchmark"
sha256sum "$bin" "$iso" > "$out/binaries.sha256"
sha256sum src/cryptanalysis/gf2_elim.rs examples/f4_f5_boolean_perf.rs > "$out/source.sha256"
mapfile -t last_cpus < <(lscpu -p=CPU | sed '/^#/d' | tail -2)
single=$(cat "/sys/devices/system/cpu/cpu${last_cpus[1]}/topology/thread_siblings_list")
other=$(cat "/sys/devices/system/cpu/cpu${last_cpus[0]}/topology/thread_siblings_list")
multi=$single,$other
printf 'one=%s\nfour=%s\n' "$single" "$multi" > "$out/reserved-cpus.txt"
sleep 60
sample() {
  local prefix=$1 state=$2 arm=$3 n=$4 seed=$5 workers=$6
  local cpus=$single
  if [ "$workers" = 4 ]; then cpus=$multi; fi
  for attempt in 0 1 2; do
    local name="$prefix-$attempt"
    printf '%q ' "$iso" run --cpus "$cpus" --out "$name.isolation.jsonl" -- env RAYON_NUM_THREADS="$workers" KIC_GF2_WRITE_TABLE_ONCE=1 KIC_GF2_VECTOR_TABLES=1 KIC_GF2_SERIAL_CLEAR="$state" timeout 75 "$bin" --arm "$arm" --n "$n" --seed "$seed" > "$name.command"
    set +e
    "$iso" run --cpus "$cpus" --out "$name.isolation.jsonl" --label "$(basename "$prefix")" -- env RAYON_NUM_THREADS="$workers" KIC_GF2_WRITE_TABLE_ONCE=1 KIC_GF2_VECTOR_TABLES=1 KIC_GF2_SERIAL_CLEAR="$state" timeout 75 "$bin" --arm "$arm" --n "$n" --seed "$seed" > "$name.jsonl" 2> "$name.stderr"
    code=$?
    set -e
    printf '%s\n' "$code" > "$name.exit"
    if [ "$code" = 0 ] && jq -e '.run.exit_status==0 and .run.contended==false and (.left_on_reserved.user_threads|length)==0' "$name.isolation.jsonl" >/dev/null 2>&1; then break; fi
  done
}
for workers in 1 4; do
  for arm in matrix-f4 matrix-f5; do
    for case in 17-230 23-230 23-231 23-232; do
      n=${case%-*}; seed=${case#*-}
      if [ "$workers" = 4 ] && [ "$seed" != 230 ]; then continue; fi
      for round in 0 1 2 3 4; do
        prefix="$arm-n$n-s$seed-w$workers-$round"
        sample "$out/controls/$prefix-a" 0 "$arm" "$n" "$seed" "$workers"
        sample "$out/controls/$prefix-b" 0 "$arm" "$n" "$seed" "$workers"
        sample "$out/paired/$prefix-a" 0 "$arm" "$n" "$seed" "$workers"
        sample "$out/paired/$prefix-b" 1 "$arm" "$n" "$seed" "$workers"
      done
    done
  done
done
printf 'complete\n' > "$out/serial-clear-phase.status"
