# Sources consulted before construction

Reviewed predecessor packets solve4-native-narrow-inverse and solve4-pivot-normalization, their Narrow.lean.txt and Normalization.lean.txt, source-fidelity.csv, and controls. Exact finite-function row model comes from solve4-elimination-stage. Narrow inverse follows literal bit test, unconditional square and right shift with64fuel; it supplies the precise inverse value obligation normalization previously assumed. This is composition of existing proofs, no new inversion algorithm.

Primary mathlib GroupWithZero.Basic documentation https://leanprover-community.github.io/mathlib4_docs/Mathlib/Algebra/GroupWithZero/Basic.html records inverse cancellation and inverse-at-zero semantics. Canonical nonzero decoding is necessary; native word nonzero alone does not imply a nonzero residue without the canonical bound. The composition reuses those established proofs rather than deciding a symbolic inverse fixture.
