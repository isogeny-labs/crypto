#!/usr/bin/env bash
# Thin orchestration only; solving, verification and comparisons are native Rust.
set -euo pipefail
out=${EPHEMERAL_OUT:?ephemeral output directory required}
mkdir -p "$out/matrix" "$out/comparison" "$out/controls"
git rev-parse HEAD > "$out/revision.txt"
git diff --binary > "$out/local.patch"
cp research/f4_f5_boolean_runtime_20261019/PROTOCOL.md "$out/PROTOCOL.md"
cp research/f4_f5_boolean_runtime_20261019/run-native.sh "$out/run-native.sh"
rustc --version > "$out/compiler.txt"
lscpu > "$out/cpu.txt"
free -b > "$out/memory.txt"
cargo build --locked --release --example f4_f5_boolean_perf --bin isolated_bench > "$out/build.log" 2>&1
bin=target/release/examples/f4_f5_boolean_perf
iso=target/release/isolated_bench
cp "$bin" "$out/baseline-benchmark"
sha256sum "$bin" "$iso" > "$out/binaries.sha256"
cpu=$(lscpu -p=CPU | sed '/^#/d' | tail -1)
cpus=$(cat "/sys/devices/system/cpu/cpu$cpu/topology/thread_siblings_list")
printf '%s\n' "$cpus" > "$out/reserved-cpus.txt"
sleep 60
if command -v perf >/dev/null; then
  tools/perf_ipc.sh --repeat 1 --out "$out/pmu.jsonl" -- true > "$out/pmu.stdout" 2> "$out/pmu.stderr" || printf '%s\n' "$?" > "$out/pmu.exit"
else
  printf '%s\n' 'perf binary unavailable; no cycle or IPC measurement' > "$out/pmu-unavailable.txt"
fi
sample() {
  local prefix=$1 state=$2 arm=$3
  shift 3
  for attempt in 0 1 2; do
    local name="$prefix-$attempt"
    printf '%q ' "$iso" run --cpus "$cpus" --out "$name.isolation.jsonl" -- env RAYON_NUM_THREADS=1 KIC_GF2_WRITE_TABLE_ONCE="$state" timeout 75 "$bin" --arm "$arm" "$@" > "$name.command"
    set +e
    "$iso" run --cpus "$cpus" --out "$name.isolation.jsonl" --label "$(basename "$prefix")" -- \
      env RAYON_NUM_THREADS=1 KIC_GF2_WRITE_TABLE_ONCE="$state" timeout 75 "$bin" --arm "$arm" "$@" > "$name.jsonl" 2> "$name.stderr"
    code=$?
    set -e
    printf '%s\n' "$code" > "$name.exit"
    if [ "$code" = 0 ] && jq -e '.run.exit_status==0 and .run.contended==false and (.left_on_reserved.user_threads|length)==0' "$name.isolation.jsonl" >/dev/null 2>&1; then break; fi
  done
}
for arm in matrix-f4 matrix-f5; do
  for phase in aa ab; do
    for round in 0 1 2 3 4; do
      sample "$out/matrix/$arm-$phase-$round-a" 0 "$arm"
      state=0
      if [ "$phase" = ab ]; then state=1; fi
      sample "$out/matrix/$arm-$phase-$round-b" "$state" "$arm"
    done
  done
  for case in 17-230 23-231 23-232; do
    n=${case%-*}
    seed=${case#*-}
    for round in 0 1 2 3 4; do
      sample "$out/matrix/$arm-n$n-s$seed-$round-a" 0 "$arm" --n "$n" --seed "$seed"
      sample "$out/matrix/$arm-n$n-s$seed-$round-b" 1 "$arm" --n "$n" --seed "$seed"
    done
  done
done
for n in 17 23; do
  for random in 0 1; do
    for round in 0 1 2 3 4; do
      for side in a b; do
        sample "$out/controls/roots-gray-n$n-r$random-$round-$side" 1 roots-gray --n "$n" --random "$random"
        sample "$out/controls/decomp-f6-n$n-r$random-$round-$side" 1 decomp-f6 --n "$n" --random "$random"
      done
      for arm in roots-gray solve-f4 decomp-f4 solve-f5 decomp-f5 solve-inherited decomp-f6 generalized-native decomp-generalized roots-moebius rootfind-mitm roots-monica rootfind-table generic-gb decomp-f6-pairs; do
        sample "$out/comparison/$arm-n$n-r$random-$round" 1 "$arm" --n "$n" --random "$random"
      done
    done
  done
done
# Existing n19 admission refusals are retained without treating them as timings.
for arm in matrix-f4 matrix-f5; do
  for state in 0 1; do
    sample "$out/matrix/$arm-n19-state$state" "$state" "$arm" --n 19
  done
done
rustc --edition 2021 -O research/f4_f5_boolean_perf_20261018/runtime.rs \
  -L dependency=target/release/deps --extern serde_json="$(find target/release/deps -name 'libserde_json-*.rlib' | head -1)" -o "$out/matrix-analyzer"
"$out/matrix-analyzer" "$out/matrix" > "$out/matrix-qualification.json"
printf '%s\n' complete > "$out/native-phase.status"
