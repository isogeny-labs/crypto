# Round11 proof map and geometric bridges

## Formal and native layers

KoblitzRound11.lean supplies22 named results. The full pinned gate audits
168 project theorems. Only propext,Classical.choice and Quot.sound are
admitted;there is no custom axiom,sorry or native decision trust.

| Lean results | Scope |
|---|---|
| saturation_other_inverse,saturation_cubic,saturation_cubic_redundant | Universal homogeneous membership identities |
| projective_chart_transition,affine_two_inverse_recovery | Chart ratio and inverse substitutions |
| hamilton_norm_product | Universal quaternion norm multiplication |
| cube_correction_residual,normalization_tangent_determinant | Cube Hensel derivative and characteristic2 normalization tangent matrix |
| weierstrass_transformation | Full universal integer coordinate-change residual |
| velu_w_residual,velu_special_coefficients | Exact specialized quotient coefficient algebra |
| local_contact_degree,recovered_multiplicity,closed_contact_degree,diagonal_antidiagonal_residual | Integer degree bookkeeping;not standalone moduli theorems |
| translated_product_identity,translated_divisor_cancellation | Algebra and formal divisor cancellation |
| equal_y_factorization,AS_recovery_residual,AS_sum,reciprocal_pole_identity,AS_genus_arithmetic | Ambient correspondence identities and conditional genus arithmetic |

The other-inverse saturation and equal-y factorization identities have
source-hash-bound IDC certificates with64 issuer and128 extra checks.
Native finite-field semantics,division-polynomial recursion,Buchberger
implementation,formal smoothness and the identification with modular
local intersection lengths are not asserted as Lean theorems.

## Why the local series calculate the diagonal

Write E0:y²+y=x³ over algebraically closed characteristic2. A general
Weierstrass deformation has five coefficients. Coordinate changes fixing
O have parameters u,r,v,t. At E0,the tangent map on the four normalized
coefficients a2,a3,a4,a6,in the order r,u,v,t,is the identity. Equivalently,
in order u,r,v,t it is the permutation matrix recorded in Lean. Hensel
normalization gives the slice E_s:y²+sxy+y=x³. Thus s is a parameter
on the deformation space with marked special fiber. The discriminant
1+s³ is a unit. This formal-slice argument,including effectiveness of
Weierstrass deformations,is written,not a full Lean scheme construction.

Prime-to2 torsion is finite etale. Its points and cyclic subgroups lift
uniquely over a strictly Henselian nilpotent thickening. Hence every11
kernel and cyclic49 subgroup in the retained label has a unique lift.
An isomorphism whose reduction carries one49 subgroup to the other also
carries their unique lifts. The target stabilizer is precisely {+1,-1}
at all400 labels. The central minus sign extends over every deformation
and acts trivially on its base. There is therefore no additional quotient
ramification between s and the local parameter of the coarse X0(49)
curve at these targets. The full11 source level similarly removes all
noncentral automorphisms. This is the level-rigidity bridge needed to
interpret the explicit isomorphism obstruction as the local order of
phi0-phiinf at O.

The actual11 basis P,I(P) is found over F1024 and checked against the
retained J matrix. Its symplectic determinant can differ from the original
choice by a scalar. Rescaling the first basis vector corrects it while
preserving lines0 and infinity and commuting with normalized Frobenius.
Thus it identifies the determinant components for these two maps,as in
round10. No conclusion for the other ten maps is inferred without their
pairing adapter. The actual49 Tate module realizes the maximal Hurwitz
order tensor Z/49 as M2(Z/49);any such split representation is conjugate.
Consequently the retained56 abstract lines label the actual cyclic49
subgroups. This existence argument remains distinct from explicit49
point coordinates,which have not been computed.

