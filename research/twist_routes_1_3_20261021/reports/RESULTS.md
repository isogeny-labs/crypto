# E00 polynomial preflight: actual result

Date: 2026-10-21. Source/protocol freeze: [66f2d617cc60160b7fece7a38759ca2c61f0bae2](https://github.com/aburan28/crypto/commit/66f2d617cc60160b7fece7a38759ca2c61f0bae2).

**Observed:** five compiled polynomial identities passed exact integer-coefficient comparison and the repository evaluated-certificate checks. All five false right-side-plus-one variants were rejected by both arithmetic paths. The separate supporting certificate/hash suite passed 22 tests. This is a correctness preflight, not a curve, subgroup-transfer, solver or performance result.

## Evidence

- [Receipt](run-20261021T144610Z/RECEIPT.json), [issuance output](run-20261021T144610Z/issue.json), [fresh check output](run-20261021T144610Z/check.json).
- [Unit-test output](run-20261021T144610Z/unit-tests.stdout), [compiler](run-20261021T144610Z/compiler.txt), [host](run-20261021T144610Z/host.txt), and stdout/stderr for every command are kept beside the receipt. All recorded commands exited 0.
- [Frozen protocol](../E00_PROTOCOL.md) and [Rust source](../tools/src/main.rs). The receipt hashes the source, protocol, lockfile, binary and imported repository modules.
- Correctness-only L0 run; no algorithm timing or performance cost was measured.

| Statement | Degree | Expanded terms | Exact comparison | False variant |
| --- | ---: | ---: | --- | --- |
| Trace quartic | 4 | 5 | passed | rejected |
| Chord division | 4 | 9 | passed | rejected |
| Complement A | 4 | 3 | passed | rejected |
| Complement B | 6 | 5 | passed | rejected |
| Model conversion | 9 | 31 | passed | rejected |

Evaluated certificates (32 issuer points plus 32 additional checker points each):

- Trace quartic: [IDC1h4275fa8068385ce3](run-20261021T144610Z/certificates/trace_quartic_discriminant.json).
- Chord division: [IDC1h57c015780fe88055](run-20261021T144610Z/certificates/trace_chord_polynomial_division.json).
- Complement A: [IDC1h30c6f3a4043fe787](run-20261021T144610Z/certificates/cover_complement_A_cleared.json).
- Complement B: [IDC1hdf7545f3ee4dca7c](run-20261021T144610Z/certificates/cover_complement_B_cleared.json).
- Model conversion: [IDC1h9293014287b4205c](run-20261021T144610Z/certificates/cover_complement_change_of_variables.json).

The term counts describe symbolic expressions, not algorithm operation counts or useful relations. The 32 additional points use the checker's domain-separated stream. They are not an independent-host replay, and the certificates have proof=null. Published records were also checked to have 32 issuer points, seed 20261021 and proof=null.

## What this changes

The concrete trace-quartic and cover-complement designs have reproducible arithmetic preconditions. E01–E17 now have specific questions, families, controls, falsifiers and dependencies in the [experiment program](../EXPERIMENTS.md). No point census, restricted-base relation experiment, Groebner comparison, cover search, Jacobian transfer, pipeline ablation or NIST-size execution has been performed here.

## Remaining obligations

Formal Lean proofs were not compiled: lake/Lean are unavailable in this environment. Exact coefficient matching is an implementation check, not a replacement for the group's geometric hypotheses, rational-map exceptional charts or formal bridge. Independent-host replay remains pending. The original three route cards remain blocked on their actual point/solver/transfer implementations and admitted fixture/cost protocols.

## Visual and canonical views

![Experiment dependencies; E01–E17 are planned, not results](../PROGRAM.pdf)

The [editable diagram](../PROGRAM.dot) and [SVG](../PROGRAM.svg) describe dependencies. There are no comparable performance values to graph. This result adds no IC/rho ratio, curve registration, method/session or measured leaderboard row; the scoreboard, progress timeline, leaderboard and browser therefore receive no new value from it.

Individual certificate/receipt paths are linked directly and stay as loose immutable artifacts; they are not repacked. Later checks use fresh output directories.
