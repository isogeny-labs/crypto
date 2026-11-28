#!/usr/bin/env bash
# Thin shell only. All research arithmetic, replay and analysis native.
set -euo pipefail
mode=${1:?build|execute};out=${2:?new outside output};ref=${3:?connected reference}
root=$(git rev-parse --show-toplevel);cd "$root";scope=research/quadratic_twist_field_cost_20261116/projective/cubic-complete
[[ $(git branch --show-current) = codex/twist-field-cost && $(git remote get-url origin) = https://github.com/isogeny-labs/crypto.git && -z $(git status --porcelain) ]]
out=$(realpath -m "$out");case "$out" in /workspace/twist-cubic-complete-run*) ;; *)exit 2;;esac
run(){ local name=$1 expected=$2 cap remain code=0;shift 2;remain=$((1794831300-$(date -u +%s)));[[ $remain -gt 0 ]];cap=$remain;[[ $cap -le 600 ]]||cap=600
 printf '%q ' "$@" > "$out/$name.command";printf '\n' >> "$out/$name.command";date -u +%FT%TZ > "$out/$name.started-at"
 timeout --signal=TERM --kill-after=10 "$cap" "$@" > "$out/$name.log" 2>&1 || code=$?
 date -u +%FT%TZ > "$out/$name.finished-at";printf '%s\n' "$code" > "$out/$name.exit";printf '%s\t%s\t%s\n' "$name" "$code" "$expected" >> "$out/stages.tsv";[[ $code = "$expected" ]]
}
if [[ $mode = build ]];then
 [[ ! -e $out ]];mkdir -m 700 "$out";cp "$ref" "$out/build-ref.json"
 run qualifier-build 0 g++ -std=c++20 -Os -s -I/workspace/twist-native-tooling/include "$scope/qualify.cpp" -lcrypto -o "$out/qualifier.template"
 cp "$out/qualifier.template" "$out/qualifier.immutable";cmp "$out/qualifier.template" "$out/qualifier.immutable";chmod 500 "$out/qualifier.immutable";sha256sum "$out/qualifier.immutable" > "$out/qualifier-before-calls.sha256"
 run cpp-build 0 g++ -std=c++20 -Os -s -ffunction-sections -fdata-sections -Wl,--gc-sections -I/workspace/twist-native-tooling/include "$scope/cubic-complete.cpp" -lcrypto -lz -o "$out/producer"
 run rust-build 0 /workspace/twist-native-tooling/rustup/toolchains/1.85.1-x86_64-unknown-linux-gnu/bin/rustc --edition=2021 -O -C strip=symbols -C codegen-units=1 "$scope/audit.rs" -o "$out/audit"
 run compare-build 0 g++ -std=c++20 -Os -s -I/workspace/twist-native-tooling/include "$scope/compare.cpp" -lcrypto -o "$out/compare"
 run protocol-check 0 python3 /workspace/canonical-protocol-20261115/tools/experiment_protocol.py validate "$scope/protocol.json"
 "$out/qualifier.immutable" build "$scope/PINS.json" "$ref" "$out" "$out/build-manifest.json" > "$out/build-qualification.log" 2>&1
