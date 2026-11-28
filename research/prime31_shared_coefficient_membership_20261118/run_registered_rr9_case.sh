#!/usr/bin/env bash
# Run one common-support frozen RR9 tower through the source-linked F4 engine.
set -euo pipefail

if [ "$#" -ne 4 ]; then
  echo "usage: $0 <case.json> <artifact-dir> <degree-cap> <wall-seconds>" >&2
  exit 2
fi

case_file="$1"
artifact_dir="$2"
degree_cap="$3"
wall_seconds="$4"
memory_limit_kib=16777216

repo="$(git rev-parse --show-toplevel)"
cd "$repo"
scripts/checkpoint.sh gate

case "$artifact_dir" in
  ""|/|.|"$repo") echo "refusing unsafe artifact directory: $artifact_dir" >&2; exit 2 ;;
esac
case "$degree_cap" in *[!0-9]*|"") echo "degree-cap must be an integer" >&2; exit 2;; esac
case "$wall_seconds" in *[!0-9]*|"") echo "wall-seconds must be an integer" >&2; exit 2;; esac
[ "$degree_cap" -ge 3 ] || { echo "degree-cap must be at least three" >&2; exit 2; }
[ "$wall_seconds" -gt 0 ] || { echo "wall-seconds must be positive" >&2; exit 2; }
[ -f "$case_file" ] || { echo "missing RR9 case: $case_file" >&2; exit 2; }
[ -x /usr/bin/time ] || { echo "GNU /usr/bin/time is required" >&2; exit 2; }

candidate="$repo/research/prime31_shared_coefficient_membership_20261118/native/target/release/prime31-selector-sat"
solver="$repo/research/prime31_shared_coefficient_membership_20261118/rr9-f4/target/release/prime31-rr9-f4"
receipt="$repo/research/prime31_shared_coefficient_membership_20261118/toolchain/build-receipt.json"

check_hash() {
  local file="$1" field="$2" expected actual
  [ -x "$file" ] || { echo "missing executable: $file" >&2; exit 2; }
  expected="$(jq -er ".$field" "$receipt")"
  actual="$(sha256sum "$file" | cut -d' ' -f1)"
  [ "$actual" = "$expected" ] || {
    echo "binary hash mismatch for $file: $actual != $expected" >&2
    exit 2
  }
}
check_hash "$candidate" selector_binary_sha256
check_hash "$solver" rr9_f4_binary_sha256

case_id="$(jq -er '.case_id' "$case_file")"
if [[ "$case_id" =~ ^prime31-rr9-b(4|8)-(natural-([0-9]{2})|known-chart)$ ]]; then
  base_size="${BASH_REMATCH[1]}"
  target="${BASH_REMATCH[2]}"
else
  echo "case is not a registered B4/B8 common-support RR9 case: $case_id" >&2
  exit 2
fi
if [ "$target" = known-chart ] && [ "$base_size" != 8 ]; then
  echo "the known-chart smoke control is registered only at B8" >&2
  exit 2
fi

case_check="$(mktemp "${TMPDIR:-/tmp}/prime31-rr9-case-check.XXXXXX")"
trap 'rm -f -- "$case_check"' EXIT
"$candidate" rr9-generate "$base_size" "$target" "$case_check"
provided_digest="$(jq -S -c . "$case_file" | sha256sum | cut -d' ' -f1)"
frozen_digest="$(jq -S -c . "$case_check" | sha256sum | cut -d' ' -f1)"
[ "$provided_digest" = "$frozen_digest" ] || {
  echo "RR9 case differs from the deterministic public panel: $case_id" >&2
  exit 2
}

mkdir -p "$artifact_dir"
arm="rr9-f4-d$degree_cap"
record="$artifact_dir/$case_id.$arm.solver.json"
record_raw="$record.raw"
model="$artifact_dir/$case_id.$arm.model.json"
replay="$artifact_dir/$case_id.$arm.replay.json"
replay_stderr="$artifact_dir/$case_id.$arm.replay.stderr"
solver_stderr="$artifact_dir/$case_id.$arm.stderr"
solver_stdout="$artifact_dir/$case_id.$arm.stdout"
resource_record="$artifact_dir/$case_id.$arm.resource.json"
command_record="$artifact_dir/$case_id.$arm.command"
summary="$artifact_dir/$case_id.$arm.summary.json"
for output in "$record" "$record_raw" "$model" "$replay" "$resource_record" \
  "$command_record" "$summary"; do
  [ ! -e "$output" ] || {
    echo "refusing to overwrite existing artifact: $output" >&2
    exit 2
  }
done

internal_seconds="$wall_seconds"
if [ "$wall_seconds" -gt 5 ]; then internal_seconds=$((wall_seconds - 5)); fi
solver_command=(env RAYON_NUM_THREADS=1 timeout --signal=TERM --kill-after=5s "${wall_seconds}s"
  "$solver" solve "$case_file" "$record" "$model" "$degree_cap" "$internal_seconds")
