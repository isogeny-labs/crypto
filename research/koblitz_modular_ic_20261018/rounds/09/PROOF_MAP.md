# Round09 proof map

## Kernel-checked statements

KoblitzRound09.lean imports the pinned round08 project. Seventeen named
new theorems are audited transitively. Only the standard propext,
Classical.choice and Quot.sound axioms occur; no sorryAx, custom axiom or
native trust. The proof statements have the following exact scopes.

| Theorems | Statement scope |
|---|---|
| dual_pairing_identity | Universal commutative-ring pairing identity |
| dual_matrix_left, dual_matrix_right, dual_matrix_order | Explicit integer matrices, inverse products and fifth power |
| root_matrix_inverse | All t in ZMod19, both products equal1 |
| rotation_reflection_commute, stable_twist | Finite ZMod5 permutation and twist conditions |
| weil_five_plus, weil_five_minus | Universal degree10 factorizations for both signs |
| pow_nine_nineteen, root_lift_plus, root_lift_minus | Universal power identity and conditional root equations |
| rooted_deck_phase | Nonzero rooted fixture forces a constant phase to1 |
| chart_transport_cancel | Nonzero-denominator chart correction cancellation |
| reflection_genus_bound | Integer implication from the specified RH inequality |
| trace_orbit_invariant | Additive action fixing a sum by permuting its summands |
| trace_dual_recovery | Finite double-sum recovery under an explicit idempotent hypothesis |

Two IDC certificates bind dual_pairing_identity and weil_five_plus to the
exact Lean source hash, with64 issuer and128 extra checks each. They
certify their universal polynomial identities, not the entire geometry.

## Mixed Miller identity — written argument

Fix a nonzero19-torsion point T of one retained elliptic factor and choose
the normalized Miller function f_T with divisor19(T)-19(O). Let g_A,B be
the normalized line quotient with divisor(A)+(B)-(A+B)-(O), including the
vertical/tangent cases by rational continuation. Put

    h_T(X,Y)=g_(T-Y,Y)(X)/(x(X)+x(Y)).

The divisor in X is(T-Y)+(O)-(T)-(-Y), since the vertical denominator has
divisor(Y)+(-Y)-2(O). The ratio f_T(X+Y)/(f_T(X)*f_T(Y)) has19 times this
divisor. Their quotient is constant in X. The standard local parameter
at O and the monic normalization of f_T and the line quotient make that
constant1. This proves the generic rational identity

    f_T(X+Y)=f_T(Y)*h_T(X,Y)^19*f_T(X).

It holds as a rational identity across charts. The native code stores
the group/line circuit for h, checks both orientations in exact one-variable
function fields, and checks80 full lift slices. Slices alone do not prove
the global identity. The divisor/normalization argument is the bridge;
it is not formalized in Lean.

Integer combinations of sigma(T) have explicit univariate corrections
prod f_T^weights=f_sum*g^19. All48 class/sum identities are checked by
cross-multiplying reduced rational functions on the elliptic curve.
The actual graph equality D*sigma(T)=T*M, inverse matrix N and beta=B give

    H_i(P)=sum_correction_i(P3)
      *product_(j=1..3) h_Tij(P_(j-1),-P3)
      /product_(j=0..4) class_correction_ij(Pj).

Consequently H_i(P)^19 is exactly the ratio between the destination
radicand at beta(P) and the signed N-combination of coefficient-Frobenius
radicands at P. This proves the displayed root lift generically. All
coefficients of the univariate factors, T points and rational operations
are retained; sigma and beta are explicit, so H is an executable circuit.

## Global cocycle and chart transport — written argument

The retained independent19-torsion Picard classes give the connected,
integral Kummer extension of degree19^2, as in round08's written
identification with the actual quotient. Since N is invertible modulo19,
the lift is a generic isomorphism over the base Frobenius. Its360-fold
iterate fixes the base: B^5=1 and sigma^360=1. It is therefore an
automorphism of that Kummer function field over its base. Because mu19 is
contained in K, each z_i can only acquire a constant mu19 multiplier.
A fixture with both roots nonzero, and with every iteration defined,
fixes both multipliers to1. Two such fixtures per graph pass. Lean proves
the nonzero-root cancellation implication, not this whole geometric bridge.

