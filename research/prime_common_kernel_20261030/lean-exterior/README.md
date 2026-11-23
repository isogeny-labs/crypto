# Finite Cauchy parity-code witness

This small project proves the actual L=4 source-polynomial counterexample
to a passive-channel elimination inference. It is not a contraction or
ECDLP algorithm. See PROOF_MAP.md for the exact seven declarations and
remaining general gaps; MATHEMATICAL_PROOF.md gives the readable proof.

Pins: Lean4.35.0; shared CryptoLean148f5dc389b341401e06d76abe7783c4ccf42f2e;
Mathlibcfc55fc58714a59dc06e6e79edc6d1d95140d3c2. Transitive package commits
are in lake-manifest.json. Dependencies are reused from the existing pinned
read-only package cache through an ignored local .lake/packages symlink;
that symlink is a machine-local development convenience, not a vendored
certificate or a hidden dependency change.

Validation commands (official pinned Lean binaries on PATH):

    lake build
    lake build +ExteriorWitness
    lake env lean --run .lake/packages/CryptoLean/tools/AuditAxioms.lean ExteriorWitness
    lake env leanchecker --fresh --verbose ExteriorWitness
    .lake/packages/CryptoLean/tools/proof-gate.sh --out receipts/proof-gate .

Fresh checker and gate outcomes are preserved under receipts/; failed
attempts are retained separately. No timings support a performance claim.

Final observed outcomes: ordinary build PASS; explicit +ExteriorWitness
source build PASS; shared proof-gate PASS (7theorems,0trusted); transitive
axiom audit PASS (7theorems,0failing,0trusted); import-inclusive
`leanchecker --fresh --verbose ExteriorWitness` exited0 and reports
`replaying ExteriorWitness with --fresh`. The final source SHA256 is
`93acd5a86cb0f5509947a5d8fe0a0b12bcb46c5be7579fdb46694f2b202be359`.
