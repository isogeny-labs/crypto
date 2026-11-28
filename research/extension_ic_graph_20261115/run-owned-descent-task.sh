#!/usr/bin/env bash
# Thin transport only. Arithmetic, projection and qualification are native Rust.
set -u -o pipefail
umask 077
if [[ $# == 2 && $1 == --execute ]]; then
  run=$2
  plan=$run/runtime-plan.json
  cpu=$(jq -er .pinned_cpu "$plan") || exit 2
  export IC_SOURCE_REVISION
  IC_SOURCE_REVISION=$(jq -er .source_revision "$plan") || exit 2
  ulimit -v "$(( $(jq -er .memory_gib "$plan") * 1024 * 1024 ))" || exit 2
  export RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1
  export F4_FP_THREAD_OPS=0 F4_STAIR=counter F4_FP_NARROW=2 F4_FP_SYMBOLIC_PAR=1
  export F4_FP_PACKED=1 F4_FP_GM=1 F4_FP_DEFERRED_PAR=1 F4_FP_FL=1
  export F4_FP_FL_BLOCKED=1 F4_FP_LINEAR_INTERREDUCE=1
  unset F4_DEBUG F4_FP_LAZY_SIMD F4_FP_LAZY_AVX512 F4_FP_LAZY_AVX2
  unset GAUDRY_DEBUG GAUDRY_DEBUG_SOLVE GAUDRY_LAZY_ELIM GAUDRY_LAZY_VERIFY GAUDRY_BORDER_RETRY
  for key in $(compgen -e | awk '/^JV_/'); do unset "$key"; done
  [[ ! -e $run/pre-workers.sha256 && ! -e $run/flow.jsonl ]] || exit 2
  mapfile -t modes < <(jq -er '.mode_order[]' "$plan")
  [[ ${modes[*]} == 'original-nagao full-only anchored' ]] || exit 2
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
    # Every arm is attempted and every failure retained. No automatic retry.
  done
  exit 0
fi
[[ $# == 0 ]] || { echo 'usage: run-task.sh inside sealed Runpod bundle' >&2; exit 2; }
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
cd "$here" || exit 2
command -v ps >/dev/null || exit 2
if ps -eo stat=,comm= | awk '$2 ~ /^(rustc|cargo|clippy-driver)$/ && $1 !~ /^Z/ {active=1} END{exit !active}'; then echo 'Finish builds before measurement.' >&2; exit 2; fi
sha256sum -c bundle.sha256 > bundle-recheck.log || exit 2
[[ $(jq -er .backend anchored-descent-plan.json) == runpod_user_authorized ]] || exit 2
remaining=$(( $(jq -er .deadline_epoch allocation.json) - $(date -u +%s) ))
[[ $remaining -ge 4500 ]] || { echo 'Insufficient runner lifetime; no measurement started.' >&2; exit 2; }
cpu=$(awk '/^Cpus_allowed_list:/{split($2,a,/[,-]/);print a[1]}' /proc/self/status)
mkdir run || exit 2
run=$here/run
cp driver analyzer input.json source-checkpoint.tar RESUME-before.md build-rustc.txt allocation.json bundle.sha256 bundle-recheck.log "$run/" || exit 2
cp run-task.sh "$run/wrapper" || exit 2
source=$(cat source-revision.txt)
jq --arg source "$source" --arg driver "$(sha256sum driver | cut -d ' ' -f1)" --arg analyzer "$(sha256sum analyzer | cut -d ' ' -f1)" --arg cpu "$cpu" --slurpfile allocation allocation.json '. + {source_revision:$source,binary_sha256:$driver,analyzer_sha256:$analyzer,pinned_cpu:$cpu,runpod_allocation:$allocation[0]}' anchored-descent-plan.json > "$run/runtime-plan.json" || exit 2
[[ $(sha256sum input.json | cut -d ' ' -f1) == $(jq -er .input_sha256 "$run/runtime-plan.json") ]] || exit 2
uname -a > "$run/os.txt"
lscpu -J > "$run/cpu.json" || exit 2
{ for file in /sys/fs/cgroup/cpu.max /sys/fs/cgroup/memory.max; do [[ ! -f $file ]] || cat "$file"; done; awk '/^Cpus_allowed_list:/' /proc/self/status; } > "$run/cgroup.txt"
cp /opt/ic-run/bootstrap-rustc.txt "$run/bootstrap-rustc.txt" || exit 2
ldd "$run/driver" > "$run/driver.ldd.txt" || exit 2
awk '/not found/{bad=1}END{exit bad}' "$run/driver.ldd.txt" || exit 2
(cd "$run" && sha256sum driver analyzer input.json wrapper runtime-plan.json source-checkpoint.tar RESUME-before.md build-rustc.txt bootstrap-rustc.txt allocation.json bundle.sha256 os.txt cpu.json cgroup.txt driver.ldd.txt) > "$run/pre-call.sha256" || exit 2
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
cp repeat-check "$run/repeat-check" || exit 2
mkdir "$run/prior42" || exit 2
cp prior42/*.raw.json "$run/prior42/" || exit 2
(cd "$run" && sha256sum repeat-check prior42/*.raw.json) > "$run/qualifier-inputs.sha256" || exit 2
timeout --signal=TERM --kill-after=5s "$(jq -er .qualifier_timeout_seconds "$run/runtime-plan.json")s" env -u IC_TASK_TOKEN "$run/analyzer" "$run" "$run/summary.json" > "$run/analysis-original.log" 2>&1
analysis=$?
printf '%s\n' "$analysis" > "$run/analysis-original.exit-code.txt"
(cd "$run" && sha256sum -c pre-call.sha256) > "$run/pre-call-recheck.log" || exit 2
(cd "$run" && sha256sum -c pre-workers.sha256) > "$run/pre-workers-recheck.log" || exit 2
(cd "$run" && sha256sum -c post-call.sha256) > "$run/post-call-recheck.log" || exit 2
if [[ $analysis == 0 ]]; then
  timeout --signal=TERM --kill-after=5s 60s env -u IC_TASK_TOKEN "$run/repeat-check" "$run/prior42" "$run" "$run/repetition.json" > "$run/repetition.log" 2>&1
  repetition=$?
  printf '%s\n' "$repetition" > "$run/repetition.exit-code.txt"
  if [[ $repetition != 0 ]]; then echo "New capture qualified; geometry repetition checker exited $repetition."; exit "$repetition"; fi
  (cd "$run" && sha256sum -c qualifier-inputs.sha256) > "$run/qualifier-inputs-recheck.log" || exit 2
  (cd "$run" && sha256sum summary.json repetition.json analysis-original.log analysis-original.exit-code.txt repetition.log repetition.exit-code.txt qualifier-inputs.sha256) > "$run/qualified-output.sha256"
fi
echo "Measured wrapper $status; original qualifier $analysis."
exit "$analysis"
