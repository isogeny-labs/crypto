# Integrate the shared-selector and public-tag study

User authorization: 2026-11-25, publication followed by integration of the
completed study, subject to repository checks. Paid experiments are not
authorized by this task. Tracking PR #5206 merged only this task card; the
implementation follows on the branch below.

Lane: `m31-shared-selector-integration-20261125`.
Implementation PR: [#5213](https://github.com/isogeny-labs/crypto/pull/5213).
Source problem: ordinary M31 ECDLP over `p=2147483647`, subgroup order
`r=2147446831`.

## Scope

- [x] Package the Rust exact-integer capacity checker, written proofs, and raw
  formula certificates with their sampling and formalization boundaries.
- [x] Package the pinned M31 shared-terminal support/public-label replay gate,
  preserving its archival-fixture and producer-evidence standing.
- [x] Add study-local Cargo targets, portable replay commands, an evidence
  brief/PDF/diagram, and a source-linked continuation handoff.
- [x] Run the affected native checks: six tests and exact retained-output
  comparisons. Root-library attempts remain incomplete and documented.
- [x] Publish the complete implementation in PR #5213 and pass local
  publication, inventory, prose, parity, ledger, and size checks. [pcc:unverified]
- [ ] Integrate PR #5213 after applicable GitHub checks pass.

The new-claim coordinate-input/cost screen is not satisfied by this archived
fixture, so this change adds no PCC ledger IDs. Results retain unsigned
producer standing. [pcc:unverified]

The construction/extraction research goal remains open. This publication does
not assert a new relation search, measured speedup, LFD result, or exponent
improvement. It does not edit the owned PairProjection, recursive-join, or H016
implementation paths. [pcc:unverified]
