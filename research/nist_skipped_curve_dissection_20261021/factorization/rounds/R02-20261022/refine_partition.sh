#!/usr/bin/env bash
# Certify hints against their source cofactor, then replay the original target.
set -euo pipefail
here=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
study=$(cd -- "$here/../.." && pwd)
parent=$1 hints=$2 out=$3 cofactor=$4
[[ $cofactor =~ ^[0-9]+$ ]]
jq -e '(.id|test("^[a-zA-Z0-9_-]+$")) and (.value|test("^[0-9]+$")) and
  all((.factors+.residuals)[]; (.[0]|test("^[0-9]+$")) and
    (.[1]|type=="number" and .>0 and .==floor))' "$parent" >/dev/null
mkdir -p "$out"
[[ -z $(find "$out" -mindepth 1 -maxdepth 1 -print -quit) ]] || exit 1
out=$(cd -- "$out" && pwd)
mkdir -p "$out/certs-final"
id=$(jq -r .id "$parent")
expected=$(jq -r --arg id "$id" 'select(.id==$id)|.value' "$here/targets.jsonl")
[[ $expected =~ ^[0-9]+$ ]]
jq -r --arg out "$out" --arg c "$cofactor" '"ID=\""+.id+"\"; N="+.value+
  "; SOURCE_C="+$c+"; BASE_PARTS=["+
  ([ (.factors+.residuals)[]|.[0] as $p|range(.[1])|$p ]|join(","))+
  "]; OUT=\""+$out+"\";"' "$parent" > "$out/input.gp"
jq -Rs -r '"EXTRA_FACTORS=["+(split("\n")|map(select(test("^[0-9]+$")))|unique|join(","))+"];"' \
  "$hints" >> "$out/input.gp"
printf 'foreach(EXTRA_FACTORS,q,if(!(q>1 && q<SOURCE_C && SOURCE_C%%q==0),error("not a proper source-cofactor divisor")));\n' >> "$out/input.gp"
gp=${GP_BIN:-gp}
CERTIFY_INPUT="$out/input.gp" "$gp" -q -f -s 128000000 --default parisizemax=2000000000 \
  "$study/certify_parts.gp" > "$out/partition.json" 2> "$out/certify.stderr"
jq -e '.product_verified and .known_factors_prime and .certificates_replay_valid' "$out/partition.json" >/dev/null
jq -r --arg n "$expected" --arg certs "$out/certs-final" \
  '"REPLAY=[\""+.id+"\","+$n+","+.value+",["+
  ([.factors[]|"["+.[0]+","+(.[1]|tostring)+"]"]|join(","))+"],["+
  ([.residuals[]|"["+.[0]+","+(.[1]|tostring)+"]"]|join(","))+
  "],\""+$certs+"\"];"' "$out/partition.json" > "$out/replay.gp"
REPLAY_INPUT="$out/replay.gp" "$gp" -q -f -s 128000000 --default parisizemax=2000000000 \
  "$study/verify_factors.gp" > "$out/verification.json" 2> "$out/verify.stderr"
jq -e .verified "$out/verification.json" >/dev/null
jq -s '.[0]+{verification:.[1]}' "$out/partition.json" "$out/verification.json" > "$out/result.json"
