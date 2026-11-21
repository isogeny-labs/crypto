#!/usr/bin/env bash
# Thin local build-independent capture/run wrapper; all research computation is Rust.
set -u -o pipefail
umask 077
[[ $# == 1 || $# == 2 ]] || { echo 'usage: run-local.sh NEW-RUN-DIR [FROZEN-PLAN.json]' >&2; exit 2; }
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
repo=$(/usr/bin/git -C "$here" rev-parse --show-toplevel)
cd "$repo" || exit 2
plan=${2:-$here/local-pilot-plan.json}
for program in jq taskset timeout sha256sum ps awk; do command -v "$program" >/dev/null || exit 2; done
if ps -eo stat=,comm= | awk '$2 ~ /^(rustc|cargo|clippy-driver)$/ && $1 !~ /^Z/ { active=1 } END { exit !active }'; then
  echo 'Compiler/build activity exists; do not time this run yet.' >&2; exit 2
fi
[[ -z $(/usr/bin/git status --porcelain --untracked-files=normal) ]] || { echo 'Publish the clean source/spec first.' >&2; exit 2; }
source_revision=$(/usr/bin/git rev-parse HEAD)
component_hashes=$(jq -er '.component_hashes // "validation/local-pilot-component-source.sha256"' "$plan") || exit 2
sha256sum -c "$here/$component_hashes" || exit 2
mkdir "$1" || exit 2
run_dir=$(cd "$1" && pwd)
driver_kind=$(jq -r '.driver_kind // "graph-ic"' "$plan")
case "$driver_kind" in
  graph-ic)
    driver_binary=target/release/examples/gaudry_cubic_graph_compare
    analyzer_binary=$here/native_analysis/target/release/cubic-graph-analysis
    ;;
  s4-f4-component|s4-krylov-component|s4-delayed-component)
    driver_binary=target/release/examples/gaudry_cubic_s4_f4_probe
    analyzer_binary=$here/native_analysis/target/release/s4-component-analysis
    ;;
  s4-trace-component)
    driver_binary=target/release/examples/gaudry_cubic_s4_trace_compare
    analyzer_binary=$here/native_analysis/target/release/s4-trace-analysis
    ;;
  cover-sieve-graph|cover-sieve-validation)
    driver_binary=target/release/examples/jv_sieve_graph_compare
    if [[ "$driver_kind" == cover-sieve-validation ]]; then
      analyzer_binary=$here/native_analysis/target/release/cover-validation-analysis
    else
      analyzer_binary=$here/native_analysis/target/release/cover-graph-analysis
    fi
    ;;
  *) echo 'Unknown frozen driver kind.' >&2; exit 2;;
esac
# Pin the documented native defaults rather than inherit another study's
# engine overrides. SIMD selection uses the captured CPU automatically.
export F4_FP_THREAD_OPS=0 F4_STAIR=counter F4_FP_NARROW=2 F4_FP_SYMBOLIC_PAR=1
export F4_FP_PACKED=1 F4_FP_GM=1 F4_FP_DEFERRED_PAR=1 F4_FP_FL=1
export F4_FP_FL_BLOCKED=1 F4_FP_LINEAR_INTERREDUCE=1
unset F4_DEBUG F4_FP_LAZY_SIMD F4_FP_LAZY_AVX512 F4_FP_LAZY_AVX2
unset GAUDRY_DEBUG GAUDRY_DEBUG_SOLVE GAUDRY_LAZY_ELIM GAUDRY_LAZY_VERIFY GAUDRY_BORDER_RETRY
cp "$driver_binary" "$run_dir/driver" || exit 2
cp "$analyzer_binary" "$run_dir/analyzer" || exit 2
binary_sha=$(sha256sum "$run_dir/driver" | cut -d ' ' -f1)
cpu=$(awk '/^Cpus_allowed_list:/{split($2,a,/[,-]/); print a[1]}' /proc/self/status)
jq --arg source "$source_revision" --arg binary "$binary_sha" --arg cpu "$cpu" \
  '. + {source_revision:$source,binary_sha256:$binary,pinned_cpu:$cpu}' \
  "$plan" > "$run_dir/runtime-plan.json" || exit 2
