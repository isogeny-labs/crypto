#!/usr/bin/env bash
# Metadata/patch tests only; all mutations stay under a new test directory.
set -euo pipefail
umask 077
cd -- "$(dirname -- "$0")"
pf_root=$PWD
mkdir tests # exclusive: retained results are never reused
: > tests/results.jsonl
pf_rehash() {
    local pf_name=$1
    jq --arg name "$pf_name" --arg sha "$(sha256sum "$pf_fixture/proposal/$pf_name" | cut -d ' ' -f 1)" \
      --argjson bytes "$(stat -c %s "$pf_fixture/proposal/$pf_name")" \
      '.files[$name]={sha256:$sha,bytes:$bytes}' "$pf_fixture/proposal/MANIFEST.json" > "$pf_fixture/proposal/MANIFEST.tmp"
    mv "$pf_fixture/proposal/MANIFEST.tmp" "$pf_fixture/proposal/MANIFEST.json"
}
pf_fixture_new() {
    pf_fixture="$pf_root/tests/$1"
    mkdir -p "$pf_fixture/proposal" "$pf_fixture/bound"
    cp corrected-proposal/* "$pf_fixture/proposal/"
    cp positive/bound/* "$pf_fixture/bound/"
    jq --arg root "$pf_fixture/bound" 'map(.path=$root+"/"+(.path|split("/")[-1]))' \
      "$pf_fixture/proposal/SOURCE_BINDINGS.json" > "$pf_fixture/proposal/bindings.tmp"
    mv "$pf_fixture/proposal/bindings.tmp" "$pf_fixture/proposal/SOURCE_BINDINGS.json"
    pf_rehash SOURCE_BINDINGS.json
}
pf_run() {
    set +e
    timeout 60s bash preflight.sh "$pf_fixture/proposal" \
      "$(sha256sum "$pf_fixture/proposal/MANIFEST.json" | cut -d ' ' -f 1)" "$pf_fixture/result" \
      > "$pf_fixture/stdout.log" 2> "$pf_fixture/stderr.log"
    pf_exit=$?
    set -e
}
pf_expect_failure() {
    local pf_expected=$1
    pf_run
    test "$pf_exit" -ne 0 && test "$pf_exit" -ne 124
    jq -e --arg expected "$pf_expected" '.success == false and .compiled == false and .stage == $expected' \
      "$pf_fixture/result/RECEIPT.json" >/dev/null
    jq -cn --arg case "$(basename "$pf_fixture")" --arg stage "$pf_expected" --argjson exit "$pf_exit" \
      '{case:$case,passed:true,expected_rejection_stage:$stage,exit_code:$exit}' >> tests/results.jsonl
}
pf_fixture_new valid_relocated_fixture
pf_run
test "$pf_exit" -eq 0
jq -e '.success == true and .compiled == false and (.files|length) == 23 and (.staged_files|length) == 13' \
  "$pf_fixture/result/RECEIPT.json" >/dev/null
jq -cn '{case:"valid_relocated_fixture",passed:true,exit_code:0}' >> tests/results.jsonl

pf_fixture_new stale_bound_source
printf '\n// stale test source\n' >> "$pf_fixture/bound/solvers.rs"
pf_expect_failure source_bindings

pf_fixture_new tampered_patch_without_rehash
printf '\nmalformed unbound test patch\n' >> "$pf_fixture/proposal/worker-transport.patch"
pf_expect_failure proposal_files

pf_fixture_new rehashed_wrong_patch_destination
sed -i 's#^+++ b/src/ml_dataset/solvers.rs$#+++ b/src/ml_dataset/unbound.rs#' "$pf_fixture/proposal/worker-transport.patch"
pf_rehash worker-transport.patch
pf_expect_failure worker_patch_paths

pf_fixture_new rehashed_destination_content_mismatch
printf '\n// mismatched proposed destination\n' >> "$pf_fixture/proposal/solvers.proposed.rs"
pf_rehash solvers.proposed.rs
pf_expect_failure patched_worker_equivalence

pf_fixture_new rehashed_donor_body_change
printf '\n// unauthorized donor change\n' >> "$pf_fixture/proposal/dynamic_boolean_search_scheduler.rs"
pf_rehash dynamic_boolean_search_scheduler.rs
pf_expect_failure donor_equivalence

pf_fixture_new rehashed_stale_registration_context
sed -i 's/mod boolean_rows;/mod nonexistent_rows;/' "$pf_fixture/proposal/registration.patch"
pf_rehash registration.patch
pf_expect_failure registration_patch_apply

pf_fixture_new bound_symlink
mv "$pf_fixture/bound/solvers.rs" "$pf_fixture/bound/solvers.retained"
ln -s solvers.retained "$pf_fixture/bound/solvers.rs"
pf_expect_failure source_bindings

pf_fixture_new bound_size_mismatch
jq 'map(if (.path|endswith("/solvers.rs")) then .bytes += 1 else . end)' \
  "$pf_fixture/proposal/SOURCE_BINDINGS.json" > "$pf_fixture/proposal/bindings.tmp"
mv "$pf_fixture/proposal/bindings.tmp" "$pf_fixture/proposal/SOURCE_BINDINGS.json"
pf_rehash SOURCE_BINDINGS.json
pf_expect_failure source_bindings

pf_fixture="$pf_root/tests/valid_relocated_fixture"
pf_original_receipt=$(sha256sum "$pf_fixture/result/RECEIPT.json")
pf_run
test "$pf_exit" -ne 0 && test "$pf_exit" -ne 124
test "$(sha256sum "$pf_fixture/result/RECEIPT.json")" = "$pf_original_receipt"
jq -cn '{case:"existing_output_refused_receipt_unchanged",passed:true}' >> tests/results.jsonl
jq -se 'length == 10 and all(.[]; .passed == true)' tests/results.jsonl
