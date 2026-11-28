#!/usr/bin/env bash
set -euo pipefail
set -C
study_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
output_arg=${1:?usage: check.sh NEW_OUTPUT_DIRECTORY [attempt-NNN]}
gate_attempt=${2:-attempt-002}
[[ "$gate_attempt" =~ ^attempt-[0-9]{3}$ ]] || exit 2
for tool in cargo rustc jq timeout sha256sum; do command -v "$tool" >/dev/null; done
rustc_path=$(command -v -- "${RUSTC:-rustc}")
test ! -e "$output_arg"
mkdir -p -- "$output_arg"
output_dir=$(CDPATH= cd -- "$output_arg" && pwd)
stage=preflight
finish() {
  result=$?
  trap - EXIT
  jq -n --arg stage "$stage" --argjson result "$result" \
    '{schema:"isogeny.shared-selector-publication-replay/v1",stage:$stage,exit_code:$result,new_relation_search_calls:0,performance_claim:false}' \
    > "$output_dir/post-call.json"
  exit "$result"
}
trap finish EXIT
"$rustc_path" --version --verbose > "$output_dir/toolchain.txt"
sha256sum "$study_dir/Cargo.toml" "$study_dir/Cargo.lock" "$study_dir/check.sh" \
  "$study_dir/capacity-checker/src/exact_bounds.rs" \
  "$study_dir/shared-terminal-gate/src/"*.rs "$study_dir/shared-terminal-gate/run.sh" \
  > "$output_dir/source-sha256.txt"
cmp "$study_dir/capacity-checker/src/exact_bounds.rs" "$study_dir/capacity-checker/frozen/exact_bounds.rs"
for source in main.rs reused_native.rs archive_source.rs; do
  cmp "$study_dir/shared-terminal-gate/src/$source" "$study_dir/shared-terminal-gate/results/attempt-001/frozen/src/$source"
done
jq -n --arg gate_attempt "$gate_attempt" \
  '{schema:"isogeny.shared-selector-publication-precall/v1",mode:"deterministic replay of frozen M31 evidence",p:2147483647,r:2147446831,randomness:"no new randomness; archived source seed retained",source_manifest:"source-sha256.txt",gate_attempt:$gate_attempt,test_threads:1,per_cargo_call_seconds:60,gate_call_seconds:45,new_relation_search_calls:0,byte_comparisons:["bounds.tsv","certificates.json","gate-result.json"],performance_claim:false}' \
  > "$output_dir/pre-call.json"
export CARGO_TARGET_DIR="$output_dir/target"
export RUSTC="$rustc_path"
stage=cargo-tests
timeout 60s cargo test --manifest-path "$study_dir/Cargo.toml" --offline --locked --release --bins -- --test-threads=1 \
  > "$output_dir/cargo-tests.stdout" 2> "$output_dir/cargo-tests.stderr"
stage=capacity-replay
timeout 60s cargo run --manifest-path "$study_dir/Cargo.toml" --offline --locked --release --bin exact-bounds \
  -- --certificate "$output_dir/certificates.json" > "$output_dir/bounds.tsv" 2> "$output_dir/capacity.stderr"
cmp "$study_dir/capacity-checker/evidence/bounds.tsv" "$output_dir/bounds.tsv"
cmp "$study_dir/capacity-checker/evidence/certificates.json" "$output_dir/certificates.json"
stage=gate-replay
bash "$study_dir/shared-terminal-gate/run.sh" "$gate_attempt" \
  > "$output_dir/gate-wrapper.stdout" 2> "$output_dir/gate-wrapper.stderr"
gate_dir="$study_dir/shared-terminal-gate/results/$gate_attempt"
cmp "$study_dir/shared-terminal-gate/results/attempt-001/result.json" "$gate_dir/result.json"
for name in result.json build-manifest.json pre-execution-manifest.json post-execution-manifest.json tests.stdout toolchain.txt binaries.sha256; do
  cp -- "$gate_dir/$name" "$output_dir/gate-$name"
done
cp -- "$gate_dir/frozen/source-manifest.sha256" "$output_dir/gate-source-manifest.sha256"
stage=complete
printf '%s\n' 'PASS: six distinct Cargo tests; frozen capacity and M31 gate outputs match byte-for-byte.'
