#!/usr/bin/env bash
# Thin capture/run wrapper; matrix reconstruction/solve/analysis are Rust.
set -u -o pipefail
umask 077
[[ $# == 1 ]] || { echo 'usage: run-rref-counter-local.sh NEW-RUN-DIR' >&2; exit 2; }
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
repo=$(/usr/bin/git -C "$here" rev-parse --show-toplevel)
cd "$repo" || exit 2
if ps -eo stat=,comm= | awk '$2 ~ /^(rustc|cargo|clippy-driver)$/ && $1 !~ /^Z/ { active=1 } END { exit !active }'; then echo 'Finish builds before counter validation.' >&2; exit 2; fi
[[ -z $(/usr/bin/git status --porcelain --untracked-files=normal) ]] || { echo 'Publish clean source/spec first.' >&2; exit 2; }
plan=$here/rref-counter-replay-plan.json
sha256sum -c "$here/$(jq -er .component_hashes "$plan")" || exit 2
sha256sum -c "$here/validation/rref-counter-replay-build.sha256" || exit 2
input=$(jq -er .input_path "$plan") || exit 2
expected=$(jq -er .input_sha256 "$plan") || exit 2
actual=$(sha256sum "$input" | cut -d ' ' -f1)
[[ $actual == "$expected" ]] || { echo 'Original matrix input hash mismatch.' >&2; exit 2; }
cpu=$(awk '/^Cpus_allowed_list:/{split($2,a,/[,-]/); print a[1]}' /proc/self/status)
[[ $cpu == 0 ]] || exit 2
mkdir "$1" || exit 2
run=$(cd "$1" && pwd)
cp "$input" "$run/input.raw.json" || exit 2
cp target/release/examples/jv_relation_counter_replay "$run/driver" || exit 2
cp "$here/native_analysis/target/release/rref-counter-analysis" "$run/analyzer" || exit 2
cp "$here/run-rref-counter-local.sh" "$run/wrapper" || exit 2
cp .resume/RESUME.md "$run/RESUME-before.md" || exit 2
source=$(/usr/bin/git rev-parse HEAD)
jq --arg source "$source" --arg driver "$(sha256sum "$run/driver" | cut -d ' ' -f1)" --arg analyzer "$(sha256sum "$run/analyzer" | cut -d ' ' -f1)" --arg cpu "$cpu" '. + {source_revision:$source,binary_sha256:$driver,analyzer_sha256:$analyzer,pinned_cpu:$cpu}' "$plan" > "$run/runtime-plan.json" || exit 2
/usr/bin/git ls-files -z -- Cargo.toml Cargo.lock src/lib.rs src/cryptanalysis/jv\* src/cryptanalysis/gaudry\* src/cryptanalysis/f4_fp\* src/cryptanalysis/large_prime_filter\* src/cryptanalysis/residual_walk.rs src/cryptanalysis/bitops.rs src/cryptanalysis/ic_measurement.rs src/prime_hyperelliptic examples/jv_relation_counter_replay.rs research/extension_ic_graph_20261115 | tar --null -T - -cf "$run/source-checkpoint.tar" || exit 2
uname -a > "$run/os.txt"
lscpu -J > "$run/cpu.json"
{ cat /sys/fs/cgroup/cpu.max; cat /sys/fs/cgroup/memory.max; rg '^Cpus_allowed_list:' /proc/self/status; } > "$run/cgroup.txt"
"$HOME/.cargo/bin/rustc" -Vv > "$run/rustc.txt"
argv=("$run/driver" --input "$run/input.raw.json" --out "$run/raw.json" --source-revision "$source")
jq -n --args '$ARGS.positional' -- "${argv[@]}" > "$run/argv.json" || exit 2
sha256sum "$run/input.raw.json" "$run/driver" "$run/analyzer" "$run/wrapper" "$run/runtime-plan.json" "$run/argv.json" "$run/source-checkpoint.tar" "$run/RESUME-before.md" "$run/os.txt" "$run/cpu.json" "$run/cgroup.txt" "$run/rustc.txt" > "$run/pre-call.sha256" || exit 2
date -u +%FT%TZ > "$run/started-at.txt"
cp /sys/fs/cgroup/cpu.stat "$run/cpu-stat-before.txt"
jq -cn --arg source "$source" '{record_type:"execution_start",source_revision:$source,kind:"recorded_matrix_replay"}' > "$run/flow.jsonl"
ulimit -v "$(( $(jq -er .memory_gib "$plan") * 1024 * 1024 ))" || exit 2
export RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1
TIMEFORMAT=$'real_s=%R\nuser_s=%U\nsys_s=%S\ncpu_percent=%P'
{ time timeout --signal=TERM --kill-after=5s "$(jq -er .timeout_seconds "$plan")s" taskset -c "$cpu" "${argv[@]}" > "$run/stdout.txt" 2> "$run/stderr.txt"; } 2> "$run/time.txt"
status=$?
printf '%s\n' "$status" > "$run/exit-code.txt"
jq -cn --arg source "$source" --argjson status "$status" '{record_type:"execution",source_revision:$source,kind:"recorded_matrix_replay",exit_code:$status}' >> "$run/flow.jsonl"
date -u +%FT%TZ > "$run/finished-at.txt"
cp /sys/fs/cgroup/cpu.stat "$run/cpu-stat-after.txt"
sha256sum "$run/exit-code.txt" "$run/time.txt" "$run/stdout.txt" "$run/stderr.txt" "$run/flow.jsonl" "$run/cpu-stat-before.txt" "$run/cpu-stat-after.txt" > "$run/post-call.sha256"
[[ ! -f "$run/raw.json" ]] || sha256sum "$run/raw.json" >> "$run/post-call.sha256"
echo "Recorded counter replay exited $status; originals retained at $run"
exit "$status"
