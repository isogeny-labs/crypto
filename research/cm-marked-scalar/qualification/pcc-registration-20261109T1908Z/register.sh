#!/usr/bin/env bash
# Metadata only. This does not run, sign or impersonate a scientific verifier.
set -euo pipefail
cd /home/ec2-user/crypto-cm-marked-scalar-20261108
conductor check --summary 'Register hash-pinned unverified producer assertions after PCC namespace release' --scope path:pcc/claims --scope path:pcc/ledger.jsonl
cm_record=research/cm-marked-scalar/qualification/pcc-registration-20261109T1908Z
mkdir -p "$cm_record"
test ! -e "$cm_record/claim-path.txt"
git show HEAD:tools/pcc/pcc.py > "$cm_record/contract-source.py"
sha256sum "$cm_record/contract-source.py" > "$cm_record/contract-source.sha256"
git show HEAD:pcc/ledger.jsonl > "$cm_record/base-ledger.jsonl"
# jq's canonical encoding is accepted only if EVERY existing hash matches.
jq -e -s 'length>0 and (all(range(0;length); . as $i | true))' "$cm_record/base-ledger.jsonl" >/dev/null
jq -e -s ' . as $all | all(range(0;length); . as $i | $all[$i].schema=="pcc.ledger-entry/v1" and $all[$i].seq==$i and $all[$i].prev==(if $i==0 then ("0"*64) else $all[$i-1].hash end) and (["genesis","claim","verdict","bypass","note"]|index($all[$i].type))!=null)' "$cm_record/base-ledger.jsonl" >/dev/null
jq -cS 'del(.hash)' "$cm_record/base-ledger.jsonl" > "$cm_record/canonical-bodies.jsonl"
while IFS= read -r cm_body; do printf '%s' "$cm_body" | sha256sum | cut -d ' ' -f1; done < "$cm_record/canonical-bodies.jsonl" > "$cm_record/computed-hashes.txt"
jq -r .hash "$cm_record/base-ledger.jsonl" > "$cm_record/expected-hashes.txt"
cmp "$cm_record/computed-hashes.txt" "$cm_record/expected-hashes.txt"
cm_day=$(date -u +%Y%m%d)
cm_random=$(od -An -N3 -tx1 /dev/urandom | tr -d ' \n')
cm_id="PCC-$cm_day-$cm_random"
cm_path="pcc/claims/$cm_id.json"
cm_commit=$(git rev-parse HEAD)
cm_time=$(date -u +%FT%TZ)
cm_proofs=$(for cm_proof in research/cm-marked-scalar/block-tuning-v1/plans.json research/cm-marked-scalar/block-tuning-v1/selected-qualification.json research/cm-marked-scalar/block-v4/admission-receipt.json research/cm-marked-scalar/qualification/block-compact-v1/qualifier.json research/cm-marked-scalar/qualification/block-compact-cold-v1/qualifier.json research/cm-marked-scalar/qualification/block-compact-v1/source.sha256 research/cm-marked-scalar/qualification/block-compact-cold-v1/source.sha256; do jq -n --arg path "$cm_proof" --arg hash "$(sha256sum "$cm_proof" | cut -d ' ' -f1)" '{kind:"attestation",subject:{path:$path,sha256:$hash}}'; done | jq -s .)
jq -n --arg id "$cm_id" --arg commit "$cm_commit" --arg created "$cm_time" --argjson proofs "$cm_proofs" '{schema:"pcc.claim/v1",id:$id,kind:"statement",statement:"Producer-run records report a finite 27090-row block census and 80 minimum sets, 400 selected point/count checks, and relation D+A 230,265,303,451,611 above the paired regular references; V4 preflight refused with other CPU4.24seconds and zero timing workers; inline planner qualification reports 4950 program/GMP and3150 point/full-count cases, and625 full cold paths. These are unverified producer assertions, not a signed measurement, execution proof or e2e speedup.",scope:"P-192 primary; P-224/P-256/P-384/P-521; fixed original mu; frozen finite census and supplied software cases only. Attestation pins are not signed replay/recompute proofs or signed scientific verdicts.",proofs:$proofs,producer:{commit:$commit,session:"01a1b7e7-2a27-7a62-8d80-9599231ead3e",model:null},created_at:$created}' > "$cm_record/claim.json"
cm_claim_hash=$(jq -cS 'del(.standing)' "$cm_record/claim.json" | tr -d '\n' | sha256sum | cut -d ' ' -f1)
cm_prev=$(tail -1 "$cm_record/base-ledger.jsonl" | jq -er .hash)
cm_seq=$(wc -l < "$cm_record/base-ledger.jsonl")
jq -cnS --arg id "$cm_id" --arg claim_hash "$cm_claim_hash" --arg path "$cm_path" --arg prev "$cm_prev" --arg at "$cm_time" --argjson seq "$cm_seq" '{schema:"pcc.ledger-entry/v1",seq:$seq,prev:$prev,type:"claim",at:$at,body:{claim_id:$id,claim_sha256:$claim_hash,path:$path}}' > "$cm_record/new-entry-body.json"
cm_entry_hash=$(tr -d '\n' < "$cm_record/new-entry-body.json" | sha256sum | cut -d ' ' -f1)
jq -cS --arg hash "$cm_entry_hash" '.+{hash:$hash}' "$cm_record/new-entry-body.json" > "$cm_record/new-entry.jsonl"
# Recheck immediately before shared namespace writes; preserve an exact prefix.
conductor check --summary 'Guard canonical PCC claim and append-only ledger mutation immediately before write' --scope path:pcc/claims --scope path:pcc/ledger.jsonl
test "$(git rev-parse HEAD)" = "$cm_commit"
mkdir -p pcc/claims
if [ -e pcc/ledger.jsonl ]; then cmp pcc/ledger.jsonl "$cm_record/base-ledger.jsonl"; else cp "$cm_record/base-ledger.jsonl" pcc/ledger.jsonl; fi
test ! -e "$cm_path"
( set -o noclobber; cat "$cm_record/claim.json" > "$cm_path" )
cat "$cm_record/new-entry.jsonl" >> pcc/ledger.jsonl
head -n "$cm_seq" pcc/ledger.jsonl | cmp - "$cm_record/base-ledger.jsonl"
test "$(wc -l < pcc/ledger.jsonl)" -eq "$((cm_seq+1))"
printf '%s\n' "$cm_path" > "$cm_record/claim-path.txt"
jq -n --arg id "$cm_id" --arg path "$cm_path" --arg hash "$cm_claim_hash" --argjson prefix "$cm_seq" '{schema:"cm-native-pcc-registration/v1",claim_id:$id,claim_path:$path,claim_sha256:$hash,base_entries_hash_checked:$prefix,base_prefix_preserved:true,appended_entries:1,standing:"unverified",verifier_run:false,signature_created:false,python_executed:false,scope:"Native shell/jq metadata registration only; attestation files are unverified producer data"}' > "$cm_record/receipt.json"
rm "$cm_record/canonical-bodies.jsonl"
printf '%s\n' "$cm_path"
