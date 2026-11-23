#!/usr/bin/env bash
# Source-only preflight. Arguments: frozen proposal, trusted manifest SHA, NEW output.
# No compilation, dataset generation, cloud calls, repository or index writes.
set -euo pipefail
umask 077
test "$#" -eq 3 || { echo 'usage: preflight.sh PROPOSAL MANIFEST_SHA NEW_OUTPUT' >&2; exit 64; }
pf_proposal=$1 pf_anchor=$2 pf_out=$3
[[ $pf_anchor =~ ^[0-9a-f]{64}$ ]] || exit 64
[[ $pf_proposal = /* && $pf_out = /* ]] || exit 64
test -d "$pf_proposal" && test ! -L "$pf_proposal" || exit 64
test "$(realpath -e -- "$pf_proposal")" = "$pf_proposal" || exit 64
test "$(realpath -e -- "$(dirname -- "$pf_out")")" = "$(dirname -- "$pf_out")" || exit 64
mkdir -- "$pf_out" # exclusive: existing outputs, including receipts, are never reused
mkdir -- "$pf_out/proposal" "$pf_out/bound" "$pf_out/staged" "$pf_out/checks"
: > "$pf_out/checks/files.jsonl"
: > "$pf_out/checks/staged-files.jsonl"
pf_tool_sha=$(sha256sum -- "$0" | cut -d ' ' -f 1)
pf_stage=manifest
pf_total=0
pf_success=false
pf_finish() {
    local pf_status=$?
    trap - EXIT
    jq -n --arg stage "$pf_stage" --arg anchor "$pf_anchor" \
        --arg original "$pf_proposal" --arg output "$pf_out" --arg tool_sha "$pf_tool_sha" \
        --argjson exit_code "$pf_status" --argjson success "$pf_success" \
        --slurpfile files "$pf_out/checks/files.jsonl" --slurpfile staged "$pf_out/checks/staged-files.jsonl" \
        '{schema:"local.original-owned-search-source-preflight/v1",success:$success,
          exit_code:$exit_code,stage:$stage,proposal:$original,manifest_sha256:$anchor,
          scratch:$output,preflight_source_sha256:$tool_sha,compiled:false,native_solver_executed:false,cloud_requests:0,
          frozen_source_modified:false,partial_source_tree_only:true,files:$files,staged_files:$staged,
          scope:"hash/size, exact donor substitution, patch application and supplied destination byte equality only"}' \
        > "$pf_out/RECEIPT.json.pending"
    ln -- "$pf_out/RECEIPT.json.pending" "$pf_out/RECEIPT.json"
    rm -- "$pf_out/RECEIPT.json.pending"
    chmod 0444 "$pf_out/RECEIPT.json"
    exit "$pf_status"
}
trap pf_finish EXIT
pf_hash() { sha256sum -- "$1" | cut -d ' ' -f 1; }
pf_copy() {
    local pf_from=$1 pf_to=$2 pf_sha=$3 pf_bytes=$4
    test -f "$pf_from" && test ! -L "$pf_from"
    test "$(realpath -e -- "$pf_from")" = "$pf_from"
    test "$pf_bytes" -le 2097152
    test "$(stat -c %s -- "$pf_from")" -eq "$pf_bytes"
    # Bounded, nonblocking, no-follow final open; verify the exact copied bytes.
    dd if="$pf_from" of="$pf_to" bs=65536 count=33 iflag=nofollow,nonblock status=none
    test "$(stat -c %s -- "$pf_to")" -eq "$pf_bytes"
    test "$(pf_hash "$pf_to")" = "$pf_sha"
    pf_total=$((pf_total + pf_bytes))
    test "$pf_total" -le 8388608
    jq -cn --arg path "$pf_from" --arg snapshot "$pf_to" --arg sha "$pf_sha" \
        --argjson bytes "$pf_bytes" '{path:$path,snapshot:$snapshot,sha256:$sha,bytes:$bytes,verified:true}' \
        >> "$pf_out/checks/files.jsonl"
}
pf_copy "$pf_proposal/MANIFEST.json" "$pf_out/proposal/MANIFEST.json" "$pf_anchor" "$(stat -c %s -- "$pf_proposal/MANIFEST.json")"
jq -e '
  .schema == "original-owned-search-transport-proposal/v1" and .compiled == false and
  .native_executed == false and .shared_source_edited == false and
  (.files | keys == (["original_boolean_search.rs","dynamic_boolean_search_scheduler.rs",
    "dynamic_boolean_search.rs","dynamic_boolean_search_bqi.rs","dynamic_boolean_search_check.rs",
    "registration.patch","worker-transport.patch","solvers.proposed.rs","SOURCE_BINDINGS.json","README.md"] | sort)) and
  all(.files[]; (.sha256 | test("^[0-9a-f]{64}$")) and
    (.bytes | type == "number" and floor == . and . >= 0 and . <= 2097152))
' "$pf_out/proposal/MANIFEST.json" >/dev/null
pf_stage=proposal_files
while IFS=$'\t' read -r pf_name pf_sha pf_bytes; do
    pf_copy "$pf_proposal/$pf_name" "$pf_out/proposal/$pf_name" "$pf_sha" "$pf_bytes"
done < <(jq -r '.files | to_entries[] | [.key,.value.sha256,.value.bytes] | @tsv' "$pf_out/proposal/MANIFEST.json")
pf_stage=source_bindings
jq -e '
  type == "array" and length == 12 and
  all(.[]; (keys == ["bytes","path","sha256"]) and
    (.path | test("^/[A-Za-z0-9_./-]+$")) and (.sha256 | test("^[0-9a-f]{64}$")) and
    (.bytes | type == "number" and floor == . and . >= 0 and . <= 2097152)) and
  ([.[].path | split("/")[-1]] | sort == (["checked_boolean_equations.rs","boolean_identity_bqi.rs",
    "sat_original_owner.rs","koblitz_m3.rs","original_boolean_rows.rs","sat_initial_native.rs",
    "solvers.rs","mod.rs","search.rs","scheduler.rs","bridge.rs","check.rs"] | sort))
' "$pf_out/proposal/SOURCE_BINDINGS.json" >/dev/null
while IFS=$'\t' read -r pf_path pf_sha pf_bytes; do
    pf_copy "$pf_path" "$pf_out/bound/$(basename -- "$pf_path")" "$pf_sha" "$pf_bytes"
done < <(jq -r '.[] | [.path,.sha256,.bytes] | @tsv' "$pf_out/proposal/SOURCE_BINDINGS.json")
pf_stage=donor_equivalence
cmp "$pf_out/bound/search.rs" "$pf_out/proposal/dynamic_boolean_search.rs"
cmp "$pf_out/bound/bridge.rs" "$pf_out/proposal/dynamic_boolean_search_bqi.rs"
cmp "$pf_out/bound/check.rs" "$pf_out/proposal/dynamic_boolean_search_check.rs"
test "$(grep -c '^#\[path="search.rs"\]$' "$pf_out/bound/scheduler.rs")" -eq 1
sed 's/^#\[path="search.rs"\]$/#[path="dynamic_boolean_search.rs"]/' \
    "$pf_out/bound/scheduler.rs" > "$pf_out/checks/scheduler.expected.rs"
cmp "$pf_out/checks/scheduler.expected.rs" "$pf_out/proposal/dynamic_boolean_search_scheduler.rs"
pf_stage=stage_bound_files
mkdir -p "$pf_out/staged/src/ml_dataset" "$pf_out/staged/src/cryptanalysis"
for pf_name in checked_boolean_equations.rs boolean_identity_bqi.rs; do
    cp -- "$pf_out/bound/$pf_name" "$pf_out/staged/src/cryptanalysis/$pf_name"
done
for pf_name in sat_original_owner.rs koblitz_m3.rs original_boolean_rows.rs sat_initial_native.rs solvers.rs mod.rs; do
    cp -- "$pf_out/bound/$pf_name" "$pf_out/staged/src/ml_dataset/$pf_name"
done
for pf_name in original_boolean_search.rs dynamic_boolean_search.rs dynamic_boolean_search_scheduler.rs dynamic_boolean_search_bqi.rs dynamic_boolean_search_check.rs; do
    cp -- "$pf_out/proposal/$pf_name" "$pf_out/staged/src/ml_dataset/$pf_name"
done
pf_git() {
    env -u GIT_DIR -u GIT_WORK_TREE -u GIT_INDEX_FILE \
      GIT_CONFIG_NOSYSTEM=1 GIT_CONFIG_GLOBAL=/dev/null GIT_CONFIG_SYSTEM=/dev/null \
      GIT_CEILING_DIRECTORIES="$pf_out" git -C "$pf_out/staged" apply "$@"
}
pf_paths() {
    local pf_patch=$1 pf_expected=$2
    # Plain unified hunks only: no binary patches, renames, modes or new files.
    if grep -Ev '^(--- a/|\+\+\+ b/|@@ |[ +\-]|\\ No newline at end of file)' "$pf_patch"; then return 1; fi
    diff -u <(printf '%s\n' "$pf_expected") <(sed -n 's#^--- a/##p' "$pf_patch")
    diff -u <(printf '%s\n' "$pf_expected") <(sed -n 's#^+++ b/##p' "$pf_patch")
}
pf_stage=registration_patch_paths
pf_paths "$pf_out/proposal/registration.patch" 'src/ml_dataset/sat_original_owner.rs'
pf_stage=registration_patch_apply
pf_git --check --whitespace=nowarn "$pf_out/proposal/registration.patch" > "$pf_out/checks/registration.check.log" 2>&1
pf_git --whitespace=nowarn "$pf_out/proposal/registration.patch" > "$pf_out/checks/registration.apply.log" 2>&1
pf_stage=worker_patch_paths
pf_paths "$pf_out/proposal/worker-transport.patch" $'src/ml_dataset/solvers.rs\nsrc/ml_dataset/mod.rs'
pf_stage=worker_patch_apply
pf_git --check --whitespace=nowarn "$pf_out/proposal/worker-transport.patch" > "$pf_out/checks/worker.check.log" 2>&1
pf_git --whitespace=nowarn "$pf_out/proposal/worker-transport.patch" > "$pf_out/checks/worker.apply.log" 2>&1
pf_stage=patched_worker_equivalence
cmp "$pf_out/staged/src/ml_dataset/solvers.rs" "$pf_out/proposal/solvers.proposed.rs"
test "$(grep -Fc 'pub(in crate::ml_dataset::solvers) mod boolean_search;' "$pf_out/staged/src/ml_dataset/sat_original_owner.rs")" -eq 1
test "$(grep -Fc 'if retained.boolean_search_report_unavailable(){' "$pf_out/staged/src/ml_dataset/mod.rs")" -eq 1
pf_stage=staged_inventory
while IFS= read -r pf_file; do
    jq -cn --arg path "${pf_file#"$pf_out/staged/"}" --arg sha "$(pf_hash "$pf_file")" \
      --argjson bytes "$(stat -c %s -- "$pf_file")" '{path:$path,sha256:$sha,bytes:$bytes}'
done < <(find "$pf_out/staged" -type f | LC_ALL=C sort) > "$pf_out/checks/staged-files.jsonl"
test "$(wc -l < "$pf_out/checks/staged-files.jsonl")" -eq 13
pf_success=true
pf_stage=complete
