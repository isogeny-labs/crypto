#!/bin/bash
set -euo pipefail
study=$(cd "$(dirname "$0")" && pwd)
inputs=$1
binary=$2
out=$3
[[ ! -e "$out" ]] || exit 2
mkdir -p "$out"
dest=s3://setec-astronomy-artifacts/runs/n83-top-quotient-20261028-r1/local/
expected=$(jq -r '.arms[0].instrument.binary_sha256' "$study/protocol.json")
printf '%s  %s\n' "$expected" "$binary" | sha256sum -c -
expected=$(jq -r .extensions.input_index_sha256 "$study/protocol.json")
printf '%s  %s\n' "$expected" "$inputs/inputs.json" | sha256sum -c -
cp "$binary" "$out/instrument"
cp "$study/protocol.json" "$study/FORMAT.json" "$inputs/inputs.json" "$out/"
cp -a "$study/validation" "$out/development-validation"
started=$(date -u +%FT%TZ)
upload(){ aws s3 sync "$out" "$dest" --only-show-errors; }
finish(){
 code=$?;trap - EXIT
 if [[ -n ${uploader:-} ]];then kill "$uploader" 2>/dev/null || true;wait "$uploader" 2>/dev/null || true;fi
 jq -n --arg start "$started" --arg end "$(date -u +%FT%TZ)" --argjson code "$code" '{run_id:"n83-top-quotient-20261028-r1",started_at:$start,finished_at:$end,exit_code:$code,backend:"existing_host",wall_valid:false}' > "$out/terminal.json"
 (cd "$out" && find . -type f ! -name manifest.sha256 -print0 | sort -z | xargs -0 sha256sum > manifest.sha256)
 upload || exit 91
 exit "$code"
}
trap finish EXIT
uname -a > "$out/host.txt"
lscpu >> "$out/host.txt"
upload
(while sleep 30;do upload;done) &
uploader=$!
ulimit -v 8388608
timeout --kill-after=5 600 "$binary" run "$inputs" "$out/certificates" > "$out/producer.log" 2>&1
timeout --kill-after=5 600 "$binary" replay "$inputs" "$out/certificates" > "$out/replay.log" 2>&1
