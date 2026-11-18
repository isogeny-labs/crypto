# Indexing correction to the frozen review and interface note

The earlier REVIEW.md sentence saying "retained maps/BQI use positive global IDs" is incorrect for BQI. Likewise, the RUST_INTERFACE.md instruction to preserve positive IDs applies only to CheckedMap/DIMACS wires, not to sparse polynomial or BQI monomial indices. Those frozen artifacts remain unchanged; this addendum supersedes their indexing wording.

Verified the actual source contract:

- checked_boolean_equations.rs line 2 states `DIMACS wire w -> index w-1`.
- sparse_equations.rs `wire` rejects w=0 and w>n, then returns w-1. Original sparse polynomial coordinates are therefore zero-based global indices in [0,N).
- boolean_identity_bqi.rs line 11 specifies strictly increasing monomial indices in [0,N); its parser rejects a monomial value >=n. BQI uses the same zero-based global sparse coordinate convention.

Correct constructor rule: map each local factor coordinate through its actual positive CheckedMap block wire, validate 1<=wire<=original_N, then emit sparse/BQI variable index wire-1. Do not assume local coordinate equals global index. Original sparse indices remain [0,original_N). Fresh selector and gate sparse indices begin exactly at original_N, increasing deterministically with checked arithmetic; the corresponding DIMACS wire would be index+1 only if a DIMACS representation is required. Final sparse/BQI indices must lie strictly below the extended variable count. Retain the proposed block-major allocation, versioning and full ordered-generator identity requirements.

This correction changes no polynomial-count calculation or mathematical equivalence argument. No production edits, builds or native runs were performed.

Verified source SHA-256:

- checked_boolean_equations.rs: fc91690f91232c8d2fc7ed9c72821db51c0f7c9d1367a8cb85237be9c1073fc8
- sparse_equations.rs: d866047d290454ae75b9bb8eb586db2b0ce0a7d9efcb5d7ba4edb9dfd1d257ae
- boolean_identity_bqi.rs: 567734de7ab93a7320360fe0c7fec6b60b09d1cc9fd477965d1acb8674822ce7
