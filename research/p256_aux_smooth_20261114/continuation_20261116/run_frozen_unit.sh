#!/usr/bin/env bash
# Execute a separately copied qualifier and pin it before this unit call.
set -euo pipefail
[[ $# == 3 ]] || { echo 'Usage: run_frozen_unit.sh INPUT.m FRESH_OUTPUT_DIR NATIVE_REPLAYER' >&2; exit 2; }
input_file=$1
result_dir=$2
native_replayer=$3
qualifier_dir="$result_dir.qualifier"
[[ -f "$input_file" && -x "$native_replayer" && ! -e "$result_dir" && ! -e "$qualifier_dir" ]] || { echo 'Inputs invalid or output/qualifier already exists' >&2; exit 2; }
study_dir=$(cd "$(dirname "$0")" && pwd)
mkdir -p "$qualifier_dir"
cp "$study_dir/submit_unit.sh" "$qualifier_dir/submit_unit.sh"
cp "$native_replayer" "$qualifier_dir/native_replayer"
chmod 444 "$qualifier_dir/submit_unit.sh"
chmod 555 "$qualifier_dir/native_replayer"
input_sha=$(sha256sum "$input_file" | cut -d ' ' -f 1)
qualifier_sha=$(sha256sum "$qualifier_dir/submit_unit.sh" | cut -d ' ' -f 1)
replayer_sha=$(sha256sum "$qualifier_dir/native_replayer" | cut -d ' ' -f 1)
jq -n --arg source_commit "$(git -C "$study_dir" rev-parse HEAD)" \
  --arg at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" --arg input_sha "$input_sha" \
  --arg qualifier_sha "$qualifier_sha" --arg replayer_sha "$replayer_sha" \
  '{schema:"p256.calculator_pre_call/v1",source_commit:$source_commit,at:$at,input_sha256:$input_sha,qualifier_sha256:$qualifier_sha,native_replayer_sha256:$replayer_sha,http_cap_seconds:25,benchmark:false}' \
  > "$qualifier_dir/pre-call-manifest.json"
chmod 444 "$qualifier_dir/pre-call-manifest.json"
set +e
bash "$qualifier_dir/submit_unit.sh" "$input_file" "$result_dir"
unit_exit=$?
set -e
jq -n --argjson exit "$unit_exit" --arg at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --arg pre_sha "$(sha256sum "$qualifier_dir/pre-call-manifest.json" | cut -d ' ' -f 1)" \
  '{schema:"p256.calculator_post_call/v1",at:$at,qualifier_exit:$exit,pre_call_manifest_sha256:$pre_sha}' \
  > "$qualifier_dir/post-call-receipt.json"
exit "$unit_exit"
