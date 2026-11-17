# Claim-to-proof map

Lean4.19.0, Mathlib c44e0c8ee63ca166450922a373c7409c5d26b00b.
Exact signatures are in the linked source; none of the hypotheses below
stand in for a desired degree or successful relation search.

| Report claim | Lean declaration | Hypotheses and scope |
| --- | --- | --- |
| Boolean equations give precisely two values | `PencilSelector.boolean_iff_equations`, `boolean_iff_bool` | Any field; finite label length; field-valued bits, not Boolean-ring reduction |
| Raw secant roots give actual nonsingular base relations | `PencilSelector.system_implies_base_relation` | Short-curve discriminant nonzero; finite target nonsingular; both signs ±1; every Boolean and secant equation satisfied |
| Every base relation has a root and conversely | `PencilSelector.complete_selector_iff` | Actual nonsingular P,Q,R supplied; both labels and both signs existential; no complete-split assumption |
| Identity target means inverse pair | `PencilSelector.identity_target_iff` | Actual nonsingular points; identity target handled separately from finite slopes |
| Complete auxiliary-variable ideal vanishes in n+3 | `PencilTop.vanishes` | Any field, arbitrary two core parameters, n selector squares |
| Degree n+2 has a surviving witness | `PencilTop.last_nonzero`, `highWitness_not_mem` | Field with 2≠0; witness is m*x times all n selector variables |
| n+3 is first, not only an upper bound | `PencilTop.first_vanishing` | Same hypotheses; monotonicity excludes all smaller degrees |
| Actual canonical top ideal equals normalized core plus bit squares | `PencilCanonical.canonical_top_ideal` | Nonzero quadratic coefficients in both slots; all other lower coefficients/weights/target arbitrary |
| Actual generator invariant is n+3 | `PencilCanonical.canonical_first_vanishing` | Both quadratic coefficients nonzero, 2≠0; top forms selected using actual `totalDegree` |
| Every original polynomial evaluates to its named scalar residual | `PencilCanonical.eval_g1`, `eval_g2`, `eval_q`, `eval_bit`, `zeroes_iff` | Any field, every field coordinate and bit; no solution hypothesis needed |
| Exact projection of a Boolean affine variable | `PencilElimination.one_selector_iff`, `two_selectors_iff` | Any field, including zero weights; no division or missing chart |
| Universal elimination identity | `PencilElimination.elimination_identity` | Any field; polynomial-ring identity; IDC1 evaluated-tier record is separate |
| Larger fixture has precisely eight abscissae | `PencilFixture.exact_membership`, `base_cardinality` | Every on-curve x,y over ZMod17; all labels and both signs |
| Sixteen distinct signed points; non-full fibres | `PencilFixture.signed_cardinality`, `fibre_cardinalities` | Exact finite checks by kernel-reduced `decide`, not `native_decide` |
| Concrete generator zeros are the scalar selector system | `PencilFixture.zeroes_iff_system` | Four independent field-valued bits; exact coefficient/sign/vector bridge |
| Same concrete generators are complete for actual group addition | `PencilFixture.polynomial_complete` | Every nonsingular P,Q,R; LHS includes membership in the exact eight-column set; all signs/labels retained |
| No assumed base/ordinate certificate for solver outputs | `PencilFixture.roots_imply_relation` | Complete raw zeroes, signs ±1, finite nonsingular R; on-curve/base/group witnesses derived |
| Concrete canonical degree7 | `PencilFixture.canonical_degree_seven` | Any target coordinates and all four sign branches; no field equations omitted |
| Actual dense membership polynomial | `PencilFixture.dense_membership_polynomial` | Equality in the polynomial ring; product over the same eight abscissae |
| Concrete dense canonical degree9 | `PencilFixture.denseRemainder_degree`, `dense_degree_nine` | Same exact membership polynomial; any finite target abscissa |
| Ambient envelope arithmetic | `PencilFixture.ambient_budget_arithmetic` | Only binomial arithmetic 3432 and55; no theorem about an F4 implementation |

## Core of the upper/lower proof

Writing x,z,m for the three core variables and b_i for n bit variables, the
normalized canonical top ideal is

    (m², xz, x²-alpha*m*x, z²-beta*m*z, b_1², ..., b_n²).

Every monomial of degree n+3 either contains a repeated bit or has core
degree at least3. The six core degree3 monomials reduce using the displayed
generators, so it belongs to the ideal. The homogeneous monomial expansion
extends this to every homogeneous polynomial at that degree.

For the lower witness, map into the polynomial ring in U,V,b_i modulo
their pure squares: m→U, x→(alpha/2)U+V, z→0, b_i→b_i. Each generator
maps into the square ideal. The degree n+2 witness m*x*product(b_i) maps,
modulo those squares, to the squarefree monomial U*V*product(b_i), which
is not in the square ideal. Vanishing propagates upward, so the surviving
witness excludes every earlier first-vanishing degree.

This is a genuine polynomial-ring proof. Finite-field functional identities
or a first-fall observation are not substituted for the canonical invariant.
Semantics quantify coordinates in the stated field. Algebraic-extension roots
are not rational relations; their separation/rejection remains charged in any
solver run. The canonical ideal is exactly the displayed generator ideal.

## Explicit open obligations

The one-bit projection is semantically exact but its projected highest-form
degree is not separately formalized. No affine solving-degree theorem, full
Hilbert-series theorem, 17-summand composition, runtime theorem, group-order
proof, external novelty proof or experiment is claimed. The dense comparator
uses the exact same base, but this continuation does not add a separate new
dense S3-to-group equivalence theorem. Accepted modules are audited using
the unchanged repository auditor, with standard foundations only.
