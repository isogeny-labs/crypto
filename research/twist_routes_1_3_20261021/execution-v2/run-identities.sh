#!/usr/bin/env bash
set -euo pipefail
study_dir="$(cd -- "$(dirname -- "$0")/.." && pwd)"
repo_dir="$(cd -- "$study_dir/../.." && pwd)"
cd -- "$repo_dir"
output="${1:?fresh output directory}"
test ! -e "$output"
revision="$(git rev-parse HEAD)"
pinned="/tmp/twist-v2-pinned/$revision/identities"
mkdir -p "$(dirname -- "$pinned")"
cp "$study_dir/identity-runner/target/release/twist-followup-identities" "$pinned"
receipt="$study_dir/execution-v2/receipts/$(basename -- "$output")"
test ! -e "$receipt.json"
started="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
set +e
( ulimit -v 4194304; timeout --signal=TERM --kill-after=5s 60s "$pinned" "$output" ) >"$receipt.stdout" 2>"$receipt.stderr"
status=$?
set -e
node - "$receipt" "$revision" "$pinned" "$output" "$started" "$status" <<'NODE'
const fs=require('fs'),c=require('crypto');
const [receipt,source,binary,output,started,status]=process.argv.slice(2);
const sha=p=>c.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const protocol='research/twist_routes_1_3_20261021/execution-v2/IDENTITIES_PROTOCOL_V1.md';
fs.writeFileSync(receipt+'.json',JSON.stringify({schema:'twist.identity-run-receipt/v1',source_commit:source,
binary_sha256:sha(binary),argv:[binary,output],started_utc:started,finished_utc:new Date().toISOString(),
exit_code:Number(status),timeout_s:60,address_space_kib:4194304,threads:1,isolation:'L0',wall_time_admitted:false,
protocol_path:protocol,protocol_sha256:sha(protocol),stdout_sha256:sha(receipt+'.stdout'),stderr_sha256:sha(receipt+'.stderr'),performance_claim:false},null,2)+'\n');
NODE
exit "$status"
