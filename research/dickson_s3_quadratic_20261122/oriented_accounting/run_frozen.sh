#!/usr/bin/env bash
# Metadata/execution wrapper only; all science remains in the frozen Rust binary.
set -euo pipefail
set -o noclobber
[[ $# -ge 3 ]] || exit 2
oa_prefix=$1
oa_inputs=$2
oa_binary=$3
shift 3
[[ "$oa_prefix" =~ ^[a-z0-9-]+$ && -f "$oa_binary" && -x "$oa_binary" && ! -L "$oa_binary" ]] || exit 2
[[ -z "$(find "$oa_binary" -maxdepth 0 -perm /222 -print)" ]] || exit 2
oa_root=$(git -C "$(dirname "$0")" rev-parse --show-toplevel)
cd "$oa_root"
oa_evidence=research/dickson_s3_quadratic_20261122/oriented_accounting/evidence
sha256sum -c "$oa_inputs" > "$oa_evidence/$oa_prefix-input-check.log"
oa_before=$(sha256sum "$oa_binary" | cut -d ' ' -f1)
oa_argv=$(jq -cn --args '$ARGS.positional' -- "$oa_binary" "$@")
jq -n --arg at "$(date -u +%FT%TZ)" --arg commit "$(git rev-parse HEAD)" \
  --arg binary "$oa_binary" --arg digest "$oa_before" --arg inputs "$oa_inputs" \
  --arg inputs_sha256 "$(sha256sum "$oa_inputs" | cut -d ' ' -f1)" --argjson argv "$oa_argv" \
  '{schema:"oriented-accounting-pre-call/v1",created_at_utc:$at,base_commit:$commit,
    working_sources_bound_by_inventory:true,binary:$binary,binary_sha256:$digest,
    input_inventory:$inputs,input_inventory_sha256:$inputs_sha256,argv:$argv,
    outer_timeout_seconds:120,kill_after_seconds:5,address_space_limit_kib:3145728,
    rayon_threads:1,purpose:"bounded native development correctness; no speed ranking",
    scientific_comparison_admitted:false,receipt_status:"unsigned producer evidence"}' \
    > "$oa_evidence/$oa_prefix-pre-call.json"
set +e
( ulimit -v 3145728; export RAYON_NUM_THREADS=1; timeout --signal=TERM --kill-after=5s 120s "$oa_binary" "$@" ) \
  > "$oa_evidence/$oa_prefix.stdout" 2> "$oa_evidence/$oa_prefix.stderr"
oa_status=$?
set -e
oa_after=$(sha256sum "$oa_binary" | cut -d ' ' -f1)
jq -n --arg at "$(date -u +%FT%TZ)" --argjson status "$oa_status" --arg before "$oa_before" --arg after "$oa_after" \
  --arg pre_sha256 "$(sha256sum "$oa_evidence/$oa_prefix-pre-call.json" | cut -d ' ' -f1)" \
  --arg stdout_sha256 "$(sha256sum "$oa_evidence/$oa_prefix.stdout" | cut -d ' ' -f1)" \
  --arg stderr_sha256 "$(sha256sum "$oa_evidence/$oa_prefix.stderr" | cut -d ' ' -f1)" \
  '{schema:"oriented-accounting-post-call/v1",finished_at_utc:$at,exit_status:$status,
    binary_sha256_before:$before,binary_sha256_after:$after,binary_unchanged:($before==$after),
    pre_call_sha256:$pre_sha256,stdout_sha256:$stdout_sha256,stderr_sha256:$stderr_sha256,
    evidence_status:"unsigned producer evidence; no signed PCC verdict"}' > "$oa_evidence/$oa_prefix-post-call.json"
cat "$oa_evidence/$oa_prefix-post-call.json"
[[ "$oa_before" == "$oa_after" ]] || exit 1
exit "$oa_status"
