#!/usr/bin/env bash
# Exact, non-timing GitHub-hosted wrapper for the frozen norm-branch census.
set -euo pipefail

if [[ $# -ne 9 ]]; then
  echo 'usage: run.sh OUTPUT_DIR PRODUCER CHECKER SOURCE_REVISION PARENT_PROTOCOL PARENT_DIGEST RUNNER_PROTOCOL RUNNER_DIGEST BUILD_INFO' >&2
  exit 2
fi

output=$1
producer=$2
checker=$3
source_revision=$4
parent_protocol=$5
parent_digest_file=$6
runner_protocol=$7
runner_digest_file=$8
build_info=$9
output_cap_bytes=67108864

[[ "$source_revision" =~ ^[a-f0-9]{40}$ ]]
[[ ! -e "$output" ]]
mkdir "$output"

finish() {
  status=$?
  trap - EXIT
  set +e
  jq -n \
    --argjson exit_status "$status" \
    --arg source_revision "$source_revision" \
    --arg run_id "${GITHUB_RUN_ID:-unknown}" \
    --arg run_attempt "${GITHUB_RUN_ATTEMPT:-unknown}" \
    '{schema:"norm.branch-census-runner-exit/v1",exit_status:$exit_status,
      source_revision:$source_revision,github_run_id:$run_id,
      github_run_attempt:$run_attempt}' > "$output/job-exit.json"
  (
    cd "$output" || exit 1
    rm -f SHA256SUMS
    find . -maxdepth 1 -type f ! -name SHA256SUMS -printf '%P\0' \
      | sort -z \
      | xargs -0 -r sha256sum > SHA256SUMS
  )
  retained_bytes=$(du -sb "$output" 2>/dev/null | awk '{print $1}')
  if [[ "$retained_bytes" =~ ^[0-9]+$ \
    && "$retained_bytes" -gt "$output_cap_bytes" \
    && "$status" -eq 0 ]]; then
    status=92
    if [[ -f "$output/execution.json" ]]; then
      jq '.status="incomplete_output_cap" | del(.verdict)' \
        "$output/execution.json" > "$output/execution.next.json"
      mv "$output/execution.next.json" "$output/execution.json"
    fi
    jq -n \
      --argjson exit_status "$status" \
      --arg source_revision "$source_revision" \
      --arg run_id "${GITHUB_RUN_ID:-unknown}" \
      --arg run_attempt "${GITHUB_RUN_ATTEMPT:-unknown}" \
      '{schema:"norm.branch-census-runner-exit/v1",exit_status:$exit_status,
        source_revision:$source_revision,github_run_id:$run_id,
        github_run_attempt:$run_attempt}' > "$output/job-exit.json"
    (
      cd "$output" || exit 1
      rm -f SHA256SUMS
      find . -maxdepth 1 -type f ! -name SHA256SUMS -printf '%P\0' \
        | sort -z \
        | xargs -0 -r sha256sum > SHA256SUMS
    )
  fi
  exit "$status"
}
trap finish EXIT
trap 'exit 143' TERM
trap 'exit 130' INT

for file in "$producer" "$checker" "$parent_protocol" "$parent_digest_file" \
  "$runner_protocol" "$runner_digest_file" "$build_info"; do
  [[ -f "$file" ]]
done
[[ -x "$producer" && -x "$checker" ]]

parent_sha=$(awk 'NR==1 {print $1}' "$parent_digest_file")
runner_sha=$(awk 'NR==1 {print $1}' "$runner_digest_file")
[[ "$parent_sha" =~ ^[a-f0-9]{64}$ && "$runner_sha" =~ ^[a-f0-9]{64}$ ]]
[[ "$(sha256sum "$parent_protocol" | cut -d' ' -f1)" == "$parent_sha" ]]
[[ "$(sha256sum "$runner_protocol" | cut -d' ' -f1)" == "$runner_sha" ]]

producer_sha=$(sha256sum "$producer" | cut -d' ' -f1)
checker_sha=$(sha256sum "$checker" | cut -d' ' -f1)
jq -e \
  --arg source "$source_revision" \
  --arg parent "$parent_sha" \
  --arg runner "$runner_sha" \
  --arg producer "$producer_sha" \
  --arg checker "$checker_sha" \
  '.schema=="norm.branch-census-build/v1"
    and .source_revision==$source
    and .parent_protocol_sha256==$parent
    and .runner_protocol_sha256==$runner
    and .producer_sha256==$producer
    and .checker_sha256==$checker' \
  "$build_info" >/dev/null

