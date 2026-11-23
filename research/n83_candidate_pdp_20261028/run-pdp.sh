#!/bin/bash
set -euo pipefail
study=$(cd "$(dirname "$0")" && pwd)
bindir=$1
catalog=$2
baseline=$3
candidate=$4
matched=$5
out=$6
[[ ! -e "$out" ]] || { echo 'Fresh results required' >&2; exit 2; }
mkdir -p "$out/instruments"
started=$(date -u +%FT%TZ)
dest=s3://setec-astronomy-artifacts/runs/n83-candidate-paired-falls-20261028-r1/local/
upload(){ aws s3 sync "$out" "$dest" --only-show-errors; }
finish(){
 code=$?; trap - EXIT
 if [[ -n ${uploader:-} ]];then kill "$uploader" 2>/dev/null || true;wait "$uploader" 2>/dev/null || true;fi
 jq -n --argjson code "$code" --arg start "$started" --arg end "$(date -u +%FT%TZ)" '{exit_code:$code,started_at:$start,finished_at:$end,run_id:"n83-candidate-paired-falls-20261028-r1",backend:"existing_host",wall_valid:false}' > "$out/terminal.json"
 (cd "$out" && find . -type f ! -name manifest.sha256 -print0 | sort -z | xargs -0 sha256sum > manifest.sha256)
 upload || exit 91
 exit "$code"
}
trap finish EXIT
for name in pdp fall compare catalog;do
 path="$bindir/$name";if [[ $name == catalog ]];then path=$catalog;fi
 expected=$(jq -r --arg name "$name" '.extensions.executables[$name].sha256' "$study/protocol-pdp.json")
 printf '%s  %s\n' "$expected" "$path" | sha256sum -c -
 cp "$path" "$out/instruments/$name"
done
cp "$study/protocol-pdp.json" "$study/PDP_PLAN.md" "$study/FORMAT.json" "$out/"
uname -a > "$out/host.txt"; lscpu >> "$out/host.txt"
upload
(while sleep 30;do upload;done) &
uploader=$!
ulimit -v 8388608
"$out/instruments/fall" selfcheck > "$out/first-fall-controls.json"
for arm in baseline spike40;do
 bases=$baseline;if [[ $arm == spike40 ]];then bases=$candidate;fi
 mkdir -p "$out/$arm"/{closure,replay,status}
 timeout --kill-after=5 1200 "$out/instruments/pdp" generate "$out/$arm/inputs" "$bases" "$matched" "$arm" > "$out/$arm/generate.log" 2>&1
 timeout --kill-after=5 600 "$out/instruments/fall" run "$out/$arm/inputs" "$out/$arm/first-fall" > "$out/$arm/first-fall.log" 2>&1
 while IFS= read -r id;do
  set +e
  timeout --kill-after=5 60 "$out/instruments/fall" replay "$out/$arm/inputs" "$out/$arm/first-fall/$id.json" > "$out/$arm/replay/$id.first-fall.log" 2>&1
  code=$?
  set -e
  jq -n --arg id "$id" --argjson code "$code" '{id:$id,exit_code:$code}' > "$out/$arm/status/$id.first-fall.json"
 done < <(jq -r '.[].id' "$out/$arm/inputs/inputs.json")
 timeout --kill-after=5 600 "$out/instruments/catalog" catalog "$out/$arm/inputs" "$out/$arm/catalog" > "$out/$arm/catalog.log" 2>&1
 while IFS= read -r id;do
  set +e
  timeout --kill-after=5 60 "$out/instruments/catalog" closure "$out/$arm/inputs" "$id" "$out/$arm/closure/$id" > "$out/$arm/closure/$id.log" 2>&1
  code=$?;replay=null
  if [[ $code == 0 ]];then
   timeout --kill-after=5 60 "$out/instruments/catalog" replay "$out/$arm/inputs" "$id" "$out/$arm/closure/$id" > "$out/$arm/replay/$id.closure.log" 2>&1
   replay=$?
  fi
  set -e
  jq -n --arg arm "$arm" --arg id "$id" --argjson code "$code" --argjson replay "$replay" '{arm:$arm,id:$id,closure_exit_code:$code,replay_exit_code:$replay}' > "$out/$arm/status/$id.json"
  cat "$out/$arm/status/$id.json"
 done < <(jq -r '.[] | select(.family!="prime") | .id' "$out/$arm/inputs/inputs.json")
 "$out/instruments/catalog" summarize "$out/$arm/inputs" "$out/$arm"
 upload
done
"$out/instruments/compare" "$out"
