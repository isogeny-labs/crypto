#!/bin/bash
set -euo pipefail
study=$(cd "$(dirname "$0")" && pwd)
binary=$1
prior=$2
out=$3
[[ ! -e "$out" ]] || exit 2
mkdir -p "$out/inputs"
started=$(date -u +%FT%TZ)
dest=s3://setec-astronomy-artifacts/runs/n83-paired-top-20261028-r1/local/
upload(){ aws s3 sync "$out" "$dest" --only-show-errors; }
finish(){
 code=$?;trap - EXIT
 if [[ -n ${uploader:-} ]];then kill "$uploader" 2>/dev/null || true;wait "$uploader" 2>/dev/null || true;fi
 jq -n --argjson code "$code" --arg start "$started" --arg end "$(date -u +%FT%TZ)" '{exit_code:$code,started_at:$start,finished_at:$end,backend:"existing_host",run_id:"n83-paired-top-20261028-r1",wall_valid:false}' > "$out/terminal.json"
 (cd "$out" && find . -type f ! -name manifest.sha256 -print0 | sort -z | xargs -0 sha256sum > manifest.sha256)
 upload || exit 91
 exit "$code"
}
trap finish EXIT
expected=$(jq -r '.arms[0].instrument.binary_sha256' "$study/protocol-top.json")
printf '%s  %s\n' "$expected" "$binary" | sha256sum -c -
cp "$binary" "$out/instrument"
cp "$study/protocol-top.json" "$study/TOP_FORMAT.json" "$study/TOP_DESIGN.md" "$out/"
cp -a "$study/validation" "$out/development-validation"
uname -a > "$out/host.txt"
lscpu >> "$out/host.txt"
while IFS=$'\t' read -r arm index_hash;do
 printf '%s  %s\n' "$index_hash" "$prior/$arm/inputs/inputs.json" | sha256sum -c -
 mkdir "$out/inputs/$arm"
 cp -a "$prior/$arm/inputs" "$out/inputs/$arm/inputs"
done < <(jq -r '.extensions.input_indices[]|[.arm,.sha256]|@tsv' "$study/protocol-top.json")
while IFS=$'\t' read -r arm id source_hash;do
 printf '%s  %s\n' "$source_hash" "$out/inputs/$arm/inputs/$id/system.sing" | sha256sum -c -
done < <(jq -r '.extensions.cases[]|[.arm,.id,.source_sha256]|@tsv' "$study/protocol-top.json")
upload
(while sleep 30;do upload;done) &
uploader=$!
ulimit -v 8388608
set +e
timeout --signal=TERM --kill-after=5 900 "$out/instrument" run "$out/inputs" "$out/certificates" > "$out/producer.log" 2>&1
producer=$?
timeout --signal=TERM --kill-after=5 900 "$out/instrument" replay "$out/inputs" "$out/certificates" > "$out/replay.log" 2>&1
replay=$?
set -e
jq -n --argjson p "$producer" --argjson r "$replay" '{producer_exit_code:$p,replay_exit_code:$r}' > "$out/status.json"