install -m644 "$parent_protocol" "$output/parent-protocol.json"
install -m644 "$runner_protocol" "$output/runner-protocol.json"
install -m644 "$build_info" "$output/build-info.json"
printf '%s  produce\n%s  check\n' "$producer_sha" "$checker_sha" \
  > "$output/binaries.sha256"

"$producer" "$output/artifact.json" "$source_revision" "$parent_sha" \
  > "$output/producer.stdout" 2> "$output/producer.stderr"
"$checker" "$output/artifact.json" "$output/checker.json" \
  > "$output/checker.stdout" 2> "$output/checker.stderr"
"$checker" "$output/artifact.json" "$output/checker-replay.json" \
  > "$output/checker-replay.stdout" 2> "$output/checker-replay.stderr"

cmp "$output/checker.json" "$output/checker-replay.json"
cmp "$output/checker.stdout" "$output/checker-replay.stdout"
cmp "$output/checker.stderr" "$output/checker-replay.stderr"

jq -e --arg source "$source_revision" --arg protocol "$parent_sha" \
  '.schema=="norm.branch-census/v1"
    and .source_revision==$source
    and .protocol_sha256==$protocol
    and (.cells|length)==40' \
  "$output/artifact.json" >/dev/null
jq -e --arg source "$source_revision" --arg protocol "$parent_sha" \
  '.schema=="norm.branch-census-check/v1"
    and .source_revision==$source
    and .protocol_sha256==$protocol
    and .pass==true
    and (.errors|length)==0' \
  "$output/checker.json" >/dev/null

producer_verdict=$(grep -Eo 'verdict=(ADVANCE_B2|KILL_MEASURED_FAMILY|INCONCLUSIVE)' \
  "$output/producer.stdout")
checker_verdict=$(grep -Eo 'verdict=(ADVANCE_B2|KILL_MEASURED_FAMILY|INCONCLUSIVE)' \
  "$output/checker.stdout")
[[ -n "$producer_verdict" && "$producer_verdict" == "$checker_verdict" ]]
verdict=${producer_verdict#verdict=}

artifact_sha=$(sha256sum "$output/artifact.json" | cut -d' ' -f1)
checker_receipt_sha=$(sha256sum "$output/checker.json" | cut -d' ' -f1)
jq -n \
  --arg experiment_id 'EXP-NORM-BRANCH-CENSUS-GHA-20261118-v1' \
  --arg source_revision "$source_revision" \
  --arg parent_protocol_sha256 "$parent_sha" \
  --arg runner_protocol_sha256 "$runner_sha" \
  --arg producer_sha256 "$producer_sha" \
  --arg checker_sha256 "$checker_sha" \
  --arg artifact_sha256 "$artifact_sha" \
  --arg checker_receipt_sha256 "$checker_receipt_sha" \
  --arg verdict "$verdict" \
  --arg run_id "${GITHUB_RUN_ID:-unknown}" \
  --arg run_attempt "${GITHUB_RUN_ATTEMPT:-unknown}" \
  '{schema:"experiment.export/v1",experiment_id:$experiment_id,
    status:"verified_complete",measurement_class:"exact_non_timing",
    backend:"github-actions-hosted-x86_64",source_revision:$source_revision,
    github:{run_id:$run_id,run_attempt:$run_attempt},
    protocols:{mathematical_sha256:$parent_protocol_sha256,
      execution_sha256:$runner_protocol_sha256},
    binaries:{producer_sha256:$producer_sha256,checker_sha256:$checker_sha256},
    commands:[
      ["produce","artifact.json",$source_revision,$parent_protocol_sha256],
      ["check","artifact.json","checker.json"],
      ["check","artifact.json","checker-replay.json"]
    ],
    outputs:{artifact_sha256:$artifact_sha256,
      checker_receipt_sha256:$checker_receipt_sha256,
      checker_replay_byte_identical:true},
    verdict:$verdict,timing_endpoint:false}' > "$output/execution.json"

output_bytes=$(du -sb "$output" | awk '{print $1}')
[[ "$output_bytes" =~ ^[0-9]+$ && "$output_bytes" -le "$output_cap_bytes" ]]
