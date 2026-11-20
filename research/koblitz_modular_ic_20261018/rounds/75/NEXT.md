# From finite uniformizer classes to exact pole divisors

Lift each saved finite-fiber uniformizer coordinate column to the
normal order over `F2[z]`, using canonical polynomial representatives
for coefficients in the `F32` fiber. Check its projections and
nilpotency index again after reduction. For each branch, construct
local arithmetic modulo increasing powers of the base prime and use
the uniformizer to compute exact valuations and leading residues of
`b,c,lambda,nu` and every registered R51/R52/R53 denominator.

The wild `z=0` branches may require precision beyond the base-prime
fiber to separate leading coefficients. Record required precision and
verify each valuation with two consecutive powers. At unramified
branches, the base prime is already the uniformizer; compute residue
values directly from the primitive projections.

Build the normal infinity chart and exact overlap with R70's affine
order, then compute the same valuations there. Assemble the complete
proper pole divisors and a prescribed Riemann--Roch basis. A level-77
map candidate must pass function-field identities, descent conjugates
and nonzero action on the order-11 subgroup. Separately test the
repeated index-seven branch pattern for an actual intermediate field.