Let rho_i,k be the product of the five retained chart transition factors,
with rho_i,0=1. In source chart k and target chart l, the lifted formula is

    z'_i,l=product_j sigma(z_j,k)^N_ji * H_i(sigma(P))
      *rho_i,l(B*sigma(P))/product_j sigma(rho_j,k(P))^N_ji.

This is substitution and cancellation of the chart definitions. All200
old factors pass coefficient-Frobenius checks at both fixtures. The
parsed checker verifies every36 chart pair, both roots and both fixtures,
576 squares total, and recomposes2,880 semilinear steps using shared field
primitives. Full global identities use the written rational argument.

## Invariant rational generators — written argument

The exact field basis b_k and its trace dual d_k satisfy
sum_k d_k*sigma^j(b_k)=delta_(j,0). Native arithmetic verifies all360
identities directly, in addition to129,600 trace-duality entries and64
trace controls. Let L be the retained geometrically integral quotient
function field over K and rho its semilinear function action of order360.
The fixed trace sum I_(f,k)=sum_j rho^j(b_k*f) is invariant by cyclic
reindexing. Expanding the double sum gives

    sum_k d_k*I_(f,k)
      =sum_j (sum_k d_k*sigma^j(b_k))*rho^j(f)=f.

Lean proves the reindexing and this double-sum implication for arbitrary
finite index types; the native field/basis interpretation remains separate.
Taking f to be all twelve chart coordinates shows that adjoining K to
the field E generated overF2 by these invariants gives L. To identify E
with L^rho, use the linear independence of the K basis after scalar
extension from the invariant field: finite Galois descent gives
K tensor_F2 L^rho ≅ L as vector spaces. Expressing an invariant in
L=K E uniquely in that basis forces all coefficients except the constant
one to vanish. Hence L^rho=E. This is a written application of Galois
field/vector-space descent, not a generated polynomial presentation.

The point controls compute5,760 invariant values for four coordinate
families,16 recoveries and128 identities I(alpha(P))=I(P)^2. They are not
a claim that arbitrary K-point invariant values belong toF2. The12×360
family is a rational generating family, potentially redundant. Its
polynomial relation ideal, saturation, gluing of invariant affine opens
and closed projective embedding are not computed.

## Low-genus and relation limits

L7 checks all50 signed cases in one known eight-dimensional Tate-module
representation. Integer traces/Newton identities give exact characteristic
polynomials; the stable reflection-fixed part is E_t^2. The finite Lean
lemmas support the twist and commutator calculations. No claim covers
other Jacobian factors. The RH lemma assumes the inequality and does not
compute ramification, the quotient genus or equations.

M12 evaluates eight coupled functions on the720 known norm columns only.
Each coordinate-field calculation checks all deck-cycle components and
zero consistency. No outside zero, multiplicity or pole calculation has
been supplied. There is no new complete principal relation or rank.
The earlier odd reverse divisor and independent forward norms remain
frozen; general reduction and individual descent are still open.

## Sources and trust boundary

The dual-isogeny and Picard framework is explained in
[Milne, Abelian Varieties](https://www.jmilne.org/math/xnotes/AVs.pdf).
The Kummer framework is in [Stacks Kummer theory](https://stacks.math.columbia.edu/tag/03PK).
For the field/vector-space descent used above, see
[Stacks Galois descent](https://stacks.math.columbia.edu/tag/0CDQ);
for separable trace duality, see
[Stacks trace and norm](https://stacks.math.columbia.edu/tag/0BIE).
These references support the frameworks; the explicit circuits and
application to the four retained graphs are this study's written/native
construction. The whole construction is not a Lean theorem. The native
field semantics, connected-cover identification, global divisor geometry
and modular-factor geometry remain explicit trust boundaries.
