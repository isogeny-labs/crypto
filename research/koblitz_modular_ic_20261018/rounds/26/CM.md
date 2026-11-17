# R26: diamond norms, Prym differences and proper cusp transport

2026-10-28 UTC. Exact native certificates, kernel algebra and written geometric
bridges are distinguished throughout. Source curve C is the retained degree16
incidence normalization over E:v^2+v=u^3+u. It is the genus61 modular curve
under the R15/R16 written moduli/good-reduction bridge. No new factor inclusion,
subgroup evaluator, polarized embedding or IC cost/rank result is asserted.

## The invariant norm kills the intended factor

R15 already states that the primitive order10 character acts by minus one
at its unique involution. R16 identifies exactly the R25 action
sigma(lambda,nu)=(lambda+1+c,nu+b) with that diamond involution. Thus this
round does not discover a new action: it connects the retained character
identification to R25's actual quotient, avoiding a false transfer inference.

Delta34 satisfies d11=1,d7=-1 and delta^2=1 modulo77. The character
chi=epsilon_-7*eta11 has value-1 there. Native CHAR26-02 enumerates all60
units, with6 in each of the10 character fibers, and verifies primitive
exponents1,3,7,9 all have sigma value-1 modulo11. The source Tate negation
and line action have exact polynomial residuals0. The twelve nontrivial
subgroup norms (four characters times orders2,5,10) are0; the four trivial
subgroup controls are1. This is kernel-checked finite arithmetic too.

The geometric conclusion uses the standard diamond/nebentypus eigenspace
identification: every conjugate in the intended CM newform factor has minus
sigma, and the identity specializes to its good-reduction11-adic module.
The norm of any nontrivial diamond quotient consequently annihilates the
intended factor under that retained bridge. In particular the degree2
invariant quotient C->H does not supply its transfer. This is stronger than
merely observing Frobenius-polynomial collisions, but it is not a newly
computed matrix for the whole integral Jacobian lattice.

## The correct group direction is Prym

On divisor classes use (1-sigma) rather than the invariant norm. For an
actual11-torsion class v on the minus factor,

    (1+sigma)v=0, (1-sigma)v=2v, 6*(1-sigma)v=v.

These are conditional universal group identities proved in Lean. A nonzero
actual v therefore remains nonzero under the difference. The projector
6*(1-sigma) modulo11 has no11-denominator. This does not construct the
unknown A5->J(C) factor inclusion: it specifies where to construct/evaluate
one and avoids the excluded invariant quotient norm.

Pair collector columns as [P]-[sigma(P)] and principal relations as
f/sigma(f); that ratio has norm1. Fold Frobenius/deck dependencies and test
actual factor images before reporting rank. A sigma-stable closed point can
produce a zero difference. No new relation columns are evaluated here.

Characteristic2 matters: minus eigenfunctions are not a separate function
space, since -1=1 in the function field. The divisor/Picard differences above
use integral coefficients or11-torsion, where2 is invertible. On the b-unit
open theta=nu/b instead satisfies sigma(theta)=theta+1 and

    theta^2+theta=V/b^2.

The general quadratic function norm is
N(p+q*nu)=p^2+b*p*q+V*q^2. These field identities are proved in Lean; neither
they nor constructed-forward classes are arbitrary-divisor reverse evaluation.

## Exact diamond2 transport on original coordinates

The retained R18 map T re-normalizes at2P11. With A=1+c,K=b+A*c,
r=c^3/K,s=K/c,

    b'=bc/r^3, c'=1+A/r,
    lambda'=(lambda+s)/r,
    nu'=(nu+lambda*b+bc)/r^3.

TRANSPORT26-01 checks the universal Tate equation and line change exactly
before imposing the level11 condition. In the exact base field F2(u)[v]/
(v^2+v+u^3+u), it checks b',c' equal the original rational base functions at

    U=(u+v)/u^2, V=(uv+u+v)/u^3.

Every intermediate base iterate remains on E; the fifth is exactly(u,v).
The O local chart gives U=t(1+t)q,V=tq+(t^3+t^4)q^2, with U/t and V/t
both residue1. This pins O->A0 and a uniformizer of order1.

