# Standalone backend build repair, 2026-10-27

This is software integration maintenance, not a new relation-yield, solver-degree,
proof or timing result. Baseline is PR head
`51d8b2932c64e9adf47e1afab3ef43eff233c5ed`, after four normal main merges.

## Observed failures and repair

- [Native job122436115778](https://github.com/isogeny-labs/crypto/actions/runs/40574197260/job/122436115778)
  fails to compile its existing backend tests: the imported current-main field
  module requires `super::operation_tally`, and the curve-ID test requires
  `docs/ic/calibration.json`. The exact six compiler errors reproduce locally.
  The metadata crate now declares the canonical repository module, with every
  other source byte identical. Its sparse checkout includes both required files.
- [F13 job122436119178](https://github.com/isogeny-labs/crypto/actions/runs/40574197515/job/122436119178)
  passes the existing producer, independent audit, exact certificate checks and
  controls, then fails backend-test compilation because the sparse checkout
  omits `docs/identity-certificates/schoenhage-10mult.json`. Downloaded CI artifact
  `12765514982` has ZIP SHA256
  `c35545f6b09d199061ce4a8feec7955b17672945687c31b9f39bd456c2e63b91`.
  The seven standalone replay workflows import that same identity backend and
  now include its existing test fixture. Their commands, triggers, conditions,
  budgets and verification predicates are unchanged.

The six other downloaded replay artifacts independently confirm the identical
missing identity-JSON compiler error in their backend-test logs. Their ZIP hashes,
artifact IDs and exact diagnostics are retained in the raw archive. Their native
audits passed before that test-compilation failure. No fresh local scientific
rerun is inferred from this review, and resulting-head hosted checks remain due.
The unrelated workflow-budget failure concerns Certicom metal-qualification
timeouts; neither that workflow nor the global checker is changed.

## Local validation and preserved evidence

The repaired metadata crate passes42 existing backend tests; the unchanged F13
auditor passes22 existing backend tests. Four workflow parser tests pass over
all510 workflows. A separate structural comparison proves that
each workflow mapping is identical after removing the added checkout paths.
These are ordinary local software tests; no producer, auditor CLI, corruption
CLI, registry emission, scientific experiment or Lean elaboration is launched.

`runs.tar.xz` preserves compiler/setup failures, successful build/test logs,
the pre-edit metadata source as `.rs.txt`, exact source hashes, commands and CI
diagnostics. `RUNS.sha256` binds restored files; `restore-runs.sh` checks them.
The first offline dependency attempt lacked a cached serde_json; a subsequent
minimal fixture omitted an already-required identity JSON. These setup errors
remain distinct from the exact six-error CI reproduction. The root-library
attempt and its actual status are in `validation-summary.json`; historical
4479-test receipts are not substituted for the refreshed tree.

Sources checked before selecting the fix: current repository AGENTS and active
study requirements, the failing CI diagnostics, canonical
`src/binary_ecc/operation_tally.rs`, `curve_id.rs` and
`identity_certificate.rs`, and the eight sparse workflow declarations. This
reuses existing backend code and fixtures; it adds no cryptanalytic mechanism.

## Holds retained

The refreshed root-library check also exposes an active registry integration
regression. The pre-merge4493-model registry at `2d1407270` has SHA256
`87724546ad2e971572c4f62ae0fbc4b2bfed8cd028ae30ded78bc4c4bb9287ca`.
The merged20592-model registry at `51d8b2932` has SHA256
`e7e8cd2ce2570e6e6931674ff653fbaf0cd513b447738364dbb5440a0974a420`.
Exact slug comparison finds1921 missing old models (958 prime-field,963 extension),
18020 new models and150 changed shared records. The native parser's strict
extension test expects2287 but the merged registry contains1324. Lowering that
assertion would hide the lost study population; it is left unchanged.

The canonical `registry-integration-review.json` retains every missing slug
and changed-shared slug. This metadata comparison does not validate or reject
the new/changed models mathematically. Restore the study's registered population
while preserving and validating current-main records, representations and cover
bindings; do not replace the entire registry with either side. Native identity,
cover/catalog, membership and byte checks must then pass on the combined inputs.
That reconciliation and any dependent scientific replay remain open under the
existing launch/coordination gates. The two-line backend repair alone does not
resolve this regression or make the full library suite green.

The canonical JSON ledger stays directly reviewable as a loose record under the
existing JSON policy. Its redundant archive copy is dehydrated: the restore
script copies that same hash-bound record back into the raw tree before checking
every original file hash. Repacking the restored tree with the same omission is
byte-identical. The initial all-in-one archive exceeded the64KiB binary policy;
no upload or policy exception was used. Full local raw bytes remain preserved.

The uniform96-arm/896-target/210-comparison study remains unexecuted, its
recurrence draft unelaborated, and208 public/269 audited historical declarations
and17 identity records remain historical. No old result, certificate, input
manifest, registry, benchmark or dated report is rewritten. Both catalog
provenance anchors remain in the normal parent history.

Scientific launch still needs the pending checkpoint-only upload authorization
and secure token access. No gate/upload/credential reuse or paid compute occurs.
Conductor exits127; no shared reservation is claimed. Auto-merge, found enabled
again, was disabled. Keep this PR open even if its merge-set checks pass.
Current instructions/rules name exactly `gate`, `size`, `artifacts`; new-head
hosted results must be checked separately. At the reviewed baseline, gate passes
and size/artifacts are cancelled, not passes. Mathematical, coordination and
completion obligations in the existing problem record remain open.

The full-library test process was interrupted at600 test-phase seconds after a
successful release compile; Cargo exits101. Its nested one-test subprocess
summary is not a completed5057-test suite. No full-library pass is claimed.
