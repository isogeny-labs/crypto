#!/bin/bash
set -euo pipefail
study=$(cd "$(dirname "$0")" && pwd)
binary=$1
results=$2
[[ ! -e "$results" ]] || { echo 'Fresh result directory required' >&2; exit 2; }
mkdir -p "$results"
dest=s3://setec-astronomy-artifacts/runs/n83-candidate-spike40-e27-20261028-r1/local/
upload(){ aws s3 sync "$results" "$dest" --exclude '*.partial' --only-show-errors; }
started=$(date -u +%FT%TZ)
finish(){
 code=$?;trap - EXIT
 if [[ -n ${uploader_pid:-} ]];then kill "$uploader_pid" 2>/dev/null || true;wait "$uploader_pid" 2>/dev/null || true;fi
 jq -n --argjson code "$code" --arg start "$started" --arg end "$(date -u +%FT%TZ)" --argjson pid "$$" '{exit_code:$code,started_at:$start,finished_at:$end,pid:$pid,run_id:"n83-candidate-spike40-e27-20261028-r1",backend:"existing_host",wall_valid:false}' > "$results/terminal.json"
 (cd "$results" && find . -type f ! -name manifest.sha256 -print0 | sort -z | xargs -0 sha256sum > manifest.sha256)
 aws s3 sync "$results" "$dest" --only-show-errors || exit 91
 exit "$code"
}
trap finish EXIT
expected=$(jq -r '.arms[0].instrument.binary_sha256' "$study/protocol-base.json")
printf '%s  %s\n' "$expected" "$binary" | sha256sum -c -
cp "$binary" "$results/instrument"
cp "$study/protocol-base.json" "$study/FORMAT.json" "$results/"
cp -a "$study/validation" "$results/development-validation"
uname -a > "$results/host.txt"
lscpu >> "$results/host.txt"
"$binary" selfcheck > "$results/controls.json" 2> "$results/expected-negative-controls.log"
upload
(while sleep 30;do upload;done) &
uploader_pid=$!
(ulimit -v 8388608; timeout --signal=TERM --kill-after=20 5400 "$binary" run "$results/data" 27 20 6) > "$results/producer.log" 2>&1
