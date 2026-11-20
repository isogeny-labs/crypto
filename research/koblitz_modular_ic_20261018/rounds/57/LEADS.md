# A pole obstruction that selects the next complete space

This is a written local derivation, separate from the executed R57 matrix
certificates. It concerns a curve map to an ordinary Koblitz elliptic curve;
it does not exclude modular-Jacobian correspondences.

For E_a: y²+xy=x³+a x²+1 over characteristic2, put T=(0,1),s=y+1.
The derivative with respect to x is1 atT, so s is a uniformizer. The exact
local equation is

    x*(1+s+a*x+x²)=s².

The parenthesized factor is a unit atT. Hence ord_T(x)=2,y is a unit,
and ord_T(y/x)=-2. Under every nonconstant morphism from the proper
normal source curve, every point overT has pullback pole order2e≥2.
A nonconstant proper map toE is surjective, so this fiber cannot be empty.
Every function in the complete L(P1+P2+P3+P4) has pole multiplicity≤1.
Therefore this entire24-section simple-pole space cannot contain a pullback
of y/x under such a map. The R56 pullback_pole Lean implication supplies
the valuation multiplication step, assuming the standard ramification
compatibility. The written local-unit and surjectivity steps are not new
Lean theorems here.

The next compatible prescribed bound is 2P, degree160. On the retained
proper genus61 source, 160>2g-2=120, so RR gives h0(2P)=100 exactly.
Use the complete ambient 2D0 (degree256,h0=196), impose all96 removed
coefficients, then divide byA². The removed coefficients are42 at the
e7 branch and6 at the e1 branch, each overF4:2*(42+6)=96.

The retained R50 packet contains196 selected quadratic products. Before
reusing them as a COMPLETE 2D0 basis, certify their independence with257
regular-cusp jets and their full ambient bounds. Count196 alone is not
independence. If they fail, complete the product span from all2346 pairs;
keep the failed candidate set and recover a full196-basis before restriction.
This avoids relying on an unverified normal-generation assertion.

For the100-section kernel, require the same complete C K/R K/K R+H C
products asR57, actual source bindings, both coefficient-descent orientations,
and the induced sigma action. This is a concrete larger-size construction,
not a coefficient-box search. Smaller-support residue restrictions remain
useful for correspondences and for locating the four explicit fixed classes.

Source references: R27 proper genus/complete ambient, R50 product packet,
R56 complete divisor and pullback_pole, R57 complete24-space, and
[Stacks RR](https://stacks.math.columbia.edu/tag/0B5B). A literature search
also located standard normal-generation references; the proposed acceptance
uses direct257-jet independence instead of assuming their applicability.
