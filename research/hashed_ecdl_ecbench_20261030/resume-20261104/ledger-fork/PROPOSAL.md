# Proposed ledger reconciliation — not applied

The source and development receipts are uploaded in crypto PR2984. Current
main and the PR append different events after the same 81-row prefix. Main has
84 rows and the PR 86. A read-only merge preview reports conflicts in
`pcc/ledger.jsonl` and the November4 session journal. Their complete original
bytes, source revisions and hashes are preserved in `manifest.json` and four
gzip snapshots. These are raw operational observations, not registered claims.

The protected canonical ledger and primary journal remain untouched while
[Copilot's request](https://github.com/isogeny-labs/crypto/pull/2984#issuecomment-6638594871)
is pending. No ledger import/reconciliation command was found in the current
PCC CLI or contract. Its official `append_entry` primitive can create a draft
outside the protected path; that draft and its source-hash mapping are in
`proposal.json` and `proposed-canonical-ledger.jsonl.gz`.

## Concrete action requiring an owner exception

1. Preserve both complete ledgers, both ordered parent journals and every
   original claim JSON byte-for-byte. Refresh snapshots if either ref advances.
2. After Copilot coordination, merge main normally without rebase/force. Adopt
   current main's ledger bytes as the new canonical prefix.
3. Use the existing canonical append primitive to replay only the five original
   unsigned `claim` event bodies listed in `manifest.json`, retaining their
   claim IDs, file hashes and bodies. Keep all original claim files unchanged.
   Archive the original row envelopes; new canonical envelopes get new sequence,
   previous-hash, timestamp and hash fields. Append a migration note with both
   source hashes. No signed verdict or key is recreated or changed.
4. Require the canonical chain and current-main prefix checks, immutable-source
   comparisons and all normal protection/review checks. Preserve complete
   ordered parent journal bodies; never concatenate conflicted binary files.
5. Qualify the exact combined code and source manifests. If runtime inputs
   change, preserve frozen v3 and file a new pre-data protocol amendment before
   any affected study data. Current budget/S3/image gates remain mandatory.

The prepared 90-row draft preserves main's 84-row prefix and the five claim bodies,
with one new migration note. Its SHA-256, every old/new event hash and the
canonical tool's SHA are in `proposal.json`. It changes no protected file.
Its integrity checks are producer data; no independent verdict is claimed.
[pcc:unverified]

## Why approval is needed

[AGENTS.md lines173-176](https://github.com/isogeny-labs/crypto/blob/b3ce573b3df1c2ece0113b90f2c3a375493f9ad6/AGENTS.md#L173-L176)
says: “Never edit or rewrite `pcc/ledger.jsonl`.”
[Lines836-837](https://github.com/isogeny-labs/crypto/blob/b3ce573b3df1c2ece0113b90f2c3a375493f9ad6/AGENTS.md#L836-L837)
say: “Conflicts in frozen evidence, ledger rows, or claim files are never
resolved by picking a side: report them to the user.”

The proposed canonical-path replacement needs an explicit exception to the
no-rewrite rule. Both original branches remain preserved. No permission is
requested to weaken a check, change a key/verifier, discard evidence, sign a
verdict or start cloud/scientific work.

## Working handoff

- What was done: source/P1/policy corrections uploaded; release validation raw
  receipts attached separately; both divergent ledgers/journals archived.
- Decisions: leave protected files untouched; prepare an inspectable draft with
  the canonical implementation; request a narrowly scoped owner exception.
- Open threads: owner exception, Copilot coordination, combined-source validation,
  current-head CI/review, then fresh budget/checkpoint/image admission.
- Errors: concurrent appends cannot be merged by ordinary line concatenation;
  the required current-main prefix check reports a mismatch. No reconciliation
  or protected-file write has been performed.