cp .resume/RESUME.md "$run_dir/RESUME-before.md" || exit 2
/usr/bin/git ls-files -z -- Cargo.toml Cargo.lock src/cryptanalysis/gaudry_cubic.rs \
  src/cryptanalysis/gaudry_cubic_graph.rs src/cryptanalysis/gaudry_cubic_symmetric.rs \
  src/cryptanalysis/gaudry_cubic_sampling.rs src/cryptanalysis/gaudry_cubic_prepared.rs \
  src/cryptanalysis/gaudry_cubic_s4_f4.rs \
  src/cryptanalysis/gaudry_cubic_krylov.rs \
  src/cryptanalysis/residual_walk.rs src/cryptanalysis/jv_bitops.rs \
  src/cryptanalysis/jv_bitops_uniform.rs \
  src/cryptanalysis/f4_fp.rs src/cryptanalysis/f4_fp src/cryptanalysis/groebner_f4.rs src/cryptanalysis/fx_hash.rs \
  examples/gaudry_cubic_graph_compare.rs examples/gaudry_cubic_s4_f4_probe.rs \
  examples/gaudry_cubic_s4_trace_compare.rs \
  examples/jv_sieve_graph_compare.rs src/cryptanalysis/jv_cover.rs \
  src/cryptanalysis/jv_sieve.rs src/cryptanalysis/jv_sieve_graph.rs \
  src/cryptanalysis/jv_rho_strong.rs src/cryptanalysis/jv_isogeny_walk.rs \
  src/cryptanalysis/large_prime_filter.rs \
  src/cryptanalysis/large_prime_filter_mst.rs \
  src/cryptanalysis/bitops.rs src/cryptanalysis/ic_measurement.rs \
  src/prime_hyperelliptic/ops.rs docs/units/BITOPS.md \
  docs/units/bitop-v1.vectors.json docs/units/bitop-v2.vectors.json \
  research/pencil_large_prime_20261029/Cargo.toml \
  research/pencil_large_prime_20261029/src/lib.rs research/pencil_large_prime_20261029/src/meter.rs \
  research/pencil_large_prime_20261029/src/sparse.rs research/extension_ic_graph_20261115 \
  | tar --null -T - -cf "$run_dir/source-checkpoint.tar" || exit 2
uname -a > "$run_dir/os.txt"
lscpu -J > "$run_dir/cpu.json"
{ cat /sys/fs/cgroup/cpu.max; cat /sys/fs/cgroup/memory.max; rg '^Cpus_allowed_list:' /proc/self/status; } > "$run_dir/cgroup.txt"
"${HOME}/.cargo/bin/rustc" -Vv > "$run_dir/rustc.txt"
primes=$(jq -er '.primes|map(tostring)|join(",")' "$run_dir/runtime-plan.json")
seeds=$(jq -er '.seeds|map(tostring)|join(",")' "$run_dir/runtime-plan.json")
rounds=$(jq -er .rounds "$run_dir/runtime-plan.json")
warmups=$(jq -er .warmup_rounds "$run_dir/runtime-plan.json")
seconds=$(jq -er .timeout_seconds "$run_dir/runtime-plan.json")
memory_kib=$(( $(jq -er .memory_gib "$run_dir/runtime-plan.json") * 1024 * 1024 ))
argv=("$run_dir/driver" --primes "$primes" --seeds "$seeds" \
  --warmup-rounds "$warmups" --rounds "$rounds" \
  --source-revision "$source_revision" --out "$run_dir/raw.jsonl")
if [[ "$driver_kind" == cover-sieve-graph || "$driver_kind" == cover-sieve-validation ]]; then
  keep_percent=$(jq -er .retained_percent "$run_dir/runtime-plan.json")
  argv+=(--retained-percent "$keep_percent")
  if [[ $(jq -r '.operation_accounting // false' "$run_dir/runtime-plan.json") == true ]]; then
    argv+=(--bitops)
  fi
  if [[ $(jq -r '.graph_strategy // "pivot"' "$run_dir/runtime-plan.json") == mst ]]; then
    argv+=(--mst-graph)
  fi
  if [[ $(jq -r '.sieve_recipe // "historical_default"' "$run_dir/runtime-plan.json") == incidence_shared ]]; then
    argv+=(--incidence-shared)
  fi
  if [[ "$driver_kind" == cover-sieve-graph ]]; then
    argv+=(--worker-timeout-seconds "$(jq -r '.worker_timeout_seconds // 0' "$run_dir/runtime-plan.json")")
  fi
  if [[ "$driver_kind" == cover-sieve-validation ]]; then
    argv+=(--validation-only --worker-timeout-seconds "$(jq -er .worker_timeout_seconds "$run_dir/runtime-plan.json")")
  fi
