#!/usr/bin/env bash
set -euo pipefail
base=/home/ec2-user/crypto-pencil-signed-graph-20261029/research/pencil_lifecycle_20261030/integration/followup-continuation-v2/packets/retained-duplicate-executed
root=/var/tmp/pencil-readonly-accounting-v2-20261101
for name in missing_identity null_identity index_mismatch altered_archive missing_manifest; do
 dst="$root/controls/$name"
 mkdir -p "$dst/input/src"
 for f in archive.jsonl input-index.jsonl run.stdout run.stderr receipt.json analysis.json run.py pre-execution-manifest.json; do cp "$base/$f" "$dst/input/$f"; done
 for f in pruned.rs duplicate.rs graph.rs; do cp "$base/src/$f" "$dst/input/src/$f"; done
 case "$name" in
 missing_identity) jq -c 'if input_line_number==1 then del(.identity.workload_id) else . end' "$base/run.stdout" > "$dst/input/run.stdout" ;;
 null_identity) jq -c 'if input_line_number==1 then .identity.algorithm_seed=null else . end' "$base/run.stdout" > "$dst/input/run.stdout" ;;
 index_mismatch) jq -c 'if input_line_number==1 then .workload_id="wrong-index-target" else . end' "$base/input-index.jsonl" > "$dst/input/input-index.jsonl" ;;
 altered_archive) jq -c 'if input_line_number==1 then .workload_id="altered-archive-target" else . end' "$base/archive.jsonl" > "$dst/input/archive.jsonl" ;;
 missing_manifest) mv "$dst/input/pre-execution-manifest.json" "$dst/removed-manifest.json" ;;
 esac
 set +e
 "$root/target/debug/pencil-readonly-accounting" "$dst/input" "$dst/result.json" > "$dst/stdout" 2> "$dst/stderr"
 rc=$?
 set -e
 jq -n --arg name "$name" --argjson exit "$rc" '{control:$name,native_exit:$exit,passed:($exit!=0),scope:"copied-file mutation; no curve computation"}' > "$dst/receipt.json"
 test "$rc" -ne 0
 jq -e '.provenance_pass==false and .reconciliation_pass==false and (.provenance_errors|length)>0' "$dst/result.json" >/dev/null
 done
jq -s '{schema:"research.accounting-v2-negative-controls/v1",controls:.,all_pass:all(.passed)}' "$root"/controls/*/receipt.json > "$root/controls/summary.json"
