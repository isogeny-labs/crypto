# Round 13 proof map

The exact formal inventory is 63 new named theorems: 20 in
KoblitzRound13, 24 in KoblitzRound13Boundary, eight in
KoblitzRound13Data, three in KoblitzRound13Square and eight in
KoblitzRound13SquareData. Only passing final builds and the transitive
axiom audit count. Failed elaborations are retained as failures. No
custom axiom, sorry or native decision trust is admitted. The ledger
records the full audit count and three source-bound IDC identities.

| Lean results | Exact scope |
|---|---|
| quadratic_norm_residual/zero, conjugate_root_factorization, same_x_two_branches, recover_linear_y | Universal quadratic norm and branch algebra with explicit hypotheses |
| norm_tower, determinant_multiplicative | Actual finite-free algebra norm transitivity and 11 by 11 determinant multiplicativity |
| inverse_cancellation_saturation, shared_factor_saturation, associative_saturation/residual, two_product_saturation, reciprocal_chain_saturation | Polynomial residual identities; not ideal elimination |
| power_series_constant_uniqueness | Equality when the difference is already known to be constant and its constant coefficient vanishes; global regularity is a written hypothesis bridge |
| sum_one_unique, simple_norm_fiber | A simple norm zero has one simple residue-degree-one point above it when the divisor norm formula applies |
| five_deck_augmentation_zero, actual_deck_table, uniform_deck_norm, actual_deck_augmentation_zero, projected_deck_zero | Exact finite deck table, its integral augmented norm, and preservation by any additive map |
| graph0_stage0 through graph3_stage2, passes/missing | 24 actual restricted-circuit certificates: constants; all inverse repairs; all shared-factor repairs |
| graph0_passes_all_Q14 through graph3_missing_square_relation | Eight actual models satisfying every Q14 repair and violating an actual Q15 coefficient relation |
| weighted_square_sum, linear_square_saturation, membership_square_depth | Characteristic-two square identities and conservative h-membership depth |
| ordinary_base_polynomial_*_Rabin | Three kernel evaluations of explicit binary bit-checker conditions; not a formal irreducibility theorem |
| five_fiber_degrees, pushed_partitions, source_degree_certificate | Exact finite degree arithmetic |

## Written norm and Riemann–Roch bridge

Use the retained minimal model E49: y²+xy=x³+x²+1 over F2 and the
retained eta conversion. Its coordinate functions have poles 2 and 3
at infinity. Cyclic-121 Hecke correspondences on X0(49) have 132 branches:
one q^121, ten zeta11^b*q with b nonzero modulo 11, and 121 roots of q.
The branch with middle b=0 is noncyclic and is excluded. The branch
product is a global rational function. Good reduction at 2 applies to
these odd-level correspondences. There are no poles away infinity for
these two products; their pole orders are 504 and 528. This modular
identification and good-reduction argument is written, not a Lean theorem.

The native tower computes the last 121-fold norm as two determinants
of size 11. Truncation precision includes every possible exponent entering
the first 640 output coefficients: 121*640 inputs. All pivots are units;
otherwise the program rejects. Euler-product, multiplicativity and direct
F1024 root-of-unity controls check the tower separately. Lean proves norm
transitivity but does not formalize the series implementation.

In L(m*infinity), 1, x^i and x^j*y have distinct nonconstant pole orders
2i and 2j+3, exhausting the elliptic Riemann–Roch dimension. Principal
parts therefore recover A+B*y; a globally regular difference is constant
on this proper geometrically connected curve, and its constant q
coefficient is checked zero. This proves equality under the global norm
bridge. Agreement of 640 coefficients alone is not the argument. The
135/111 extra positive coefficients are further rejection controls.

Quadratic conjugation is y -> y+x in characteristic two. Thus the norm
to F2(x) is A²+x*A*B+(x³+x²+1)*B². Exact factor products, Rabin conditions,
and the elliptic equation resolve every branch, including common A/B
factors and x=0 ramification. O18 reparses and reconstructs these divisors.

## From pushed points to source closed points

Two distinct selected 11-kernels give a cyclic-121 isogeny between the
targets after forgetting the scalar basis. The intermediate correspondence
has the above Hecke norm. Its local norm divisor formula has coefficients
sum(f_i*v_i). All six ordinary pushed zeros are simple. The displayed
compatible ordinary points, checked by actual kernel transport, therefore
exhaust the fiber of the cyclic-121 correspondence: there cannot be an
additional extension-field point with positive contribution.

