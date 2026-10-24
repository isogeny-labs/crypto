# Complementary quadratic controlled-pole spaces

CM25 keeps the actual18 incidence constraints of CM23:ten F32 points
and eight independent F1024 holdouts. It searches x=N/q²,y=M/q³ with
N in all2^15 F2 combinations of the quadratic monomials,and M in their
F32 span. Eight denominators are registered:1,b,c,1+c,b+(1+c)c,lambda,
nu,1+lambda. The75-bit y solver retains every affine solution space;
all actual spaces are enumerated and none hits the nullity cap.

| q | x-valid a=0 | x-valid a=1 | finite fits a=0,a=1 |
|---|---:|---:|---|
|1|46|12|3,1|
|b|39|6|1,0|
|c|36|11|1,0|
|1+c|36|3|1,0|
|b+(1+c)c|42|8|0,0|
|lambda|39|9|1,0|
|nu|46|9|1,0|
|1+lambda|43|12|1,0|

All524288 numerator/sign cases complete. The ten fits are actual constant
maps:independent sparse multivariate arithmetic verifies N=x0*q² and
M=y0*q³ as free polynomial identities over F32. This is stronger than
observing equal values at the finite points. Lean independently checks
their elliptic constant residues by the earlier binary convolution/
polynomial-reduction implementation. It does not prove search exhaustion.

This is a complementary bounded family:it allows quadratic N but only
F2 numerator coefficients,while CM23 used a larger F32 affine N space.
Neither is a complete prescribed cuspidal Riemann–Roch space. If D is the
sum of the coordinate pole divisors,the new bounds are2D+2(q)_0 and
2D+3(q)_0. Actual proper supports and multiplicities are uncomputed.
No nonconstant integral factor map,divisor evaluator or subgroup image
is supplied. The original proper-normalization and function-field map
obligations remain open;no nonexistence outside this ansatz is inferred.
