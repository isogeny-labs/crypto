# C-zero interpolation and coprime residual

Nine checked field/polynomial declarations establish the source-derived interpolation identities, a common interpolant, explicit Bezout coefficients, coprime support polynomials and divisibility of the residual by their product. Six fixed controls and fifteen separate axiom readbacks passed.

For support polynomials u1,u2 and ordinate polynomials v1,v2, the literal numerators A,B,C satisfy A*dq-B*ds=C*(dq*q2-dr) and A*dr-B*dt=C*dq*r2. Under B nonzero and C zero, l=v1-(A/B)u1=v2-(A/B)u2. The explicit Bezout numerator is B. Therefore input divisibility u_i divides f-v_i squared implies u1*u2 divides f-l squared.

Source basis: retained projective_law_gate.rs lines950-1023, its PROOF_AUDIT and prior degree22 source review; polynomial identities use pinned Mathlib through ZeroTest. These are normalized common-scale field identities. The native weighted payload transport, degree-one output curve membership, divisor class sum, word implementation and compiled execution are not proved here. No novelty or performance claim is made.

Controls cover nonzero support difference, equal q with nonzero B, shared-root B=0, nonzero C, identical-support degeneracy and an explicit common interpolant. Development failures are retained: algebra1, probe1 and controls1/2. Algebra2/3 are passing intermediate versions; algebra4 is the final proof. Final input bindings were unchanged. Readbacks use only propext, Classical.choice and Quot.sound; compiled dependencies retain their trust boundary.

Independent review is pending. No scientific execution occurred. The full 1,260-target obligation and expired campaign deadline remain unchanged; measured cost ratios are null.
