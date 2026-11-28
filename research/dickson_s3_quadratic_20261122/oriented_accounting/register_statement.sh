#!/usr/bin/env bash
# Native metadata registration only: jq serializes JSON; sha256sum hashes bytes.
# This script does not execute Python, a scientific verifier, or a signing tool.
set -euo pipefail
umask 077

s3_pcc_usage() {
    cat <<'USAGE'
Usage: register_statement.sh [--preview | --append]
  --expected-commit <40 lowercase hex>
  --expected-last-ledger-hash <64 lowercase hex>
  --receipt <canonical repository-relative file>
  --id <PCC-YYYYMMDD-xxxxxx>
  --statement <nonempty text>
  [--at <YYYY-MM-DDTHH:MM:SSZ>] [--session <text>] [--model <text>]
  [--expected-claim-sha256 <64 lowercase hex>]

The default is --preview: print the exact proposed claim and ledger entry.
Use the preview's created_at and claim_sha256 when calling --append.
--append requires --expected-claim-sha256 to bind the reviewed claim.
The receipt must already be committed at expected-commit and unchanged locally.
The working-tree ledger must already exist; materialize its tracked bytes first
if using a sparse checkout. Existing committed ledger bytes must remain a prefix.

Contract:
  kind=statement, claim_type=data, unsigned attestation subject={path,sha256}.
  Claim digest is SHA256 of sorted compact UTF-8 JSON, excluding only standing.
  Entry digest is SHA256 of sorted compact UTF-8 JSON, excluding only hash.
  Raw receipt bytes are pinned by SHA256. No keyring or verdict is modified.
  Statement claims can never be verified under the inspected PCC contract.
  Registration records unverified producer data; prose stays [pcc:unverified].
  producer.commit is the exact current committed source/evidence revision.

The metadata contract mirrors tools/pcc/pcc.py at the pinned digest below.
The historical jq/sha256sum precedent is
research/cm-marked-scalar/qualification/pcc-registration-20261109T1908Z/register.sh.
This version adds mandatory claim_type, committed-receipt binding, preview
binding, a cooperative lock, and an explicit expected ledger-tail guard.

The lock coordinates users of this script, not arbitrary noncooperating writers.
The tail and full prefix are checked again immediately before append and after
append. On a failure after the first repository write, temporary records are
retained for inspection; the script never truncates or rolls back the ledger.
USAGE
}

s3_pcc_die() { printf 'register_statement: %s\n' "$*" >&2; exit 1; }
s3_pcc_hash_file() { sha256sum -- "$1" | cut -d ' ' -f 1; }
s3_pcc_hash_stdin() { sha256sum | cut -d ' ' -f 1; }

s3_pcc_mode=preview
s3_pcc_expected_commit=
s3_pcc_expected_tail=
s3_pcc_receipt=
s3_pcc_id=
s3_pcc_statement=
s3_pcc_at=
s3_pcc_session=
s3_pcc_model=
s3_pcc_expected_claim=
while [[ $# -gt 0 ]]; do
    case "$1" in
        --help|-h) s3_pcc_usage; exit 0 ;;
        --preview) s3_pcc_mode=preview; shift ;;
        --append) s3_pcc_mode=append; shift ;;
        *)
            [[ $# -ge 2 ]] || s3_pcc_die "missing value for $1"
            case "$1" in
                --expected-commit) s3_pcc_expected_commit=$2 ;;
                --expected-last-ledger-hash) s3_pcc_expected_tail=$2 ;;
                --receipt) s3_pcc_receipt=$2 ;;
                --id) s3_pcc_id=$2 ;;
                --statement) s3_pcc_statement=$2 ;;
                --at) s3_pcc_at=$2 ;;
                --session) s3_pcc_session=$2 ;;
                --model) s3_pcc_model=$2 ;;
                --expected-claim-sha256) s3_pcc_expected_claim=$2 ;;
                *) s3_pcc_die "unknown argument $1" ;;
            esac
            shift 2
            ;;
    esac
done

[[ "$s3_pcc_expected_commit" =~ ^[0-9a-f]{40}$ ]] || s3_pcc_die "an exact expected commit is required"
[[ "$s3_pcc_expected_tail" =~ ^[0-9a-f]{64}$ ]] || s3_pcc_die "an expected last ledger hash is required"
[[ "$s3_pcc_id" =~ ^PCC-[0-9]{8}-[0-9a-f]{6}$ ]] || s3_pcc_die "invalid claim id"
[[ "$s3_pcc_statement" =~ [^[:space:]] ]] || s3_pcc_die "a nonempty statement is required"
[[ -n "$s3_pcc_receipt" ]] || s3_pcc_die "a receipt path is required"
if [[ -n "$s3_pcc_expected_claim" ]]; then
    [[ "$s3_pcc_expected_claim" =~ ^[0-9a-f]{64}$ ]] || s3_pcc_die "invalid expected claim hash"
