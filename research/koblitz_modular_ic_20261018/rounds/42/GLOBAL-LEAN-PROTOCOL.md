# R42 full integer polynomial recovery proofs, frozen before compilation

Use the three successful GLOBAL42 cases, literal integer coefficient arrays
and source CM exponents. Python ONLY renders coefficients/exponents into
Lean expressions. No arithmetic or certificate solving is done in Python.

For each n11/83/131 prove universally in any commutative ring:
the genuine raw denominator formula; its exact Phi reduction;
the exported integer inverse congruence Dred*A=1+Phi*Q+r*H;
unit status when Phi=0 and characteristic-r; and the actual CM-block
quadratic Bezout recovery identity for both signs s²=1.
The theorem statements themselves contain the polynomial identities,
not an unproved array-checker predicate. Fifteen declarations expected.

Pinned Lean4.34.1/mathlibd13f23b,-j1-M4096/1200s compile and audit. Reuse
the accepted CMRecovery42 object only after compile/audit exit0 and exact
source comparison. Record dependency object/source hashes; no duplicate
base producer. Compile/audit must exit0 with zero failing/trusted.
Preserve any rejected attempt or cap. A failed polynomial proof cannot be
reported as accepted because the native certificate was correct.

These prove actual coefficient identities and their conditional algebraic
interpretation. CM embeddings, target primality, whole-Jacobian projection,
proper curve/model identification and geometric transfer are not imported.
No larger-n point transfer or full integral Hom-lattice result follows.
