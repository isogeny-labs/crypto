# Round 05 validation

All receipts are in runs.tar.xz. Historical failed attempts are preserved,
with successful replacements in separately named files.

| Check | Receipt | Outcome |
|---|---|---|
| Root library | root-lib-tests-01.log | 4396 passed, 0 failed, 118 ignored |
| Native study | native-tests-02.log | 106 passed, 0 failed, 1 ignored |
| Exact replay | replay-04.log | Six JSON records and generated quotient Lean source identical |
| Prior round replay | prior-round04-replay-01.log | All seven outputs identical; separate library/divisor checks pass |
| Reverse composition | Separate checker in replay | Both 4x4 compositions checked using BigInt; scalar32000 |
| Quotient identities | Separate checker and Lean | Five integral inverses/pairings; nine failed first-graph matches retained |
| Interior divisor | Separate checker in replay | Factorization, all616 zero labels, rank5/588 and six pole-image torsion checks pass |
| New point images | M7 flag and proof map | Unevaluated; evaluated relation matrix stays rank4/544 |
| Lean kernel | lean-build-06.log | All modules build;18 new named theorems |
| Transitive axiom audit | lean-axioms-03.log | 18 audited;0 failures;0 native trusts |
| Symbolic identities | record-02.log | Three IDs;64 issuer plus128 extra points each; exact Lean scope bound |
| Dashboard / site | dashboard-check-01.log, site-tests-01.log | Dashboard current;49 site tests pass |
| Raw/source hashes | record-raw-hash-check-02.log, record-source-hash-check-02.log | Ledger bindings verified |
| Archive | runs.sha256 and runs.tar.xz.sha256 | Full member and fresh-extraction checks; deterministic repack identical |
| PDF | build_report.py and pdf-review/ | Editable Typst/SVG, all rendered pages visually inspected |

The source-level geometry uses standard modular moduli, adjunction,
prime-to-characteristic torsion and a supersingular deformation. Those
arguments are written in full in PROOF_MAP.md. Neither the symbolic row
nor its rank is represented as an evaluated new factor-base map.

Failed runs include the initial point-twist assumption, the narrow
maximal-order screen, nine graph mismatches, the first generated-module
Lake declaration and two unsuccessful torsion simplifications in Lean.
The first assembly used the previous binary while a rebuild was still
running. Its raw hash check failed on the two live logs. That failed
ledger and its identities are preserved under assembly-attempt-01;
the final assembly ran after the rebuild and excludes its own receipt logs.
Error-recovery sorryAx outputs in failed Lean logs are not accepted proofs;
the final build and transitive audit have no such trust.

The focused CI now replays rounds01–05 before the pinned Lean build, with
the existing25-minute cap unchanged. Remote queued jobs are not a pass.
No browser screenshot check was run: no browser is installed locally.
The report's PDF pages were inspected separately. No IC timing was measured.
