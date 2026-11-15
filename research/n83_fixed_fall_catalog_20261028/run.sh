#!/bin/bash
set -euo pipefail
study=$(cd "$(dirname "$0")" && pwd)
inputs=$1
binary=$2
fall=$3
out=$4
[[ ! -e "$out" ]] || { echo 'Fresh output directory required' >&2; exit 2; }
mkdir -p "$out"/{closure,replay,status,instruments}
dest=s3://setec-astronomy-artifacts/runs/n83-fixed-fall-catalog-20261028-r1/local/
for pair in "catalog:$binary" "first_fall:$fall"; do
 key=${pair%%:*}; path=${pair#*:}
 expected=$(jq -r --arg key "$key" '.extensions.executables[$key].sha256' "$study/protocol.json")
 printf '%s  %s\n' "$expected" "$path" | sha256sum -c -
 cp "$path" "$out/instruments/$key"
done
expected=$(jq -r .extensions.input_index_sha256 "$study/protocol.json")
printf '%s  %s\n' "$expected" "$inputs/inputs.json" | sha256sum -c -
cp "$inputs/inputs.json" "$study/protocol.json" "$study/FORMAT.json" "$out/"
cp -a "$study/validation" "$out/development-validation"
started=$(date -u +%FT%TZ)
upload() { aws s3 sync "$out" "$dest" --only-show-errors; }
finish() {
 code=$?
 trap - EXIT
 if [[ -n ${uploader:-} ]]; then kill "$uploader" 2>/dev/null || true; wait "$uploader" 2>/dev/null || true; fi
 jq -n --argjson code "$code" --arg start "$started" --arg end "$(date -u +%FT%TZ)" '{exit_code:$code,started_at:$start,finished_at:$end,backend:"existing_host",run_id:"n83-fixed-fall-catalog-20261028-r1",wall_valid:false}' > "$out/terminal.json"
 (cd "$out" && find . -type f ! -name manifest.sha256 -print0 | sort -z | xargs -0 sha256sum > manifest.sha256)
 upload || exit 91
 exit "$code"
}
trap finish EXIT
uname -a > "$out/host.txt"
lscpu >> "$out/host.txt"
upload
(while sleep 30; do upload; done) &
uploader=$!
# This wrapper is launched under an outer 5400-second budget. Every case is retained.
ulimit -v 8388608
"$fall" selfcheck > "$out/first-fall-controls.json"
timeout --kill-after=5 600 "$fall" run "$inputs" "$out/first-fall" > "$out/first-fall.log" 2>&1
while IFS= read -r id; do
 set +e
 timeout --kill-after=5 60 "$fall" replay "$inputs" "$out/first-fall/$id.json" > "$out/replay/$id.first-fall.jsonl" 2>&1
 code=$?
 set -e
 jq -n --arg id "$id" --argjson exit_code "$code" '{id:$id,exit_code:$exit_code}' > "$out/status/$id.first-fall.json"
done < <(jq -r '.[].id' "$inputs/inputs.json")
timeout --kill-after=5 600 "$binary" catalog "$inputs" "$out/catalog" > "$out/catalog.log" 2>&1
while IFS= read -r id; do
 set +e
 timeout --kill-after=5 60 "$binary" closure "$inputs" "$id" "$out/closure/$id" > "$out/closure/$id.log" 2>&1
 code=$?
 replay=null
 if [[ $code == 0 ]]; then
  timeout --kill-after=5 60 "$binary" replay "$inputs" "$id" "$out/closure/$id" > "$out/replay/$id.closure.jsonl" 2>&1
  replay=$?
 fi
 set -e
 jq -n --arg id "$id" --argjson code "$code" --argjson replay "$replay" '{id:$id,closure_exit_code:$code,replay_exit_code:$replay}' > "$out/status/$id.json"
 cat "$out/status/$id.json"
 upload
done < <(jq -r '.[] | select(.family!="prime") | .id' "$inputs/inputs.json")
"$binary" summarize "$inputs" "$out"
