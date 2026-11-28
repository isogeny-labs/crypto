#!/usr/bin/env bash
set -euo pipefail
cd -- "$(dirname -- "$0")"
task_rustc="${RUSTC:-rustc}"
task_output="${1:-replay-output}"
task_cpu="${TASK_CPU:-$(awk '/Cpus_allowed_list/{split($2,a,/[-,]/); print a[1]}' /proc/self/status)}"
if [[ -e "$task_output" ]]; then
  echo "Refusing to overwrite an existing validation directory: $task_output" >&2
  exit 2
fi
mkdir -p "$task_output/source/upstream"
task_sources=(main.rs transport.rs independent_reference.rs PROTOCOL.md SEEDS.json upstream/arithmetic.rs upstream/original_chord.rs upstream/original_reference.rs upstream/SOURCE.json)
for task_source in "${task_sources[@]}"; do
  cp -- "$task_source" "$task_output/source/$task_source"
  chmod a-w -- "$task_output/source/$task_source"
done
(
  cd -- "$task_output/source"
  sha256sum -- "${task_sources[@]}" > ../prebuild.sha256
)
"$task_rustc" --version --verbose > "$task_output/compiler.txt"
uname -sm > "$task_output/platform.txt"
printf '%s\n' "$task_cpu" > "$task_output/cpu.txt"
(
  ulimit -v 1048576
  taskset -c "$task_cpu" timeout 120 "$task_rustc" --edition=2021 -O -Dwarnings \
    -Ccodegen-units=1 -Clink-arg=-Wl,--threads=1 \
    "$task_output/source/main.rs" -o "$task_output/checker"
) > "$task_output/build.stdout" 2> "$task_output/build.stderr"
chmod a-w -- "$task_output/checker"
for task_run in 1 2; do
  (
    cd -- "$task_output/source"
    sha256sum -- "${task_sources[@]}"
    sha256sum ../checker
  ) > "$task_output/precall-$task_run.sha256"
  (
    ulimit -v 524288
    RUST_BACKTRACE=1 taskset -c "$task_cpu" timeout 60 \
      "$task_output/checker" "$task_output/run$task_run"
  ) > "$task_output/summary-$task_run.json" 2> "$task_output/run$task_run.stderr"
  (
    cd -- "$task_output/source"
    sha256sum -- "${task_sources[@]}"
    sha256sum ../checker
  ) > "$task_output/postcall-$task_run.sha256"
  cmp -- "$task_output/precall-$task_run.sha256" "$task_output/postcall-$task_run.sha256"
done
for task_file in cases.jsonl intersections.jsonl prefixes.jsonl seeds.jsonl; do
  cmp -- "$task_output/run1/$task_file" "$task_output/run2/$task_file"
done
cmp -- "$task_output/summary-1.json" "$task_output/summary-2.json"
cat -- "$task_output/summary-1.json"
