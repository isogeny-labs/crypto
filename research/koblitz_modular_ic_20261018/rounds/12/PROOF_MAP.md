# Round 12 proof map and written geometric bridges

## Exact formal scope

KoblitzRound12.lean has sixteen named theorems. The generated
KoblitzRound12Boundary.lean has eight further named theorems checking the
actual four boundary circuits and their violated saturated relations.
Full audit counts are recorded in ledger.json. There are no custom
axioms, sorry terms, or native decision trusts. Standard Lean foundations
are reported by the transitive audit. A separate lean4checker --fresh run
was unavailable: lean4checker is not installed on this runtime's PATH.

| Declarations | Exact scope |
|---|---|
| constant_multiplication_saturation, constant_inverse_saturation, saturated_constant_product | Universal ideal-membership identities for actual constant gates |
| strict_saturation_from_counterpoint | Nonmembership under an evaluation vanishing on an ideal, plus a displayed h-multiple in the ideal |
| homogeneous_constant_false_point | Scalar characteristic-2 boundary witness |
| graph0_all_boundary_gates through graph3_all_boundary_gates | Kernel evaluation of every emitted restricted circuit gate and residual |
| graph0_missing_linear_relation through graph3_missing_linear_relation | The four actual emitted points violate the selected saturated linear relation |
| velu_discriminant_residual, two_seven_quotient_coefficients | Exact coefficient algebra before/after characteristic-2 specialization |
| seven_parameter_factorization, seven_parameter_ratio, eta_y_recovery_unique | Hauptmodul factorization, ratio and coordinate recovery with explicit unit hypotheses |
| uniformizer_order_unit_factor | Power-series order is unchanged by a factor of order zero |
| coprime_primary_image_zero | An element killed by 8 and 11 vanishes in an additive group |
| F4_six_supersingular_target_points | All six displayed points satisfy the minimal elliptic equation when w²+w+1=0 in characteristic 2 |
| antidiagonal_SS_degree, SS_closed_point_count, eight_divisor_degree_partition | Exact finite degree bookkeeping |

The multiplication-saturation and Velu-discriminant residual identities
have source-bound IDC certificates with 64 issuer and 128 additional
checks each. These certify the universal identities, not the entire
modular construction. Native finite-field implementation, circuit parser
semantics, supersingular moduli identification and deformation geometry
remain written bridges. Partial formalization is explicit.

## What the finite boundary theorems mean

Each circuit value is an actual point in the scalar F2 subspace of K,
where K=F2[z]/(z^360+z^5+z^3+z^2+1). At h=0, all constant-node values
are zero, Add and coefficient Frobenius give linear scalar equations,
and every multiplication or inverse gives a zero-product condition.
Coefficient Frobenius fixes the scalar subfield. Native bit Gaussian
elimination and bounded branching find all four displayed models.

The generated Lean data contains every gate, point bit and residual
index from these actual programs. Its kernel checker evaluates exactly
these restricted equations. It does not impose Boolean equations on the
original scheme. To interpret its Boolean values as a point of the full
coefficient ring, send each value to coefficient 0 and set coefficients
1 through 359 to zero. The multiplication tensor sends 1*1 to 1 and
coefficient Frobenius fixes 1. Thus every checked scalar equation gives
all 360 coefficient equations. Native checks explicitly visit all
1,679,760 scalar equations. This tensor-to-scalar interpretation remains
written; it is not silently imported as a custom Lean axiom.

For the selected gate out, constant-node a and constant c=1, the universal
membership identity places h*(out-input) in the original ideal. The
boundary point has out-input=1. Evaluation therefore proves the linear
relation is not in that ideal. Each of the four actual homogenized ideals
is strictly smaller than its h-saturation. This concerns the auxiliary
presentation. It is not an obstruction to a proper quotient model.

Constant-only gate evaluation adds further linear relations. Original
constant nodes have h-membership depth 0. Addition or coefficient
Frobenius preserves the maximum existing depth. Multiplication by a
constant of depth k yields its linear scaling relation at depth k+1.
Two constant factors give depth at most 1+max(k,l). A constant inverse
uses the displayed inverse identity. Native propagation records these
depths at every node and all scale relations; the maximum is 5.
Expanding constants through the pinned basis tensor specifies every
scalar polynomial losslessly. The resulting intermediate ideal contains
the original one, has the same h-localization, and lies in its saturation.
No complete saturation, independence or elimination is inferred.

## Actual 49-level targets from local 7-isogenies

The maximal Hurwitz order and its actual unit maps are the retained
round-10/11 construction. Over F4096, all 7-torsion points are explicit.
Choose P,I(P) so J acts by the reduction modulo 7 of
J=[[2,17],[17,-2]] modulo 49. The 192 norm-7 quaternions give all eight
7-kernels. Bind every chosen map to Velu by the doubled-map identity on
3,962 nonkernel points. For twice-quaternion coordinates of norm 7,
the sum of absolute coefficients is at most 10. The difference degree is
at most (2*sqrt(7)+10)^2, strictly below 256. The finite identities
therefore imply a global map identity. Cancellation of [2] is in the
endomorphism domain. The point controls alone are not the proof.