elif [[ "$driver_kind" == graph-ic ]]; then
  oracles=$(jq -er .oracle_cli "$run_dir/runtime-plan.json")
  sampler=$(jq -r '.sampler // "fresh"' "$run_dir/runtime-plan.json")
  cap=$(jq -er .max_residuals "$run_dir/runtime-plan.json")
  argv+=(--oracles "$oracles" --sampler "$sampler" --max-residuals "$cap")
  if [[ $(jq -r '.record_start_events // false' "$run_dir/runtime-plan.json") == true ]]; then argv+=(--execution-starts); fi
else
  residual_count=$(jq -er .residuals_per_instance "$run_dir/runtime-plan.json")
  batch_count=$(jq -er .batch_iterations "$run_dir/runtime-plan.json")
  f4_degree=$(jq -er .f4_max_degree "$run_dir/runtime-plan.json")
  f4_deadline=$(jq -er .f4_deadline_ms "$run_dir/runtime-plan.json")
  argv+=(--residuals-per-instance "$residual_count" --batch-iterations "$batch_count" \
    --f4-max-degree "$f4_degree" --f4-deadline-ms "$f4_deadline")
  if [[ "$driver_kind" == s4-trace-component ]]; then
    trace_certifier=$(jq -r '.trace_certifier // "exhaustive"' "$run_dir/runtime-plan.json")
    argv+=(--trace-certifier "$trace_certifier")
  fi
  if [[ "$driver_kind" == s4-krylov-component ]]; then argv+=(--extraction-comparison); fi
  if [[ "$driver_kind" == s4-delayed-component ]]; then
    argv+=(--extraction-comparison --delayed-comparison)
    if [[ $(jq -r .reference_kind "$run_dir/runtime-plan.json") == scalar ]]; then argv+=(--scalar-reference); fi
  fi
fi
jq -n --args '$ARGS.positional' -- "${argv[@]}" > "$run_dir/argv.json" || exit 2
sha256sum "$run_dir/driver" "$run_dir/analyzer" "$run_dir/runtime-plan.json" \
  "$run_dir/argv.json" "$run_dir/source-checkpoint.tar" "$run_dir/RESUME-before.md" \
  "$run_dir/os.txt" "$run_dir/cpu.json" "$run_dir/cgroup.txt" "$run_dir/rustc.txt" \
  > "$run_dir/pre-call.sha256" || exit 2
date -u +%FT%TZ > "$run_dir/started-at.txt"
if [[ -r /sys/fs/cgroup/cpu.stat ]]; then cp /sys/fs/cgroup/cpu.stat "$run_dir/cpu-stat-before.txt"; fi
echo "Local run: $run_dir"
export RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1
ulimit -v "$memory_kib" || exit 2
TIMEFORMAT=$'real_s=%R\nuser_s=%U\nsys_s=%S\ncpu_percent=%P'
{ time {
  timeout --signal=TERM --kill-after=5s "${seconds}s" taskset -c "$cpu" "${argv[@]}" \
    > "$run_dir/stdout.txt" 2> "$run_dir/stderr.txt" &
  worker_pid=$!
  printf '%s\n' "$worker_pid" > "$run_dir/worker.pid"
  wait "$worker_pid"
}; } 2> "$run_dir/time.txt"
run_status=$?
if [[ -r /sys/fs/cgroup/cpu.stat ]]; then cp /sys/fs/cgroup/cpu.stat "$run_dir/cpu-stat-after.txt"; fi
printf '%s\n' "$run_status" > "$run_dir/exit-code.txt"
date -u +%FT%TZ > "$run_dir/finished-at.txt"
sha256sum "$run_dir/exit-code.txt" "$run_dir/time.txt" "$run_dir/stdout.txt" \
  "$run_dir/stderr.txt" > "$run_dir/post-call.sha256"
if [[ -f "$run_dir/raw.jsonl" ]]; then sha256sum "$run_dir/raw.jsonl" >> "$run_dir/post-call.sha256"; fi
for captured in "$run_dir"/progress-*.stderr "$run_dir"/public-*.json "$run_dir"/cpu-stat-*.txt; do
  if [[ -f "$captured" ]]; then sha256sum "$captured" >> "$run_dir/post-call.sha256"; fi
done
echo "Local run exited $run_status; raw evidence retained at $run_dir"
exit "$run_status"
