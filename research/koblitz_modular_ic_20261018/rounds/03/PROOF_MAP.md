# Round 03 proof map and readable arguments

Lean 4.34.1 and mathlib `d13f23b723b8a846827a245b89c10fc7d3f11612`
remain pinned. The new source is [KoblitzRound 03.lean](../../lean/KoblitzRound 03.lean).
Thirteen new named lemmas compile. The transitive audit counts 14 declarations,
including the equation generated for `intersection`. They use only `propext`,
`Classical.choice`, `Quot.sound`, or no axioms. There is no admitted proof,
custom research axiom or native evaluation bypass in the final source.
The earlier 13 named lemmas remain in their original module. Failed builds
and their temporary sorryAx diagnostics are retained as failed receipts.
`lean4checker` remains unavailable; the actual Lean kernel and axiom auditor run.

| Claim | Lean declaration | Exact hypotheses and external bridge |
|---|---|---|
| Determinant compatibility gives a normalized square | `determinant_normalized_square` | Field; chi nonzero, epsilon*eta=chi squared, epsilon+p*eta=0; identification of Dirichlet/Hecke characters is external |
| The necessary exponent can be halved | `determinant_half_exponent` | Commutative ring; u^(2m)=1 and u squared=-p imply (-p)^m=1 |
| CM norm formula | `cm_norm_identity` | Commutative ring; w squared+w+2=0; conjugation and actual elliptic endomorphism ring are external |
| Alternating pairing transforms by det(M) | `alternating_determinant` | Universal commutative-ring identity in all eight variables |
| Anti-isometry graph is isotropic | `anti_isometry_graph_isotropic` | Pairing coefficient times det(M)=-1; proves cancellation of the two explicit product forms for all vectors |
| Graph has zero left-factor intersection | `graph_transverse_left` | Additive homomorphism psi; explicit membership witness (a, 0)=(psi(b),b) |
| Graph has zero right-factor intersection | `graph_transverse_right` | Same graph, psi injective; explicit membership witness |
| Quotient map preserves the left factor | `graph_quotient_left_injective` | Additive quotient map, explicit implication identifying its left-factor kernel with graph membership; no Jacobian assumption |
| Cubic-cover substitution | `binary_cubic_cover_substitution` | Commutative ring of characteristic 2, arbitrary u,v,a,d; rational-map extension and geometric degree are external |
| Five-point conic pullback intersections | `conic_pullback_intersections` | Exact integral intersection arithmetic for the specified degree/multiplicity pairs; blowup geometry external |
| Adjunction gives genus 6 | `decic_adjunction_genus` | Explicit adjunction intersection equality; establishing it for the actual smooth curve is external |
| Positive-sign genus 4 count contradiction | `genus 4_positive_sign_count_obstruction` | Finite sets with an injection, cardinalities4 and 2; scheme-points/base-change bridge external |
| Negative-sign genus 4 count contradiction | `genus 4_negative_sign_count_obstruction` | Count77 and supplied published upper bound71; census bound not formalized |

## The two del Pezzo twists

