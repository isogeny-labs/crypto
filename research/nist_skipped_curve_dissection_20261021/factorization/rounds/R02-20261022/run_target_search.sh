#!/usr/bin/env bash
# Run the frozen R02 allocation; certify and separately replay every split.
set -euo pipefail
here=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
study=$(cd -- "$here/../.." && pwd)
id=$1 out=$2 hint=$3
[[ $id =~ ^[a-zA-Z0-9_-]+$ ]]
mkdir -p "$out"
[[ -z $(find "$out" -mindepth 1 -maxdepth 1 -print -quit) ]] || exit 1
out=$(cd -- "$out" && pwd)
gp=${GP_BIN:-gp}
jq --arg id "$id" '.[]|select(.id==$id)' "$here/baseline-results.json" > "$out/initial.json"
[[ $(jq -r .status "$out/initial.json") == partial ]]
expected=$(jq -r .value "$out/initial.json")
current="$out/initial.json"
index=0
partial() { jq -e '.status=="partial"' "$current" >/dev/null; }
largest() { jq -r '.residuals|sort_by(.[0]|[length,.])|last|.[0]' "$current"; }
certify() {
  local hints=$1 snapshot=$2
  [[ -s $hints ]] || return 0
  mkdir -p "$snapshot/certs-final"
  jq -r --arg out "$snapshot" '"ID=\""+.id+"\"; N="+.value+"; BASE_PARTS=["+
    ([ (.factors+.residuals)[]|.[0] as $p|range(.[1])|$p ]|join(","))+
    "]; OUT=\""+$out+"\";"' "$current" > "$snapshot/input.gp"
  jq -Rs -r '"EXTRA_FACTORS=["+(split("\n")|map(select(test("^[0-9]+$")))|join(","))+"];"' \
    "$hints" >> "$snapshot/input.gp"
  CERTIFY_INPUT="$snapshot/input.gp" timeout --kill-after=10s 300 "$gp" -q -f \
    -s 128000000 --default parisizemax=2000000000 "$study/certify_parts.gp" \
    > "$snapshot/partition.json" 2> "$snapshot/certify.stderr"
  jq -e '.product_verified and .known_factors_prime and .certificates_replay_valid' \
    "$snapshot/partition.json" >/dev/null
  jq -r --arg n "$expected" --arg certs "$snapshot/certs-final" \
    '"REPLAY=[\""+.id+"\","+$n+","+.value+",["+
    ([.factors[]|"["+.[0]+","+(.[1]|tostring)+"]"]|join(","))+"],["+
    ([.residuals[]|"["+.[0]+","+(.[1]|tostring)+"]"]|join(","))+
    "],\""+$certs+"\"];"' "$snapshot/partition.json" > "$snapshot/replay.gp"
  REPLAY_INPUT="$snapshot/replay.gp" timeout --kill-after=10s 300 "$gp" -q -f \
    -s 128000000 --default parisizemax=2000000000 "$study/verify_factors.gp" \
    > "$snapshot/verification.json" 2> "$snapshot/verify.stderr"
  jq -e .verified "$snapshot/verification.json" >/dev/null
  jq -s '.[0]+{verification:.[1]}' "$snapshot/partition.json" "$snapshot/verification.json" > "$snapshot/result.json"
  current="$snapshot/result.json"
}
attempt() {
  local method=$1 b1=$2 runs=$3 b2=$4 start=$5 seconds=$6
  index=$((index+1))
  last="$out/attempt-$index-$method"
  bash "$here/run_method.sh" "$method" "$(largest)" "$last" "$b1" "$runs" "$b2" "$start" "$seconds"
  certify "$last/candidate-factors.txt" "$last/certified"
}
if [[ -f $hint ]]; then
  cp "$hint" "$out/factordb.json"
  original=$(largest)
  jq -r --arg c "$original" '.factors[][0]|select(test("^[0-9]+$") and .!=$c and .!="1")' \
    "$hint" > "$out/public-hints.txt"
  certify "$out/public-hints.txt" "$out/public-hints-certified"
fi
if partial; then attempt pm1 10000000 1 10000000000 2 300; fi
for start in 6 7 11 13 17 19 23 29; do
  partial || break
  attempt pp1 1000000 1 1000000000 "$start" 90
done
left=100
while ((left>0)) && partial; do
  attempt ecm 11000000 "$left" 0 0 900
  used=$(jq -r .attempted_runs "$last/receipt.json")
  ((used>0 && used<=left)) || { echo 'Invalid ECM accounting' >&2; exit 1; }
  left=$((left-used))
  [[ -s $last/candidate-factors.txt ]] || break
done
if partial; then
  pari="$out/pari-direct"
  mkdir -p "$pari"
  printf 'C=%s; OUT="%s";\n' "$(largest)" "$pari" > "$pari/input.gp"
  date -u +%FT%TZ > "$pari/started.txt"
  set +e
  SEARCH_INPUT="$pari/input.gp" timeout --kill-after=10s 650 "$gp" -q -f \
    -s 128000000 --default parisizemax=2000000000 "$here/pari_search.gp" \
    > "$pari/stdout.txt" 2> "$pari/stderr.txt"
  status=$?
  set -e
  printf '%s\n' "$status" > "$pari/exit-status.txt"
  date -u +%FT%TZ > "$pari/finished.txt"
  [[ $status == 0 && -s $pari/proposal.json ]]
  jq -r '.candidate_factors[]' "$pari/proposal.json" > "$pari/candidate-factors.txt"
  certify "$pari/candidate-factors.txt" "$pari/certified"
fi
cp "$current" "$out/result.json"
jq -n --argjson unused "$left" --argjson attempts "$index" \
  '{schema:"nist-factor-target-attempt/v2",unused_ecm_allocation:$unused,gmp_ecm_processes:$attempts}' \
  > "$out/accounting.json"
echo "R02 finished: $(jq -r '.id+": "+.status' "$out/result.json")"
