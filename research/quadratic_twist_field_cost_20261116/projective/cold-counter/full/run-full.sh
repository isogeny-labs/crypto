#!/usr/bin/env bash
# One scientific worker; thin shell dispatch only. Archive helper does ONLY file/codec IO.
set -euo pipefail
mode=${1:?build or execute};out=${2:?new exact full output};ref=${3:?fresh connected reference}
root=$(git rev-parse --show-toplevel);cd "$root";scope=research/quadratic_twist_field_cost_20261116/projective/cold-counter/full
[[ $(git branch --show-current) = codex/twist-field-cost && $(git remote get-url origin) = https://github.com/isogeny-labs/crypto.git && -z $(git status --porcelain) ]]
out=$(realpath -m "$out");[[ $out = /workspace/twist-cold-counter-fullrun-20261116 ]]
run(){ local name=$1 expected=$2 remain cap code=0;shift 2;remain=$((1794828000-$(date -u +%s)));[[ $remain -gt 0 ]];cap=$remain;[[ $cap -le 600 ]]||cap=600
 printf '%q ' "$@" > "$out/$name.command";printf '\n' >> "$out/$name.command";date -u +%FT%TZ > "$out/$name.started-at"
 timeout --signal=TERM --kill-after=10 "$cap" "$@" > "$out/$name.log" 2>&1 || code=$?
 date -u +%FT%TZ > "$out/$name.finished-at";printf '%s\n' "$code" > "$out/$name.exit";printf '%s\t%s\t%s\n' "$name" "$code" "$expected" >> "$out/stages.tsv";[[ $code = "$expected" ]]
}
if [[ $mode = build ]];then
 [[ ! -e $out ]];mkdir -m 700 "$out";cp "$ref" "$out/build-ref.json"
 jq -e --arg h "$(git rev-parse HEAD)" '.head==$h and .state=="open" and .auto_merge==null and .source=="connected-github-read"' "$ref" > "$out/build-ref-check.log"
 run qualifier-build 0 g++ -std=c++20 -O2 -I/workspace/twist-native-tooling/include "$scope/qualify.cpp" -lcrypto -o "$out/qualifier.template"
 cp "$out/qualifier.template" "$out/qualifier.immutable";cmp "$out/qualifier.template" "$out/qualifier.immutable";chmod 500 "$out/qualifier.immutable";sha256sum "$out/qualifier.immutable" > "$out/qualifier-before-calls.sha256"
 # Exact successful preflight executables, with source/compiler/binary bindings in PINS.
 # Hard links preserve every original byte/path and avoid duplicate physical binary storage.
 for name in producer audit references;do ln "/workspace/twist-cold-counter-run02-20261116/$name" "$out/$name";cmp "/workspace/twist-cold-counter-run02-20261116/$name" "$out/$name";done
 run analysis-build 0 g++ -std=c++20 -O2 -I/workspace/twist-native-tooling/include "$scope/analyze.cpp" -lcrypto -o "$out/analysis"
 run protocol-check 0 python3 /workspace/canonical-protocol-20261115/tools/experiment_protocol.py validate "$scope/protocol.json"
 "$out/qualifier.immutable" build "$scope/PINS.json" "$ref" "$out" "$out/build-manifest.json" > "$out/build-qualification.log" 2>&1
elif [[ $mode = execute ]];then
 [[ -f $out/build-manifest.json && ! -e $out/pre-call-manifest.json ]];cp "$ref" "$out/call-ref.json";sha256sum -c "$out/qualifier-before-calls.sha256" > "$out/qualifier-pre-call-check.log"
 "$out/qualifier.immutable" pre "$scope/PINS.json" "$ref" "$out" "$out/pre-call-manifest.json" > "$out/pre-call-qualification.log" 2>&1
 jq '.head="0000000000000000000000000000000000000000"' "$ref" > "$out/false-reference.json"
 run reject-qualifier-head 101 "$out/qualifier.immutable" pre "$scope/PINS.json" "$out/false-reference.json" "$out" "$out/false-head-manifest.json";[[ ! -e $out/false-head-manifest.json ]];grep -q 'reference head mismatch' "$out/reject-qualifier-head.log"
 jq '(.repo_files|keys[0])as $key|.repo_files[$key]="0000000000000000000000000000000000000000000000000000000000000000"' "$scope/PINS.json" > "$out/false-pins.json"
 run reject-qualifier-source 101 "$out/qualifier.immutable" pre "$out/false-pins.json" "$ref" "$out" "$out/false-source-manifest.json";[[ ! -e $out/false-source-manifest.json ]];grep -q 'source PINS binding mismatch' "$out/reject-qualifier-source.log"
 run make-reference 0 "$out/references" /workspace/twist-native-run02-20261116/audit.json 9e48248cede5094609574f2ff78bf4f85901ae04a931bcd08755520f032ef95b /workspace/twist-rootcover-run01-20261116/audit.json 832f103357feb6beafbf636939a039058ece28f790b45d374f902b68601765fe "$out/reference.tsv.gz"
 head=$(git rev-parse HEAD)
 for position in $(seq 0 37);do
  for tag in A B;do
   index=$position;[[ $tag = A ]]||index=$((37-position));printf -v token '%02d' "$index";id="$tag-$token";first=$((index*12));end=$((first+12));[[ $end -le 455 ]]||end=455
   mkdir "$out/$id"
   run "count-$id" 0 /workspace/twist-callgrind-20261116/install/bin/valgrind --tool=callgrind --instr-atstart=yes --collect-atstart=no --cache-sim=no --branch-sim=no --dump-line=no --dump-instr=no --compress-strings=yes --compress-pos=yes "--callgrind-out-file=$out/$id/raw-counter" "$out/producer" "$tag" "$first" "$end" "$out/$id" "$out/$id/raw-counter" "$head" full
   run "audit-$id" 0 "$out/audit" "$out/$id/cases.tsv.gz" "$out/reference.tsv.gz" "$head" "$out/audit-$id.json"
   if [[ $id = A-00 ]];then
    for item in cost raw-summary missing source scalar witness uncharged-method;do
     run "corrupt-$item" 0 "$out/producer" corrupt "$out/$id" "$out/false-$item" "$item"
     run "reject-$item" 101 "$out/audit" "$out/false-$item/cases.tsv.gz" "$out/reference.tsv.gz" "$head" "$out/reject-$item-receipt.json";[[ ! -e $out/reject-$item-receipt.json ]]
     run "pack-false-$item" 0 python3 "$scope/pack-bytes.py" "$out/false-$item" "$out/false-$item.tar.xz" --remove-verified-new-copies
    done
    run audit-original-again 0 "$out/audit" "$out/$id/cases.tsv.gz" "$out/reference.tsv.gz" "$head" "$out/audit-$id-again.json";cmp "$out/audit-$id.json" "$out/audit-$id-again.json"
   fi
   run "pack-$id" 0 python3 "$scope/pack-bytes.py" "$out/$id" "$out/$id.tar.xz" --remove-verified-new-copies
  done
 done
 run analyze 0 "$out/analysis" "$out" "$head" /workspace/twist-native-run02-20261116/audit.json /workspace/twist-rootcover-run01-20261116/audit.json "$out/analysis.json"
 "$out/qualifier.immutable" post "$scope/PINS.json" "$ref" "$out" "$out/post-call-manifest.json" > "$out/post-call-qualification.log" 2>&1
else exit 2;fi