fi
if [[ "$s3_pcc_mode" == append && -z "$s3_pcc_expected_claim" ]]; then
    s3_pcc_die "--append requires the reviewed preview's --expected-claim-sha256"
fi
for s3_pcc_tool in git jq sha256sum cut mktemp realpath flock date cmp head wc tail od tr dirname cat rm mkdir; do
    command -v "$s3_pcc_tool" >/dev/null || s3_pcc_die "required native tool missing: $s3_pcc_tool"
done
if [[ -z "$s3_pcc_at" ]]; then s3_pcc_at=$(date -u +%FT%TZ); fi
[[ "$s3_pcc_at" =~ ^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z$ ]] ||
    s3_pcc_die "timestamp must use UTC YYYY-MM-DDTHH:MM:SSZ"
[[ "$(date -u -d "$s3_pcc_at" +%FT%TZ)" == "$s3_pcc_at" ]] ||
    s3_pcc_die "invalid UTC timestamp"
[[ "$(printf '%s' "$s3_pcc_id" | cut -c 5-12)" == "$(date -u -d "$s3_pcc_at" +%Y%m%d)" ]] ||
    s3_pcc_die "claim id date and creation timestamp differ"

s3_pcc_repo=$(git -C "$(dirname -- "$0")" rev-parse --show-toplevel)
cd -- "$s3_pcc_repo"
[[ "$(git rev-parse HEAD)" == "$s3_pcc_expected_commit" ]] || s3_pcc_die "HEAD differs from expected commit"
s3_pcc_contract_hash=$(git show "$s3_pcc_expected_commit:tools/pcc/pcc.py" | s3_pcc_hash_stdin)
[[ "$s3_pcc_contract_hash" == 00036f8b524bf070a5cf36e02b2599148d390bdf30562d81a1a073df7777da2a ]] ||
    s3_pcc_die "PCC metadata contract changed; inspect it before adapting this writer"
[[ -f "$s3_pcc_receipt" && ! -L "$s3_pcc_receipt" ]] || s3_pcc_die "receipt must be a regular non-symlink file"
s3_pcc_receipt_real=$(realpath -- "$s3_pcc_receipt")
[[ "$s3_pcc_receipt_real" == "$s3_pcc_repo/"* ]] || s3_pcc_die "receipt is outside repository"
[[ "$(realpath --relative-to="$s3_pcc_repo" -- "$s3_pcc_receipt")" == "$s3_pcc_receipt" ]] ||
    s3_pcc_die "receipt path must be canonical and repository-relative"
[[ -f pcc/ledger.jsonl && ! -L pcc/ledger.jsonl && ! -L pcc && ! -L pcc/claims ]] ||
    s3_pcc_die "materialize the regular tracked ledger; PCC paths must not be symlinks"

s3_pcc_tmp=$(mktemp -d -t s3-pcc-register.XXXXXXXX)
s3_pcc_wrote=0
trap 's3_pcc_status=$?; if [[ "$s3_pcc_status" -eq 0 || "$s3_pcc_wrote" -eq 0 ]]; then rm -rf -- "$s3_pcc_tmp"; else printf "Registration failed after a write; retained records: %s\n" "$s3_pcc_tmp" >&2; fi' EXIT
s3_pcc_common_dir=$(git rev-parse --path-format=absolute --git-common-dir)
s3_pcc_lock="$s3_pcc_common_dir/pcc-statement-registration.lock"
[[ ! -L "$s3_pcc_lock" ]] || s3_pcc_die "registration lock must not be a symlink"
exec 9>"$s3_pcc_lock"
flock -x -w 5 9 || s3_pcc_die "another metadata registration holds the cooperative lock"

