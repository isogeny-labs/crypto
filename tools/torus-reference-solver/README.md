# Bounded public-input relation reference

Setup::build accepts public Spec and arity2/4/6; Setup::solve receives only
the target. The entire declared small base is constructed internally.
The result is an original-circuit-checked root or finite exhaustion;
unsupported inputs/resource limits return errors rather than absence.

Inputs must satisfy the earlier target interface's external prime-field,
exact-order, map and nonsingular/complete-conversion curve contracts. This
crate does not certify those instance facts. Raw image aliases remain
distinct; identity leaves are explicitly excluded. Caps are64 raw columns
and131072 signed half tuples. Raw masks use64 bits; Vandermonde field labels
must remain distinct. No witness or private query coefficients enter solve.

The classical meet-in-the-middle reference uses uniform-matroid
representative families to preserve disjoint completions, at most20 original
half witnesses per sum for six members. It still enumerates every signed
half triple and is a cubic uncapped reference, not a scalable subquadratic
solver. Selected event counters are not normalized field-work/timing metrics.

See ../../research/curve25519_torus_native_20261115/root-reference-v1/PROTOCOL.md
and the source-linked reports/root-reference-v1/REPORT.md for exact controls,
source reconstruction, identity scope and classification boundaries.
