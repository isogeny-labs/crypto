#!/bin/bash
set -euo pipefail
study=$(cd "$(dirname "$0")" && pwd)
binary=$1
bases=$2
results=$3
[[ ! -e "$results" ]] || { echo 'Fresh results required' >&2; exit 2; }
mkdir -p "$results"
dest=s3://setec-astronomy-artifacts/runs/n83-fixed-degree-e27-20261028-r1/local/
image=$(jq -r .image "$study/request.json")
expected=$(jq -r '.arms[0].instrument.binary_sha256' "$study/protocol.json")
printf '%s  %s\n' "$expected" "$binary" | sha256sum -c -
started=$(date -u +%FT%TZ)
upload() { aws s3 sync "$results" "$dest" --only-show-errors; }
finish() {
 code=$?
 trap - EXIT
 if [[ -n ${uploader:-} ]]; then kill "$uploader" 2>/dev/null || true; wait "$uploader" 2>/dev/null || true; fi
 jq -n --argjson code "$code" --arg start "$started" --arg end "$(date -u +%FT%TZ)" --arg host "$(hostname)" --arg image "$image" '{state:(if $code==0 then "succeeded" else "failed" end),exit_code:$code,started_at:$start,finished_at:$end,host:$host,image:$image,backend:"local",run_id:"n83-fixed-degree-e27-20261028-r1"}' > "$results/terminal.json"
 (cd "$results" && find . -type f ! -name manifest.sha256 -print0 | sort -z | xargs -0 sha256sum > manifest.sha256)
 upload || exit 91
 exit "$code"
}
trap finish EXIT
uname -a > "$results/host.txt"
lscpu >> "$results/host.txt"
sha256sum "$binary" > "$results/binary.sha256"
jq -r '.arms[0].instrument.commit' "$study/protocol.json" > "$results/source-revision.txt"
cp "$study/protocol.json" "$study/request.json" "$results/"
upload
(while sleep 30; do upload; done) &
uploader=$!
sudo docker run --rm --name n83-fixed-degree-e27-r1 --cpus 4 --memory 16g --user "$(id -u):$(id -g)" \
 -v "$binary:/opt/fixed-sweep:ro" -v "$study/run-container.sh:/opt/fixed-run.sh:ro" \
 -v "$bases:/bases:ro" -v "$results:/results" \
 --entrypoint /usr/bin/timeout "$image" --signal=TERM --kill-after=20 1800 /bin/bash /opt/fixed-run.sh > "$results/producer.log" 2>&1
