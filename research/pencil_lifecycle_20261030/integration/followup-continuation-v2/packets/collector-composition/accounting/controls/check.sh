#!/usr/bin/env bash
set -euo pipefail
base=/home/ec2-user/crypto-pencil-signed-graph-20261029/research/pencil_lifecycle_20261030/integration/followup-continuation-v2/packets/retained-duplicate-executed
root=/var/tmp/pencil-readonly-accounting-20261101
for name in duplicate missing conflicting_setup identity_mismatch; do
  dst="$root/controls/$name"
  mkdir -p "$dst/input/src"
  for f in archive.jsonl input-index.jsonl run.stdout run.stderr receipt.json analysis.json run.py; do cp "$base/$f" "$dst/input/$f"; done
  for f in pruned.rs duplicate.rs graph.rs; do cp "$base/src/$f" "$dst/input/src/$f"; done
  case "$name" in
    duplicate) head -1 "$base/run.stdout" >> "$dst/input/run.stdout" ;;
    missing) tail -n +2 "$base/run.stdout" > "$dst/input/run.stdout" ;;
    conflicting_setup) jq -c 'if input_line_number == 2 then .setup_checks_field.m += 1 else . end' "$base/run.stdout" > "$dst/input/run.stdout" ;;
    identity_mismatch) jq -c 'if input_line_number == 1 then .identity.workload_id = "wrong-target-control" else . end' "$base/run.stdout" > "$dst/input/run.stdout" ;;
  esac
  set +e
  "$root/target/debug/pencil-readonly-accounting" "$dst/input" "$dst/result.json" > "$dst/stdout" 2> "$dst/stderr"
  rc=$?
  set -e
  jq -n --arg name "$name" --argjson exit "$rc" --arg raw "$(sha256sum "$dst/input/run.stdout" | cut -d' ' -f1)" '{control:$name,native_exit:$exit,expected_nonzero:true,passed:($exit!=0),modified_raw_sha256:$raw,curve_operations:0}' > "$dst/receipt.json"
  test "$rc" -ne 0
  jq -e '.reconciliation_pass == false and (.errors|length)>0' "$dst/result.json" >/dev/null
 done
jq -s '{schema:"research.accounting-negative-controls/v1",controls:.,all_pass:all(.passed),scope:"mutated copies of saved JSON only; no curve computation or measurements"}' "$root"/controls/*/receipt.json > "$root/controls/summary.json"
