#!/usr/bin/env bash
# Replay the retained public API responses without making another network request.
set -euo pipefail
here=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
results="$here/../rounds/R02-20261022/results.json"
raw="$here/raw"
out=$1
mkdir -p "$out"
[[ -z $(find "$out" -mindepth 1 -maxdepth 1 -print -quit) ]]
out=$(cd -- "$out" && pwd)
validate_api() {
  jq -e '(.status|type=="string") and (.factors|type=="array" and length>0) and
    all(.factors[]; length==2 and (.[0]|type=="string" and test("^[1-9][0-9]*$")) and
      (.[1]|type=="number" and .>=1 and .<=1024 and .==floor))' "$1" >/dev/null
}
jq -e 'length==9 and all(.[]; (.id|test("^[a-zA-Z0-9_-]+$")) and
  (.value|test("^[1-9][0-9]*$")) and all((.factors+.residuals)[];
    (.[0]|test("^[1-9][0-9]*$")) and (.[1]|type=="number" and .>=1 and .<=1024 and .==floor)))' \
  "$results" >/dev/null
printf 'DATA=List();\n' > "$out/inputs.gp"
while IFS= read -r id; do
  validate_api "$raw/$id.json"
  jq -r --arg id "$id" --slurpfile api "$raw/$id.json" '
    def parts: map("["+.[0]+","+(.[1]|tostring)+"]")|join(",");
    .[]|select(.id==$id)|"listput(DATA,[\""+.id+"\","+.value+",["+
      ($api[0].factors|parts)+"],["+((.factors+.residuals)|parts)+"]]);"' \
    "$results" >> "$out/inputs.gp"
done < <(jq -r '.[].id' "$results")
validate_api "$raw/cofactor.json"
cofactor=33492266050798185653682380062483799213318454801653963493134064721059735936944520907295931496614455858076048437184499090478420204888353
jq -r --arg n "$cofactor" '"listput(DATA,[\"remaining_cofactor\","+$n+",["+
  ([.factors[]|"["+.[0]+","+(.[1]|tostring)+"]"]|join(","))+"],[["+$n+",1]]]);"' \
  "$raw/cofactor.json" >> "$out/inputs.gp"
FDB_INPUTS="$out/inputs.gp" "${GP_BIN:-gp}" -q -f "$here/verify_partitions.gp" \
  > "$out/verification.jsonl" 2> "$out/stderr.txt"
jq -s -e 'length==11 and ([.[]|select(has("target"))]|length==10) and
  all(.[]|select(has("target")); .api_product_verified and .local_product_verified and
    .api_partition_is_coarsening_of_local) and
  any(.[]; .control=="wrong_integer" and .rejected)' "$out/verification.jsonl" >/dev/null
cat "$out/verification.jsonl"
