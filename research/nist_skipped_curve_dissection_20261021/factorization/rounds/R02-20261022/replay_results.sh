#!/usr/bin/env bash
# Replay exact targets, factor products and PARI primality certificates.
set -euo pipefail
here=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
study=$(cd -- "$here/../.." && pwd)
root=$(git -C "$here" rev-parse --show-toplevel)
raw=$(cd -- "$1" && pwd)
out=$2
mkdir -p "$out"
[[ -z $(find "$out" -mindepth 1 -maxdepth 1 -print -quit) ]] || {
  echo 'Choose a fresh output directory' >&2; exit 1;
}
out=$(cd -- "$out" && pwd)
gp=${GP_BIN:-gp}
fixture="$root/backlog/XP-20261021-nist-skipped-curves/curves.json"
# Validate before emitting GP literals; never convert integers through jq numbers.
jq -e 'all(.curves[]; (.name|test("^[a-zA-Z0-9_]+$")) and
  all([.field.p,.params.a.raw,.params.b.raw,.order][];
      test("^0x[0-9a-fA-F]+$")))' "$fixture" >/dev/null
jq -r '"INPUTS=["+([.curves[]|"[\""+.name+"\","+.field.p+","+
  .params.a.raw+","+.params.b.raw+","+.order+"]"]|join(","))+"];"' \
  "$fixture" > "$out/inputs.gp"
CURVE_INPUTS="$out/inputs.gp" "$gp" -q -f "$study/targets.gp" > "$out/targets.jsonl"
cmp "$out/targets.jsonl" "$here/targets.jsonl"
[[ $(jq length "${RESULTS_JSON:-$here/results.json}") == 9 ]]
while IFS= read -r target; do
  id=$(jq -r .id <<<"$target")
  expected=$(jq -r .value <<<"$target")
  jq -e --arg id "$id" 'map(select(.id==$id)) | length==1' "${RESULTS_JSON:-$here/results.json}" >/dev/null
  jq --arg id "$id" '.[]|select(.id==$id)' "${RESULTS_JSON:-$here/results.json}" > "$out/$id.result.json"
  jq -e 'all((.factors+.residuals)[]; (.[0]|test("^[0-9]+$")) and
    (.[1]|type=="number" and .>0 and .==floor)) and
    (.value|test("^[0-9]+$"))' "$out/$id.result.json" >/dev/null
  jq -r --arg n "$expected" --arg certs "$raw/final/$id/certs-final" \
    '"REPLAY=[\""+.id+"\","+$n+","+.value+",["+
    ([.factors[]|"["+.[0]+","+(.[1]|tostring)+"]"]|join(","))+"],["+
    ([.residuals[]|"["+.[0]+","+(.[1]|tostring)+"]"]|join(","))+
    "],\""+$certs+"\"];"' "$out/$id.result.json" > "$out/$id.gp"
  REPLAY_INPUT="$out/$id.gp" "$gp" -q -f -s 128000000 \
    --default parisizemax=2000000000 "$study/verify_factors.gp" \
    > "$out/$id.verification.json" 2> "$out/$id.stderr"
  jq -e .verified "$out/$id.verification.json" >/dev/null
  jq -S . "$out/$id.verification.json" > "$out/$id.actual.json"
  jq -S .verification "$out/$id.result.json" > "$out/$id.expected.json"
  cmp "$out/$id.actual.json" "$out/$id.expected.json"
done < "$out/targets.jsonl"
echo 'Verified all nine exact target partitions and their primality certificates.'
