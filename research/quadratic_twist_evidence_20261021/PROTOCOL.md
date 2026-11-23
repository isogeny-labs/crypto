# Historical evidence import and native audit

This protocol is frozen before the final native audit/control runs. It does not
change the historical experiment workload or its recorded results.

## Requested deliverable

Move the quadratic-twist experiment PR to `aburan28/crypto`, preserve the source,
raw results, execution transcript and provenance, and make the evidence reviewable.
The earlier PR in `ecdlp-prime-field-smallness` is superseded after the replacement
opens. No merge or performance promotion is part of this import.

## Frozen inputs

- `historical/source-and-evidence.tar.xz`, with its adjacent SHA-256 file.
- The archive holds the exact Python source and records of the completed
  2026-10-21 run; its internal `SHA256SUMS` binds 12 files.
- Three saved pairing/IC fixtures, in order: field/subgroup pairs `(43,11)`,
  `(67,17)`, `(211,53)`. Their complete models, points, twist parameters, relation
  rows and answers are in the historical JSON.
- The smooth/anomalous twist examples retain their original parameters.

## Native audit obligations

1. Verify every internal hash and bind the recorded experiment source hash.
2. Check the historical success/reference flags as recorded metadata, without
   treating those flags as an independent witness of past process execution.
3. Independently recompute point membership, subgroup orders, Miller pairings,
   all 81 scalar-pairing identities and all 3,219 bilinearity identities.
4. Verify each saved IC relation, matrix rank, projected factor-base logarithm,
   and displayed recovered scalar. No original Python process is launched.
5. Independently count the two twist examples over their prime fields.
6. Reject a corrupted source. Reject an algebraically false saved relation even
   after its file checksum is updated to match the corruption.

Success requires all checks to pass and each negative control to fail for its
specified reason. Any failed check exits nonzero and retains its transcript.
There is one frozen workload, no random search or performance acceptance target.

## Accounting and limits

This is a correctness/integrity audit. Build and audit timestamps are provenance,
not algorithm timing. No matched-rho cost, external-tool speed comparison,
isolation claim, throughput result, or end-to-end speedup is produced.

The full 942-model census, S3/S4 census, higher-degree trace-zero enumeration,
subfield descent and relation collection are not rerun by this native auditor.
They remain historical Python evidence. Full native parity and checked Lean
formalization remain separate obligations; their absence is explicit in the
receipt and results report.
