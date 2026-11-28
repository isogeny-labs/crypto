#!/usr/bin/env bash
# Thin native build/run metadata only. Scientific work stays in Rust.
set -euo pipefail
set -o noclobber

if [[ $# -lt 3 ]]; then
    printf 'usage: run_frozen.sh RECEIPT_PREFIX INPUT_INVENTORY IMMUTABLE_BINARY [ARG ...]\n' >&2
    exit 2
fi
run_prefix=$1
run_inputs=$2
run_binary=$3
shift 3
[[ "$run_prefix" =~ ^[a-z0-9-]+$ ]] || exit 2
[[ -f "$run_binary" && -x "$run_binary" && ! -L "$run_binary" ]] || exit 2
if [[ -n "$(find "$run_binary" -maxdepth 0 -perm /222 -print)" ]]; then
    printf 'refusing writable qualifier: %s\n' "$run_binary" >&2
    exit 2
fi
run_root=$(git -C "$(dirname "$0")" rev-parse --show-toplevel)
cd "$run_root"
run_evidence=research/dickson_s3_quadratic_20261122/evidence
sha256sum -c "$run_inputs" > "$run_evidence/$run_prefix-input-check.log"
run_before=$(sha256sum "$run_binary" | cut -d ' ' -f 1)
run_argv=$(jq -cn --args '$ARGS.positional' -- "$run_binary" "$@")
jq -n --arg at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
    --arg commit "$(git rev-parse HEAD)" --arg binary "$run_binary" \
    --arg binary_sha256 "$run_before" --arg inputs "$run_inputs" \
    --arg inputs_sha256 "$(sha256sum "$run_inputs" | cut -d ' ' -f 1)" \
    --argjson argv "$run_argv" \
    '{schema:"dickson-s3-pre-call/v1",created_at_utc:$at,base_commit:$commit,
      working_tree_sources_bound_by_inventory:true,binary:$binary,
      binary_sha256:$binary_sha256,input_inventory:$inputs,
      input_inventory_sha256:$inputs_sha256,argv:$argv,
      outer_timeout_seconds:120,kill_after_seconds:5,
      purpose:"bounded development correctness; no comparative performance claim",
      receipt_status:"unsigned producer evidence",scientific_comparison_admitted:false}' \
    > "$run_evidence/$run_prefix-pre-call.json"
set +e
timeout --signal=TERM --kill-after=5s 120s "$run_binary" "$@" \
    > "$run_evidence/$run_prefix.stdout" 2> "$run_evidence/$run_prefix.stderr"
run_status=$?
set -e
run_after=$(sha256sum "$run_binary" | cut -d ' ' -f 1)
jq -n --arg at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
    --argjson status "$run_status" --arg before "$run_before" --arg after "$run_after" \
    --arg pre_sha256 "$(sha256sum "$run_evidence/$run_prefix-pre-call.json" | cut -d ' ' -f 1)" \
    --arg stdout_sha256 "$(sha256sum "$run_evidence/$run_prefix.stdout" | cut -d ' ' -f 1)" \
    --arg stderr_sha256 "$(sha256sum "$run_evidence/$run_prefix.stderr" | cut -d ' ' -f 1)" \
    '{schema:"dickson-s3-post-call/v1",finished_at_utc:$at,exit_status:$status,
      binary_sha256_before:$before,binary_sha256_after:$after,binary_unchanged:($before==$after),
      pre_call_sha256:$pre_sha256,stdout_sha256:$stdout_sha256,stderr_sha256:$stderr_sha256,
      evidence_status:"unsigned producer evidence; no signed PCC verdict"}' \
    > "$run_evidence/$run_prefix-post-call.json"
cat "$run_evidence/$run_prefix-post-call.json"
[[ "$run_before" == "$run_after" ]] || exit 1
exit "$run_status"
