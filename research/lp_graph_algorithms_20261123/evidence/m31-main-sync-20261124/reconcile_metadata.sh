#!/usr/bin/env bash
# Metadata-only reconciliation of two pinned Git revisions. No repository write.
# Native git/jq/sha256sum implement the existing PCC JSON/hash append contract.
set -euo pipefail
umask 077
[[ $# == 3 ]] || { printf '%s\n' 'usage: reconcile_metadata.sh REPO NEW_OUTPUT_DIR UTC_TIMESTAMP' >&2; exit 2; }
repo=$(cd "$1" && pwd)
output=$2
at=$3
[[ "$at" =~ ^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z$ ]]
[[ ! -e "$output" ]]
mkdir -p "$output"
output=$(cd "$output" && pwd)
export GIT_NO_LAZY_FETCH=1 GIT_OPTIONAL_LOCKS=0
branch=8233cbb8fc844b7321616923f52b8d4805e95e52
main=81f0308c75204f3a1c0ec52fdf78396eee0898ed
base=da7777094392d3ccd37db78dcfc761c32e7e7813
claim_id=PCC-20261123-c7e76c
claim_path="pcc/claims/$claim_id.json"
evidence_rel=research/lp_graph_algorithms_20261123/evidence/m31-main-sync-20261124
inputs="$output/inputs"
checks="$output/checks"
candidate="$output/candidate"
evidence="$candidate/$evidence_rel"
mkdir -p "$inputs" "$checks" "$candidate/docs" "$candidate/pcc" "$evidence"
trap 'status=$?; printf "%s\n" "$status" >"$output/helper.exit"' EXIT

hash_file() { sha256sum "$1" | cut -d ' ' -f 1; }
hash_stdin() { sha256sum | cut -d ' ' -f 1; }
extract() {
  local ref=$1 path=$2 destination=$3 expected=$4 status=0
  printf 'git -C %q show %q\n' "$repo" "$ref:$path" >"$destination.command"
  git -C "$repo" show "$ref:$path" >"$destination" 2>"$destination.stderr" || status=$?
  printf '%s\n' "$status" >"$destination.exit"
  [[ "$status" == 0 ]]
  [[ "$(git -C "$repo" hash-object --no-filters "$destination")" == "$expected" ]]
  chmod 444 "$destination"
}
prefix_check() {
  local prefix=$1 whole=$2 label=$3 count
  count=$(wc -c <"$prefix")
  head -c "$count" "$whole" | cmp - "$prefix" >"$checks/$label.stdout" 2>"$checks/$label.stderr"
  printf '%s\n' 0 >"$checks/$label.exit"
}
chain_check() {
  local ledger=$1 label=$2 line
  jq -es '
    . as $all | length > 0 and all(range(0;length); . as $i |
      $all[$i].schema == "pcc.ledger-entry/v1" and $all[$i].seq == $i and
      $all[$i].prev == (if $i == 0 then ("0"*64) else $all[$i-1].hash end) and
      ($all[$i].hash | type == "string" and test("^[0-9a-f]{64}$")) and
      (["genesis","claim","verdict","bypass","note"] | index($all[$i].type)) != null)
  ' "$ledger" >"$checks/$label-schema.stdout"
  jq -cS 'del(.hash)' "$ledger" >"$checks/$label-bodies.jsonl"
  jq -r '.hash' "$ledger" >"$checks/$label-expected.txt"
  while IFS= read -r line; do printf '%s' "$line" | hash_stdin; done \
    <"$checks/$label-bodies.jsonl" >"$checks/$label-computed.txt"
  cmp "$checks/$label-expected.txt" "$checks/$label-computed.txt"
  printf '%s\n' 0 >"$checks/$label.exit"
}

extract "$base" pcc/ledger.jsonl "$inputs/base-ledger.jsonl" 7ede6dc4e55d04f299831fd987b9fb6aaccccc8c
extract "$branch" pcc/ledger.jsonl "$inputs/branch-ledger.jsonl" 432539bfa8f957dacde7ab3fc1d5ed6ac3bb7191
extract "$main" pcc/ledger.jsonl "$inputs/main-ledger.jsonl" cdab7bd6f9f7fad4c0f0bd6e760e6ac593e404c5
extract "$base" docs/ACTIVE.md "$inputs/base-ACTIVE.md" e384093afca6914cc9c3e57a5474d330e0152e42
extract "$branch" docs/ACTIVE.md "$inputs/branch-ACTIVE.md" feec6b2a2f93187c0e86a827868efca62912b37d
extract "$main" docs/ACTIVE.md "$inputs/main-ACTIVE.md" 3956d1b912b5bae99534631d639ba61e3e626446
extract "$branch" "$claim_path" "$inputs/claim.json" ba81d9e3cb4c1c60173836d3b865217eadcd6f9a
extract "$main" tools/pcc/pcc.py "$inputs/pcc.py" 66e1c71ce94edd434804912e78c3858a9b24d007
[[ "$(hash_file "$inputs/pcc.py")" == 85d787ed7cec0c02a0e2cb81cf435691d448221c1d46e3a4cecb54e64d674fcb ]]
sha256sum "$inputs/"*.jsonl "$inputs/"*.md "$inputs/claim.json" "$inputs/pcc.py" >"$checks/input-before.sha256"
prefix_check "$inputs/base-ledger.jsonl" "$inputs/branch-ledger.jsonl" base-branch-prefix
prefix_check "$inputs/base-ledger.jsonl" "$inputs/main-ledger.jsonl" base-main-prefix
chain_check "$inputs/main-ledger.jsonl" main-chain
base_entries=$(jq -s 'length' "$inputs/base-ledger.jsonl")
base_bytes=$(wc -c <"$inputs/base-ledger.jsonl")
tail -c "+$((base_bytes + 1))" "$inputs/branch-ledger.jsonl" >"$evidence/branch-envelope.jsonl"
[[ "$(wc -l <"$evidence/branch-envelope.jsonl")" == 1 ]]
[[ "$(wc -c <"$evidence/branch-envelope.jsonl")" == 404 ]]
jq -e --arg id "$claim_id" --argjson seq "$base_entries" \
  --arg prev "$(jq -sr '.[-1].hash' "$inputs/base-ledger.jsonl")" '
  .schema == "pcc.ledger-entry/v1" and .type == "claim" and .seq == $seq and .prev == $prev
  and .body.claim_id == $id and .body.path == "pcc/claims/PCC-20261123-c7e76c.json"
  and .body.claim_sha256 == "159e292d4bb26867ecd759e73f0d5240ebdb4fce26cf723ce77564d28f436113"
  and .hash == "88f8ab42dd8ca69561ce94f71d3732665cb4c1b3726f0752c5359b56ee8da436"
  and (has("sig") | not)
' "$evidence/branch-envelope.jsonl" >"$checks/branch-envelope.stdout"
[[ "$(jq -cjS 'del(.hash)' "$evidence/branch-envelope.jsonl" | hash_stdin)" == "$(jq -r '.hash' "$evidence/branch-envelope.jsonl")" ]]
jq -es --arg id "$claim_id" 'all(.[]; .body.claim_id? != $id)' "$inputs/main-ledger.jsonl" >"$checks/main-claim-absent.stdout"
claim_digest=$(jq -cjS 'del(.standing)' "$inputs/claim.json" | hash_stdin)
[[ "$claim_digest" == 159e292d4bb26867ecd759e73f0d5240ebdb4fce26cf723ce77564d28f436113 ]]
main_entries=$(jq -s 'length' "$inputs/main-ledger.jsonl")
main_tail=$(jq -sr '.[-1].hash' "$inputs/main-ledger.jsonl")
jq -cnS --slurpfile old "$evidence/branch-envelope.jsonl" --argjson seq "$main_entries" \
  --arg prev "$main_tail" --arg at "$at" '
  {schema:"pcc.ledger-entry/v1",seq:$seq,prev:$prev,type:"claim",at:$at,body:$old[0].body}
' >"$checks/new-envelope-body.json"
new_hash=$(jq -cjS '.' "$checks/new-envelope-body.json" | hash_stdin)
jq -cS --arg hash "$new_hash" '. + {hash:$hash}' "$checks/new-envelope-body.json" >"$evidence/new-envelope.jsonl"
jq -e --slurpfile old "$evidence/branch-envelope.jsonl" '.body == $old[0].body and (has("sig") | not)' \
  "$evidence/new-envelope.jsonl" >"$checks/unchanged-body.stdout"
cat "$inputs/main-ledger.jsonl" "$evidence/new-envelope.jsonl" >"$candidate/pcc/ledger.jsonl"
prefix_check "$inputs/main-ledger.jsonl" "$candidate/pcc/ledger.jsonl" main-candidate-prefix
[[ "$(jq -s 'length' "$candidate/pcc/ledger.jsonl")" == "$((main_entries + 1))" ]]
chain_check "$candidate/pcc/ledger.jsonl" candidate-chain

# Use git's native three-way text merge, retaining its original diagnostic.
# Resolve only one insertion conflict with an empty base and exactly our M31
# task on the branch side; preserve both insertion chunks, main first.
merge_status=0
git merge-file -p --diff3 -L graph-m31-branch -L graph-m31-base -L graph-m31-main \
  "$inputs/branch-ACTIVE.md" "$inputs/base-ACTIVE.md" "$inputs/main-ACTIVE.md" \
  >"$checks/ACTIVE-merge.diff3" 2>"$checks/ACTIVE-merge.stderr" || merge_status=$?
printf '%s\n' "$merge_status" >"$checks/ACTIVE-merge.exit"
[[ "$merge_status" == 1 ]]
awk '
  BEGIN {state=0; conflicts=0; ours_lines=0; base_lines=0; theirs_lines=0; bad=0}
  /^<<<<<<< graph-m31-branch$/ {if(state!=0) bad=1; state=1; conflicts++; next}
  /^\|\|\|\|\|\|\| graph-m31-base$/ {if(state!=1) bad=1; state=2; next}
  /^=======$/ {if(state!=2) bad=1; state=3; next}
  /^>>>>>>> graph-m31-main$/ {
    if(state!=3) bad=1;
    printf "%s%s", theirs, ours; state=0; next
  }
  state==1 {ours=ours $0 "\n"; ours_lines++; if($0 !~ /^- \[ \] Graph-IC M31 captured-stream replay /) bad=1; next}
  state==2 {base_lines++; next}
  state==3 {theirs=theirs $0 "\n"; theirs_lines++; next}
  {print}
  END {if(bad || state!=0 || conflicts!=1 || ours_lines!=1 || base_lines!=0 || theirs_lines<1) exit 1}
' "$checks/ACTIVE-merge.diff3" >"$candidate/docs/ACTIVE.md"
! rg -n '^(<<<<<<<|\|\|\|\|\|\|\||=======|>>>>>>>)' "$candidate/docs/ACTIVE.md"
rg -F 'Graph-IC M31 captured-stream replay' "$inputs/branch-ACTIVE.md" >"$checks/M31-original-line.txt"
rg -F 'Graph-IC M31 captured-stream replay' "$candidate/docs/ACTIVE.md" >"$checks/M31-candidate-line.txt"
cmp "$checks/M31-original-line.txt" "$checks/M31-candidate-line.txt"
awk '!/^- \[ \] Graph-IC M31 captured-stream replay /' "$candidate/docs/ACTIVE.md" >"$checks/ACTIVE-without-M31.md"
cmp "$inputs/main-ACTIVE.md" "$checks/ACTIVE-without-M31.md"

sha256sum "$inputs/"*.jsonl "$inputs/"*.md "$inputs/claim.json" "$inputs/pcc.py" >"$checks/input-after.sha256"
cmp "$checks/input-before.sha256" "$checks/input-after.sha256"
install -m 555 "${BASH_SOURCE[0]}" "$evidence/reconcile_metadata.sh"
jq -nS --arg branch "$branch" --arg main "$main" --arg base "$base" --arg at "$at" \
  --arg claim "$claim_path" --arg claim_sha256 "$claim_digest" --arg claim_raw_sha256 "$(hash_file "$inputs/claim.json")" \
  --arg main_sha256 "$(hash_file "$inputs/main-ledger.jsonl")" --arg main_tail "$main_tail" \
  --argjson main_entries "$main_entries" --argjson main_bytes "$(wc -c <"$inputs/main-ledger.jsonl")" \
  --arg archive_path "$evidence_rel/branch-envelope.jsonl" --arg archive_sha256 "$(hash_file "$evidence/branch-envelope.jsonl")" \
  --arg new_path "$evidence_rel/new-envelope.jsonl" --arg new_sha256 "$(hash_file "$evidence/new-envelope.jsonl")" \
  --arg candidate_ledger_sha256 "$(hash_file "$candidate/pcc/ledger.jsonl")" \
  --arg active_sha256 "$(hash_file "$candidate/docs/ACTIVE.md")" \
  --arg helper_sha256 "$(hash_file "$evidence/reconcile_metadata.sh")" \
  --slurpfile old "$evidence/branch-envelope.jsonl" --slurpfile new "$evidence/new-envelope.jsonl" '
  {schema:"graph-ic.pcc-main-reconciliation/v1",branch_commit:$branch,main_commit:$main,common_base:$base,
   registration_at:$at,claim:{path:$claim,canonical_sha256:$claim_sha256,raw_sha256:$claim_raw_sha256,
     git_blob:"ba81d9e3cb4c1c60173836d3b865217eadcd6f9a",unchanged:true},
   canonical_main:{ledger_sha256:$main_sha256,entries:$main_entries,bytes:$main_bytes,tail:$main_tail,
     git_blob:"cdab7bd6f9f7fad4c0f0bd6e760e6ac593e404c5"},
   original_envelope:{path:$archive_path,sha256:$archive_sha256,entry:$old[0]},
   new_envelope:{path:$new_path,sha256:$new_sha256,entry:$new[0]},
   candidates:{ledger_sha256:$candidate_ledger_sha256,active_sha256:$active_sha256},
   checks:{base_exact_prefix_of_branch:true,base_exact_prefix_of_main:true,main_chain_hashes:true,
     branch_chain_via_verified_common_prefix_and_original_envelope:true,main_claim_absent:true,
     claim_digest_matches_original_body:true,claim_body_unchanged:true,original_envelope_bytes_preserved:true,
     main_exact_byte_prefix_of_candidate:true,candidate_chain_hashes:true,exactly_one_envelope_appended:true,
     active_native_merge_conflicts:1,active_conflict_is_insertion_only:true,
     active_keeps_all_main_bytes_plus_unchanged_M31_line:true,input_hashes_unchanged:true},
   reproducibility:{helper_path:"reconcile_metadata.sh",helper_sha256:$helper_sha256,
     arguments:["REPOSITORY_PATH","NEW_OUTPUT_DIRECTORY",$at]},
   scope:"Unsigned metadata transport only; no scientific verification, signing, new claim, or standing change.",
   python_executed:false,scientific_verifier_run:false,signature_created:false,standing:"unverified"}
' >"$evidence/registration-map.json"
(
  cd "$candidate"
  sha256sum docs/ACTIVE.md pcc/ledger.jsonl "$evidence_rel/branch-envelope.jsonl" \
    "$evidence_rel/new-envelope.jsonl" "$evidence_rel/registration-map.json" "$evidence_rel/reconcile_metadata.sh"
) >"$evidence/SHA256SUMS"
chmod 444 "$candidate/docs/ACTIVE.md" "$candidate/pcc/ledger.jsonl" "$evidence/"*.jsonl \
  "$evidence/registration-map.json" "$evidence/SHA256SUMS"
printf 'Candidate metadata ready: %s\nMain entries: %s; new claim sequence: %s; new tail: %s\n' \
  "$candidate" "$main_entries" "$main_entries" "$new_hash"