The actual 49-Tate module realizes the split maximal order modulo 49.
Its reduction is calibrated by the actual 7-basis. For a retained cyclic
49-line S, alpha7 kills its order-7 subgroup; alpha7(S)/7 modulo 49 is
the resulting order-7 subgroup of the quotient. Applying the inverse
post-Velu unit identifies it with actual 7-torsion coordinates there.
This yields all 8*7=56 nonbacktracking chains. Their composite kernels
are cyclic of order 49; the excluded backtrack has noncyclic kernel.
The compatibility of the abstract split module with a lift of the
actual 7-basis is a Tate-module/Skolem-Noether argument, not explicit
coordinates of points of order 49.

Odd-order torsion is finite etale. Its points and subgroups lift uniquely
through nilpotent thickenings. The relevant framework is
[Stacks, finite-etale invariance](https://stacks.math.columbia.edu/tag/0BQB).
Thus the selected special-fiber roots lift to the intended kernel, not
merely arbitrary roots of a division polynomial. Native Newton iteration
checks all 216 root series through order 127. The general Velu formula
specializes to the A',B' in REPORT.md; see
[Miret–Moreno–Rio, section 1](https://ddd.uab.cat/pub/pubmat/pubmat_a2007vEXTRA/pubmat_a2007vextrap147.pdf).

The actual degree-7 Hauptmodul has j0=A(t)/t^7 and j7=A(t)/t, with
A(t)=t^8+t^4+1. These are the frozen modular coordinates from
[Kilford–McMurdy](https://arxiv.org/pdf/1110.6801). At the supersingular
fiber, t has constant w or w². The squared ratio and cube-root equations
recover the two possible series. One and only one satisfies the modular
equation through order 63. Their two-edge product is a fourth power,
giving the retained eta x coordinate. Recovering minimal y by leading
term division distinguishes the branches where x²+x+1 vanishes.
All six resulting targets satisfy the actual elliptic equation, and
their Hurwitz orbits and Frobenius agree with the retained labels.

## Local multiplicities and completeness of the SS strata

The round-11 Weierstrass slice E_s=y²+sxy+y=x³ is retained. The source
full 11-level removes all noncentral automorphisms, while the central
minus sign extends and acts trivially on the deformation base. Thus s
is a uniformizer on the source moduli curve. This is the same written
moduli bridge as round 11.

A lifted 11-quotient has A=s+s²*c and B=1+s*c+s³+s⁴*c. Normalize it back
to E_z using its recorded special-fiber unit. For
x=u²*X+r, y=u³*Y+u²*v*X+t, the a2 equation gives r=s*v+v², the a3
equation gives u³=1+s*r, and the remaining a4,a6 equations have unit
Jacobian in v,t. The a1 equation gives z=s/u. This determines the
parameter change to all orders. Exported coefficients are substituted
and checked through order 15. All kernel sums and target series lie
in F4; no unstated compositum-field adapter is used.

Compose the 49-level x-series with z(s). At each target x is nonzero,
so the derivative of the minimal elliptic equation with respect to y
is a unit. Its y-series is uniquely recovered by Hensel recursion.
Consequently the eight displayed function jets are the actual local
functions on the source. Every zero has a nonzero coefficient before
order 16. Their first nonzero orders are therefore exact, not lower
bounds. The 400 old diagonal contacts are an independent construction
control and agree, including the twenty contacts of order 14.

All supersingular elliptic curves in this characteristic are represented
by E0 over the algebraic closure. Cyclic49 lines and full11 bases modulo
its actual automorphisms give precisely the retained 3,080 source labels.
The native F2 permutation uses actual Frobenius, determinant-normalized
11-level, and the same source-unit canonicalization. All 308 orbits have
degree 10, and every valuation is orbit invariant. This proves complete
supersingular closed support under the written moduli bridge. It does
not exclude ordinary zeros. Subtraction from the independently retained
noncuspidal degrees gives the seven positive ordinary residual degrees.

Elliptic negation on the genus-one base is not the modular inertia action.
It can exchange target points with different modular stabilizers. This
is why the first one-branch stabilizer bound was invalid. The final
contacts use the actual coordinate jets; the preliminary export is
retained and explicitly superseded.

All supersingular target values are in the computed eight-element
E49(F4), and are killed by 8. Integral correspondences, Frobenius and
norms preserve this torsion property. Their projection to a group killed
by 11 vanishes by the formal coprime-primary lemma. This uses the retained
projection maps; it is not a new runtime image evaluation or rank gain.
