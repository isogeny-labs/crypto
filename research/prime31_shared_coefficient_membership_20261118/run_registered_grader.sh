#!/usr/bin/env bash
# Run the isolated generic four-plus-four truth grader for one frozen rung.
# Its tables and witnesses are never inputs to a candidate solver process.
set -euo pipefail

if [ "$#" -lt 2 ] || [ "$#" -gt 3 ]; then
  echo "usage: $0 <4|6|8> <artifact-dir> [wall-seconds]" >&2
  exit 2
fi

base_size="$1"
artifact_dir="$2"
wall_seconds="${3:-900}"
memory_limit_kib=16777216

repo="$(git rev-parse --show-toplevel)"
cd "$repo"
scripts/checkpoint.sh gate

case "$base_size" in 4|6|8) ;; *) echo "base size must be 4, 6, or 8" >&2; exit 2 ;; esac
case "$artifact_dir" in
  ""|/|.|"$repo") echo "refusing unsafe artifact directory: $artifact_dir" >&2; exit 2 ;;
esac
case "$wall_seconds" in *[!0-9]*|"") echo "wall-seconds must be a positive integer" >&2; exit 2;; esac
[ "$wall_seconds" -gt 0 ] || { echo "wall-seconds must be positive" >&2; exit 2; }
[ -x /usr/bin/time ] || { echo "GNU /usr/bin/time is required" >&2; exit 2; }

grader="$repo/research/prime31_shared_coefficient_membership_20261118/native/target/release/prime31_selector_grader"
receipt="$repo/research/prime31_shared_coefficient_membership_20261118/toolchain/build-receipt.json"
[ -x "$grader" ] || { echo "missing executable: $grader" >&2; exit 2; }
expected_hash="$(jq -er '.grader_binary_sha256' "$receipt")"
actual_hash="$(sha256sum "$grader" | cut -d' ' -f1)"
[ "$actual_hash" = "$expected_hash" ] || {
  echo "grader binary hash mismatch: $actual_hash != $expected_hash" >&2
  exit 2
}

mkdir -p "$artifact_dir"
stem="$artifact_dir/prime31-w8-b$base_size"
grader_record="$stem.grader.json"
grader_pending="$stem.grader.pending"
grader_raw="$stem.grader.raw"
grader_stderr="$stem.grader.stderr"
resource_record="$stem.grader.resource.json"
command_record="$stem.grader.command"
summary="$stem.grader.summary.json"
for output in "$grader_record" "$grader_pending" "$grader_raw" "$resource_record" \
  "$command_record" "$summary"; do
  [ ! -e "$output" ] || {
    echo "refusing to overwrite existing artifact: $output" >&2
    exit 2
  }
done

grader_command=(timeout --signal=TERM --kill-after=10s "${wall_seconds}s" "$grader" "$base_size")
printf '%q ' "${grader_command[@]}" >"$command_record"
printf '\n' >>"$command_record"

set +e
(
  ulimit -v "$memory_limit_kib"
  exec /usr/bin/time --quiet \
    -f '{"user_seconds":%U,"system_seconds":%S,"elapsed_seconds":%e,"max_rss_kib":%M,"exit_status":%x}' \
    -o "$resource_record" "${grader_command[@]}"
) >"$grader_pending" 2>"$grader_stderr"
grader_exit=$?
set -e

if ! jq -e . "$resource_record" >/dev/null 2>&1; then
  jq -n --arg status missing_resource_record \
    '{status:$status,pcc:"unverified"}' >"$resource_record"
fi

if [ "$grader_exit" -eq 124 ]; then
  status=timeout
elif [ "$grader_exit" -eq 137 ]; then
  elapsed_seconds="$(jq -r '.elapsed_seconds // -1' "$resource_record")"
  if awk -v elapsed="$elapsed_seconds" -v cap="$wall_seconds" \
    'BEGIN { exit !(elapsed >= cap) }'; then
    status=timeout
  else
    status=sigkill_unknown
  fi
elif grep -Eiq 'bad_alloc|out of memory|cannot allocate memory|memory allocation' "$grader_stderr"; then
  status=oom
elif [ "$grader_exit" -ne 0 ]; then
  status=process_error
elif jq -e --argjson base_size "$base_size" \
  '.schema == "prime31.selector-isolated-grader/v1" and .base_size == $base_size' \
  "$grader_pending" >/dev/null 2>&1; then
  status=complete
else
  status=malformed_grader_record
fi

panel_summary=null
grader_record_sha256=""
if [ "$status" = complete ]; then
  mv "$grader_pending" "$grader_record"
  grader_record_sha256="$(sha256sum "$grader_record" | cut -d' ' -f1)"
  panel_summary="$(jq -c '{natural_positives,natural_negatives,natural_multiplicity_sum,
    left,right,planted_count:(.planted | length)}' "$grader_record")"
else
  mv "$grader_pending" "$grader_raw"
fi

command_sha256="$(sha256sum "$command_record" | cut -d' ' -f1)"
jq -n --arg status "$status" --argjson base_size "$base_size" \
  --argjson grader_exit "$grader_exit" --argjson wall_cap_seconds "$wall_seconds" \
  --argjson memory_cap_bytes "$((memory_limit_kib * 1024))" \
  --arg grader_binary_sha256 "$actual_hash" --arg command_sha256 "$command_sha256" \
  --arg grader_record_sha256 "$grader_record_sha256" --argjson panel "$panel_summary" \
  --slurpfile resource "$resource_record" \
  '{schema:"prime31.selector-grader-summary/v1",base_size:$base_size,status:$status,
    grader_exit:$grader_exit,wall_cap_seconds:$wall_cap_seconds,
    memory_cap_bytes:$memory_cap_bytes,grader_binary_sha256:$grader_binary_sha256,
    command_sha256:$command_sha256,
    grader_record_sha256:(if $grader_record_sha256 == "" then null else $grader_record_sha256 end),
    panel:$panel,process_resources:$resource[0],candidate_data_consumed:false,
    pcc:"unverified"}' >"$summary"

printf '%s\n' "$summary"
