#!/usr/bin/env bash
# Independent process replay. Never edit an ecbench sealed session.
set -euo pipefail
artifact=${1:?shard artifact directory}
replay="$artifact.independent-replay"
test ! -e "$replay"
mkdir -p "$replay"
kind=$(jq -r .kind "$artifact/job.json")
if test "$kind" = quartic; then
 exec target/release/ecbench verify --dir "$artifact/session" --replay-all --exit-code --out "$replay/receipt.json"
fi
test "$(jq -r .exit_code "$artifact/status.json")" = 0
mapfile -d '' -t argv < <(jq -jr '.argv[]+"\u0000"' "$artifact/job.json")
mapfile -d '' -t settings < <(jq -jr '.env|to_entries[]|(.key+"="+.value+"\u0000")' "$artifact/job.json")
sha256sum "${argv[0]}" > "$replay/binary.sha256"
timeout --kill-after=30s 7200s prlimit --as=68719476736 -- \
  env "${settings[@]}" "${argv[@]}" > "$replay/rows.jsonl" 2> "$replay/stderr.txt"
jq -cS 'del(.diagnostic_seconds)' "$artifact/rows.jsonl" > "$replay/expected.jsonl"
jq -cS 'del(.diagnostic_seconds)' "$replay/rows.jsonl" > "$replay/actual.jsonl"
cmp "$replay/expected.jsonl" "$replay/actual.jsonl"
jq -n '{schema:"ic-algebraic-independent-replay/v1",status:"identical_non_timing_fields",scope:"one source-pinned shard; no full-DLP inference"}' > "$replay/receipt.json"
cat "$replay/receipt.json"