s3_pcc_validate_ledger() {
    jq -e -s '
      . as $all | length > 0 and all(range(0;length); . as $i |
        $all[$i].schema == "pcc.ledger-entry/v1" and
        $all[$i].seq == $i and
        ($all[$i].hash | type == "string" and test("^[0-9a-f]{64}$")) and
        $all[$i].prev == (if $i == 0 then ("0"*64) else $all[$i-1].hash end) and
        (["genesis","claim","verdict","bypass","note"] | index($all[$i].type)) != null)
    ' "$1" >/dev/null || s3_pcc_die "ledger schema, sequence, or links are invalid"
    jq -cS 'del(.hash)' "$1" > "$s3_pcc_tmp/canonical-bodies.jsonl"
    jq -r '.hash' "$1" > "$s3_pcc_tmp/expected-hashes.txt"
    while IFS= read -r s3_pcc_body; do
        printf '%s' "$s3_pcc_body" | s3_pcc_hash_stdin
    done < "$s3_pcc_tmp/canonical-bodies.jsonl" > "$s3_pcc_tmp/computed-hashes.txt"
    cmp -- "$s3_pcc_tmp/computed-hashes.txt" "$s3_pcc_tmp/expected-hashes.txt" >/dev/null ||
        s3_pcc_die "ledger hash mismatch or canonical-serializer incompatibility"
}

cat -- pcc/ledger.jsonl > "$s3_pcc_tmp/base-ledger.jsonl"
s3_pcc_validate_ledger "$s3_pcc_tmp/base-ledger.jsonl"
s3_pcc_count=$(jq -s 'length' "$s3_pcc_tmp/base-ledger.jsonl")
s3_pcc_tail=$(jq -sr '.[-1].hash' "$s3_pcc_tmp/base-ledger.jsonl")
[[ "$s3_pcc_tail" == "$s3_pcc_expected_tail" ]] || s3_pcc_die "ledger tail differs from expected last hash"
git show "$s3_pcc_expected_commit:pcc/ledger.jsonl" > "$s3_pcc_tmp/committed-ledger.jsonl"
s3_pcc_committed_bytes=$(wc -c < "$s3_pcc_tmp/committed-ledger.jsonl")
head -c "$s3_pcc_committed_bytes" "$s3_pcc_tmp/base-ledger.jsonl" |
    cmp - "$s3_pcc_tmp/committed-ledger.jsonl" >/dev/null ||
    s3_pcc_die "committed ledger is not an exact byte prefix"
s3_pcc_base_bytes=$(wc -c < "$s3_pcc_tmp/base-ledger.jsonl")
[[ "$(tail -c 1 "$s3_pcc_tmp/base-ledger.jsonl" | od -An -tu1 | tr -d '[:space:]')" == 10 ]] ||
    s3_pcc_die "existing ledger must end in newline; do not rewrite its prefix"
s3_pcc_claim_path="pcc/claims/$s3_pcc_id.json"
[[ ! -e "$s3_pcc_claim_path" && ! -L "$s3_pcc_claim_path" ]] || s3_pcc_die "claim path already exists"
if git cat-file -e "$s3_pcc_expected_commit:$s3_pcc_claim_path" 2>/dev/null; then
    s3_pcc_die "claim id already exists in committed history"
fi
if jq -e -s --arg id "$s3_pcc_id" 'any(.[]; .body.claim_id? == $id)' "$s3_pcc_tmp/base-ledger.jsonl" >/dev/null; then
    s3_pcc_die "claim id already occurs in ledger"
fi
git show "$s3_pcc_expected_commit:$s3_pcc_receipt" > "$s3_pcc_tmp/receipt"
cmp -- "$s3_pcc_receipt" "$s3_pcc_tmp/receipt" >/dev/null ||
    s3_pcc_die "receipt bytes differ from the expected commit"
s3_pcc_receipt_hash=$(s3_pcc_hash_file "$s3_pcc_tmp/receipt")
jq -n --arg id "$s3_pcc_id" --arg statement "$s3_pcc_statement" --arg at "$s3_pcc_at" --arg commit "$s3_pcc_expected_commit" --arg session "$s3_pcc_session" --arg model "$s3_pcc_model" --arg path "$s3_pcc_receipt" --arg sha "$s3_pcc_receipt_hash" '
    {schema:"pcc.claim/v1",id:$id,kind:"statement",claim_type:"data",statement:$statement,
     scope:"Unsigned producer development records for Dickson selectors with quartic and quadratic S3 coupling. Source review is within the same assistant team. No signed scientific verification, performance result, or complete-cost comparison is asserted. [pcc:unverified]",
     proofs:[{kind:"attestation",subject:{path:$path,sha256:$sha}}],
     producer:{commit:$commit,session:$session,model:$model},created_at:$at}
    ' > "$s3_pcc_tmp/claim.json"
s3_pcc_claim_hash=$(jq -cjS 'del(.standing)' "$s3_pcc_tmp/claim.json" | s3_pcc_hash_stdin)
if [[ -n "$s3_pcc_expected_claim" && "$s3_pcc_claim_hash" != "$s3_pcc_expected_claim" ]]; then
    s3_pcc_die "proposed claim differs from reviewed claim hash"
