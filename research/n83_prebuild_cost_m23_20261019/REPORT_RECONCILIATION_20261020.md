# Concurrent report reconciliation

Local commit `4ab4d655eb89ffbedd4cced722a94637c81fad9a` and concurrent
remote commit `3a111af9e` both documented the completed XOR-cache card.
The ordinary merge conflicted only in `RESULT.md` and
`xor-cache-v2/STATUS.md`. Their descriptive updates were combined.

Both archive copies and both manifests are retained. Byte comparison
passed for each pair:

- Archive SHA-256:
  `cf3310c7cb597c144b30a427683ed41d37cf4e2ec2425945d259a1173e776abd`.
- Manifest SHA-256:
  `55a87e7e0ca6561c48d1ad558ec8b3d8ba385e4cb4c26a6b6188688732eea4ee`.

The counter-only report and the complete native-audit report remain
separately identified. The former's renderer limitation is retained with
a dated local rendering addendum. Its cache-order parenthetical was
corrected from off/on/off/on to the actual off/on/on/off card. No raw
measurement, certificate, ledger row or frozen input was selected over
another or discarded. No solver implementation changed in this merge.

The separate conflict with the stacked parent in
`src/cryptanalysis/sat.rs` remains pending the requested Copilot resolution.
The active full-base experiment continues with its already-pinned binary;
a subsequent merge resolution cannot replace that experiment's source
identity or receipts.
