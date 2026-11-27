#!/usr/bin/env bash
# Execute only a separately frozen copy of this wrapper.
set -euo pipefail
attempt=${1:?usage: run.sh ABSOLUTE_ATTEMPT_DIRECTORY tests|initial|replay}
mode=${2:?usage: run.sh ABSOLUTE_ATTEMPT_DIRECTORY tests|initial|replay}
[[ "$attempt" = /* ]] || { echo 'Attempt path must be absolute' >&2; exit 2; }
case "$mode" in
  tests) command_line=("$attempt/build/count-selector-tests" --test-threads=1); output="$attempt/tests.stdout" ;;
  initial|replay) command_line=("$attempt/build/count-selector"); output="$attempt/$mode.json" ;;
  *) echo 'Unknown mode' >&2; exit 2 ;;
esac
[[ "$(readlink -f -- "${BASH_SOURCE[0]}")" = "$attempt/frozen/measurement/run.sh" ]] || {
  echo 'Execute the frozen wrapper in this attempt' >&2; exit 2;
}
[[ ! -e "$attempt/pre-call-$mode.json" && ! -e "$output" ]] || {
  echo 'Refusing to overwrite a call' >&2; exit 2;
}
(
  cd "$attempt"
  sha256sum --check source-manifest.sha256
  sha256sum --check binaries.sha256
) > "$attempt/integrity-$mode.stdout"
qualifier_hash=$(sha256sum "$attempt/frozen/measurement/QUALIFIER.md" | cut -d ' ' -f 1)
binary_hash=$(sha256sum "${command_line[0]}" | cut -d ' ' -f 1)
jq -n --arg mode "$mode" --arg started "$(date -u +%FT%TZ)" \
  --arg qualifier_sha256 "$qualifier_hash" --arg executable_sha256 "$binary_hash" \
  '{schema:"isogeny.m31-counts-precall/v1",mode:$mode,started_utc:$started,
    qualifier_sha256:$qualifier_sha256,executable_sha256:$executable_sha256,
    timeout_seconds:60,kill_grace_seconds:3,logical_cpu:0,virtual_memory_kib:524288,
    threads:1,wall_time_admission:false}' > "$attempt/pre-call-$mode.json"
finish() {
  code=$?
  output_hash=""
  if [[ -f "$output" ]]; then output_hash=$(sha256sum "$output" | cut -d ' ' -f 1); fi
  jq -n --arg mode "$mode" --arg finished "$(date -u +%FT%TZ)" \
    --arg output_sha256 "$output_hash" --argjson exit_code "$code" \
    '{schema:"isogeny.m31-counts-postcall/v1",mode:$mode,finished_utc:$finished,
      exit_code:$exit_code,output_sha256:$output_sha256,wall_time_admission:false}' \
    > "$attempt/post-call-$mode.json"
}
trap finish EXIT
export OMP_NUM_THREADS=1 RAYON_NUM_THREADS=1 OPENBLAS_NUM_THREADS=1
ulimit -v 524288
timeout -k 3s 60s taskset -c 0 "${command_line[@]}" > "$output" 2> "$attempt/$mode.stderr"
if [[ "$mode" != tests ]]; then
  jq -e '.status == "pass" and .clock_calls == 0 and .new_relation_search_calls == 0' \
    "$output" > "$attempt/json-format-$mode.stdout"
fi
