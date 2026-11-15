#!/bin/bash
set -euo pipefail
study=$(cd "$(dirname "$0")" && pwd)
binary=$1
out=$2
[[ ! -e "$out" ]] || { echo 'Fresh witness output required' >&2; exit 2; }
mkdir -p "$out"/{inputs,v2,case-specs,cases,replay,status,logs}
started=$(date -u +%FT%TZ)
dest=s3://setec-astronomy-artifacts/runs/n83-degree4-witness-20261029-r1/local/
upload(){ aws s3 sync "$out" "$dest" --region us-east-2 --only-show-errors; }
finish(){
 code=$?;trap - EXIT
 if [[ -n ${uploader:-} ]];then kill "$uploader" 2>/dev/null || true;wait "$uploader" 2>/dev/null || true;fi
 jq -n --argjson code "$code" --arg start "$started" --arg end "$(date -u +%FT%TZ)" '{exit_code:$code,started_at:$start,finished_at:$end,backend:"existing_host",run_id:"n83-degree4-witness-20261029-r1",wall_valid:false}' > "$out/terminal.json"
 (cd "$out" && find . -type f ! -name manifest.sha256 -print0 | sort -z | xargs -0 sha256sum > manifest.sha256)
 upload || exit 91
 exit "$code"
}
trap finish EXIT
trap 'exit 143' TERM
expected=$(jq -r '.arms[0].instrument.binary_sha256' "$study/protocol-degree4-witness.json")
printf '%s  %s\n' "$expected" "$binary" | sha256sum -c -
cp "$binary" "$out/instrument"
cp "$study/protocol-degree4-witness.json" "$study/DEGREE4_WITNESS_FORMAT.json" "$study/DEGREE4_WITNESS_DESIGN.md" "$out/"
cp -a "$study/validation-degree4-witness" "$out/development-validation"
jq '.extensions.parent_archives' "$study/protocol-degree4-witness.json" > "$out/parent-reference.json"
uname -a > "$out/host.txt"
lscpu >> "$out/host.txt"
while IFS=$'\t' read -r key arm id directory v2directory;do
 mkdir -p "$out/inputs/$arm/$id" "$out/v2/$arm/$id"
 cp "$directory/input.json" "$directory/system.sing" "$out/inputs/$arm/$id/"
 cp "$v2directory/basis.rows.u64le" "$v2directory/operations.jsonl" "$v2directory/result.json" "$out/v2/$arm/$id/"
 jq --arg key "$key" '.extensions.cases[]|select(.key==$key)' "$study/protocol-degree4-witness.json" > "$out/case-specs/$key.json"
done < <(jq -r '.extensions.cases[]|[.key,.arm,.id,.directory,.v2directory]|@tsv' "$study/protocol-degree4-witness.json")
upload
(while sleep 30;do upload;done) &
uploader=$!
ulimit -v 8388608
"$out/instrument" selfcheck "$out/controls" > "$out/controls.log" 2>&1
while IFS=$'\t' read -r key arm id prefixdirectory suffixdirectory;do
 set +e
 timeout --signal=TERM --kill-after=5 1800 "$out/instrument" run "$out/case-specs/$key.json" "$out/inputs/$arm" "$out/v2/$arm/$id" "$prefixdirectory" "$suffixdirectory" "$out/cases/$key" > "$out/logs/$key.log" 2>&1
 code=$?;replay=null
 if [[ $code == 0 ]];then
  timeout --signal=TERM --kill-after=5 1800 "$out/instrument" replay "$out/case-specs/$key.json" "$out/inputs/$arm" "$out/v2/$arm/$id" "$prefixdirectory" "$suffixdirectory" "$out/cases/$key" > "$out/replay/$key.log" 2>&1
  replay=$?
 fi
 set -e
 jq -n --arg key "$key" --arg arm "$arm" --arg id "$id" --argjson code "$code" --argjson replay "$replay" '{key:$key,arm:$arm,id:$id,run_exit_code:$code,replay_exit_code:$replay}' > "$out/status/$key.json"
 cat "$out/status/$key.json"
 "$out/instrument" summary "$out"
 cp "$out/summary.json" "$out/status/summary-after-$key.json"
 upload
done < <(jq -r '.extensions.cases[]|[.key,.arm,.id,.prefixdirectory,.suffixdirectory]|@tsv' "$study/protocol-degree4-witness.json")
"$out/instrument" summary "$out"
jq -e '.all_verified' "$out/summary.json" >/dev/null
