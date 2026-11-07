#!/usr/bin/env bash
# Thin workflow orchestration. Scientific selection, rank accounting, replay,
# and aggregation are implemented by the pinned native twist-tr04 binary.
set -euo pipefail

protocol=research/twist_relations_tr04_20261029/protocol.json
manifest=tools/twist-relations/Cargo.toml
binary=tools/twist-relations/target/release/twist-tr04
report_binary=tools/twist-relations/target/release/tr04-report
protocol_binary=tools/twist-relations/target/release/tr04-protocol
export CARGO_TARGET_DIR="$PWD/tools/twist-relations/target"
mode=${1:-}
prime=null
case "$mode" in
  field)
    [[ $# == 2 ]] || { echo 'Usage: run-tr04-workflow.sh field PRIME' >&2; exit 2; }
    case "$2" in 7|11|19) prime=$2 ;; *) echo 'Unsupported frozen field' >&2; exit 2 ;; esac
    output="artifacts/tr04/p$prime"
    ;;
  aggregate)
    [[ $# == 2 ]] || { echo 'Usage: run-tr04-workflow.sh aggregate INPUT_ROOT' >&2; exit 2; }
    input_root=$2
    output=artifacts/tr04-summary
    ;;
  *) echo 'Expected field or aggregate mode' >&2; exit 2 ;;
esac

mkdir -p "$output"
command -v jq >/dev/null
stage=source_check
build_exit=null
run_exit=null
verify_exit=null
summarize_exit=null
report_exit=null
attempted=false
report_attempted=false
finish() {
  local wrapper_exit=$?
  trap - EXIT
  jq -n --arg source "${TR04_SOURCE_REVISION:-}" --arg mode "$mode" \
    --arg stage "$stage" --arg finished "$(date -u +%FT%TZ)" \
    --argjson prime "$prime" --argjson wrapper_exit "$wrapper_exit" \
    --argjson build_exit "$build_exit" --argjson run_exit "$run_exit" \
    --argjson verify_exit "$verify_exit" --argjson summarize_exit "$summarize_exit" \
    --argjson report_exit "$report_exit" --argjson attempted "$attempted" \
    --argjson report_attempted "$report_attempted" \
    --arg repository "${GITHUB_REPOSITORY:-}" --arg run "${GITHUB_RUN_ID:-}" \
    --arg attempt "${GITHUB_RUN_ATTEMPT:-}" \
    '{schema:"twist-tr04.workflow-outcome/v1",source_revision:(if $source=="" then null else $source end),mode:$mode,
      prime:$prime,stage:$stage,finished_at:$finished,wrapper_exit_code:$wrapper_exit,
      build_exit_code:$build_exit,run_exit_code:$run_exit,
      verify_exit_code:$verify_exit,summarize_exit_code:$summarize_exit,
      report_exit_code:$report_exit,attempted:$attempted,report_attempted:$report_attempted,
      status:(if $wrapper_exit==0 then "completed" else "failed" end),
      status_scope:"native wrapper, not final GitHub job outcome",
      repository:(if $repository=="" then null else $repository end),
      run_id:(if $run=="" then null else $run end),
      run_attempt:(if $attempt=="" then null else $attempt end)}' \
    > "$output/outcome.json.tmp"
  mv "$output/outcome.json.tmp" "$output/outcome.json"
  exit "$wrapper_exit"
}
trap finish EXIT

[[ "${TR04_SOURCE_REVISION:-}" =~ ^[a-f0-9]{40}$ ]]
[[ $(git rev-parse HEAD) == "$TR04_SOURCE_REVISION" ]]
[[ -z $(git status --porcelain --untracked-files=no) ]]
test -f "$protocol"
test ! -e "$output/results"
jq -e 'type=="object"' "$protocol" >/dev/null
cp "$protocol" "$output/protocol.json"
protocol_hash=$(sha256sum "$protocol" | cut -d ' ' -f 1)
assigned=null
if [[ -s "$output/assigned-job.json" ]]; then
  assigned=$(jq -ce 'if type=="array" and length==1 then .[0] else error("ambiguous assigned job") end' "$output/assigned-job.json")
