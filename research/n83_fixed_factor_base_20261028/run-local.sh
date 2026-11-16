#!/bin/bash
set -euo pipefail
binary=$1
protocol=$2
results=$3
[[ ! -e "$results" ]] || { echo 'Fresh result directory required' >&2; exit 2; }
mkdir -p "$results"
dest=s3://setec-astronomy-artifacts/runs/n83-fixed-factor-base-e27-20261028-r4/local/
upload() { aws s3 sync "$results" "$dest" --exclude '*.partial' --only-show-errors; }
started=$(date -u +%FT%TZ)
finish() {
  code=$?
  trap - EXIT
  if [[ -n ${uploader_pid:-} ]]; then kill "$uploader_pid" 2>/dev/null || true; wait "$uploader_pid" 2>/dev/null || true; fi
  jq -n --argjson exit_code "$code" --arg start "$started" --arg end "$(date -u +%FT%TZ)" --arg host "$(hostname)" --argjson pid "$$" \
    '{state:(if $exit_code==0 then "succeeded" else "failed" end),exit_code:$exit_code,started_at:$start,finished_at:$end,host:$host,pid:$pid,run_id:"local-r4",backend:"local"}' > "$results/terminal.json"
  upload || exit 91
  exit "$code"
}
trap finish EXIT
printf '%s  %s\n' dea894c5bd3cf7e718a235c2a9335f6a2b2896036c0118bacbe22fc103d1e55b "$binary" | sha256sum --check
sha256sum "$binary" > "$results/binary.sha256"
printf '%s\n' a3d2c86829dcb70723fee72a5f2b4566d63cf8ad > "$results/source-revision.txt"
uname -a > "$results/host.txt"
lscpu >> "$results/host.txt"
cp "$protocol" "$results/protocol.json"
printf '%s\n' "$dest" > "$results/storage-uri.txt"
"$binary" selfcheck > "$results/controls.json" 2> "$results/expected-negative-controls.log"
upload
(while sleep 30; do upload; done) &
uploader_pid=$!
(ulimit -v 8388608; timeout --signal=TERM --kill-after=20 3300 "$binary" run "$results/data" 27 20 6) > "$results/producer.log" 2>&1
(cd "$results" && find . -type f ! -name manifest.sha256 ! -name terminal.json -print0 | sort -z | xargs -0 sha256sum > manifest.sha256)
upload
