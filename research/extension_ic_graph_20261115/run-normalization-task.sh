#!/usr/bin/env bash
# Thin capture only; matrix work, checks and result generation are Rust.
set -u -o pipefail
umask 077
if [[ $# == 2 && $1 == --execute ]]; then
  run=$2
  plan=$run/runtime-plan.json
  ulimit -v "$(( $(jq -er .memory_gib "$plan") * 1024 * 1024 ))" || exit 2
  source=$(jq -er .source_revision "$plan") || exit 2
  cpu=$(jq -er .pinned_cpu "$plan") || exit 2
  argv=("$run/driver" "$run/input.raw.json" "$run/raw.json" "$source")
  jq -n --args '$ARGS.positional' -- "${argv[@]}" > "$run/replay.argv.json" || exit 2
  [[ ! -e $run/pre-workers.sha256 ]] || exit 2
  (cd "$run" && sha256sum input.raw.json replay.argv.json) > "$run/pre-workers.sha256" || exit 2
  jq -cn --arg source "$source" '{record_type:"execution_start",mode:"normalization-replay",source_revision:$source}' >> "$run/flow.jsonl"
  TIMEFORMAT=$'real_s=%R\nuser_s=%U\nsys_s=%S\ncpu_percent=%P'
  { time timeout --signal=TERM --kill-after=5s "$(jq -er .worker_timeout_seconds "$plan")s" taskset -c "$cpu" "${argv[@]}" > "$run/replay.summary.jsonl" 2> "$run/replay.stderr"; } 2> "$run/replay.time.txt"
  status=$?
  printf '%s\n' "$status" > "$run/replay.exit-code.txt"
  jq -cn --arg source "$source" --argjson status "$status" '{record_type:"execution",mode:"normalization-replay",source_revision:$source,exit_code:$status}' >> "$run/flow.jsonl"
  exit "$status"
fi
[[ $# == 0 ]] || exit 2
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
cd "$here" || exit 2
command -v ps >/dev/null || exit 2
if ps -eo stat=,comm= | awk '$2 ~ /^(rustc|cargo|clippy-driver)$/ && $1 !~ /^Z/ { active=1 } END { exit !active }'; then echo 'Finish builds before replay.' >&2; exit 2; fi
sha256sum -c bundle.sha256 > bundle-recheck.log || exit 2
plan=$here/normalization-plan.json
remaining=$(( $(jq -er .deadline_epoch allocation.json) - $(date -u +%s) ))
[[ $remaining -ge 300 ]] || { echo 'Insufficient remaining runner lifetime.' >&2; exit 2; }
cpu=$(awk '/^Cpus_allowed_list:/{split($2,a,/[,-]/); print a[1]}' /proc/self/status)
mkdir run || exit 2
run=$here/run
cp driver analyzer input.raw.json source-checkpoint.tar RESUME-before.md build-rustc.txt allocation.json bundle.sha256 bundle-recheck.log "$run/" || exit 2
cp run-task.sh "$run/wrapper" || exit 2
source=$(cat source-revision.txt)
jq --arg source "$source" --arg cpu "$cpu" --slurpfile allocation allocation.json '.+{source_revision:$source,pinned_cpu:$cpu,runpod_allocation:$allocation[0]}' "$plan" > "$run/runtime-plan.json" || exit 2
uname -a > "$run/os.txt"
lscpu -J > "$run/cpu.json" || exit 2
{ for file in /sys/fs/cgroup/cpu.max /sys/fs/cgroup/memory.max; do [[ ! -f $file ]] || cat "$file"; done; awk '/^Cpus_allowed_list:/' /proc/self/status; } > "$run/cgroup.txt"
cp /opt/ic-run/bootstrap-rustc.txt "$run/bootstrap-rustc.txt" || exit 2
ldd "$run/driver" > "$run/driver.ldd.txt" || exit 2
awk '/not found/{bad=1}END{exit bad}' "$run/driver.ldd.txt" || exit 2
(cd "$run" && sha256sum driver analyzer input.raw.json wrapper runtime-plan.json source-checkpoint.tar RESUME-before.md os.txt cpu.json cgroup.txt build-rustc.txt bootstrap-rustc.txt allocation.json driver.ldd.txt bundle.sha256) > "$run/pre-call.sha256" || exit 2
date -u +%FT%TZ > "$run/started-at.txt"
TIMEFORMAT=$'real_s=%R\nuser_s=%U\nsys_s=%S\ncpu_percent=%P'
{ time timeout --signal=TERM --kill-after=5s "$(jq -er .timeout_seconds "$run/runtime-plan.json")s" bash "$run/wrapper" --execute "$run"; } > "$run/stdout.txt" 2> "$run/time.txt"
status=$?
printf '%s\n' "$status" > "$run/exit-code.txt"
date -u +%FT%TZ > "$run/finished-at.txt"
(cd "$run" && sha256sum exit-code.txt started-at.txt finished-at.txt time.txt) > "$run/post-call.sha256"
for file in "$run/flow.jsonl" "$run/pre-workers.sha256" "$run/raw.json" "$run"/*.summary.jsonl "$run"/*.exit-code.txt "$run"/*.stderr "$run"/*.time.txt "$run"/*.argv.json; do [[ ! -f $file ]] || (cd "$run" && sha256sum "$(basename "$file")") >> "$run/post-call.sha256"; done
[[ $status == 0 ]] || exit "$status"
"$run/analyzer" "$run" "$run/summary.json" > "$run/analysis-original.log" 2>&1
analysis=$?
printf '%s\n' "$analysis" > "$run/analysis-original.exit-code.txt"
(cd "$run" && sha256sum -c pre-call.sha256) > "$run/pre-call-recheck.log" || exit 2
(cd "$run" && sha256sum -c pre-workers.sha256) > "$run/pre-workers-recheck.log" || exit 2
(cd "$run" && sha256sum -c post-call.sha256) > "$run/post-call-recheck.log" || exit 2
if [[ $analysis == 0 ]]; then (cd "$run" && sha256sum summary.json raw-export.json analysis-original.log analysis-original.exit-code.txt) > "$run/qualified-output.sha256"; fi
echo "Recorded replay exited $status; copied qualifier exited $analysis."
exit "$analysis"
