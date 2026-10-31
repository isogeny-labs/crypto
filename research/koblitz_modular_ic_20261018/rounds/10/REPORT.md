# Round10 — F2 equations, denominator saturation and coupled divisors

2026-10-19 UTC. Parentf75e3ddaf54148ffe874a5f927020aae107bc92e.
All four determinant19 quotients now have explicit quadratic F2
presentations and closed affine embeddings of specified descended dense
opens. The presentations include inverse witnesses,so denominator
saturation is built into those affine models. A closed projective
embedding of the whole proper quotient remains open.

The divisor of x0*xinf is complete:22 closed cusp points,with zero and
pole degrees2640 each. All eight pole divisors and cuspidal zeros are
computed. Seven noncuspidal zero divisors remain unresolved. The diagonal
component has400 identified supersingular moduli labels but460 total
multiplicity still unaccounted for. No relation rank or IC cost is promoted.

![Polynomial construction and divisor strata](routes.svg)

## Four actual polynomial presentations over F2

The construction expands each K=F2^360 coordinate in the fixed power basis.
All coefficients are0 or1. Every extension-coordinate operation has360
scalar polynomial equations. Its addition and coefficient-Frobenius
blocks are linear; multiplication and inverse blocks are quadratic.
The five elliptic equations,two Kummer equations and twelve descent
coordinates are imposed through the retained rational Frobenius circuits.

| Graph | Operation blocks | F2 variables | F2 equations | Maximum degree |
|---|---|---|---|---|
| 0 | 1171 | 421560 | 424080 | 2 |
| 1 | 1171 | 421560 | 424080 | 2 |
| 2 | 1148 | 413280 | 415800 | 2 |
| 3 | 1148 | 413280 | 415800 | 2 |

The total is1,679,760 equations. This is an explicit auxiliary-coordinate
presentation,not a smaller relation ideal in the previous trace generators.
The JSON stores every indexed operation and residual,with a shared table
of719 reduced basis powers. This lossless block format specifies every
scalar polynomial. Seventy-two representative scalar equations are also
expanded to literal sparse monomials,and4608 local gate controls pass.
There is no unspecified correction function or symbolic arithmetic oracle.

Coefficient Frobenius acts linearly on the360 coefficient variables.
Squaring those indeterminates would change the scheme:over dual numbers,
coefficient Frobenius fixes the nilpotent parameter while absolute
squaring kills it. No Boolean equations are imposed. Native controls check
all129600 basis products,360 basis-Frobenius identities and64 dual-number
identities. For each graph,two K-valued fixtures recover all twelve input
coefficient blocks through the trace-dual inverse. All360 split components
are checked. The generator accounts for3359520 equation-component checks
and144000 unit-component checks;the parser repeats the checks and recovers
34560 input coefficient values across the eight fixtures.

## What saturation and embedding have been established

For every inverse gate,the polynomial equations state d*u=1 in
K tensor the base algebra. Taking determinants of multiplication matrices
shows Norm(d)*Norm(u)=1. Consequently the graph ideal is saturated by
the product of these denominator norms. The inverse witnesses are unique,
so the polynomial graph represents the rational construction on its
specified open exactly. The Lean proofs establish inverse cancellation,
ideal cancellation,power saturation and the determinant implication.

