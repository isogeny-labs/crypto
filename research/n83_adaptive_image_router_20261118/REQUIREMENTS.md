# Requirements trace: n=83 adaptive image router

| Requirement | Evidence planned | Status at freeze |
|---|---|---|
| Exact cover, field, and target subgroup identity | prior projector certificate plus native replay | pending replay |
| Arbitrary-class norm remains a homomorphism | existing 486-class exhaustive implementation test | previously checked; replay pending |
| Projector is constant on norm/kernel cosets | algebraic derivation, Lean lemma, exact fixture | pending |
| Exact Jacobian relation implies projected elliptic relation | all ordered `GF(8)` pairs | pending |
| Projected elliptic relation does not generally lift | nonzero-kernel negative control and exact pair counts | pending |
| Image-closed relations add no independent label | injectivity of `phi^*` on order-`r` torsion | pending derivation replay |
| Growing-degree boundary is explicit | Mumford degree cap and separate oracle obligation | pending report |
| Relation/matrix/individual-log accounting | classification table with explicit unavailable entries | pending report |
| No overclaim beyond projector-only routing | assessment open obligations and PR wording | pending |
| Reproducible, native certificate | Rust binary and deterministic JSON | pending |
| Evidence-linked visual and inspected PDF | editable diagram, SVG, Typst source, PDF | pending |

The study reports exact finite counts and algebraic implications only.  It has
no performance measurement and makes no end-to-end speed claim.
