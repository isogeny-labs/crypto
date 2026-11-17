# Round07 validation

All raw receipts, including failures, are preserved in runs.tar.xz.

| Check | Receipt | Outcome |
|---|---|---|
| Root library | root-lib-tests-01.log | 4405 passed,0 failed,118 ignored |
| Native study | native-tests-01.log | 106 passed,0 failed,1 ignored |
| First fresh replay | replay-01.log | 49 JSON records, expanded text and generated Lean identical |
| Final archive replay | ARCHIVE_VALIDATION.txt | All50 JSON records, expanded text and generated Lean identical |
| Prior round06 | prior-round06-replay-01.log | All records replay unchanged |
| Even forward norms | n4-even-independent-forward.json | All7200 divisor entries checked; independent result equals source |
| First full-map row | m8b-independent-F32-check.json | Separate F32 images and complete principal sum |
| Second full-map row | m9b-independent-small-field-check.json | All120 new images checked; independent ranks7 and6 |
| Geometric orbit bases | m9c-geometric-base-check.json | All24 source degrees60; five phase sets disjoint;60 actual nontrivial Galois steps and half-period tests |
| Mixed quotient atlases | q5-mixed-translation-atlases.json | All1444 graph points, kernel/Frobenius/pairing checks and8 invariant circuits |
| Expanded cofactor cover | q5b-expanded-cofactor-cover.json | Exact curve identity,722 kernel checks and64 scalar comparisons |
| Odd reverse divisor | odd-verified/o4-odd-reverse-divisor.json | All20 fibers,2200 entries,F2 formal descent,independent forward equality |
| Odd separate parser | odd-verified/o4b-odd-coordinate-check.json | 2200 entries;4400 curve/torsion checks; selected lines; independent nonzero order11 source |
| Lean kernel | lean-build-03.log | Full pinned project builds;17 new named results |
| Transitive axiom audit | lean-axioms-02.log | 18 declarations,0 failures,0 native trusts |
| Identity certificates | record-01.log | Two direct identities, each64 issuer plus128 extra checks |
| Dashboard/site | dashboard-check-01.log,site-tests-01.log | Dashboard current;49 site tests pass |
| Workflow parse | workflow-tests-01.log | Four tests pass; focused25-minute cap unchanged |
| Raw/source hashes | record-raw-hash-check-01.log,record-source-hash-check-01.log | Every ledger binding verified |
| Archive | ARCHIVE_VALIDATION.txt | Fresh extraction/member hashes/replay and deterministic repack |
| Report | build_report.py,report.typ,routes.svg | Editable sources; every PDF page visually inspected |

The first odd adapter inherited the F32 q-mod11 Frobenius entry and
degree5 embedding modulus. No fiber from those attempts was accepted.
Logs01–03 were interrupted by tool resets; stale probe04 was explicitly
stopped after its source update failed because Node had disappeared.
Successful probe05 uses the actual degree10 modulus, full group-order
checks and bounded primary extraction. A mixed invariant sample initially
hit infinity; the replacement excludes19-kernel translates. Build04 needed
an explicit BigInt type, and build09 was interrupted by the runtime reset.
The ledger preserves these failures and their successful replacements.

The independent forward calculations do not use the Gram matrix to obtain
their norms. They share field and group primitives with the construction;
separate small-field arithmetic checks the final images and relation ranks.
This is a same-agent replay and verification, not an external audit.

The mixed intrinsic invariant image ideal and full chart elimination remain
unfinished. The reverse divisors are unreduced and the odd representative
is specialized to11-torsion. Global modular/Picard arguments and the
binary-mask-to-polynomial bridge remain written, with formal hypotheses
exposed in PROOF_MAP.md. lean4checker is unavailable; the actual Lean kernel
build and transitive audit ran. No timing or IC efficiency claim is made.

The canonical narratives, dashboard source and study status visual are
updated. Performance/leaderboard/progress cost plots are unchanged because
this round measures no costs. Remote CI is assessed on the pushed head;
local success does not represent pending remote checks as passed.
