#!/bin/bash
set -euo pipefail
cd /work
mkdir -p results
# The configured legacy lab results bucket is not used for new writes.
attempt=${ISOLAB_RESULTS_URI%/}
attempt=${attempt##*/}
dest="s3://setec-astronomy-artifacts/runs/n83-fixed-factor-base-e27-20261028-r3/$attempt/"
printf '%s\n' "$dest" > results/storage-uri.txt
upload() { aws s3 sync results "$dest" --exclude '*.partial' --only-show-errors; }
finish() {
  code=$?
  trap - EXIT
  if [[ -n ${uploader_pid:-} ]]; then kill "$uploader_pid" 2>/dev/null || true; wait "$uploader_pid" 2>/dev/null || true; fi
  printf '{"exit_code":%s}\n' "$code" > results/job-exit.json
  upload || exit 91
  exit "$code"
}
trap finish EXIT
uname -a > results/host.txt
lscpu >> results/host.txt
cp /opt/fb-protocol.json results/protocol.json
cp /opt/fb-source-revision.txt results/source-revision.txt
sha256sum /opt/n83-factor-base > results/binary.sha256
/opt/n83-factor-base selfcheck > results/controls.json 2> results/expected-negative-controls.log
upload
(while sleep 30; do upload; done) &
uploader_pid=$!
timeout --signal=TERM --kill-after=20 3300 /opt/n83-factor-base run /work/results/data 27 20 6 > results/producer.log 2>&1
upload
(cd results && find . -type f ! -name manifest.sha256 ! -name job-exit.json -print0 | sort -z | xargs -0 sha256sum > manifest.sha256)
upload
