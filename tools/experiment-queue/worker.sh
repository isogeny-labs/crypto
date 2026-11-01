#!/usr/bin/env bash
# Executed only in the digest-pinned, UID-1000 Isolab task image.
set -euo pipefail
entry=$1
: "${ISOLAB_OUTPUT_DIR:?}" "${ISOLAB_RESULTS_URI:?}"
work_root=${ISOLAB_WORK_ROOT:-/work}
input_root=$work_root/inputs
mkdir -p "$ISOLAB_OUTPUT_DIR" "$input_root"
command_rc=125 verify_rc=125 verified=false
finish() {
  final_rc=$?
  trap - EXIT
  set +e
  jq -n --argjson e "$entry" --argjson command "$command_rc" --argjson verify "$verify_rc" \
    --argjson checked "$verified" --arg pod "${POD_UID:-}" \
    '{schema:"crypto.isolab-worker-result/v1",source_revision:$e.source_commit,image:$e.image.uri,
      pod_uid:$pod,command_exit:$command,verify_exit:$verify,inputs_and_runtime_verified:$checked,
      wall_valid:false}' > "$ISOLAB_OUTPUT_DIR/queue-job.json"
  (cd "$ISOLAB_OUTPUT_DIR" && find . -type f ! -path ./SHA256SUMS -print0 | sort -z | xargs -0 -r sha256sum > SHA256SUMS)
  aws s3 cp "$ISOLAB_OUTPUT_DIR/" "$ISOLAB_RESULTS_URI" --recursive --no-follow-symlinks --only-show-errors || exit 91
  exit "$final_rc"
}
trap finish EXIT
while IFS= read -r item; do
  file=$(jq -r .path <<< "$item")
  expected=$(jq -r .sha256 <<< "$item")
  [[ -f $file && ! -L $file && $(sha256sum "$file" | cut -d' ' -f1) == "$expected" ]]
done < <(jq -c '.image.runtime_files[]' <<< "$entry")
while IFS= read -r item; do
  file=$(jq -r .path <<< "$item")
  [[ $file != /* && /$file/ != */../* && /$file/ != */./* ]]
  mkdir -p -- "$(dirname -- "$input_root/$file")"
  aws s3 cp "$(jq -r .uri <<< "$item")" "$input_root/$file" --only-show-errors
  [[ $(sha256sum "$input_root/$file" | cut -d' ' -f1) == "$(jq -r .sha256 <<< "$item")" ]]
done < <(jq -c '.inputs[]?' <<< "$entry")
verified=true
export EXPERIMENT_OUT="$ISOLAB_OUTPUT_DIR" EXPERIMENT_INPUTS="$input_root"
mapfile -d '' -t argv < <(jq -j --arg out "$EXPERIMENT_OUT" --arg inputs "$EXPERIMENT_INPUTS" \
  '.command[] | split("{out}")|join($out)|split("{inputs}")|join($inputs),"\u0000"' <<< "$entry")
set +e
"${argv[@]}" > "$ISOLAB_OUTPUT_DIR/command.stdout" 2> "$ISOLAB_OUTPUT_DIR/command.stderr"
command_rc=$?
set -e
[[ $command_rc == 0 ]] || exit "$command_rc"
mapfile -d '' -t argv < <(jq -j --arg out "$EXPERIMENT_OUT" --arg inputs "$EXPERIMENT_INPUTS" \
  '.verify[] | split("{out}")|join($out)|split("{inputs}")|join($inputs),"\u0000"' <<< "$entry")
set +e
"${argv[@]}" > "$ISOLAB_OUTPUT_DIR/verify.stdout" 2> "$ISOLAB_OUTPUT_DIR/verify.stderr"
verify_rc=$?
set -e
exit "$verify_rc"
