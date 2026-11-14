# R56 checked base geometry and derived pullback divisor

The actual source equation has base V²+z²V+z+z³=0. In h=1/z,
W=h²V it becomes W²+W+h+h³=0. Its two residue roots W=0,1 are simple
(derivative with respect to W is1),so the base normalization has exactly two
rational smooth points Q0,Q1 above infinity,each with uniformizer h.
Completeness/Hensel lifting gives unique roots W0,W0+1 in F2[[h]].

Infinity56-02checks full source chart identities,exact telescoping before
truncation,and both jets moduloh128. Decisive leading numerator orders give:

| Actual function | ord at Q0 | ord at Q1 |
|---|---:|---:|
| R | -3 | 2 |
| b | -7 | 1 |
| c | -4 | -1 |
| A=1+c | -4 | -1 |
| Delta | -37 | -1 |

The leading coefficients here are1. The finite jets suffice because the exact
formal roots agree to order128,and every decisive numerator coefficient is
strictly below128. No finite truncatedzero is treated as infinite valuation.
R,b,c,A are regular on the retained finite base chart;only Q0,Q1 are omitted.
Note b has a zero at Q1,a genuine boundary denominator issue despite its pole
at Q0. This matters for modular target charts and reverse divisor evaluation.

Let X be the smooth projective normalization of the retained source function
field and pi:X→E its degree16base morphism. The retained normal affine algebra
B is the chart over finite z. Source separability is checked by the nonzero
linear coefficient/derivative. The R55zeroquotient proves all finite Azeros
are four reduced degree20points P1,...,P4. Since both base infinity points
are poles of A,every point over infinity has negative Avaluation. Thus,using
the standard valuation pullback law,one obtains the derived complete divisor
in pullback notation:

    div_X(A)=P1+P2+P3+P4 - 4*pi^*(Q0) - pi^*(Q1).

Both zero and pole degree are80;deg pi^*(Qi)=16. This uses the retained
normal-source/function-field bridge and standard normalization/pullback facts;
it is not a claim to an explicit proper embedding or an independently checked
full infinity integral basis. Check56-01now splits those pullbacks into the fourdegree2closedpoints
R07,R01,R17,R11,with ramification7,1,7,1. See CM.md for the completed
localfactor argument and full divisor coefficients.

The first native Newton data gives another checked pole bound. For T=lambda,
s=h4T has residual roots a,a² atQ0,while atQ1all16roots have valuation3.
Hence every extension valuation gives ord(lambda)=-4e overQ0 and -e overQ1.
In particular lambda has the same pole divisor as A. The ratio lambda/A is
regular and nonzero at infinity,with residue in F4. Unit56-01now checks fullBezout products showing lambda andb are
units on the entiredegree80zeroquotient. Thus lambda/A has exactlysimple
poles at allfourfinitezeros and no infinitypole.

This suggests an actual prescribed RR space: L((A)_infinity),and equivalently
A^-1 L((A)_infinity)=L((A)_zero). Complete source valuations at the infinity
branches can turn this into finite linear constraints on the normal RR basis,
replacing earlier coefficient-box spaces. Full cover branch factors and rationaluniformizers are now checked. Source
basis leading coefficients and exactambientdivisor matching remain necessary
to implement that complete space.

Sources:retained R52/R53actual source and R55quotient;Stacks04GM Hensel,
Stacks0C1B valuation/ramification setup;GarrettNewton polygons for separable
root counts and full denominator criteria. Native products and jets are in
runs/infinity56-02;interpretation premises are explicit above.
