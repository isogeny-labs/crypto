#!/usr/bin/env bash
# Thin capture/run only. Relation algorithms, replay, rank and analysis are Rust.
set -u -o pipefail
umask 077
if [[ $# == 2 && $1 == --execute ]]; then
  run=$2
  plan=$run/runtime-plan.json
  cpu=$(jq -er .pinned_cpu "$plan") || exit 2
  limit=$(jq -er .worker_timeout_seconds "$plan") || exit 2
  ulimit -v "$(( $(jq -er .memory_gib "$plan") * 1024 * 1024 ))" || exit 2
  export RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1
  for mode in smooth two-lp; do
    jq -cn --arg mode "$mode" --arg source "$(jq -er .source_revision "$plan")" '{record_type:"execution_start",mode:$mode,source_revision:$source}' >> "$run/flow.jsonl"
    argv=("$run/driver" --mode "$mode" --p "$(jq -er .p "$plan")" --seed "$(jq -er .seed "$plan")" \
      --small-percent "$(jq -er .small_percent "$plan")" --b-count "$(jq -er .b_count "$plan")" \
      --source-revision "$(jq -er .source_revision "$plan")" --out "$run/$mode.raw.json")
    jq -n --args '$ARGS.positional' -- "${argv[@]}" > "$run/$mode.argv.json" || exit 2
    TIMEFORMAT=$'real_s=%R\nuser_s=%U\nsys_s=%S\ncpu_percent=%P'
    { time timeout --signal=TERM --kill-after=5s "${limit}s" taskset -c "$cpu" "${argv[@]}" \
      > "$run/$mode.summary.jsonl" 2> "$run/$mode.stderr"; } 2> "$run/$mode.time.txt"
    status=$?
    printf '%s\n' "$status" > "$run/$mode.exit-code.txt"
    jq -cn --arg mode "$mode" --argjson status "$status" --arg source "$(jq -er .source_revision "$plan")" '{record_type:"execution",mode:$mode,exit_code:$status,source_revision:$source}' >> "$run/flow.jsonl"
    [[ $status == 0 ]] || exit "$status"
  done
  exit 0
fi
[[ $# == 1 ]] || { echo 'usage: run-two-lp-local.sh NEW-RUN-DIR' >&2; exit 2; }
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
repo=$(/usr/bin/git -C "$here" rev-parse --show-toplevel)
cd "$repo" || exit 2
if ps -eo stat=,comm= | awk '$2 ~ /^(rustc|cargo|clippy-driver)$/ && $1 !~ /^Z/ { active=1 } END { exit !active }'; then
  echo 'Finish builds before this diagnostic.' >&2; exit 2
fi
[[ -z $(/usr/bin/git status --porcelain --untracked-files=normal) ]] || { echo 'Publish clean source/spec first.' >&2; exit 2; }
plan=$here/cover-two-lp-component-plan.json
sha256sum -c "$here/$(jq -er .component_hashes "$plan")" || exit 2
sha256sum -c "$here/validation/cover-two-lp-component-build.sha256" || exit 2
mkdir "$1" || exit 2
run=$(cd "$1" && pwd)
cp target/release/examples/jv_sieve_two_large_prime_probe "$run/driver" || exit 2
cp "$here/native_analysis/target/release/two-lp-component-analysis" "$run/analyzer" || exit 2
cp "${BASH_SOURCE[0]}" "$run/wrapper" || exit 2
cp .resume/RESUME.md "$run/RESUME-before.md" || exit 2
cpu=$(awk '/^Cpus_allowed_list:/{split($2,a,/[,-]/); print a[1]}' /proc/self/status)
jq --arg source "$(/usr/bin/git rev-parse HEAD)" --arg binary "$(sha256sum "$run/driver" | cut -d ' ' -f1)" --arg analyzer "$(sha256sum "$run/analyzer" | cut -d ' ' -f1)" --arg cpu "$cpu" '. + {source_revision:$source,binary_sha256:$binary,analyzer_sha256:$analyzer,pinned_cpu:$cpu}' "$plan" > "$run/runtime-plan.json" || exit 2
/usr/bin/git ls-files -z -- Cargo.toml Cargo.lock src/lib.rs src/cryptanalysis/jv\* src/cryptanalysis/gaudry\* src/cryptanalysis/f4_fp\* src/cryptanalysis/large_prime_filter\* src/cryptanalysis/residual_walk.rs src/cryptanalysis/bitops.rs src/cryptanalysis/ic_measurement.rs src/prime_hyperelliptic examples/jv_sieve_two_large_prime_probe.rs research/extension_ic_graph_20261115 | tar --null -T - -cf "$run/source-checkpoint.tar" || exit 2
uname -a > "$run/os.txt"
lscpu -J > "$run/cpu.json"
{ cat /sys/fs/cgroup/cpu.max; cat /sys/fs/cgroup/memory.max; rg '^Cpus_allowed_list:' /proc/self/status; } > "$run/cgroup.txt"
"$HOME/.cargo/bin/rustc" -Vv > "$run/rustc.txt"
sha256sum "$run/driver" "$run/analyzer" "$run/wrapper" "$run/runtime-plan.json" "$run/source-checkpoint.tar" "$run/RESUME-before.md" "$run/os.txt" "$run/cpu.json" "$run/cgroup.txt" "$run/rustc.txt" > "$run/pre-call.sha256" || exit 2
date -u +%FT%TZ > "$run/started-at.txt"
cp /sys/fs/cgroup/cpu.stat "$run/cpu-stat-before.txt"
TIMEFORMAT=$'real_s=%R\nuser_s=%U\nsys_s=%S\ncpu_percent=%P'
{ time timeout --signal=TERM --kill-after=5s "$(jq -er .timeout_seconds "$run/runtime-plan.json")s" bash "$run/wrapper" --execute "$run"; } > "$run/stdout.txt" 2> "$run/time.txt"
status=$?
printf '%s\n' "$status" > "$run/exit-code.txt"
date -u +%FT%TZ > "$run/finished-at.txt"
cp /sys/fs/cgroup/cpu.stat "$run/cpu-stat-after.txt"
sha256sum "$run/exit-code.txt" "$run/started-at.txt" "$run/finished-at.txt" "$run/time.txt" "$run/cpu-stat-before.txt" "$run/cpu-stat-after.txt" > "$run/post-call.sha256"
for file in "$run/flow.jsonl" "$run"/*.raw.json "$run"/*.summary.jsonl "$run"/*.exit-code.txt "$run"/*.stderr "$run"/*.time.txt "$run"/*.argv.json; do
  [[ ! -f $file ]] || sha256sum "$file" >> "$run/post-call.sha256"
done
echo "Local component exited $status; originals retained at $run"
exit "$status"