For each of six points, O15 enumerates all compatible K-rational middle
j-triples; O16 checks actual division-polynomial membership, doubling
invariance and four exact 7/11 commuting squares. On ordinary elliptic
curves in characteristic two, geometric normal forms y²+xy=x³+b have
j=1/b. This does not assume every K-rational curve admits that normal
form over K. An ordinary quadratic twist adds a2*x² and becomes this
form by y -> y+s*x over at most a quadratic extension. Its x-coordinate
doubling, odd division polynomials and normalized isogeny x-maps are
unchanged. The induced Frobenius correction is simultaneous negation
on the basis, which the full-level coarse model quotients out. Thus
the x-kernel fiber calculation is independent of this twist choice.
The kernel polynomials determine actual cyclic subgroups because doubling
generates F_l^*/{+1,-1} for l=7,11. The two 7-kernels give a cyclic49
chain: in all six constructions their endpoint j-invariants are distinct,
excluding a dual second 7-isogeny. Both producer and parsed verifier
reject if this check fails. The transported chains identify both level49
targets.

The remaining scalar deck group is F11^*/{+1,-1}, of order five. A chosen
first generator P11 up to sign is specified by its x-coordinate. In the
second independent kernel, the fixed nonzero Weil pairing specifies Q11
uniquely once P11 is chosen, modulo their simultaneous sign. Consequently
its fiber algebra is K[t]/kernel_P11(t). The retained F2 descent uses the
cyclotomic determinant correction on Q; the first x-coordinate still
transforms by ordinary Frobenius. This identifies the exported algebra
with the F2 source fiber. The moduli, pairing and descent identification
remains written; no global Lean scheme model or pointwise 11-torsion y
coordinates are supplied.

All six degree-five polynomials are irreducible over their degree50/29
base fields and separable. The simple Hecke zero pulls back simply under
this etale cover. Hence two closed points have degree250 and four have
degree145, each multiplicity one. Their total degrees are exactly500 and
580, matching the independent round12 ordinary degrees. O18 verifies the
exported towers without rerunning candidate searches; arithmetic primitives
are shared, so this is not an external implementation audit.

## Why both ordinary projected images are zero

Write lambda_i=2^i modulo11 for i=0,...,4. Changing the scalar basis by
(P,Q)->(uP,u^-1Q) sends lambda to lambda/u². The actual five permutation
rows for u=1,...,5 are

    0 1 2 3 4
    3 4 0 1 2
    4 0 1 2 3
    1 2 3 4 0
    2 3 4 0 1

For the even tuple v_i=phi_lambda_i+phi_-lambda_i this is its permutation.
For the odd tuple v_i=(-1)^i*(phi_lambda_i-phi_-lambda_i), the sign
from replacing lambda by -lambda cancels the parity change. The same
unsigned table acts. The generated Lean table checks all 25 cases.
These are the retained signed tuples and augmentation in
[round06.rs, tuple](../../native/src/round06.rs), followed by its
additive transfer/Frobenius projections; the calculation does not
substitute a different tuple map.

For each fixed column i, summing 5*v_i-sum(v_j) over all deck elements
is zero integrally. Each new source closed point is its entire scalar
fiber over a pushed point. Summing over the pushed Frobenius orbit still
gives zero. The retained transfer and integral Frobenius projectors are
additive, so both images vanish. No coprimality with a sampled group order
is needed. This is a structural image evaluation, not newly computed
pointwise projection coordinates. O17 explicitly records that distinction.

## Actual saturation evidence and limits

The full coefficient ring has K=F2[z]/(z^360+z^5+z^3+z²+1). The scalar
boundary restrictions send model bits into coefficient0 and all other
coefficients to zero. Coefficient Frobenius fixes these bits. Native
checking visits all original scalar equations and all imposed repairs.
As in round12, the tensor-to-scalar interpretation remains a written
bridge; the Lean certificates check the exact restricted DAGs and models.

Q13 adds inverse-cancellation and shared-factor quadrics, each with a
displayed h-multiple in the original ideal. The three-leaf associative
linear family is empty on these programs: no query ran for that family.
Q14 groups products with the same four input leaves and proves membership
at h-power2. Q15 propagates exact K-linear forms using additions and
constant multiples, then squares them in characteristic two. Its
h-membership depth is at most5. The relations and actual coefficients are
exported. At a scalar boundary point, each K-linear relation is expanded
into all360 bit equations, not reduced to its constant bit alone.

Every graph has a counterpoint after each of constants, inverses, shared
factors and Q14. These witnesses prove those intermediate ideals are
strictly smaller than saturation under the displayed memberships. After
adding Q15, nonzero boundary points still exist. This last observation
alone does not show they are extraneous. Full h-saturated ideals, boundary
gluing, proper F2 polarized closed embeddings and their verification have
not been constructed. Existing six-chart Kummer geometry from round08 is
retained; it does not settle the projective F2 circuit boundary.
