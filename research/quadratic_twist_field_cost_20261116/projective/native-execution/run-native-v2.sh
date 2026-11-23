#!/usr/bin/env bash
# Thin build/run dispatch only. All qualification, arithmetic, replay and analysis are native.
set -euo pipefail
mode=${1:?build or execute};out=${2:?fresh outside-checkout output};ref=${3:?fresh connected reference}
root=$(git rev-parse --show-toplevel);cd "$root"
scope=research/quadratic_twist_field_cost_20261116/projective
native=$scope/native-execution
[[ $(git branch --show-current) = codex/twist-field-cost ]]
[[ $(git remote get-url origin) = https://github.com/isogeny-labs/crypto.git ]]
[[ -z $(git status --porcelain) ]]
out=$(realpath -m "$out");case "$out" in /workspace/twist-native-run*) ;; *)exit 2;;esac
case "$out" in "$root"|"$root"/*)exit 2;;esac
run(){
 local name=$1 expected=$2 code=0 remain cap;shift 2
 remain=$((1794815400-$(date -u +%s)));[[ $remain -gt 0 ]];cap=$remain;[[ $cap -le 600 ]]||cap=600
 printf '%q ' "$@" > "$out/$name.command";printf '\n' >> "$out/$name.command"
 if timeout --signal=TERM --kill-after=10 "$cap" "$@" > "$out/$name.log" 2>&1;then code=0;else code=$?;fi
 printf '%s\n' "$code" > "$out/$name.exit";printf '%s\t%s\t%s\n' "$name" "$code" "$expected" >> "$out/stages.tsv"
 [[ $code = "$expected" ]]
}
if [[ $mode = build ]];then
 [[ ! -e $out ]];mkdir -m 700 "$out"
 jq -e --arg h "$(git rev-parse HEAD)" '.head==$h and .state=="open" and .auto_merge==null and .source=="connected-github-read"' "$ref" > "$out/build-ref-check.log"
 cp "$ref" "$out/build-ref.json"
 run qualifier-build 0 g++ -std=c++20 -O2 -I/workspace/twist-native-tooling/include "$native/qualify-v2.cpp" -lcrypto -o "$out/qualifier.template"
 cp "$out/qualifier.template" "$out/qualifier.immutable";cmp "$out/qualifier.template" "$out/qualifier.immutable";chmod 500 "$out/qualifier.immutable"
 sha256sum "$out/qualifier.immutable" > "$out/qualifier-before-calls.sha256"
 run cpp-build 0 g++ -std=c++20 -O2 -Wall -Wextra -I/workspace/twist-native-tooling/include "$scope/projective.cpp" -lcrypto -lz -o "$out/producer"
 export CARGO_HOME=/workspace/twist-native-tooling/cargo RUSTUP_HOME=/workspace/twist-native-tooling/rustup CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR=/workspace/twist-native-run01-20261116/target
 export PATH=/workspace/twist-native-tooling/cargo/bin:$PATH
 run rust-build 0 cargo build --release --locked --offline --manifest-path "$native/audit-native/Cargo.toml"
 cp "$CARGO_TARGET_DIR/release/twist_native_bounded_audit" "$out/audit"
 run ir-cpp-build 0 g++ -std=c++20 -O2 -g -Wall -Wextra -I/workspace/twist-native-tooling/include -I/workspace/twist-callgrind-20261116/install/include "$scope/ir.cpp" -lcrypto -lz -o "$out/ir"
 run ir-rust-build 0 cargo build --release --locked --offline --manifest-path "$scope/ir-audit/Cargo.toml"
 cp "$CARGO_TARGET_DIR/release/twist_ir_audit" "$out/ir-audit"
 run describe-build 0 g++ -std=c++20 -O2 -I/workspace/twist-native-tooling/include "$scope/describe.cpp" -lcrypto -o "$out/describe"
 # Only the unchanged official protocol/schema metadata validator uses Python.
 run protocol-check 0 python3 /workspace/canonical-protocol-20261115/tools/experiment_protocol.py validate "$native/protocol-v2.json"
 "$out/qualifier.immutable" build "$native/PINS-v2.json" "$ref" "$out" "$out/build-manifest.json" > "$out/build-qualification.log" 2>&1
elif [[ $mode = execute ]];then
 [[ -f $out/build-manifest.json && ! -e $out/pre-call-manifest.json ]];sha256sum -c "$out/qualifier-before-calls.sha256" > "$out/qualifier-pre-call-check.log"
 cp "$ref" "$out/call-ref.json"
 "$out/qualifier.immutable" pre "$native/PINS-v2.json" "$ref" "$out" "$out/pre-call-manifest.json" > "$out/pre-call-qualification.log" 2>&1
 jq '.head="0000000000000000000000000000000000000000"' "$ref" > "$out/false-reference.json"
 run reject-qualifier-head 101 "$out/qualifier.immutable" pre "$native/PINS-v2.json" "$out/false-reference.json" "$out" "$out/false-reference-manifest.json"
 [[ ! -e $out/false-reference-manifest.json ]];grep -q 'reference head mismatch' "$out/reject-qualifier-head.log"
 jq '(.repo_files|keys[0])as $key|.repo_files[$key]="0000000000000000000000000000000000000000000000000000000000000000"' "$native/PINS-v2.json" > "$out/false-pins.json"
 run reject-qualifier-source 101 "$out/qualifier.immutable" pre "$out/false-pins.json" "$ref" "$out" "$out/false-source-manifest.json"
 [[ ! -e $out/false-source-manifest.json ]];grep -q 'source PINS binding mismatch' "$out/reject-qualifier-source.log"
 for tag in C A B;do run "produce-$tag" 0 "$out/producer" produce "$out/$tag" "$tag";done
 run independent-audit 0 "$out/audit" "$out/C" "$out/A" "$out/B" "$out/audit.json"
 input_sha=$(sha256sum "$out/audit.json");input_sha=${input_sha%% *}
 run describe 0 "$out/describe" "$out/audit.json" "$input_sha" "$out/description.json"
 for item in map missing-policy target scalar log cost degree witness root source columns missing-failure;do
  run "reject-$item" 101 "$out/audit" "/workspace/twist-projective-run03-20261116/false-$item" "$out/A" "$out/B" "$out/reject-$item-receipt.json"
  [[ ! -e $out/reject-$item-receipt.json ]]
 done
 for tag in A B;do
  mkdir "$out/IR-$tag"
  run "count-IR-$tag" 0 /workspace/twist-callgrind-20261116/install/bin/valgrind --tool=callgrind --instr-atstart=yes --collect-atstart=no --cache-sim=no --branch-sim=no "--callgrind-out-file=$out/IR-$tag/counter" "$out/ir" "$out/IR-$tag/cases.json" "$out/IR-$tag/counter"
  run "audit-IR-$tag" 0 "$out/ir-audit" "$out/IR-$tag/cases.json" "$out/audit.json" "$out/audit-IR-$tag.json"
 done
 for item in cost raw-summary missing source;do
  run "reject-IR-$item" 101 "$out/ir-audit" "/workspace/twist-ir-run01-20261116/false-$item/cases.json" "$out/audit.json" "$out/reject-IR-$item-receipt.json"
  [[ ! -e $out/reject-IR-$item-receipt.json ]]
 done
 "$out/qualifier.immutable" post "$native/PINS-v2.json" "$ref" "$out" "$out/post-call-manifest.json" > "$out/post-call-qualification.log" 2>&1
else exit 2;fi
