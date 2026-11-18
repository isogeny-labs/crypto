#!/usr/bin/env bash
# Thin bounded transport; all scientific work and qualification are Rust.
set -u -o pipefail
umask 077
if [[ $# == 2 && $1 == --execute ]]; then
  run=$2
  plan=$run/runtime-plan.json
  cpu=$(jq -er .pinned_cpu "$plan") || exit 2
  export IC_SOURCE_REVISION
  IC_SOURCE_REVISION=$(jq -er .source_revision "$plan") || exit 2
  export RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1
  ulimit -v "$(( $(jq -er .memory_gib "$plan") * 1024 * 1024 ))" || exit 2
  [[ ! -e $run/pre-workers.sha256 && ! -e $run/flow.jsonl ]] || exit 2
  mapfile -t modes < <(jq -er '.mode_order[]' "$plan")
  [[ ${modes[*]} == 'counted-rref library-wiedemann randomized-lanczos' ]] || exit 2
  for mode in "${modes[@]}"; do
    [[ ! -e $run/$mode.raw.json && ! -e $run/$mode.exit-code.txt ]] || exit 2
    argv=("$run/driver" "$mode" "$run/input.json" "$run/$mode.raw.json")
    jq -n --args '$ARGS.positional' -- "${argv[@]}" > "$run/$mode.argv.json" || exit 2
  done
  (cd "$run" && sha256sum input.json runtime-plan.json ./*.argv.json) > "$run/pre-workers.sha256" || exit 2
  for mode in "${modes[@]}"; do
    mapfile -t argv < <(jq -er '.[]' "$run/$mode.argv.json")
    jq -cn --arg mode "$mode" '{record_type:"execution_start",mode:$mode}' >> "$run/flow.jsonl"
    TIMEFORMAT=$'real_s=%R\nuser_s=%U\nsys_s=%S\ncpu_percent=%P'
    { time timeout --signal=TERM --kill-after=5s "$(jq -er .worker_timeout_seconds "$plan")s" taskset -c "$cpu" env -u IC_TASK_TOKEN "${argv[@]}" > "$run/$mode.summary.jsonl" 2> "$run/$mode.stderr"; } 2> "$run/$mode.time.txt"
    status=$?
    printf '%s\n' "$status" > "$run/$mode.exit-code.txt"
    jq -cn --arg mode "$mode" --argjson status "$status" '{record_type:"execution",mode:$mode,exit_code:$status}' >> "$run/flow.jsonl"
    # All arms attempted, including after a prior failure; no retry.
  done
  exit 0
fi
[[ $# == 0 ]] || exit 2
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
cd "$here" || exit 2
if ps -eo stat=,comm= | awk '$2 ~ /^(rustc|cargo|clippy-driver)$/ && $1 !~ /^Z/ {active=1} END{exit !active}'; then echo 'Finish builds before measurement.' >&2; exit 2; fi
sha256sum -c bundle.sha256 > bundle-recheck.log || exit 2
[[ $(jq -er .backend sparse-kernel-plan.json) == runpod_user_authorized ]] || exit 2
remaining=$(( $(jq -er .deadline_epoch allocation.json) - $(date -u +%s) ))
[[ $remaining -ge 4500 ]] || exit 2
cpu=$(awk '/^Cpus_allowed_list:/{split($2,a,/[,-]/);print a[1]}' /proc/self/status)
mkdir run || exit 2
run=$here/run
cp driver analyzer input.json source-checkpoint.tar frozen-root-source.tar frozen-root-source.sha256 source-revision.txt build-rustc.txt allocation.json bundle.sha256 bundle-recheck.log "$run/" || exit 2
cp run-task.sh "$run/wrapper" || exit 2
source=$(cat source-revision.txt)
jq --arg source "$source" --arg driver "$(sha256sum driver | cut -d ' ' -f1)" --arg analyzer "$(sha256sum analyzer | cut -d ' ' -f1)" --arg cpu "$cpu" --slurpfile allocation allocation.json '. + {source_revision:$source,binary_sha256:$driver,analyzer_sha256:$analyzer,pinned_cpu:$cpu,runpod_allocation:$allocation[0]}' sparse-kernel-plan.json > "$run/runtime-plan.json" || exit 2
[[ $(sha256sum input.json | cut -d ' ' -f1) == $(jq -er .input_sha256 "$run/runtime-plan.json") ]] || exit 2
uname -a > "$run/os.txt"
lscpu -J > "$run/cpu.json" || exit 2
{ for file in /sys/fs/cgroup/cpu.max /sys/fs/cgroup/memory.max; do [[ ! -f $file ]] || cat "$file"; done; awk '/^Cpus_allowed_list:/' /proc/self/status; } > "$run/cgroup.txt"
cp /opt/ic-run/bootstrap-rustc.txt "$run/bootstrap-rustc.txt" || exit 2
ldd "$run/driver" > "$run/driver.ldd.txt" || exit 2
awk '/not found/{bad=1}END{exit bad}' "$run/driver.ldd.txt" || exit 2
(cd "$run" && sha256sum driver analyzer input.json wrapper runtime-plan.json source-revision.txt source-checkpoint.tar frozen-root-source.tar frozen-root-source.sha256 build-rustc.txt bootstrap-rustc.txt allocation.json bundle.sha256 os.txt cpu.json cgroup.txt driver.ldd.txt) > "$run/pre-call.sha256" || exit 2
date -u +%FT%TZ > "$run/started-at.txt"
cp /sys/fs/cgroup/cpu.stat "$run/cpu-stat-before.txt" || exit 2
TIMEFORMAT=$'real_s=%R\nuser_s=%U\nsys_s=%S\ncpu_percent=%P'
{ time timeout --signal=TERM --kill-after=5s "$(jq -er .timeout_seconds "$run/runtime-plan.json")s" bash "$run/wrapper" --execute "$run"; } > "$run/stdout.txt" 2> "$run/time.txt"
status=$?
printf '%s\n' "$status" > "$run/exit-code.txt"
date -u +%FT%TZ > "$run/finished-at.txt"
cp /sys/fs/cgroup/cpu.stat "$run/cpu-stat-after.txt" || exit 2
(cd "$run" && sha256sum exit-code.txt started-at.txt finished-at.txt time.txt cpu-stat-before.txt cpu-stat-after.txt) > "$run/post-call.sha256"
for file in "$run/flow.jsonl" "$run/pre-workers.sha256" "$run"/*.raw.json "$run"/*.summary.jsonl "$run"/*.exit-code.txt "$run"/*.stderr "$run"/*.time.txt "$run"/*.argv.json; do [[ ! -f $file ]] || (cd "$run" && sha256sum "$(basename "$file")") >> "$run/post-call.sha256"; done
[[ $status == 0 ]] || { echo "Measurement wrapper exited $status; originals retained."; exit "$status"; }
timeout --signal=TERM --kill-after=5s "$(jq -er .qualifier_timeout_seconds "$run/runtime-plan.json")s" env -u IC_TASK_TOKEN "$run/analyzer" "$run" "$run/summary.json" > "$run/analysis-original.log" 2>&1
analysis=$?
printf '%s\n' "$analysis" > "$run/analysis-original.exit-code.txt"
for manifest in pre-call pre-workers post-call; do (cd "$run" && sha256sum -c "$manifest.sha256") > "$run/$manifest-recheck.log" || exit 2; done
if [[ $analysis == 0 ]]; then (cd "$run" && sha256sum summary.json analysis-original.log analysis-original.exit-code.txt) > "$run/qualified-output.sha256" || exit 2; fi
echo "Measured wrapper $status; original qualifier $analysis."
exit "$analysis"
