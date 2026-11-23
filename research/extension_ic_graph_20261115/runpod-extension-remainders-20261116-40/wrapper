#!/usr/bin/env bash
# Capture only. Algorithms, qualification and scientific analysis are Rust.
set -u -o pipefail
umask 077
if [[ $# == 2 && $1 == --execute ]]; then
  run=$2
  plan=$run/runtime-plan.json
  cpu=$(jq -er .pinned_cpu "$plan") || exit 2
  source=$(jq -er .source_revision "$plan") || exit 2
  ulimit -v "$(( $(jq -er .memory_gib "$plan") * 1024 * 1024 ))" || exit 2
  export RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1
  mapfile -t modes < <(jq -er '.mode_order[]' "$plan")
  [[ ${#modes[@]} == 2 && ${modes[0]} == full-smooth && ${modes[1]} == quadratic-remainders ]] || exit 2
  for mode in "${modes[@]}"; do
    argv=("$run/driver" --input "$run/public.json" --mode "$mode" --out "$run/$mode.raw.json" --source-revision "$source")
    jq -n --args '$ARGS.positional' -- "${argv[@]}" > "$run/$mode.argv.json" || exit 2
  done
  (cd "$run" && sha256sum public.json ./*.argv.json) > "$run/pre-workers.sha256" || exit 2
  for mode in "${modes[@]}"; do
    mapfile -t argv < <(jq -er '.[]' "$run/$mode.argv.json")
    jq -cn --arg source "$source" --arg mode "$mode" '{record_type:"execution_start",mode:$mode,source_revision:$source}' >> "$run/flow.jsonl"
    TIMEFORMAT=$'real_s=%R\nuser_s=%U\nsys_s=%S\ncpu_percent=%P'
    { time timeout --signal=TERM --kill-after=5s "$(jq -er .worker_timeout_seconds "$plan")s" taskset -c "$cpu" "${argv[@]}" > "$run/$mode.stdout.txt" 2> "$run/$mode.stderr"; } 2> "$run/$mode.time.txt"
    status=$?
    printf '%s\n' "$status" > "$run/$mode.exit-code.txt"
    jq -cn --arg source "$source" --arg mode "$mode" --argjson status "$status" '{record_type:"execution",mode:$mode,source_revision:$source,exit_code:$status}' >> "$run/flow.jsonl"
  done
  exit 0
fi
[[ $# == 0 ]] || exit 2
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
cd "$here" || exit 2
command -v ps >/dev/null || exit 2
if ps -eo stat=,comm= | awk '$2 ~ /^(rustc|cargo|clippy-driver)$/ && $1 !~ /^Z/ {active=1} END{exit !active}'; then echo 'Finish builds before timing.' >&2; exit 2; fi
sha256sum -c bundle.sha256 > bundle-recheck.log || exit 2
plan=$here/extension-remainder-plan.json
[[ $(jq -er .backend "$plan") == runpod_user_authorized ]] || exit 2
cpu=$(awk '/^Cpus_allowed_list:/{split($2,a,/[,-]/); print a[1]}' /proc/self/status)
[[ -n $cpu ]] || exit 2
deadline=$(jq -er .deadline_epoch allocation.json) || exit 2
[[ $(( deadline - $(date -u +%s) )) -ge 1640 ]] || { echo 'Insufficient remaining runner lifetime; no measurement started.' >&2; exit 2; }
mkdir run || exit 2
run=$here/run
cp driver analyzer source-checkpoint.tar RESUME-before.md build-rustc.txt allocation.json bundle.sha256 bundle-recheck.log public.json "$run/" || exit 2
cp run-task.sh "$run/wrapper" || exit 2
source=$(cat source-revision.txt)
jq --arg source "$source" --arg driver "$(sha256sum "$run/driver" | cut -d ' ' -f1)" --arg analyzer "$(sha256sum "$run/analyzer" | cut -d ' ' -f1)" --arg cpu "$cpu" --slurpfile allocation allocation.json '. + {source_revision:$source,binary_sha256:$driver,analyzer_sha256:$analyzer,pinned_cpu:$cpu,runpod_allocation:$allocation[0]}' "$plan" > "$run/runtime-plan.json" || exit 2
uname -a > "$run/os.txt"
lscpu -J > "$run/cpu.json" || exit 2
{ for file in /sys/fs/cgroup/cpu.max /sys/fs/cgroup/memory.max; do [[ ! -f $file ]] || cat "$file"; done; awk '/^Cpus_allowed_list:/' /proc/self/status; } > "$run/cgroup.txt"
cp /opt/ic-run/bootstrap-rustc.txt "$run/bootstrap-rustc.txt" || exit 2
ldd "$run/driver" > "$run/driver.ldd.txt" || exit 2
awk '/not found/{bad=1}END{exit bad}' "$run/driver.ldd.txt" || exit 2
(cd "$run" && sha256sum driver analyzer wrapper runtime-plan.json source-checkpoint.tar RESUME-before.md os.txt cpu.json cgroup.txt build-rustc.txt bootstrap-rustc.txt allocation.json driver.ldd.txt bundle.sha256 public.json) > "$run/pre-call.sha256" || exit 2
date -u +%FT%TZ > "$run/started-at.txt"
if [[ -f /sys/fs/cgroup/cpu.stat ]]; then cp /sys/fs/cgroup/cpu.stat "$run/cpu-stat-before.txt"; fi
TIMEFORMAT=$'real_s=%R\nuser_s=%U\nsys_s=%S\ncpu_percent=%P'
{ time timeout --signal=TERM --kill-after=5s "$(jq -er .timeout_seconds "$run/runtime-plan.json")s" bash "$run/wrapper" --execute "$run"; } > "$run/stdout.txt" 2> "$run/time.txt"
status=$?
printf '%s\n' "$status" > "$run/exit-code.txt"
date -u +%FT%TZ > "$run/finished-at.txt"
if [[ -f /sys/fs/cgroup/cpu.stat ]]; then cp /sys/fs/cgroup/cpu.stat "$run/cpu-stat-after.txt"; fi
(cd "$run" && sha256sum exit-code.txt started-at.txt finished-at.txt time.txt) > "$run/post-call.sha256"
for file in "$run/flow.jsonl" "$run/pre-workers.sha256" "$run"/*.raw.json "$run"/*.stdout.txt "$run"/*.exit-code.txt "$run"/*.stderr "$run"/*.time.txt "$run"/*.argv.json "$run"/cpu-stat-*.txt; do [[ ! -f $file ]] || (cd "$run" && sha256sum "$(basename "$file")") >> "$run/post-call.sha256"; done
if [[ $status != 0 ]]; then echo "Measurement exited $status; originals retained."; exit "$status"; fi
timeout --signal=TERM --kill-after=5s "$(jq -er .qualifier_timeout_seconds "$run/runtime-plan.json")s" "$run/analyzer" "$run" "$run/summary.json" > "$run/analysis-original.log" 2>&1
analysis=$?
printf '%s\n' "$analysis" > "$run/analysis-original.exit-code.txt"
(cd "$run" && sha256sum -c pre-call.sha256) > "$run/pre-call-recheck.log" || exit 2
(cd "$run" && sha256sum -c pre-workers.sha256) > "$run/pre-workers-recheck.log" || exit 2
(cd "$run" && sha256sum -c post-call.sha256) > "$run/post-call-recheck.log" || exit 2
if [[ $analysis == 0 ]]; then (cd "$run" && sha256sum summary.json raw-export.json analysis-original.log analysis-original.exit-code.txt) > "$run/qualified-output.sha256"; fi
echo "Runpod measurement exited $status; original qualifier exited $analysis."
exit "$analysis"