elif [[ $mode = execute ]];then
 [[ -f $out/build-manifest.json && ! -e $out/pre-call-manifest.json ]];cp "$ref" "$out/call-ref.json";sha256sum -c "$out/qualifier-before-calls.sha256" > "$out/qualifier-pre-call-check.log"
 "$out/qualifier.immutable" pre "$scope/PINS.json" "$ref" "$out" "$out/pre-call-manifest.json" > "$out/pre-call-qualification.log" 2>&1
 jq '.head="0000000000000000000000000000000000000000"' "$ref" > "$out/false-reference.json"
 run reject-qualifier-head 101 "$out/qualifier.immutable" pre "$scope/PINS.json" "$out/false-reference.json" "$out" "$out/false-head-manifest.json";[[ ! -e $out/false-head-manifest.json ]];grep -q 'reference head mismatch' "$out/reject-qualifier-head.log"
 jq '(.repo_files|keys[0])as $key|.repo_files[$key]="0000000000000000000000000000000000000000000000000000000000000000"' "$scope/PINS.json" > "$out/false-pins.json"
 run reject-qualifier-source 101 "$out/qualifier.immutable" pre "$out/false-pins.json" "$ref" "$out" "$out/false-source-manifest.json";[[ ! -e $out/false-source-manifest.json ]];grep -q 'source PINS binding mismatch' "$out/reject-qualifier-source.log"
 export PATH=/workspace/twist-lean-restore-20261115/lean-4.34.1-linux/bin:$PATH LEAN_NUM_THREADS=2 LEAN_GATE_TRUSTED=/workspace/twist-torus-20261116/official/trusted-native.txt
 paths=(/workspace/twist-lean-restore-20261115/mathlib/.lake/build/lib/lean);for dir in /workspace/twist-lean-restore-20261115/mathlib/.lake/packages/*;do [[ ! -d $dir/.lake/build/lib/lean ]]||paths+=("$dir/.lake/build/lib/lean");done;export LEAN_PATH=$(IFS=:;printf '%s' "${paths[*]}")
 for tag in A B;do
  run "proof-$tag" 0 bash /workspace/twist-torus-20261116/official/proof-gate.sh --out "$out/proof-$tag" "$scope/Cardano.lean"
  log="$out/proof-$tag/research_quadratic_twist_field_cost_20261116_projective_cubic-complete_Cardano_lean/audit.log"
  grep -q '0 failing, 0 trusted' "$log";for name in depress_identity resolvent_product proportional_common_root cardano_factorization cardano_roots_complete cube_pow61 cube_pow3721;do grep -q "ok    TwistCubicComplete.$name " "$log";done
 done
 for item in exponent proportional factor;do
  run "reject-proof-$item" 1 bash /workspace/twist-torus-20261116/official/proof-gate.sh --out "$out/reject-proof-$item" "$scope/controls/$item.lean"
  grep -Eq 'error|unsolved goals' "$out/reject-proof-$item/research_quadratic_twist_field_cost_20261116_projective_cubic-complete_controls_${item}_lean/build.log"
 done
 head=$(git rev-parse HEAD)
 run selftest-produce 0 bash -o pipefail -c '"$1" selftest "$2" | xz -T1 -3 > "$3"' _ "$out/producer" "$head" "$out/selftest.xz"
 run selftest-audit 0 bash -o pipefail -c 'xz -dc "$1" | "$2" selftest "$3" > "$4"' _ "$out/selftest.xz" "$out/audit" "$head" "$out/selftest-audit.json"
 run produce-C 0 bash -o pipefail -c '"$1" produce C "$2" | xz -T1 -3 > "$3"' _ "$out/producer" "$head" "$out/C.xz"
 for tag in A B;do run "produce-$tag" 0 bash -o pipefail -c '"$1" produce "$2" "$3" > "$4"' _ "$out/producer" "$tag" "$head" "$out/$tag.digest";done
 run independent-audit 0 bash -o pipefail -c 'xz -dc "$1" | "$2" audit "$3" "$4" "$5" | xz -T1 -3 > "$6"' _ "$out/C.xz" "$out/audit" "$head" "$out/A.digest" "$out/B.digest" "$out/audit.json.xz"
 run compare 0 bash -o pipefail -c 'xz -dc "$1" | "$2" "$3" /workspace/twist-native-run02-20261116/audit.json /workspace/twist-rootcover-run01-20261116/audit.json "$4"' _ "$out/audit.json.xz" "$out/compare" "$head" "$out/comparison.json"
 for item in source columns cap witness root row scalar log cost rank branch missing-query missing-policy missing-empty-branch drop-proportional;do
  run "corrupt-$item" 0 bash -o pipefail -c 'xz -dc "$1" | "$2" corrupt "$3" > "$4"' _ "$out/C.xz" "$out/producer" "$item" "$out/false-$item.delta"
  run "hash-false-$item" 0 bash -o pipefail -c 'xz -dc "$1" | "$2" apply "$3" "$4" | sha256sum > "$5"' _ "$out/C.xz" "$out/producer" "$out/false-$item.delta" "$out/false-$item.materialized.sha256" "$out/false-$item.stream.sha256"
  read -r actual _ < "$out/false-$item.stream.sha256";read -r expected < "$out/false-$item.materialized.sha256";[[ $actual = "$expected" ]]
  run "reject-$item" 101 bash -o pipefail -c 'xz -dc "$1" | "$2" apply "$3" "$4" | "$5" audit "$6" "$7" "$8" > "$9"' _ "$out/C.xz" "$out/producer" "$out/false-$item.delta" "$out/false-$item.reject.sha256" "$out/audit" "$head" "$out/A.digest" "$out/B.digest" "$out/reject-$item.stdout"
  [[ ! -s $out/reject-$item.stdout ]];cmp "$out/false-$item.materialized.sha256" "$out/false-$item.reject.sha256"
 done
 run audit-original-again 0 bash -o pipefail -c 'xz -dc "$1" | "$2" audit "$3" "$4" "$5" | xz -T1 -3 > "$6"' _ "$out/C.xz" "$out/audit" "$head" "$out/A.digest" "$out/B.digest" "$out/audit-again.json.xz"
 cmp "$out/audit.json.xz" "$out/audit-again.json.xz"
 "$out/qualifier.immutable" post "$scope/PINS.json" "$ref" "$out" "$out/post-call-manifest.json" > "$out/post-call-qualification.log" 2>&1
else exit 2;fi
