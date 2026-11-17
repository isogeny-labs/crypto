#!/bin/bash
set -euo pipefail
study=$(cd "$(dirname "$0")" && pwd)
binary=$1
out=$2
[[ ! -e "$out" ]] || { echo 'Fresh suffix output required' >&2; exit 2; }
mkdir -p "$out"/{inputs,v2,cases,replay,status,logs}
started=$(date -u +%FT%TZ)
dest=s3://setec-astronomy-artifacts/runs/n83-degree3-continuation-20261029-r1/local/
upload(){ aws s3 sync "$out" "$dest" --only-show-errors; }
finish(){
 code=$?;trap - EXIT
 if [[ -n ${uploader:-} ]];then kill "$uploader" 2>/dev/null || true;wait "$uploader" 2>/dev/null || true;fi
 jq -n --argjson code "$code" --arg start "$started" --arg end "$(date -u +%FT%TZ)" '{exit_code:$code,started_at:$start,finished_at:$end,backend:"existing_host",run_id:"n83-degree3-continuation-20261029-r1",wall_valid:false}' > "$out/terminal.json"
 (cd "$out" && find . -type f ! -name manifest.sha256 -print0 | sort -z | xargs -0 sha256sum > manifest.sha256)
 upload || exit 91
 exit "$code"
}
trap finish EXIT
trap 'exit 143' TERM
expected=$(jq -r '.arms[0].instrument.binary_sha256' "$study/protocol-degree3-continuation.json")
printf '%s  %s\n' "$expected" "$binary" | sha256sum -c -
cp "$binary" "$out/instrument"
cp "$study/protocol-degree3-continuation.json" "$study/DEGREE3_CONTINUATION_FORMAT.json" "$study/DEGREE3_CONTINUATION_DESIGN.md" "$out/"
cp -a "$study/validation-continuation" "$out/development-validation"
jq '.extensions.parent_archive' "$study/protocol-degree3-continuation.json" > "$out/parent-reference.json"
uname -a > "$out/host.txt";lscpu >> "$out/host.txt"
while IFS=$'\t' read -r arm id directory v2directory prefixdirectory input_hash source_hash rows_hash ops_hash result_hash parent_rows parent_ops parent_state parent_result;do
 printf '%s  %s\n' "$input_hash" "$directory/input.json" "$source_hash" "$directory/system.sing" "$rows_hash" "$v2directory/basis.rows.u64le" "$ops_hash" "$v2directory/operations.jsonl" "$result_hash" "$v2directory/result.json" "$parent_rows" "$prefixdirectory/basis.rows.u64le" "$parent_ops" "$prefixdirectory/operations.jsonl" "$parent_state" "$prefixdirectory/closure_state.json" "$parent_result" "$prefixdirectory/result.json" | sha256sum -c -
 mkdir -p "$out/inputs/$arm/$id" "$out/v2/$arm/$id"
 cp "$directory/input.json" "$directory/system.sing" "$out/inputs/$arm/$id/"
 cp "$v2directory/basis.rows.u64le" "$v2directory/operations.jsonl" "$v2directory/result.json" "$out/v2/$arm/$id/"
done < <(jq -r '.extensions.cases[]|[.arm,.id,.directory,.v2directory,.prefixdirectory,.input_json_sha256,.system_sha256,.v2.rows_sha256,.v2.operations_sha256,.v2.result_sha256,.parent_files["basis.rows.u64le"].sha256,.parent_files["operations.jsonl"].sha256,.parent_files["closure_state.json"].sha256,.parent_files["result.json"].sha256]|@tsv' "$study/protocol-degree3-continuation.json")
upload
(while sleep 30;do upload;done) &
uploader=$!
ulimit -v 8388608
"$out/instrument" selfcheck "$out/controls" > "$out/controls.json" 2> "$out/controls.log"
while IFS=$'\t' read -r key arm id prefixdirectory;do
 set +e
 timeout --signal=TERM --kill-after=5 1200 "$out/instrument" continue "$out/inputs/$arm" "$id" "$out/v2/$arm/$id" "$prefixdirectory" "$out/cases/$key" > "$out/logs/$key.log" 2>&1
 code=$?;replay=null
 if [[ $code == 0 ]];then
  timeout --signal=TERM --kill-after=5 1200 "$out/instrument" replay "$out/inputs/$arm" "$id" "$out/v2/$arm/$id" "$prefixdirectory" "$out/cases/$key" > "$out/replay/$key.log" 2>&1
  replay=$?
 fi
 set -e
 jq -n --arg key "$key" --arg arm "$arm" --arg id "$id" --argjson code "$code" --argjson replay "$replay" '{key:$key,arm:$arm,id:$id,run_exit_code:$code,replay_exit_code:$replay}' > "$out/status/$key.json"
 cat "$out/status/$key.json"
 "$out/instrument" summary "$out"
 cp "$out/summary.json" "$out/status/summary-after-$key.json"
 upload
done < <(jq -r '.extensions.cases[]|[.key,.arm,.id,.prefixdirectory]|@tsv' "$study/protocol-degree3-continuation.json")
"$out/instrument" summary "$out"