fi
jq -cnS --arg id "$s3_pcc_id" --arg sha "$s3_pcc_claim_hash" --arg path "$s3_pcc_claim_path" --arg prev "$s3_pcc_tail" --arg at "$s3_pcc_at" --argjson seq "$s3_pcc_count" '
    {schema:"pcc.ledger-entry/v1",seq:$seq,prev:$prev,type:"claim",at:$at,
     body:{claim_id:$id,claim_sha256:$sha,path:$path}}
    ' > "$s3_pcc_tmp/entry-body.json"
s3_pcc_entry_hash=$(jq -cjS '.' "$s3_pcc_tmp/entry-body.json" | s3_pcc_hash_stdin)
jq -cS --arg hash "$s3_pcc_entry_hash" '. + {hash:$hash}' "$s3_pcc_tmp/entry-body.json" > "$s3_pcc_tmp/entry.jsonl"

if [[ "$s3_pcc_mode" == append ]]; then
    [[ "$(git rev-parse HEAD)" == "$s3_pcc_expected_commit" ]] || s3_pcc_die "HEAD changed before append"
    cmp -- pcc/ledger.jsonl "$s3_pcc_tmp/base-ledger.jsonl" >/dev/null ||
        s3_pcc_die "ledger changed before append"
    [[ "$(jq -sr '.[-1].hash' pcc/ledger.jsonl)" == "$s3_pcc_expected_tail" ]] ||
        s3_pcc_die "ledger tail changed before append"
    cmp -- "$s3_pcc_receipt" "$s3_pcc_tmp/receipt" >/dev/null || s3_pcc_die "receipt changed before append"
    mkdir -p -- pcc/claims
    s3_pcc_wrote=1
    (set -o noclobber; cat -- "$s3_pcc_tmp/claim.json" > "$s3_pcc_claim_path") ||
        s3_pcc_die "claim no-overwrite creation failed"
    [[ "$(jq -cjS 'del(.standing)' "$s3_pcc_claim_path" | s3_pcc_hash_stdin)" == "$s3_pcc_claim_hash" ]] ||
        s3_pcc_die "created claim differs from the reviewed digest"
    # Recheck immediately before the append; never replace the ledger.
    cmp -- pcc/ledger.jsonl "$s3_pcc_tmp/base-ledger.jsonl" >/dev/null ||
        s3_pcc_die "ledger changed after claim creation; inspect the retained records"
    cat -- "$s3_pcc_tmp/entry.jsonl" >> pcc/ledger.jsonl
    head -c "$s3_pcc_base_bytes" pcc/ledger.jsonl | cmp - "$s3_pcc_tmp/base-ledger.jsonl" >/dev/null ||
        s3_pcc_die "ledger prefix changed during append"
    s3_pcc_validate_ledger pcc/ledger.jsonl
    [[ "$(jq -s 'length' pcc/ledger.jsonl)" -eq "$((s3_pcc_count + 1))" ]] ||
        s3_pcc_die "unexpected concurrent ledger entries after append"
    [[ "$(jq -sr '.[-1].hash' pcc/ledger.jsonl)" == "$s3_pcc_entry_hash" ]] ||
        s3_pcc_die "unexpected final ledger tail"
    [[ "$(jq -cjS 'del(.standing)' "$s3_pcc_claim_path" | s3_pcc_hash_stdin)" == "$s3_pcc_claim_hash" ]] ||
        s3_pcc_die "claim changed during append"
fi
jq -n --arg mode "$s3_pcc_mode" --arg claim_path "$s3_pcc_claim_path" --arg claim_hash "$s3_pcc_claim_hash" --arg entry_hash "$s3_pcc_entry_hash" --arg receipt_hash "$s3_pcc_receipt_hash" --arg contract_hash "$s3_pcc_contract_hash" --argjson prefix "$s3_pcc_count" --slurpfile claim "$s3_pcc_tmp/claim.json" --slurpfile entry "$s3_pcc_tmp/entry.jsonl" '
    {schema:"dickson-s3.pcc-registration/v1",mode:$mode,claim_path:$claim_path,
     claim_sha256:$claim_hash,entry_sha256:$entry_hash,receipt_sha256:$receipt_hash,
     contract_source_sha256:$contract_hash,base_entries_hash_checked:$prefix,
     appended_entries:(if $mode=="append" then 1 else 0 end),
     standing:"unverified",standing_source:"unsigned metadata only; no standing verifier executed",
     verifier_run:false,signature_created:false,python_executed:false,
     claim:$claim[0],ledger_entry:$entry[0]}
    '