Let Y be the blowup of P2 in five distinct points on the smooth rational conic
Q: xz+y squared=0. Their Frobenius types are (5) and(3, 2). No three are
collinear. The strict transform Q has self-intersection4-5=-1 and contracts
to a smooth rational point on a degree-five del Pezzo S. This is Zaitsev's
five-point conic construction, section4 of
[Forms of del Pezzo surfaces of degree 5 and 6](https://arxiv.org/abs/2302.04937).

Writing H for a plane line and E_i for exceptional divisors,
`pi^*(-K_S)=5H-2*sum(E_i)` and
`pi^*(-2K_S)=10H-4*sum(E_i)`.
The latter is orthogonal to Q, has square20, and intersection10 with -K_S.
Adjunction gives 2g-2=20-10=10, hence arithmetic genus 6. The native kernels
have dimensions 6 and 16, respectively. For a smooth section this is its
geometric genus; singular sections are not promoted to carriers.

The zero set on Y is counted using order-four Hasse jets at each exceptional
fiber. A section restricted to Q is either nowhere zero or identically zero,
since the restricted line bundle has degree 0. The native checker verifies
this independently: at every rational point of the strict transform Q the
16 evaluation coordinates are a common nonzero multiple of those at
(1:0:0), including exceptional tangent directions. If Q is contained, replace
its q+1 zero points by the one contraction point, subtracting q. Otherwise
there is no correction. The conic has q+1 rational points; the surface count
is q squared + q*(fixed points of the five-point permutation)+1.
These are written geometric arguments, with exact finite checks, not a full
formal blowup/contraction implementation in Lean.

## Integral polarizations and their finite graphs

For O=Z[w], w squared+w+2=0, conjugation sends w to -1-w. The norm of
a+bw is a squared-a*b+2b squared. The ordinary base elliptic curve has
fundamental Frobenius discriminant-7, so its geometric endomorphism ring is
O. Over the algebraic closure the norm-kernel A5 is E to the fourth power,
using augmentation basis e_i-e_4 in E to the fifth power.

Restrict the circulant with first row (0,u,v,bar(v),bar(u)). It is invariant
under cyclic permutation. The initial integral u,v sublattice has determinant
five examples; it is not all integral invariant forms. The additional integral
congruence classes have numerators (a,b,c,d) congruent to r*(1, 2, 2, 4) modulo 5.
The restricted matrix entries are divided by 5 and checked integral. This is
still a bounded parameter enumeration on one augmentation model, not every
integral representative of its isogeny class.

Exact positive leading principal minors certify positive definiteness of the
complex Hermitian form. Writing its endomorphism matrix in the basis(1,w)
and multiplying by the product elliptic alternating form gives integral J.
For both signs F=pi*S satisfies F-transpose*J*F=2J; here pi=w or -w and S is
the augmentation action. The checker uses independent integer Bareiss
elimination to verify det(J)=det(H) squared. At prime det(H)=ell the kernel
has dimension2 moduloell. The induced pairing on kernel representatives x,y
is x-transpose*J*y/ell moduloell, independent of integral lifts because
J*x and J*y vanish moduloell. It is nondegenerate in every retained case.

An elliptic intertwiner M is an anti-isometry exactly when
`pairing*det(M)=-1 moduloell`. This is the universal alternating identity
formalized above. Graph columns(M,I) have rank 2, and the graph is isotropic
in the four-dimensional polarization kernel, hence maximal isotropic. Its
factor intersections are zero. The separate checker exhausts all two-by-two
matrices for compatible characteristic polynomials, avoiding the generator's
Hom-nullspace solver.

Under the standard identification of positive integral Hermitian forms with
polarizations on E to the fourth power, Frobenius invariance supplies an F2
polarization on A5. Glue it to an elliptic curve with ell times its principal
polarization along the stable graph. Since ell is odd, the graph is etale in
characteristic 2. The product polarization has degree ell to the fourth power;
the quotient has degree ell squared, so its descended polarization is
principal. The graph criterion supplies a principally polarized quotient,
not a curve equation. See the general discussion in
[Robert, Reducible gluing of abelian varieties](https://www.math.u-bordeaux.fr/~damienrobert/pro/publications/notes/2022-09-kani.pdf).
The elliptic Hermitian identification, polarization descent, algebraic
quotient and theta/Jacobian criterion are not formalized here. The finite
matrices do not bypass these geometric bridges.

The short-vector search checks a bounded box only. It finds minimum 3 within
that box for all four determinant 19 gluing cases and no norm-one witness.
It does not prove global minimum 3 or geometric indecomposability. An exact
inverse-Hermitian coordinate bound is the next discriminating extension.

## n=5 map, rows, and descent

The source is E_a/F32: Y squared+X*Y=X cubed+a*X squared+1. Its
order-eleven subgroup is unique, since the full orders are 44 and 22.
The tuple (P,F(P),...,F to the fourth(P)) is fixed under Frobenius combined
with inverse cyclic shift. Its sum is zero on eleven-torsion because the
norm lands in E_a(F2), of order 4 or2. Thus the tuple evaluates an actual
point of the augmentation model of A5(F2). All11 points and 121 additions
per sign are checked. No quotient coordinates on the modular Jacobian or
on the glued abelian variety have been evaluated.

The same-field control is
`C_a: v^2+u^2*v=u^7+a*u^4+u`, with
`phi(u,v)=(u^3,u*v+1)`. The defining identity is proved in Lean. The
Artin--Schreier form has two poles of order 3, giving genus 3; smoothness at
zero and infinity is checked by the existing native constructor. The cubic
extension is separable and degree 3: X on E has zero and pole orders2, hence
is not a cube in its function field. The map extends to the two ramified
fibers at infinity and at(0, 0). Pullback/norm composition is[3], and
4*3-11=1 gives injectivity on eleven-torsion. The existing Lean push-pull
lemma proves that implication once the geometric composition is supplied.
All11 pullback classes per sign are also checked directly for nonzero order
and scalar compatibility. General F32 Jacobian-class pushforward is still
unsupported by the reused library; no such norm evaluator is claimed.

The factor base is every affine rational point minus infinity. A function
y+v(u), degree(v)<=3, has pole 7 at infinity; a quartic v gives pole 8. Its
norm is v squared+u squared*v+f. Completely split norms supply point-divisor
rows, with exact multiplicities. Vertical functions u-c supply conjugate
pairs or twice the ramification point. Every admitted row sums to the
identity under the independently maintained library's Mumford arithmetic.
The generator and separate verifier use different rank algorithms.

For the final a=1 extension, enumerate all effective degree-five divisors on
the rational factor base, excluding a pair of conjugates at a nonbranch
abscissa and repeated support at the branch point. Hermite interpolation
uniquely determines a polynomial v of degree<=4. Every completely split
norm of degree 7 or8 has an allowed degree-five subdivisor, so this procedure
covers the missing quartic functions without enumerating all 32 to the fifth
power coefficient tuples. The constructor explicitly checks inclusion of
every earlier admitted y+v relation. The rank rises from 16 to 20 on21 columns.
This completeness argument is written, not formalized in Lean.

Projected factor-base logs are verified by multiplying every point class by
Jac(C)(F32)/11, which is prime to 11. Their correctness is independent of the
linear-algebra solution. For individual descent, v=V+c*U gives a function
whose divisor contains the shifted Mumford divisor. A fully split residual
supplies its logarithm. There are 32 trials per offset, with no offset and
then at most eight fixed rational-point offsets. Witnesses are checked by
norm division and class addition. Only after solving is the recovered scalar
compared by elliptic multiplication. The finite group and the base field
are stated explicitly; this is no F2 carrier or modular-Jacobian evaluator.

## Conditional small-genus carrier exclusion

A curve whose Jacobian contains the full A5 factor must have genus at least4
by dimension. At genus 4 its Frobenius polynomial is exactly P_A5. For the
positive sign, the counts N1=4,N2=2 violate rational-point injection. For the
negative sign N5=77 exceeds Xarles's published maximum 71 among F2-defined
genus-four curves, [Example 16](https://arxiv.org/abs/2007.07822).
The two conditional Lean lemmas expose the injection and maximum hypotheses.

The pinned [Dragutinovic genus-five census](https://github.com/DusanDragutinovic/MT_Curves)
has no matching polynomial among 7829 vectors. The pinned
[Huang--Kedlaya--Lau genus-six census](https://github.com/junbolau/genus-6)
has no match among 38327 isogeny classes representing 72227 curves. Every
encoded genus-six polynomial is label-roundtripped and Newton-roundtripped.
Combining these with the genus-four tests excludes full-A5 F2 Jacobian
carriers through genus 6, conditional on the published bounds and census
completeness/correctness. The census classifications are not independently
reproduced or proved in Lean. This does not exclude other finite-subgroup
representations, other base fields, higher genera, or direct modular relations.

## Integer identity certificates

The native assembly issues and checks these `identity.certificate/v1`
records at 64 deterministic sample points plus 128 extra points each:

| Certificate | ID | Relation to Lean |
|---|---|---|
| [CM norm integer lift](identity-cm-norm.json) | `IDC1hea9f559dcc2b81fc` | Universal integer polynomial with the residual multiple of w squared+w+2 retained; the Lean CM norm lemma assumes that residual is zero |
| [Alternating determinant](identity-alternating-determinant.json) | `IDC1h2c8208a50a83bbaf` | Direct universal statement, with the pinned Lean source hash |
| [Cubic-cover integer lift](identity-cubic-cover.json) | `IDC1h1fe2e21ad051cd8a` | Retains the residual 2*(u*v*d+d squared); the characteristic-two Lean lemma removes it |

Odd-prime certificate samples alone do not prove a characteristic-two
specialization. The exact kernel proofs and the displayed residuals provide
that bridge. Certificates with a different statement have no misleading
Lean proof attachment.
