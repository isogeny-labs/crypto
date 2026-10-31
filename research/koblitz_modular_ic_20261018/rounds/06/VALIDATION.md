# Round06 validation

Raw receipts are preserved in runs.tar.xz; failures are never overwritten.

| Check | Receipt | Outcome |
|---|---|---|
| Root library after base merge | root-lib-tests-01.log | 4405 passed,0 failed,118 ignored |
| Native study final sources | native-tests-02.log | 106 passed,0 failed,1 ignored |
| Fresh exact replay | replay-01.log | All24 JSON outputs and generated Lean identical |
| Prior rounds | prior-round04-replay-01.log, prior-round05-replay-01.log | Both replay unchanged; separate checkers pass |
| Closed images | i1b-image-checks.json | All616 equivariance tests per sign;44 zero relative14-traces;672 schoolbook products |
| Degree7 explanation | i1c-seven-fiber-cancellation.json | All96 target fiber sums equal order2 cusp; local jets through order12 |
| Quotient checker | q4b-independent-checks.json | All250 actual kernel points; BigInt polarization and descent identities |
| Reverse checker | r3e-coordinate-checks.json | 660 split bases,60 degree11 components and7200 divisor entries |
| Matrix checker | i2-zero-column-matrix-check.json | Evaluated rank5/588; rank4 after dropping new columns; nonzero-image matrices rank4 |
| Lean kernel | lean-build-02.log | Pinned full project builds;14 new named results |
| Transitive axiom audit | lean-axioms-02.log | 15 declarations,0 failures,0 native trusts |
| Identity certificates | record-01.log | Two direct identities, each64 issuer plus128 extra checks |
| Dashboard and site | dashboard-check-01.log, site-tests-01.log | Dashboard current;49 site tests pass |
| Workflow parse | workflow-tests-01.log | Four tests pass; focused25-minute cap unchanged |
| Raw/source hashes | record-raw-hash-check-01.log, record-source-hash-check-01.log | All ledger bindings verified |
| Archive | ARCHIVE_VALIDATION.txt | Fresh extraction/member hashes/replay and deterministic repack checked |
| Report | build_report.py, report.typ, routes.svg | Editable layout; every rendered PDF page inspected |

Failures: build-01 overflowed an unused unsigned expression; build-06 needed
an explicit BigInt type; reverse-screen-01 incorrectly expected nonsplit
positive complements; split-fiber-negative-01 incorrectly attempted full
splitting for the negative seed. The successful replacements are distinct
files. The negative seed is instead complete in finite-etale coordinates.

The even reverse result is an unreduced divisor, not genus2841 Jacobian
reduction. Its forward check invokes the written/proved Gram correspondence,
not an independently computed pointwise norm. The separate verifier shares
field/group primitives with the generator. Full modular, Picard and local
divisor formalization remains open. lean4checker is unavailable; the actual
Lean kernel build and transitive audit ran. No browser is installed; PDF
pages were inspected separately. No speed or IC efficiency claim is made.

The status graph and canonical narratives are updated. Performance and
leaderboard plots are unchanged because this round measures no costs.
Remote CI must be assessed on the pushed head; local success does not
represent queued or pending remote jobs as passed.
