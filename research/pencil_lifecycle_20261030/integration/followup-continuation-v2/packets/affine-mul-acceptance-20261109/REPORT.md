# Affine multiplication caller: prospective acceptance construction

The source candidate connects the existing addition relation to Jacobian::mul, lines 1062-1073 of the pinned native source. It initializes the accumulator to identity, adds only on the low-bit guard, shifts the UInt64 scalar right once, and squares only when the shifted scalar remains nonzero.

Reach records loop boundaries; Run records the addition event order. The proposed invariant derives WellFormed for both registers from the initial canonical input and the existing addition-preservation result. The accepted-return theorem derives validator acceptance rather than assuming accepted outputs.

Termination is addressed in two constructions. UInt64 shift-right is related to natural division by two for a strict-decrease induction. A second Steps witness counts actual modeled loop iterations; fuel induction uses k.toNat < 2^fuel and gives an execution within 64 iterations. No positive-fuel premise is required for scalar zero.

Premises: prime p, 2 < p < 2^31, canonical quintic coefficient words, and a WellFormed initial divisor. The fixed controls use prime 17 and the existing X^5 representation fixture. This representation-level fixture is not asserted to be smooth.

Worked controls state scalar 0 returns identity; scalar 1 returns the input through left-identity dispatch; scalar 2 has square then accumulate; scalar 3 has accumulate, square, accumulate. The latter two use existing total-add witnesses and derive acceptance. A separate example retains validator acceptance of noncanonical inactive storage as a counterexample to using acceptance alone as the input invariant.

Status: all new theorem scripts are uncompiled. Independent source review, elaboration and separate named axiom readback remain pending. The candidate contains no scalar-to-divisor-class equality claim. Runtime/import closure and bounded publication protocol are separate future work.
