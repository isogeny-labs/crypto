# Norm-one-torus m=7 single-target study

This directory contains a frozen, public, single-target Rust experiment for an
exact seven-term decomposition over the Prime31 p+1 Dickson factor base.

- `PROTOCOL.md` freezes the original four-arm comparison.
- `PROTOCOL_AFFINE_LEAF.md` freezes the disclosed affine-leaf follow-up.
- `PROTOCOL_DEFERRED_Y.md` freezes the exact-x-first follow-up.
- `REPORT.md` gives the scoped result, literature boundary, proof boundary,
  and next goal.
- `public-precommit/` contains seed and source custody, the 45-row affine panel,
  the 54-row exact-x-first panel, certificates, exact analyses, and independent
  affine replay output.

Every setup is charged to the one target. Cross-target amortization is absent
and forbidden by the manifests. Claims `PCC-20261126-cc71bb` and
`PCC-20261126-3c0b9a` are currently `unverified`; the retained evidence is
transparent, not zero knowledge or host attestation.
