#!/usr/bin/env bash
# Thin shell build/run orchestration; native arithmetic, qualifier and replay only.
set -euo pipefail
mode=${1:?build or execute};out=${2:?fresh outside-checkout output};ref=${3:?fresh connected GitHub ref}
root=$(git rev-parse --show-toplevel);cd "$root"
scope=research/quadratic_twist_field_cost_20261116/projective/root-cover
[[ $(git branch --show-current) = codex/twist-field-cost && $(git remote get-url origin) = https://github.com/isogeny-labs/crypto.git && -z $(git status --porcelain) ]]
out=$(realpath -m "$out");case "$out" in /workspace/twist-rootcover-run*) ;; *)exit 2;;esac
run(){ local name=$1 expected=$2 code=0 remain cap;shift 2
 remain=$((1794818400-$(date -u +%s)));[[ $remain -gt 0 ]];cap=$remain;[[ $cap -le 600 ]]||cap=600
 printf '%q ' "$@" > "$out/$name.command";printf '\n' >> "$out/$name.command"
 timeout --signal=TERM --kill-after=10 "$cap" "$@" > "$out/$name.log" 2>&1 || code=$?
 printf '%s\n' "$code" > "$out/$name.exit";printf '%s\t%s\t%s\n' "$name" "$code" "$expected" >> "$out/stages.tsv";[[ $code = "$expected" ]]
}
if [[ $mode = build ]];then
 [[ ! -e $out ]];mkdir -m 700 "$out";cp "$ref" "$out/build-ref.json"
 jq -e --arg h "$(git rev-parse HEAD)" '.head==$h and .state=="open" and .auto_merge==null and .source=="connected-github-read"' "$ref" > "$out/build-ref-check.log"
 run qualifier-build 0 g++ -std=c++20 -O2 -I/workspace/twist-native-tooling/include "$scope/qualify.cpp" -lcrypto -o "$out/qualifier.template"
 cp "$out/qualifier.template" "$out/qualifier.immutable";cmp "$out/qualifier.template" "$out/qualifier.immutable";chmod 500 "$out/qualifier.immutable";sha256sum "$out/qualifier.immutable" > "$out/qualifier-before-calls.sha256"
 run cpp-build 0 g++ -std=c++20 -O2 -Wall -Wextra -I/workspace/twist-native-tooling/include "$scope/root-cover.cpp" -lcrypto -lz -o "$out/producer"
 export CARGO_HOME=/workspace/twist-native-tooling/cargo RUSTUP_HOME=/workspace/twist-native-tooling/rustup CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR=/workspace/twist-native-run01-20261116/target PATH=/workspace/twist-native-tooling/cargo/bin:$PATH
 run rust-build 0 cargo build --release --locked --offline --manifest-path "$scope/audit/Cargo.toml";cp "$CARGO_TARGET_DIR/release/twist_root_cover_audit" "$out/audit"
 run describe-build 0 g++ -std=c++20 -O2 -I/workspace/twist-native-tooling/include "$scope/describe.cpp" -lcrypto -o "$out/describe"
 run compare-build 0 g++ -std=c++20 -O2 -I/workspace/twist-native-tooling/include "$scope/compare.cpp" -lcrypto -o "$out/compare"
 # Official canonical Python METADATA validator is the explicit exception, no research dispatch.
 run protocol-check 0 python3 /workspace/canonical-protocol-20261115/tools/experiment_protocol.py validate "$scope/protocol.json"
 "$out/qualifier.immutable" build "$scope/PINS.json" "$ref" "$out" "$out/build-manifest.json" > "$out/build-qualification.log" 2>&1
