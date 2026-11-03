# Completed verification

All checks below completed successfully on the final source, SHA256
`3a4e37130c769ee3cae11d5aef6710275d880066f18bcf7e142d008653059316`.

- `lake build`: exit0, four named declarations, clean final build004.
- Shared `AuditAxioms.lean CodedPacking`: exit0;4 audited,0 failing,
  0 trusted native declarations. Foundations are only `propext`,
  `Classical.choice`, `Quot.sound`.
- `lake env leanchecker --fresh --verbose CodedPacking`: exit0;
  fresh official replay completed, log retained.
- Pinned shared `proof-gate.sh --out proof-gate .`: exit0;4 declarations,
  0 trusted. This used the isolated Git-tracked package, not a claimed run
  against its future consuming repository path.

The first attempt failed on an unavailable finite-dimensional abbreviation;
the second lacked the import supplying vector-space freeness for product
finrank. The third passed with an unused section-instance warning, removed
in the clean fourth build. Actual failed source snapshots and diagnostics
remain unchanged. They are not proofs or mathematical counterexamples.

This package formalizes only the exact field-level statements in its proof
map. The written projected-filter interpretation, code-position cardinality,
asymptotic counting substitution and limitations are not automatically
additional Lean theorems. The complete original ECDLP goal remains open.