SOURCEORBIT26-01 additionally iterates all original lambda/nu affine
coefficients five times over this same exact field. Its final coefficients
are lambda coefficient1/offsetA,nu coefficient1/lambda coefficient0/offsetb,
with all five residuals0. Thus T^5=sigma on the actual function field, not
only on finite fixtures. Since sigma^2=1,T^10=1 and T^-1=T^9. The source
preservation argument is the universal Tate isomorphism and the R16 exact
QR-triple moduli interpretation; no uncomputed ideal saturation is assumed.

## Proper chart book at all rational base cusps

The base action is translation by A0 and cycles
O->A0->C0->D0->B0->O. The explicit formulas give C0->D0,D0->B0; order5,
O->A0 and bijectivity fix the remaining images. Push each verified R24 O
chart through T^i. Its inverse is the R24 chart coordinate map after
T^(10-i). These are F2 rational function-field charts. The normal-curve
extension/isomorphism bridge identifies their proper local rings, preserving
residue field and relative ramification. Each of the five rational cusps
therefore has four rational source points of indices1,1,7,7. They exhaust
its degree16 fiber. Equations in each transported coordinate chart are
literally the R24 chart equations; original functions are given recursively
by the explicit T formulas and their checked inverse.

This succeeds where the unshifted CD26 proposal fails: its t^14 constant
term assumption is false. The failed native coefficients/residues stay in
the archive. No original-coordinate lambda/nu/S/N valuation is inferred from
transport alone. A projective equation model/proper polarized embedding
remains distinct from this chart book on the normalization.

## Complete lifted divisors of the base-derived entries

Let D_P=pi^*(P) at each rational base cusp, explicitly the sum of its two
unramified points plus7 times each of its two tame points; deg D_P=16.
Let E_Z=pi^*(Z5)=Z_1+7Z_7, where each closed source point has degree10 by
R23, so deg E_Z=80. Pulling back the complete R22 base divisors now gives:

| Original function | D_A | D_B | D_C | D_D | D_O | Additional term |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| b=nn1 | -7 | 1 | 1 | 2 | 3 | none |
| delta=nn12 | -16 | -4 | 8 | 5 | 2 | +E_Z |
| c=nn2 (two registered slots) | -4 | -1 | 1 | 2 | 2 | none |
| b(b+c)=nn20 | -14 | 0 | 3 | 6 | 5 | none |
| K=b+c+c^2=nn37 | -8 | -2 | 3 | 5 | 2 | none |

The fourth row uses the exact R22 inverse u=(b+c)/c^2, so nn20=b*u*c^2.
All supports in this table are complete, under the explicit base/source
normalization bridges. Their pole degrees are respectively112,320,80,224,
160. This covers the first six registered slots, five distinct functions.
The remaining S=nn65,N=nn75 and original lambda,nu divisors still require
source-specific cancellations and additional zero supports. The table does
not make their divisors complete.

## A specified complete RR target, not a fitted ansatz

Take D=7D_A+D_B, the union of the known pole divisors of b and c. Its degree
is128. Under the retained genus61 bridge,128>2*61-2=120 and Riemann-Roch
gives dim L(D)=128+1-61=68. This is a written geometric dimension deduction,
not a constructed basis or a Lean scheme theorem. The next experiment must
construct all68 basis elements and independently check completeness before
searching for maps/correspondences; a handful of guessed coordinate monomials
cannot discharge it. Lambda/nu/S/N divisor computation and anti-symmetric
correspondence evaluation remain concrete parallel construction obligations.

## Literature and proof scope

Best et al., section4.1, formulas4.1.3-4.1.5,
https://jvoight.github.io/articles/cmfs-mf-101222-scrubbed.pdf states the diamond
character decomposition. Molin-Page theorem37,
https://antsmath.org/ANTSXV/papers/ANTS-XV_molin-page.pdf fixes the CM character
used in R15. Stein's diamond-symbol chapter is
https://wstein.org/books/modform/modform/modular_symbols.html.
Riemann-Roch is used in its smooth proper curve form as in Sutherland18.782,
lecture22, https://www.ocw.mit.edu/courses/18-782-introduction-to-arithmetic-geometry-fall-2013/5859c5f7d2ad1eae5339d87dce144823_MIT18_782F13_lec22.pdf.
These sources and R15/16/18/22/23/24/25 inform the written bridges. Native
certificates are specific source calculations. No RR/Picard/modular-geometry
statement is inserted as a new Lean axiom.
