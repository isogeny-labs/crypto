# Pure zero-fiber algebraic obstruction: exact-certificate attempt

Frozen before executing the new certificate search. Starting point is the
already integer-verified fully symmetric j=0 S4 polynomial H4 in
`examples/quartic_twist_s4_integer.rs`. Write H4 as
`C0 + C1*B + C2*B^2 + C3*B^3` in the four elementary coordinates.
When B has degree four over Fp and all abscissae lie in Fp, H4=0 forces
all four coefficients to vanish. The actual curve/group implication still
requires the stated nonsingular-curve and summation-polynomial bridges.

Attempt to express the quartic discriminant D as a rational polynomial
combination of C0,C1,C2,C3. Use their homogeneous weights 12,9,6,3 with
weights 1,2,3,4 for the elementary coordinates. Solve the finite rational
coefficient system exactly, not numerically. If a representation exists,
clear denominators, independently multiply out the complete identity in Z,
and require a constant-coefficient +1 mutation to fail. Record the exact
combination, denominator, every coefficient and input/source hash.

If no representation is found, this search establishes no obstruction:
ideal membership can be stronger than radical membership, and other
proofs may exist. Arithmetic overflow, deadline or any failed exact check
is an incomplete attempt, never a proof of nonmembership. If the identity
holds only after multiplying by an integer, its nonvanishing in the actual
field is an explicit hypothesis. No useful-relation or speedup claim
follows from the search alone.

Compile separately, then use the repository busy reservation, one thread,
8 GiB and at most 60 s for the exact search. Native Rust only. Preserve the
report, explanatory diagram, PDF, failed attempts and the missing Lean
bridges. Do not rerun any completed signed-set oracle for this search.
