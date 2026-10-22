#!/usr/bin/env bash
# Assemble and independently certify the complete nine-target partition panel.
set -euo pipefail
here=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
study=$(cd -- "$here/../.." && pwd)
raw=$(cd -- "$1" && pwd)
gp=${GP_BIN:-gp}
[[ ! -e $raw/final ]] || { echo 'Final records already exist' >&2; exit 1; }
mkdir -p "$raw/final"
records=()
for id in published_224-twist2 skipped_224_5736-twist2 skipped_256_876-twist2; do
  record="$raw/targets/$id/result.json"
  [[ -s $record ]]
  records+=("$record")
done
jq -s --slurpfile base "$here/baseline-results.json" \
  'INDEX(.id) as $updates|$base[0]|map($updates[.id] // .)' \
  "${records[@]}" > "$raw/final/merged.json"
while IFS= read -r target; do
  id=$(jq -r .id <<<"$target")
  expected=$(jq -r .value <<<"$target")
  out="$raw/final/$id"
  mkdir -p "$out/certs-final"
  jq --arg id "$id" '.[]|select(.id==$id)' "$raw/final/merged.json" > "$out/source.json"
  jq -r --arg out "$out" '"ID=\""+.id+"\"; N="+.value+"; BASE_PARTS=["+
    ([ (.factors+.residuals)[]|.[0] as $p|range(.[1])|$p ]|join(","))+
    "]; EXTRA_FACTORS=[]; OUT=\""+$out+"\";"' "$out/source.json" > "$out/input.gp"
  CERTIFY_INPUT="$out/input.gp" "$gp" -q -f -s 128000000 --default parisizemax=2000000000 \
    "$study/certify_parts.gp" > "$out/partition.json" 2> "$out/certify.stderr"
  jq -e '.product_verified and .known_factors_prime and .certificates_replay_valid' \
    "$out/partition.json" >/dev/null
  jq -r --arg n "$expected" --arg certs "$out/certs-final" \
    '"REPLAY=[\""+.id+"\","+$n+","+.value+",["+
    ([.factors[]|"["+.[0]+","+(.[1]|tostring)+"]"]|join(","))+"],["+
    ([.residuals[]|"["+.[0]+","+(.[1]|tostring)+"]"]|join(","))+
    "],\""+$certs+"\"];"' "$out/partition.json" > "$out/replay.gp"
  REPLAY_INPUT="$out/replay.gp" "$gp" -q -f -s 128000000 --default parisizemax=2000000000 \
    "$study/verify_factors.gp" > "$out/verification.json" 2> "$out/verify.stderr"
  jq -e .verified "$out/verification.json" >/dev/null
  jq -s '.[0]+{verification:.[1]}' "$out/partition.json" "$out/verification.json" > "$out/result.json"
done < "$here/targets.jsonl"
jq -s . "$raw"/final/*/result.json > "$raw/final/results.json"
jq -e 'length==9 and all(.[];.verification.verified)' "$raw/final/results.json" >/dev/null
echo 'Replayed and certified all nine R02 partitions.'