The special-fiber normalized Velu targets are y²+y=x³+1. The shift
(x,y)->(x,y+omega),omega²+omega=1,identifies them with E0. The recorded
post-Velu unit binds each norm11 quaternion to the actual quotient map.
For all1078 nonkernel rational points,the doubled map equals the
quaternion-coordinate sum of actual unit maps. The latter sum has at
most12 terms. The degree of their difference is bounded by
(2*sqrt(11)+12)²<400. More than400 distinct zeros force equality of
endomorphisms globally. Cancellation of [2] is in the endomorphism
domain,not on a possibly nonreduced2-torsion scheme. Unit bindings have
an analogous degree36 bound and1089 point checks. Actual quaternion
endomorphisms and maximal-order identification use the round10 bridge
and [Elkies's characteristic2 account](https://abel.math.harvard.edu/~elkies/M223.24/index.html).

For each lifted kernel,c(s) is the sum of five lifted x-coordinates.
The general characteristic-independent Velu formulas specialize to the
recorded a4,a6. See [Miret–Moreno–Rio,section1](https://ddd.uab.cat/pub/pubmat/pubmat_a2007vEXTRA/pubmat_a2007vextrap147.pdf).
The odd11-division polynomial has60 simple roots at s=0;coefficient
Hensel recursion lifts each uniquely. Both roots and substitutions are
exported and rechecked through order31.

A target isomorphism is x=u²*X+r,y=u³*Y+u²*v*X+t. Its a2 equation
sets r=s*v+v²;its a3 equation sets u³=1+s*r. After fixing the special
fiber unit,the a4 and a6 equations have identity Jacobian in v,t,so
there is a unique solution through every finite order. The remaining
a1 equation is s*(u+1)=0. The universal transformation identity is Lean
checked;the parser verifies all other residual coefficients vanish.
The first nonzero coefficient of this final obstruction is the exact
contact order. The only observed orders are1,2,6,14,strictly below32.
The local equation gives a length in k[[s]],not a fitted trace-based
formula. Summing the400 positive lengths gives860. Along with the
round10 ordinary exclusion and cuspidal degree420,this exhausts degree1280
and establishes completeness of the diagonal under these bridges.

A tempting imported deformation formula was not used: [Li,Theorem1.1](https://arxiv.org/pdf/1902.10789)
explicitly assumes an odd residue field. The direct characteristic2
series avoid that unresolved applicability issue.

## Saturation and projective chart proof

Let I=(xy-zh,xu-h²,yv-h²) in F2[x,z,y,u,v,h]. The final ideal J also
contains zu-yh and zv-xh. The two h-membership identities are Lean
checked. Native Buchberger checks all10 S-pairs in grevlex with h
smallest. Every leading monomial omits h. Therefore normal forms show
multiplication by h is injective modulo J. Since I subset J and every
new generator enters I after multiplying by h,J=I:h^infinity.
Localization at h yields the Laurent torus graph,so the injective
localization shows J is prime. Its homogeneous quotient has dimension3
and Proj dimension2. The six emitted affine chart ideals and ordinary
ratio transitions construct the closed projective subscheme in P5.
[Stacks,closed relative Proj](https://stacks.math.columbia.edu/tag/084M)
gives the scheme framework. All monomials,S-pairs and h-membership
reductions are in the raw certificate. The complete four large quotient
ideals are not substituted for this specified circuit.

## Cartier identities and genus3 correspondence

On E:y²+xy=x³+x²+1,the origin is O and T=(0,1) has order2.
The group law gives x(P+T)=1/x(P). Substitute this into the standard
x-difference Cartier identity on E times E and add div(x(P)). The
vertical terms at P=T cancel,giving the translated T-fiber formula in
REPORT.md. Finite controls supplement this global divisor identity.
The fibers remain unevaluated.

On y=v,the difference of the two curve equations factors into
(u-x)*(y-u²-u*x-x²-u-x),as proved universally in Lean. The two components
are the diagonal and the displayed residual. Their pullback degrees
sum3960,so the residual has degree2680 before common polar cancellation.
A finite support has not been inferred from this equation alone.

For x+u=1,the substitution in REPORT.md gives three independent
Artin-Schreier quadratic characters. Their reduced right sides have
simple poles at {0,infinity},{1,infinity},{0,1}. A rational AS coboundary
has even pole order at its poles,so no nonzero character is trivial.
The cover is connected of degree4. At each of the three branch points,
inertia has order2,there are two points above it,and each different
exponent is2. Riemann-Hurwitz gives2g-2=-8+3*4=4,hence g=3.
The AS recovery algebra is Lean checked;normalization and ramification
arguments remain written. This ambient genus3 curve is not an evaluated
cover or a new principal relation on C.
