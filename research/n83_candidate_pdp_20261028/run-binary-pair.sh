#!/bin/bash
set -euo pipefail
study=$(cd "$(dirname "$0")" && pwd)
binary=$1
out=$2
[[ ! -e "$out" ]] || exit 2
mkdir -p "$out/inputs"
started=$(date -u +%FT%TZ)
dest=s3://setec-astronomy-artifacts/runs/n83-binary-m2-pair-20261028-r1/local/
upload(){ aws s3 sync "$out" "$dest" --only-show-errors; }
finish(){
 code=$?;trap - EXIT
 jq -n --argjson code "$code" --arg start "$started" --arg end "$(date -u +%FT%TZ)" '{exit_code:$code,started_at:$start,finished_at:$end,backend:"existing_host",run_id:"n83-binary-m2-pair-20261028-r1",wall_valid:false}' > "$out/terminal.json"
 (cd "$out" && find . -type f ! -name manifest.sha256 -print0 | sort -z | xargs -0 sha256sum > manifest.sha256)
 upload || exit 91
 exit "$code"
}
trap finish EXIT
expected=$(jq -r '.arms[0].instrument.binary_sha256' "$study/protocol-binary-pair.json")
printf '%s  %s\n' "$expected" "$binary" | sha256sum -c -
cp "$binary" "$out/instrument"
cp "$study/protocol-binary-pair.json" "$study/BINARY_PAIR_FORMAT.json" "$study/BINARY_PAIR_DESIGN.md" "$study/BinaryPairSlots.lean" "$study/BINARY_PAIR_PROOF_MAP.json" "$out/"
cp -a "$study/validation/binary-pair" "$out/development-validation"
uname -a > "$out/host.txt"
lscpu >> "$out/host.txt"
while IFS=$'\t' read -r key source inputhash systemhash;do
 mkdir "$out/inputs/$key"
 printf '%s  %s\n' "$inputhash" "$source/input.json" "$systemhash" "$source/system.sing" | sha256sum -c -
 cp "$source/input.json" "$source/system.sing" "$out/inputs/$key/"
done < <(jq -r '.extensions.cases[]|[.key,.directory,.input_sha256,.source_sha256]|@tsv' "$study/protocol-binary-pair.json")
upload
ulimit -v 2097152
set +e
timeout --signal=TERM --kill-after=5 120 "$out/instrument" run "$out" > "$out/producer.log" 2>&1
producer=$?
upload
timeout --signal=TERM --kill-after=5 120 "$out/instrument" replay "$out" > "$out/replay.log" 2>&1
replay=$?
set -e
jq -n --argjson p "$producer" --argjson r "$replay" '{producer_exit_code:$p,replay_exit_code:$r}' > "$out/status.json"
[[ "$producer" == 0 && "$replay" == 0 ]]
