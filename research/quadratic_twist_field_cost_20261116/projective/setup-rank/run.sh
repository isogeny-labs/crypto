#!/usr/bin/env bash
# Thin shell scientific dispatch. Python only official protocol and byte/archive IO.
set -euo pipefail
mode=${1:?build or execute};out=${2:?fresh output};ref=${3:?connected reference}
cd "$(git rev-parse --show-toplevel)";scope=research/quadratic_twist_field_cost_20261116/projective/setup-rank
[[ $(git branch --show-current) = codex/twist-field-cost && $(git remote get-url origin) = https://github.com/isogeny-labs/crypto.git && -z $(git status --porcelain) ]]
[[ $out = /tmp/twist-setup-rank-run02-20261116 ]]
run(){ local name=$1 expected=$2 remaining cap code=0;shift 2;remaining=$(($(date -u -d '2026-11-16T18:30:00Z' +%s)-$(date -u +%s)));[[ $remaining -gt 0 ]];cap=$remaining;[[ $cap -le 600 ]]||cap=600
 printf '%q ' "$@" > "$out/$name.command";printf '\n' >> "$out/$name.command";date -u +%FT%TZ > "$out/$name.started-at"
 timeout --signal=TERM --kill-after=10 "$cap" "$@" > "$out/$name.log" 2>&1 || code=$?
 date -u +%FT%TZ > "$out/$name.finished-at";printf '%s\n' "$code" > "$out/$name.exit";printf '%s\t%s\t%s\n' "$name" "$code" "$expected" >> "$out/stages.tsv";[[ $code = "$expected" ]]
}
if [[ $mode = build ]];then
 [[ ! -e $out ]];mkdir -m 700 "$out";cp "$ref" "$out/build-ref.json"
 jq -e --arg h "$(git rev-parse HEAD)" '.head==$h and .state=="open" and .auto_merge==null and .source=="connected-github-read"' "$ref" > "$out/ref-check.log"
 run qualifier-build 0 g++ -std=c++20 -O2 -s -I/workspace/twist-native-tooling/include "$scope/qualify.cpp" -lcrypto -o "$out/qualifier.immutable"
 chmod 500 "$out/qualifier.immutable";sha256sum "$out/qualifier.immutable" > "$out/qualifier.sha256"
 run cpp-build 0 g++ -std=c++20 -O2 -I/workspace/twist-native-tooling/include -I/workspace/twist-callgrind-20261116/install/include "$scope/counter.cpp" -lcrypto -lz -o "$out/producer"
 run semantic-build 0 g++ -std=c++20 -O2 -s -I/workspace/twist-native-tooling/include "$scope/setup-rank.cpp" -lcrypto -lz -o "$out/semantic"
 run rust-build 0 /workspace/twist-native-tooling/rustup/toolchains/1.85.1-x86_64-unknown-linux-gnu/bin/rustc --edition=2021 -O -C strip=symbols "$scope/audit.rs" -o "$out/audit"
 run references-build 0 g++ -std=c++20 -O2 -s -I/workspace/twist-native-tooling/include "$scope/references.cpp" -lcrypto -lz -o "$out/references"
 run analysis-build 0 g++ -std=c++20 -O2 -s -I/workspace/twist-native-tooling/include "$scope/analyze.cpp" -lcrypto -o "$out/analysis"
 run math-audit-build 0 /workspace/twist-native-tooling/rustup/toolchains/1.85.1-x86_64-unknown-linux-gnu/bin/rustc --edition=2021 -O -C strip=symbols "$scope/math-audit.rs" -o "$out/math-audit"
 run protocol-check 0 python3 /workspace/canonical-protocol-20261115/tools/experiment_protocol.py validate "$scope/protocol.json"
 "$out/qualifier.immutable" build "$scope/PINS.json" "$ref" "$out" "$out/build-manifest.json" > "$out/build-qualification.log" 2>&1
elif [[ $mode = execute ]];then
 [[ -f $out/build-manifest.json && ! -e $out/pre-call-manifest.json ]];cp "$ref" "$out/call-ref.json";sha256sum -c "$out/qualifier.sha256" > "$out/qualifier-check.log"
 "$out/qualifier.immutable" pre "$scope/PINS.json" "$ref" "$out" "$out/pre-call-manifest.json" > "$out/pre-qualification.log" 2>&1
 jq '.head="0000000000000000000000000000000000000000"' "$ref" > "$out/false-reference.json"
 run reject-qualifier-head 101 "$out/qualifier.immutable" pre "$scope/PINS.json" "$out/false-reference.json" "$out" "$out/false-head-manifest.json";[[ ! -e $out/false-head-manifest.json ]]
 jq '(.repo_files|keys[0])as $key|.repo_files[$key]="0000000000000000000000000000000000000000000000000000000000000000"' "$scope/PINS.json" > "$out/false-pins.json"
 run reject-qualifier-source 101 "$out/qualifier.immutable" pre "$out/false-pins.json" "$ref" "$out" "$out/false-source-manifest.json";[[ ! -e $out/false-source-manifest.json ]]
 head=$(git rev-parse HEAD)
 for tag in C A B;do
  if [[ $tag = C ]];then run "semantic-$tag" 0 bash -o pipefail -c '"$1" produce C "$2" | xz -T1 -3 > "$3"' _ "$out/semantic" "$head" "$out/C.xz"
  else run "semantic-$tag" 0 bash -c '"$1" produce "$3" "$2" > "$4"' _ "$out/semantic" "$head" "$tag" "$out/$tag.digest";fi
 done
 run semantic-audit 0 bash -o pipefail -c 'xz -dc "$1" | "$2" audit "$3" "$4" "$5" | xz -T1 -3 > "$6"' _ "$out/C.xz" "$out/math-audit" "$head" "$out/A.digest" "$out/B.digest" "$out/math-audit.json.xz"
 for item in source columns cap witness root row scalar log cost rank branch missing-query missing-policy missing-empty-branch drop-proportional premature-setup-stop;do
  run "math-corrupt-$item" 0 bash -o pipefail -c 'xz -dc "$1" | "$2" corrupt "$3" > "$4"' _ "$out/C.xz" "$out/semantic" "$item" "$out/false-$item.delta"
  run "math-hash-false-$item" 0 bash -o pipefail -c 'xz -dc "$1" | "$2" apply "$3" "$4" | sha256sum > "$5"' _ "$out/C.xz" "$out/semantic" "$out/false-$item.delta" "$out/false-$item.materialized.sha256" "$out/false-$item.stream.sha256"
  read -r actual _ < "$out/false-$item.stream.sha256";read -r expected < "$out/false-$item.materialized.sha256";[[ $actual = "$expected" ]]
  run "math-reject-$item" 101 bash -o pipefail -c 'xz -dc "$1" | "$2" apply "$3" "$4" | "$5" audit "$6" "$7" "$8" > "$9"' _ "$out/C.xz" "$out/semantic" "$out/false-$item.delta" "$out/false-$item.rejection-replay.sha256" "$out/math-audit" "$head" "$out/A.digest" "$out/B.digest" "$out/reject-math-$item.json"
  [[ ! -s $out/reject-math-$item.json ]];[[ $(cat "$out/false-$item.rejection-replay.sha256") = "$expected" ]]
 done
 run make-reference 0 bash -o pipefail -c '{ xz -dc "$1"; printf "\n"; xz -dc "$2"; } | "$3" "$4" "$5" "$6"' _ /tmp/twist-first-row-run01-20261116/math-audit.json.xz "$out/math-audit.json.xz" "$out/references" /workspace/twist-native-run02-20261116/audit.json "$out/reference.tsv.gz" "$head"
 read -r reference_hash _ < <(sha256sum "$out/reference.tsv.gz")
 for position in $(seq 0 37);do for tag in A B;do
  index=$position;[[ $tag = A ]]||index=$((37-position));printf -v token '%02d' "$index";id="$tag-$token";first=$((index*12));end=$((first+12));[[ $end -le 455 ]]||end=455
  mkdir "$out/$id"
  run "count-$id" 0 /workspace/twist-callgrind-20261116/install/bin/valgrind --tool=callgrind --instr-atstart=yes --collect-atstart=no --cache-sim=no --branch-sim=no --dump-line=no --dump-instr=no --compress-strings=yes --compress-pos=yes "--callgrind-out-file=$out/$id/raw-counter" "$out/producer" "$tag" "$first" "$end" "$out/$id" "$out/$id/raw-counter" "$head" full
  run "audit-$id" 0 "$out/audit" "$out/$id/cases.tsv.gz" "$out/reference.tsv.gz" "$head" "$out/audit-$id.json" "$reference_hash"
  if [[ $id = A-00 ]];then
   for item in cost raw-summary missing source scalar witness uncharged-method;do
    run "corrupt-$item" 0 "$out/producer" corrupt "$out/$id" "$out/false-$item" "$item"
    run "reject-$item" 101 "$out/audit" "$out/false-$item/cases.tsv.gz" "$out/reference.tsv.gz" "$head" "$out/reject-$item.json" "$reference_hash";[[ ! -e $out/reject-$item.json ]]
    run "pack-false-$item" 0 python3 "$scope/pack-bytes.py" "$out/false-$item" "$out/false-$item.tar.xz" --remove-verified-new-copies
   done
   run audit-original-again 0 "$out/audit" "$out/$id/cases.tsv.gz" "$out/reference.tsv.gz" "$head" "$out/audit-$id-again.json" "$reference_hash";cmp "$out/audit-$id.json" "$out/audit-$id-again.json"
  fi
  run "pack-$id" 0 python3 "$scope/pack-bytes.py" "$out/$id" "$out/$id.tar.xz" --remove-verified-new-copies
 done;done
 run analyze 0 bash -o pipefail -c '{ xz -dc "$1"; printf "\n"; xz -dc "$2"; } | "$3" "$4" "$5" unused unused "$6"' _ /tmp/twist-first-row-run01-20261116/math-audit.json.xz "$out/math-audit.json.xz" "$out/analysis" "$out" "$head" "$out/analysis.json"
 "$out/qualifier.immutable" post "$scope/PINS.json" "$ref" "$out" "$out/post-call-manifest.json" > "$out/post-qualification.log" 2>&1
else exit 2;fi
