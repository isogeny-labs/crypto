#!/usr/bin/env bash
set -euo pipefail
if [[ ${1:-} == --help ]]; then echo 'run-cm8-37-b3-high.sh FRESH_OUTSIDE_CHECKOUT_DIRECTORY'; exit 0; fi
[[ $# == 1 ]] || { echo 'one fresh output directory required' >&2; exit 2; }
root=$(git rev-parse --show-toplevel)
cd "$root"
[[ $(git symbolic-ref --short HEAD) == codex/quadratic-twist-evidence-import ]]
[[ $(git hash-object scripts/checkpoint.sh) == $(git rev-parse origin/main:scripts/checkpoint.sh) ]]
[[ -z $(git status --porcelain --untracked-files=normal) ]]
[[ ${CHECKPOINT_TOKEN_FILE:-} == /workspace/.checkpoint/pr2381-upload-token && -s $CHECKPOINT_TOKEN_FILE ]]
[[ ${CHECKPOINT_BUCKET:-setec-astronomy-artifacts} == setec-astronomy-artifacts && ${CHECKPOINT_PREFIX:-checkpoints} == checkpoints ]]
out=$(realpath -m "$1")
[[ ! -e $out && $out != "$root" && $out != "$root/"* ]]
bash scripts/checkpoint.sh gate
mkdir -p "$out"
git rev-parse HEAD > "$out/commit.txt"
uname -a > "$out/host.txt"
run() { local name=$1; shift; printf '%q ' "$@" > "$out/$name.argv"; printf '\n' >> "$out/$name.argv"; local rc=0; timeout 600 "$@" > "$out/$name.log" 2>&1 || rc=$?; echo "$rc" > "$out/$name.exit"; return "$rc"; }
study=research/quadratic_twist_evidence_20261021
run build-cpp c++ -std=c++17 -O2 "$study/native/cm8-37-b3-high.cpp" -lcrypto -o "$out/producer"
run build-rust env CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR="$out/target" cargo build --release --locked --offline --manifest-path "$study/native/cm8-37-b3-high-audit/Cargo.toml"
run transfer "$out/producer" transfer "$root" "$out/transfer.json"
audit="$out/target/release/quadratic-twist-cm8-37-b3-high-audit"
run transfer-audit "$audit" transfer "$root" "$out/transfer.json" "$out/transfer-audit.json"
cp "$out/transfer.json" "$out/false-transfer.json"
run corrupt-transfer "$audit" corrupt-transfer "$out/false-transfer.json"
if run reject-transfer "$audit" transfer "$root" "$out/false-transfer.json" "$out/false-transfer-receipt.json"; then exit 1; fi
[[ $(cat "$out/reject-transfer.exit") == 101 && ! -e "$out/false-transfer-receipt.json" ]]
run producer "$out/producer" census "$root" "$out/results"
audit="$out/target/release/quadratic-twist-cm8-37-b3-high-audit"
run audit "$audit" audit "$root" "$out/results" "$out/audit.json"
for kind in cap root row witness log scalar source missing-arm missing-failed-query; do
 cp -a "$out/results" "$out/control-$kind"
 run "corrupt-$kind" "$audit" corrupt "$kind" "$out/control-$kind"
 if [[ $kind == missing-failed-query ]] && grep -q '^NULL:' "$out/corrupt-$kind.log"; then
  jq -e '.new_capped_response_queries == 0' "$out/audit.json" > "$out/null-missing-failed-query.log"
  diff -qr "$out/results" "$out/control-$kind" >> "$out/null-missing-failed-query.log"
  echo 0 > "$out/null-missing-failed-query.exit"
  continue
 fi
 if run "reject-$kind" "$audit" audit "$root" "$out/control-$kind" "$out/rejected-$kind.json"; then echo "false $kind accepted" >&2; exit 1; fi
 [[ $(cat "$out/reject-$kind.exit") == 101 && ! -e "$out/rejected-$kind.json" ]]
done
cp -a "$root/$study/native/cm8-37-b3-20261030/cases" "$out/false-lower"
run corrupt-low "$audit" corrupt-low "$out/false-lower"
if run reject-low "$audit" audit "$root" "$out/results" "$out/rejected-low.json" "$out/false-lower"; then exit 1; fi
[[ $(cat "$out/reject-low.exit") == 101 && ! -e "$out/rejected-low.json" ]]
grep -q 'pinned lower index' "$out/reject-low.log"
run backend-tests env CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR="$out/target" cargo test --release --locked --offline --manifest-path "$study/native/cm8-37-b3-high-audit/Cargo.toml"
cat "$out/producer.log" "$out/audit.log"
