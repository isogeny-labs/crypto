# E29 proof extension, version 1

Commit this separate project before its first build. Retain all unsuccessful
builds and changed source revisions with exact hashes. Preserve the original
Lean project, lockfile, source and receipts. Use Lean 4.34.1 and the same
Mathlib revision d13f23b723b8a846827a245b89c10fc7d3f11612. Compile the original
TwistRoutes module into a separate prerequisite output directory, then make
that directory available to this project's imports. Never overwrite its
frozen receipts. Each module/build shard has one CPU, a 4-GiB address-space
limit and a 600-second cap.

Construct F59[w]/(w^2−2), prove irreducibility by kernel-checked finite
nonsquareness, derive w^59=−w, and verify the native pair-coordinate addition,
multiplication and conjugation formulas. Construct an actual additive
Frobenius map on Mathlib's Weierstrass point group by its finite-field algebra
homomorphism, rather than postulating an additive endomorphism. Extend the
original mixed-relation theorem to an explicit odd annihilator and prove
the zero-trace consequence for arbitrary finite integer helper rows.

Use only kernel proofs. Run tools/lean/AuditAxioms.lean on every exported
theorem and reject sorryAx, custom axioms and native_decide/compiler-trust
axioms. A clean audit alone does not prove all native/formal bridges: retain
explicit gaps for the standard twist's group isomorphism, the exact odd
subgroup/projector, the relation-module map and the E23 matrix serialization.
Later source revisions may close specific gaps, with new pinned build/audit
receipts. Do not infer full E29 completion from these partial lemmas.