After base extension back to K,the restriction of scalars splits into360
conjugate components. The equalizer equations identify adjacent components
by the actual rational lift. The retained360-fold cocycle closes the
cycle. Projection to one component and its inverse recover the intersection
of all360 translated domains. This gives the stated closed affine model
of a descended dense open;the scheme-descent bridge is written in
PROOF_MAP.md and uses [Stacks affine descent](https://stacks.math.columbia.edu/tag/0244).
It is not a full Lean formalization or a consequence of samples alone.

An eliminated saturated generating ideal in just the4320 original
coefficient variables has not been computed. Keeping the inverse and
operation variables provides the effective affine presentation. Nor does
this construction recover every boundary of the proper quotient,the full
six-chart projective gluing,or its polarization on a closed projective
model. Jacobian identification and individual descent remain separate gaps.

The projective control makes this distinction concrete. Homogenizing
xy=z,xu=1 gives xy-zh=xu-h²=0. The projective point
[x:z:y:u:h]=[0:1:0:1:0] satisfies these equations but is excluded by
zu-yh=0. Its membership certificate is

    h*(zu-yh)=y*(xu-h²)-u*(xy-zh).

Adding zu-yh gives the complete saturated prototype ideal. All three
Buchberger S-pairs reduce to zero;the leading monomials xy,xu,zu omit h,
which proves h-regularity and saturation. This repairs the prototype.
The homogenizing-variable saturation of the full quotient ideal remains
uncomputed. The false point refutes the general inference from naive
homogenization to a correct projective closure.

## Complete cusp strata for all eight functions

The modular carrier has eight base cusps and60 full-eleven-level cusp
labels above each,480 geometric cusps total. A fixed line is canonical at
five of the60 labels. Its quotient map has ramification121 there and1
at the remaining55,so each map has degree660. The six middle base cusps
have explicit F8 coordinates;both normalized Tate-level transport actions
and all local curve jets are checked. Distinct eleventh-root leading
coefficients verify cancellation orders when both maps have a pole.
The moduli/Tate interpretation is a written bridge,not an inference from
the degree counts. The base eta-coordinate divisor is independently
supported by [Kilford–McMurdy,section2](https://arxiv.org/pdf/1110.6801).

| Function | Pole degree | Cuspidal zero degree | Residual noncuspidal zero degree |
|---|---|---|---|
| x0+xinf | 2520 | 420 | 2100 |
| x0+xinf+1 | 2520 | 0 | 2520 |
| y0+yinf | 3780 | 360 | 3420 |
| y0+yinf+1 | 3780 | 0 | 3780 |
| x0*xinf | 2640 | 2640 | 0 |
| x0*xinf+1 | 2640 | 0 | 2640 |
| x0*yinf+xinf*y0 | 3900 | 450 | 3450 |
| x0*yinf+xinf*y0+1 | 3900 | 0 | 3900 |

The cross-coordinate function has30 extra zeros where the two middle
cusp images are distinct points with y=x. Exact elliptic jets give
multiplicity1 there. The residual column includes possible supersingular
points;it is not labeled ordinary without resolving that stratum.
All bihomogeneous ambient zero sections and fixed components are retained.

For x0*xinf,the complete formula is

    div(x0*xinf)=2 phi0^*(T)+2 phiinf^*(T)
      -2 phi0^*(O)-2 phiinf^*(O), T=(0,1).

Only cusps occur. At each of the two rational base cusps,the sixty labels
have multiplicity244 on the ten labels where one tested line is canonical,
and4 on the other fifty. The normalized Frobenius sends(b,d) to(b,d/2)
modulo simultaneous sign. This partitions each sixty-label fiber into
five degree1 points,one degree5 point and five degree10 points. Thus the
whole divisor has22 closed points,with signed multiplicities retained.
Every projected even/odd image is zero. This supplies a complete coupled
divisor but no new nonzero-image relation rank. The previous rank11 remains.

## Equal-map support: a precise unresolved multiplicity

For x0+xinf,the complete Cartier identity separates two components:

    div(x0+xinf)=(phi0-phiinf)^*(O)+(phi0+phiinf)^*(O)
      -2 phi0^*(O)-2 phiinf^*(O).

The two map degrees are1280 and1360,using the retained Gram entry20.
The diagonal component has cuspidal degree420,leaving860. Its ordinary
source-j support is screened by the exact collision resultant

    Res_Y(Phi11(X,Y),(1+X^4)Y^4+X^3Y^3+X^4)
      =X^88*(X+1)^8.

Exact native rational-polynomial arithmetic computes this degree96
resultant;2046 finite-field gcd controls agree. Lean checks the derivative
and factorization identities. The only ordinary candidate is j=1.
The two horizontal norm11 endomorphisms have no equal transport on any
of the56 cyclic49 lines;both charts of that finite exclusion are
kernel checked in Lean. This is a transported-level test beyond j alone.

For j=0,the actual24 automorphisms of y²+y=x³ are bound to Hurwitz
units using225 curve points and a degree≤36 identity argument. Actual
Frobenius is identified as -J+IJ. The chosen torsion-module representations
then enumerate288 norm11 elements,12 kernels,56 cyclic49 lines and660
normalized full-eleven bases. Quotienting by the12 units modulo sign
leaves3080 modular classes. Exactly400 match the two transported49
subgroups,in40 Frobenius orbits of degree10. These are explicit moduli
labels;coordinates of all49/11 torsion points have not been generated.
The Hurwitz framework is described in
[Elkies's elliptic-curve notes](https://abel.math.harvard.edu/~elkies/M223.24/index.html).

The available target-stabilizer lower bounds sum to400,not860.
The degree-exhaustion test therefore fails,leaving460 multiplicity
unaccounted for. No exact supersingular diagonal divisor is claimed.
The antidiagonal component and the other noncuspidal supports remain open.
The next discriminating calculation is the local deformation/intersection
multiplicity at these40 closed labels,with a verified local model or a
faithfully applicable CM deformation theorem. Labels and total degrees
cannot replace that calculation.

## Proofs,evidence and remaining work

Twenty-three new named Lean proofs cover inverse ideals and saturation,
coefficient algebra,homogenization,finite49 transport,collision identities
and divisor arithmetic. Full field semantics,Galois descent,Tate cusp
geometry and Hurwitz-module identification remain written bridges.
The proof map states each scope. Two source-bound universal identities
receive64 issuer and128 extra checks each. All thirteen new JSON records
replay exactly,and round09 also replays unchanged. Root/native/Lean/site
checks and the archive receipts are in VALIDATION.md.

Failures are retained:Rust API/type errors,a special-case gcd call with
untrimmed coefficients,the first Lean cancellation rewrite,and an
imprecise coefficient-counter label corrected without changing arithmetic.
The false projective point and the diagonal degree deficit remain negative
results. Earlier evidence is unchanged. Same-agent parser/replay checks
share declared primitives;there is no external audit.

Remaining acceptance obligations are a verified global projective model
with its saturated boundary ideal and polarization,and complete
noncuspidal zeros with multiplicities and pole accounting for the seven
remaining functions. No new complete evaluated relation matrix,solver run
or timing is supplied. The user assesses IC efficiency.
