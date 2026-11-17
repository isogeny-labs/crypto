#!/usr/bin/env bash
# Thin native replay/record glue. All arithmetic/checks are C++/Rust/Lean.
set -euo pipefail
[[ $# == 1 ]] || { echo 'run-pair-rank.sh FRESH_OUTSIDE_DIRECTORY' >&2; exit 2; }
root=$(git rev-parse --show-toplevel)
cd "$root"
[[ $(git symbolic-ref --short HEAD) == codex/quadratic-twist-evidence-import ]] || exit 2
out=$(realpath -m "$1")
[[ $out != "$root"/* && ! -e $out ]] || { echo 'fresh outside output required' >&2; exit 2; }
[[ -s ${CHECKPOINT_TOKEN_FILE:-/workspace/.checkpoint/pr2381-upload-token} ]] || { echo 'scoped token unavailable' >&2; exit 2; }
[[ $(git hash-object scripts/checkpoint.sh) == $(gh api repos/isogeny-labs/crypto/contents/scripts/checkpoint.sh?ref=main --jq .sha) ]] || { echo 'official checkpoint script differs' >&2; exit 2; }
export CHECKPOINT_TOKEN_FILE=${CHECKPOINT_TOKEN_FILE:-/workspace/.checkpoint/pr2381-upload-token}
scripts/checkpoint.sh gate
mkdir -p "$out"
study=research/quadratic_twist_evidence_20261021
sha256sum "$study/native/pair-rank.cpp" "$study/native/pair-rank-audit/main.rs" "$study/lean-pair-rank/PairRank.lean" "$study/PAIR_RANK_20261029.protocol.json" > "$out/INPUTS.sha256"
git rev-parse HEAD > "$out/head.txt"
step() { local label=$1;shift;printf '%q ' "$@" > "$out/$label.cmd";echo >> "$out/$label.cmd";set +e;timeout 600 "$@" > "$out/$label.log" 2>&1;local rc=$?;set -e;echo "$rc" > "$out/$label.exit";return "$rc"; }
step cpp-build c++ -std=c++17 -O2 -Wall -Wextra -Werror "$study/native/pair-rank.cpp" -lcrypto -o "$out/producer"
step rust-build cargo build --release --locked --offline --manifest-path "$study/native/pair-rank-audit/Cargo.toml"
audit=$root/$study/native/pair-rank-audit/target/release/pair-rank-audit
cert=$root/$study/native/pair-rank-audit/target/release/pair-rank-cert
step producer "$out/producer" "$root" "$out/cases"
step audit "$audit" audit "$root" "$out/cases" "$out/audit.json"
mkdir "$out/controls"
for mode in false-rank false-base missing-case missing-query false-binding false-source;do
 step "corrupt-$mode" "$audit" corrupt "$mode" "$out/cases" "$out/controls/$mode"
 [[ $(sha256sum "$out/cases/summary.json"|cut -d' ' -f1) != $(sha256sum "$out/controls/$mode/summary.json"|cut -d' ' -f1) ]] || exit 2
 if step "reject-$mode" "$audit" audit "$root" "$out/controls/$mode" "$out/controls/$mode-receipt.json"; then rc=0; else rc=$?; fi
 [[ $rc == 101 && ! -e $out/controls/$mode-receipt.json ]] || { echo "bad rejection $mode/$rc" >&2;exit 2; }
done
step certificate "$cert" issue "$root" "$out/certificate"
step backend cargo test --release --locked --offline --manifest-path "$study/native/pair-rank-audit/Cargo.toml" --bin pair-rank-cert
sha256sum -c "$out/INPUTS.sha256" > "$out/source-check.log"
echo 'native pair, exact bindings, changed-payload controls, certificate/backend passed'