elif [[ $mode = execute ]];then
 [[ -f $out/build-manifest.json && ! -e $out/pre-call-manifest.json ]];sha256sum -c "$out/qualifier-before-calls.sha256" > "$out/qualifier-pre-call-check.log";cp "$ref" "$out/call-ref.json"
 "$out/qualifier.immutable" pre "$scope/PINS.json" "$ref" "$out" "$out/pre-call-manifest.json" > "$out/pre-call-qualification.log" 2>&1
 jq '.head="0000000000000000000000000000000000000000"' "$ref" > "$out/false-reference.json"
 run reject-qualifier-head 101 "$out/qualifier.immutable" pre "$scope/PINS.json" "$out/false-reference.json" "$out" "$out/false-head-manifest.json";[[ ! -e $out/false-head-manifest.json ]];grep -q 'reference head mismatch' "$out/reject-qualifier-head.log"
 jq '(.repo_files|keys[0])as $key|.repo_files[$key]="0000000000000000000000000000000000000000000000000000000000000000"' "$scope/PINS.json" > "$out/false-pins.json"
 run reject-qualifier-source 101 "$out/qualifier.immutable" pre "$out/false-pins.json" "$ref" "$out" "$out/false-source-manifest.json";[[ ! -e $out/false-source-manifest.json ]];grep -q 'source PINS binding mismatch' "$out/reject-qualifier-source.log"
 export PATH=/workspace/twist-lean-restore-20261115/lean-4.34.1-linux/bin:$PATH LEAN_NUM_THREADS=2 LEAN_GATE_TRUSTED=/workspace/twist-torus-20261116/official/trusted-native.txt
 paths=(/workspace/twist-lean-restore-20261115/mathlib/.lake/build/lib/lean);for d in /workspace/twist-lean-restore-20261115/mathlib/.lake/packages/*;do [[ ! -d $d/.lake/build/lib/lean ]]||paths+=("$d/.lake/build/lib/lean");done;export LEAN_PATH=$(IFS=:;printf '%s' "${paths[*]}")
 for tag in A B;do
  run "proof-$tag" 0 bash /workspace/twist-torus-20261116/official/proof-gate.sh --out "$out/proof-$tag" "$scope/RootCover.lean"
  log="$out/proof-$tag/research_quadratic_twist_field_cost_20261116_projective_root-cover_RootCover_lean/audit.log"
  grep -q '0 failing, 0 trusted' "$log";for name in original_input_identity common_root_is_cover_root filtered_cover_exact discriminant_identity quadratic_root_formula quadratic_roots_complete;do grep -q "ok    TwistRootCover.$name " "$log";done
 done
 for item in identity filter quadratic;do
  run "reject-proof-$item" 1 bash /workspace/twist-torus-20261116/official/proof-gate.sh --out "$out/reject-proof-$item" "$scope/controls/$item.lean"
  grep -Eq 'error|unsolved goals' "$out/reject-proof-$item/research_quadratic_twist_field_cost_20261116_projective_root-cover_controls_${item}_lean/build.log"
 done
 for tag in C A B;do run "produce-$tag" 0 "$out/producer" produce "$out/$tag" "$tag";done
 run independent-audit 0 "$out/audit" "$out/C" "$out/A" "$out/B" "$out/audit.json"
 input_sha=$(sha256sum "$out/audit.json");input_sha=${input_sha%% *}
 run describe 0 "$out/describe" "$out/audit.json" "$input_sha" "$out/description.json"
 run compare 0 "$out/compare" "$out/audit.json" "$input_sha" /workspace/twist-native-run02-20261116/audit.json 9e48248cede5094609574f2ff78bf4f85901ae04a931bcd08755520f032ef95b "$out/comparison.json"
 for item in map missing-policy scalar log cost source columns missing-failure cap witness root branch;do
  run "corrupt-$item" 0 "$out/producer" corrupt "$out/C" "$out/false-$item" "$item"
  run "reject-$item" 101 "$out/audit" "$out/false-$item" "$out/A" "$out/B" "$out/reject-$item-receipt.json";[[ ! -e $out/reject-$item-receipt.json ]]
 done
 run audit-original-again 0 "$out/audit" "$out/C" "$out/A" "$out/B" "$out/audit-original-again.json";cmp "$out/audit.json" "$out/audit-original-again.json"
 "$out/qualifier.immutable" post "$scope/PINS.json" "$ref" "$out" "$out/post-call-manifest.json" > "$out/post-call-qualification.log" 2>&1
else exit 2;fi
