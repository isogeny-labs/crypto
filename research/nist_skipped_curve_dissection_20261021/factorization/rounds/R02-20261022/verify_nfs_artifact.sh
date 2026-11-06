#!/usr/bin/env bash
# Verify a resumed job against the card's explicit fixed input, not its labels.
set -euo pipefail
here=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
study=$(cd -- "$here/../.." && pwd)
artifact=$(cd -- "$1" && pwd)
id=$2 cofactor=$3
[[ $id == published_224-twist2 || $id == skipped_256_876-twist2 ]]
[[ $cofactor =~ ^[0-9]+$ && $(cat "$artifact/input.txt") == "$cofactor" ]]
expected=$(jq -r --arg id "$id" 'select(.id==$id)|.value' "$here/targets.jsonl")
jq -e --arg id "$id" --arg n "$expected" '.id==$id and .value==$n and
  all((.factors+.residuals)[]; (.[0]|test("^[0-9]+$")) and
    (.[1]|type=="number" and .>0 and .==floor))' "$artifact/result.json" >/dev/null
index=1
last=
while [[ -d $artifact/refinement-$index ]]; do last="$artifact/refinement-$index"; index=$((index+1)); done
[[ -n $last ]]
temp=$(mktemp -d)
trap 'rm -rf -- "$temp"' EXIT
jq -r --arg n "$expected" --arg certs "$last/certs-final" \
  '"REPLAY=[\""+.id+"\","+$n+","+.value+",["+
  ([.factors[]|"["+.[0]+","+(.[1]|tostring)+"]"]|join(","))+"],["+
  ([.residuals[]|"["+.[0]+","+(.[1]|tostring)+"]"]|join(","))+
  "],\""+$certs+"\"];"' "$artifact/result.json" > "$temp/replay.gp"
REPLAY_INPUT="$temp/replay.gp" "${GP_BIN:-gp}" -q -f -s 128000000 \
  --default parisizemax=2000000000 "$study/verify_factors.gp" > "$temp/verification.json"
jq -e .verified "$temp/verification.json" >/dev/null
cat "$temp/verification.json"
