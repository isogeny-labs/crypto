# Requirements trace: n=83 adaptive image router

| Requirement | Evidence planned | Final status |
|---|---|---|
| Exact cover, field, and target subgroup identity | prior projector certificate plus native replay | verified in `certificate.json` |
| Arbitrary-class norm remains a homomorphism | existing 486-class exhaustive implementation test | passed in the exhaustive audit path; the library-test target is blocked by an unchanged upstream duplicate symbol |
| Projector is constant on norm/kernel cosets | algebraic derivation, Lean lemma, exact fixture | verified; PCC-20261118-81b046 |
| Exact Jacobian relation implies projected elliptic relation | all ordered `GF(8)` pairs | verified: 486/486 exact pairs project to zero |
| Projected elliptic relation does not generally lift | nonzero-kernel negative control and exact pair counts | verified: 38,880 false projected cancellations |
| Image-closed relations add no independent label | injectivity of `phi^*` on order-`r` torsion | checked formula and pinned n=83 replay |
| Growing-degree boundary is explicit | Mumford degree cap and separate oracle obligation | recorded in `RESULTS.md` |
| Relation/matrix/individual-log accounting | classification table with explicit unavailable entries | stopped before admission; missing stages explicit |
| No overclaim beyond projector-only routing | assessment open obligations and PR wording | scoped in results and assessment |
| Reproducible, native certificate | Rust binary and deterministic JSON | verified; SHA-256 `cf4643...49b45` |
| Evidence-linked visual and inspected PDF | editable diagram, SVG, Typst source, PDF | passed; two A4 pages inspected at 120 dpi |

The study reports exact finite counts and algebraic implications only.  It has
no performance measurement and makes no end-to-end speed claim.
