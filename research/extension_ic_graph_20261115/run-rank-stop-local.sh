#!/usr/bin/env bash
# Thin local capture/run only; all cryptographic and result computation is Rust.
set -u -o pipefail
umask 077
if [[ $# == 2 && $1 == --execute ]]; then
  run=$2
  plan=$run/runtime-plan.json
  cpu=$(jq -er .pinned_cpu "$plan") || exit 2
  source=$(jq -er .source_revision "$plan") || exit 2
  ulimit -v "$(( $(jq -er .memory_gib "$plan") * 1024 * 1024 ))" || exit 2
  export RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1
  export F4_FP_THREAD_OPS=0 F4_STAIR=counter F4_FP_NARROW=2 F4_FP_SYMBOLIC_PAR=1
  export F4_FP_PACKED=1 F4_FP_GM=1 F4_FP_DEFERRED_PAR=1 F4_FP_FL=1
  export F4_FP_FL_BLOCKED=1 F4_FP_LINEAR_INTERREDUCE=1
  unset F4_DEBUG F4_FP_LAZY_SIMD F4_FP_LAZY_AVX512 F4_FP_LAZY_AVX2
  unset GAUDRY_DEBUG GAUDRY_DEBUG_SOLVE GAUDRY_LAZY_ELIM GAUDRY_LAZY_VERIFY GAUDRY_BORDER_RETRY
  argv=("$run/driver" --generate --p "$(jq -er .p "$plan")" --seed "$(jq -er .seed "$plan")" --out "$run/fixture.json" --source-revision "$source")
  jq -n --args '$ARGS.positional' -- "${argv[@]}" > "$run/fixture.argv.json" || exit 2
  jq -cn --arg source "$source" '{record_type:"fixture_start",mode:"fixture",source_revision:$source}' >> "$run/flow.jsonl"
  timeout --signal=TERM --kill-after=5s "$(jq -er .fixture_timeout_seconds "$plan")s" taskset -c "$cpu" "${argv[@]}" > "$run/fixture.summary.jsonl" 2> "$run/fixture.stderr"
  status=$?
  printf '%s\n' "$status" > "$run/fixture.exit-code.txt"
  jq -cn --arg source "$source" --argjson status "$status" '{record_type:"fixture",mode:"fixture",source_revision:$source,exit_code:$status}' >> "$run/flow.jsonl"
  [[ $status == 0 ]] || exit "$status"
  jq -e '.public' "$run/fixture.json" > "$run/public.json" || exit 2
  [[ ! -e "$run/pre-workers.sha256" ]] || exit 2
  for mode in ic two-lp rank-stop; do
    method=$mode
    extra=()
    if [[ $mode == rank-stop ]]; then method=two-lp; extra=(--collection-rank-stride "$(jq -er .collection_rank_stride "$plan")"); fi
    argv=("$run/driver" --method "$method" --fixture "$run/public.json" --seed "$(jq -er .seed "$plan")" --out "$run/$mode.raw.json" --source-revision "$source" \
      --small-percent "$(jq -er .small_percent "$plan")" --b-count "$(jq -er .b_count "$plan")" --max-descent-residuals "$(jq -er .max_descent_residuals "$plan")" --max-accepted-descents "$(jq -er .max_accepted_descents "$plan")")
    argv+=("${extra[@]}")
    jq -n --args '$ARGS.positional' -- "${argv[@]}" > "$run/$mode.argv.json" || exit 2
  done
  sha256sum "$run/fixture.json" "$run/public.json" "$run/fixture.argv.json" "$run/ic.argv.json" "$run/two-lp.argv.json" "$run/rank-stop.argv.json" > "$run/pre-workers.sha256" || exit 2
  for mode in ic two-lp rank-stop; do
    mapfile -t argv < <(jq -er '.[]' "$run/$mode.argv.json")
    jq -cn --arg mode "$mode" --arg source "$source" '{record_type:"execution_start",mode:$mode,source_revision:$source}' >> "$run/flow.jsonl"
    TIMEFORMAT=$'real_s=%R\nuser_s=%U\nsys_s=%S\ncpu_percent=%P'
    { time timeout --signal=TERM --kill-after=5s "$(jq -er .worker_timeout_seconds "$plan")s" taskset -c "$cpu" "${argv[@]}" > "$run/$mode.summary.jsonl" 2> "$run/$mode.stderr"; } 2> "$run/$mode.time.txt"
    status=$?
    printf '%s\n' "$status" > "$run/$mode.exit-code.txt"
    jq -cn --arg source "$source" --arg mode "$mode" --argjson status "$status" '{record_type:"execution",mode:$mode,source_revision:$source,exit_code:$status}' >> "$run/flow.jsonl"
    [[ $status == 0 ]] || exit "$status"
  done
  exit 0
fi
[[ $# == 1 ]] || { echo 'usage: run-rank-stop-local.sh NEW-RUN-DIR' >&2; exit 2; }
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
repo=$(/usr/bin/git -C "$here" rev-parse --show-toplevel)
cd "$repo" || exit 2
if ps -eo stat=,comm= | awk '$2 ~ /^(rustc|cargo|clippy-driver)$/ && $1 !~ /^Z/ { active=1 } END { exit !active }'; then echo 'Finish builds before timing.' >&2; exit 2; fi
[[ -z $(/usr/bin/git status --porcelain --untracked-files=normal) ]] || { echo 'Publish clean source/spec first.' >&2; exit 2; }
plan=$here/rank-stop-plan.json
sha256sum -c "$here/$(jq -er .component_hashes "$plan")" || exit 2
sha256sum -c "$here/validation/rank-stop-build.sha256" || exit 2
cpu=$(awk '/^Cpus_allowed_list:/{split($2,a,/[,-]/); print a[1]}' /proc/self/status)
[[ $cpu == 0 ]] || { echo 'Frozen CPU0 is unavailable.' >&2; exit 2; }
mkdir "$1" || exit 2
run=$(cd "$1" && pwd)
cp target/release/examples/jv_sieve_two_large_prime_dlp_compare "$run/driver" || exit 2
cp "$here/native_analysis/target/release/rank-stop-analysis" "$run/analyzer" || exit 2
cp "$here/run-rank-stop-local.sh" "$run/wrapper" || exit 2
cp .resume/RESUME.md "$run/RESUME-before.md" || exit 2
jq --arg source "$(/usr/bin/git rev-parse HEAD)" --arg driver "$(sha256sum "$run/driver" | cut -d ' ' -f1)" --arg analyzer "$(sha256sum "$run/analyzer" | cut -d ' ' -f1)" --arg cpu "$cpu" '. + {source_revision:$source,binary_sha256:$driver,analyzer_sha256:$analyzer,pinned_cpu:$cpu}' "$plan" > "$run/runtime-plan.json" || exit 2
/usr/bin/git ls-files -z -- Cargo.toml Cargo.lock src/lib.rs src/cryptanalysis/jv\* src/cryptanalysis/gaudry\* src/cryptanalysis/f4_fp\* src/cryptanalysis/large_prime_filter\* src/cryptanalysis/residual_walk.rs src/cryptanalysis/bitops.rs src/cryptanalysis/ic_measurement.rs src/prime_hyperelliptic examples/jv_sieve_two_large_prime_dlp_compare.rs research/extension_ic_graph_20261115 | tar --null -T - -cf "$run/source-checkpoint.tar" || exit 2
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
for file in "$run/flow.jsonl" "$run/pre-workers.sha256" "$run/fixture.json" "$run/public.json" "$run"/*.raw.json "$run"/*.summary.jsonl "$run"/*.exit-code.txt "$run"/*.stderr "$run"/*.time.txt "$run"/*.argv.json; do [[ ! -f $file ]] || sha256sum "$file" >> "$run/post-call.sha256"; done
echo "Local public-target validation exited $status; originals retained at $run"
exit "$status"