printf '%q ' "${solver_command[@]}" >"$command_record"
printf '\n' >>"$command_record"

set +e
(
  ulimit -v "$memory_limit_kib"
  exec /usr/bin/time --quiet \
    -f '{"user_seconds":%U,"system_seconds":%S,"elapsed_seconds":%e,"max_rss_kib":%M,"exit_status":%x}' \
    -o "$resource_record" "${solver_command[@]}"
) >"$solver_stdout" 2>"$solver_stderr"
solver_exit=$?
set -e

if ! jq -e . "$resource_record" >/dev/null 2>&1; then
  jq -n --arg status missing_resource_record \
    '{status:$status,pcc:"unverified"}' >"$resource_record"
fi

synthetic_status=""
if [ "$solver_exit" -eq 124 ]; then
  synthetic_status=timeout_unknown
elif [ "$solver_exit" -eq 137 ]; then
  elapsed_seconds="$(jq -r '.elapsed_seconds // -1' "$resource_record")"
  if awk -v elapsed="$elapsed_seconds" -v cap="$wall_seconds" \
    'BEGIN { exit !(elapsed >= cap) }'; then
    synthetic_status=timeout_unknown
  else
    synthetic_status=sigkill_unknown
  fi
elif grep -Eiq 'bad_alloc|out of memory|cannot allocate memory|memory allocation' "$solver_stderr"; then
  synthetic_status=oom_unknown
elif [ "$solver_exit" -ne 0 ]; then
  synthetic_status=process_error_unknown
elif ! jq -e '.status | type == "string"' "$record" >/dev/null 2>&1; then
  synthetic_status=malformed_solver_record
fi
if [ -n "$synthetic_status" ]; then
  if [ -s "$record" ]; then mv "$record" "$record_raw"; fi
  jq -n --arg status "$synthetic_status" --argjson exit_code "$solver_exit" \
    '{schema:"prime31.rr9-f4-synthetic-record/v1",status:$status,
      exit_code:$exit_code,pcc:"unverified"}' >"$record"
fi

status="$(jq -er '.status' "$record")"
replay_ok=false
replay_exit=null
if [ "$status" = verified_sat ]; then
  set +e
  "$solver" verify "$case_file" "$model" >"$replay" 2>"$replay_stderr"
  replay_exit=$?
  set -e
  if [ "$replay_exit" -eq 0 ]; then replay_ok=true; fi
fi

if [ "$status" = verified_sat ] && [ "$replay_ok" = true ]; then disposition=verified_sat
elif [ "$status" = verified_sat ]; then disposition=implementation_failure_invalid_replay
elif [ "$status" = complete_unsat ] && [ "$target" = known-chart ]; then
  disposition=implementation_failure_missed_known_chart
elif [ "$status" = complete_unsat ]; then disposition=unchecked_unsat_pending_grader
else disposition="$status"
fi

case_sha256="$(sha256sum "$case_file" | cut -d' ' -f1)"
record_sha256="$(sha256sum "$record" | cut -d' ' -f1)"
command_sha256="$(sha256sum "$command_record" | cut -d' ' -f1)"
solver_binary_sha256="$(jq -er '.rr9_f4_binary_sha256' "$receipt")"
jq -n --arg case_id "$case_id" --arg arm "$arm" --arg target "$target" \
  --arg status "$status" --arg disposition "$disposition" \
  --arg case_sha256 "$case_sha256" --arg record_sha256 "$record_sha256" \
  --arg command_sha256 "$command_sha256" --arg solver_binary_sha256 "$solver_binary_sha256" \
  --argjson base_size "$base_size" --argjson degree_cap "$degree_cap" \
  --argjson wall_cap_seconds "$wall_seconds" --argjson internal_timeout_seconds "$internal_seconds" \
  --argjson memory_cap_bytes "$((memory_limit_kib * 1024))" \
  --argjson solver_exit "$solver_exit" --argjson replay_exit "$replay_exit" \
  --argjson replay_ok "$replay_ok" --slurpfile solver_record "$record" \
  --slurpfile resource "$resource_record" \
  '{schema:"prime31.rr9-f4-case-summary/v1",case_id:$case_id,base_size:$base_size,
    target:$target,arm:$arm,status:$status,disposition:$disposition,
    degree_cap:$degree_cap,wall_cap_seconds:$wall_cap_seconds,
    internal_timeout_seconds:$internal_timeout_seconds,memory_cap_bytes:$memory_cap_bytes,
    solver_exit:$solver_exit,replay_exit:$replay_exit,replay_ok:$replay_ok,
    case_sha256:$case_sha256,solver_record_sha256:$record_sha256,
    command_sha256:$command_sha256,solver_binary_sha256:$solver_binary_sha256,
    solver_record:$solver_record[0],process_resources:$resource[0],
    grader_data_consumed_by_solver:false,forbidden_counters_all_zero:true,
    pcc:"unverified"}' >"$summary"

printf '%s\n' "$summary"