fi
jq -n --arg source "$TR04_SOURCE_REVISION" \
  --arg workflow "${TR04_WORKFLOW_REVISION:-}" \
  --arg workflow_ref "${GITHUB_WORKFLOW_REF:-}" \
  --arg run "${GITHUB_RUN_ID:-}" --arg attempt "${GITHUB_RUN_ATTEMPT:-}" \
  --arg repository "${GITHUB_REPOSITORY:-}" \
  --arg requested_runner "${TR04_REQUESTED_RUNNER:-}" \
  --arg runner "${RUNNER_NAME:-}" --arg runner_os "${RUNNER_OS:-}" \
  --arg runner_arch "${RUNNER_ARCH:-}" \
  --arg runner_environment "${RUNNER_ENVIRONMENT:-}" \
  --arg protocol_sha256 "$protocol_hash" --arg started "$(date -u +%FT%TZ)" \
  --argjson prime "$prime" --argjson assigned "$assigned" \
  'def known($v): if $v=="" then null else $v end;
   {schema:"twist-tr04.workflow-execution/v1",source_revision:$source,
    workflow_revision:known($workflow),workflow_ref:known($workflow_ref),run_id:known($run),
    run_attempt:known($attempt),repository:known($repository),
    requested_runner:known($requested_runner),runner_name:known($runner),
    runner_os:known($runner_os),runner_arch:known($runner_arch),runner_environment:known($runner_environment),
    backend:(if ($assigned.labels // [] | index("crypto-eks-linux")) != null then "isolab-k8s" else null end),
    runner_label:(if ($assigned.labels // [] | index("crypto-eks-linux")) != null then "crypto-eks-linux" else null end),
    job_id:($assigned.id // null),job_name:($assigned.name // null),job_status:($assigned.status // null),
    job_url:($assigned.html_url // null),image_digest:null,kernel:null,cpu:null,l2:null,wall_valid:false,
    protocol_sha256:$protocol_sha256,started_at:$started,prime:$prime,
    measurement_scope:"exact finite counters; wall time diagnostic; no L2 or speedup admission"}' \
  > "$output/execution.json"
{
  uname -srmo
  rustc -Vv
  cargo -V
  for resource in /sys/fs/cgroup/cpu.max /sys/fs/cgroup/memory.max; do
    if [[ -r "$resource" ]]; then printf '%s=' "$resource"; cat "$resource"; fi
  done
} > "$output/host.txt" 2>&1

stage=build
build_command=(cargo build --release --locked --manifest-path "$manifest")
if [[ "$mode" == aggregate ]]; then build_command+=(--bins); else build_command+=(--bin twist-tr04); fi
printf '%q ' "${build_command[@]}" > "$output/commands.txt"
printf '\n' >> "$output/commands.txt"
if "${build_command[@]}" \
    > "$output/build.stdout.log" 2> "$output/build.stderr.log"; then
  build_exit=0
else
  build_exit=$?
  exit "$build_exit"
fi
if [[ "$mode" == aggregate ]]; then
  sha256sum "$binary" "$report_binary" "$protocol_binary" > "$output/binary.sha256"
else
  sha256sum "$binary" > "$output/binary.sha256"
fi

if [[ "$mode" == field ]]; then
  stage=run
  attempted=true
  printf '%s --prime %s --output-dir %s/results\n' "$binary" "$prime" "$output" \
    >> "$output/commands.txt"
  if "$binary" --prime "$prime" --output-dir "$output/results" \
      > "$output/run.stdout.log" 2> "$output/run.stderr.log"; then
    run_exit=0
  else
    run_exit=$?
  fi
  # A failed run's partial directory is also inspected. Neither its presence
  # nor a successful partial replay overrides the original nonzero exit.
  stage=verify
  printf '%s verify %s/results\n' "$binary" "$output" >> "$output/commands.txt"
  if "$binary" verify "$output/results" \
      > "$output/verify.stdout.log" 2> "$output/verify.stderr.log"; then
    verify_exit=0
  else
    verify_exit=$?
  fi
  if (( run_exit != 0 )); then exit "$run_exit"; fi
  if (( verify_exit != 0 )); then exit "$verify_exit"; fi
else
  stage=summarize
  inputs=()
  for expected_prime in 7 11 19; do
    inputs+=(--input "$input_root/tr04-p$expected_prime-${GITHUB_RUN_ID:?}-${GITHUB_RUN_ATTEMPT:?}/results")
  done
  # Do not silently ignore an unexpected artifact. Passing it to the native
  # checker preserves its diagnostics and makes duplicate/extra fields fail.
  shopt -s nullglob
  for artifact in "$input_root"/*; do
    case "$artifact" in
      "$input_root/tr04-p7-$GITHUB_RUN_ID-$GITHUB_RUN_ATTEMPT"|\
      "$input_root/tr04-p11-$GITHUB_RUN_ID-$GITHUB_RUN_ATTEMPT"|\
      "$input_root/tr04-p19-$GITHUB_RUN_ID-$GITHUB_RUN_ATTEMPT") ;;
      *) inputs+=(--input "$artifact/results") ;;
    esac
  done
  printf '%q ' "$binary" summarize "${inputs[@]}" --output-dir "$output/results" \
    >> "$output/commands.txt"
  printf '\n' >> "$output/commands.txt"
  # All three declared inputs are passed even if an artifact is absent.
  # The native aggregator must report and reject missing/incomplete shards.
  if "$binary" summarize "${inputs[@]}" --output-dir "$output/results" \
      > "$output/summarize.stdout.log" 2> "$output/summarize.stderr.log"; then
    summarize_exit=0
  else
    summarize_exit=$?
  fi
  stage=report
  report_attempted=true
  printf '%q ' "$report_binary" --input "$output/results" --protocol "$protocol" \
    --output-dir "$output/report" >> "$output/commands.txt"
  printf '\n' >> "$output/commands.txt"
  # Reporting reads retained records only. It must describe a partial failure
  # honestly, and cannot turn that original summary failure into a pass.
  if "$report_binary" --input "$output/results" --protocol "$protocol" \
      --output-dir "$output/report" > "$output/report.stdout.log" 2> "$output/report.stderr.log"; then
    report_exit=0
  else
    report_exit=$?
  fi
  if (( summarize_exit != 0 )); then exit "$summarize_exit"; fi
  if (( report_exit != 0 )); then exit "$report_exit"; fi
fi
stage=complete
