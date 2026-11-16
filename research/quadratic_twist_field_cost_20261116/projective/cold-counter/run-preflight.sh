#!/usr/bin/env bash
set -euo pipefail
mode=${1:?build or execute};out=${2:?fresh outside-checkout output};ref=${3:?fresh connected ref}
root=$(git rev-parse --show-toplevel);cd "$root";scope=research/quadratic_twist_field_cost_20261116/projective/cold-counter
[[ $(git branch --show-current) = codex/twist-field-cost && $(git remote get-url origin) = https://github.com/isogeny-labs/crypto.git && -z $(git status --porcelain) ]]
out=$(realpath -m "$out");case "$out" in /workspace/twist-cold-counter-run*) ;; *)exit 2;;esac
run(){ local name=$1 expected=$2 remain cap code=0;shift 2;remain=$((1794821400-$(date -u +%s)));[[ $remain -gt 0 ]];cap=$remain;[[ $cap -le 600 ]]||cap=600
 printf '%q ' "$@" > "$out/$name.command";printf '\n' >> "$out/$name.command";timeout --signal=TERM --kill-after=10 "$cap" "$@" > "$out/$name.log" 2>&1 || code=$?
 printf '%s\n' "$code" > "$out/$name.exit";printf '%s\t%s\t%s\n' "$name" "$code" "$expected" >> "$out/stages.tsv";[[ $code = "$expected" ]]
}
if [[ $mode = build ]];then
 [[ ! -e $out ]];mkdir -m 700 "$out";cp "$ref" "$out/build-ref.json"
 jq -e --arg h "$(git rev-parse HEAD)" '.head==$h and .state=="open" and .auto_merge==null and .source=="connected-github-read"' "$ref" > "$out/build-ref-check.log"
 run qualifier-build 0 g++ -std=c++20 -O2 -I/workspace/twist-native-tooling/include "$scope/qualify.cpp" -lcrypto -o "$out/qualifier.template"
 cp "$out/qualifier.template" "$out/qualifier.immutable";cmp "$out/qualifier.template" "$out/qualifier.immutable";chmod 500 "$out/qualifier.immutable";sha256sum "$out/qualifier.immutable" > "$out/qualifier-before-calls.sha256"
 run cpp-build 0 g++ -std=c++20 -O2 -Wall -Wextra -I/workspace/twist-native-tooling/include -I/workspace/twist-callgrind-20261116/install/include "$scope/counter.cpp" -lcrypto -lz -o "$out/producer"
 run rust-build 0 /workspace/twist-native-tooling/rustup/toolchains/1.85.1-x86_64-unknown-linux-gnu/bin/rustc --edition=2021 -O "$scope/audit.rs" -o "$out/audit"
 run references-build 0 g++ -std=c++20 -O2 -I/workspace/twist-native-tooling/include "$scope/references.cpp" -lcrypto -lz -o "$out/references"
 # Only official Python METADATA validation is used. No Python dispatch/arithmetic.
 run protocol-check 0 python3 /workspace/canonical-protocol-20261115/tools/experiment_protocol.py validate "$scope/protocol-preflight.json"
 "$out/qualifier.immutable" build "$scope/PINS-preflight.json" "$ref" "$out" "$out/build-manifest.json" > "$out/build-qualification.log" 2>&1
elif [[ $mode = execute ]];then
 [[ -f $out/build-manifest.json && ! -e $out/pre-call-manifest.json ]];cp "$ref" "$out/call-ref.json";sha256sum -c "$out/qualifier-before-calls.sha256" > "$out/qualifier-pre-call-check.log"
 "$out/qualifier.immutable" pre "$scope/PINS-preflight.json" "$ref" "$out" "$out/pre-call-manifest.json" > "$out/pre-call-qualification.log" 2>&1
 jq '.head="0000000000000000000000000000000000000000"' "$ref" > "$out/false-reference.json"
 run reject-qualifier-head 101 "$out/qualifier.immutable" pre "$scope/PINS-preflight.json" "$out/false-reference.json" "$out" "$out/false-head-manifest.json";[[ ! -e $out/false-head-manifest.json ]];grep -q 'reference head mismatch' "$out/reject-qualifier-head.log"
 jq '(.repo_files|keys[0])as $key|.repo_files[$key]="0000000000000000000000000000000000000000000000000000000000000000"' "$scope/PINS-preflight.json" > "$out/false-pins.json"
 run reject-qualifier-source 101 "$out/qualifier.immutable" pre "$out/false-pins.json" "$ref" "$out" "$out/false-source-manifest.json";[[ ! -e $out/false-source-manifest.json ]];grep -q 'source PINS binding mismatch' "$out/reject-qualifier-source.log"
 run make-reference 0 "$out/references" /workspace/twist-native-run02-20261116/audit.json 9e48248cede5094609574f2ff78bf4f85901ae04a931bcd08755520f032ef95b /workspace/twist-rootcover-run01-20261116/audit.json 832f103357feb6beafbf636939a039058ece28f790b45d374f902b68601765fe "$out/reference.tsv.gz"
 head=$(git rev-parse HEAD)
 for tag in A B;do
  mkdir "$out/$tag"
  run "count-$tag" 0 /workspace/twist-callgrind-20261116/install/bin/valgrind --tool=callgrind --instr-atstart=yes --collect-atstart=no --cache-sim=no --branch-sim=no --dump-line=no --dump-instr=no --compress-strings=yes --compress-pos=yes "--callgrind-out-file=$out/$tag/raw-counter" "$out/producer" "$tag" 0 1 "$out/$tag" "$out/$tag/raw-counter" "$head" preflight
  run "audit-$tag" 0 "$out/audit" "$out/$tag/cases.tsv.gz" "$out/reference.tsv.gz" "$head" "$out/audit-$tag.json"
 done
 for item in cost raw-summary missing source scalar witness uncharged-method;do
  run "corrupt-$item" 0 "$out/producer" corrupt "$out/A" "$out/false-$item" "$item"
  run "reject-$item" 101 "$out/audit" "$out/false-$item/cases.tsv.gz" "$out/reference.tsv.gz" "$head" "$out/reject-$item-receipt.json";[[ ! -e $out/reject-$item-receipt.json ]]
 done
 for tag in A B;do run "audit-$tag-again" 0 "$out/audit" "$out/$tag/cases.tsv.gz" "$out/reference.tsv.gz" "$head" "$out/audit-$tag-again.json";cmp "$out/audit-$tag.json" "$out/audit-$tag-again.json";done
 "$out/qualifier.immutable" post "$scope/PINS-preflight.json" "$ref" "$out" "$out/post-call-manifest.json" > "$out/post-call-qualification.log" 2>&1
else exit 2;fi
