# Lean proof of the canonical top-ideal degree reduction

2026-10-31. **Kernel checked:** the canonical highest-total-degree ideals of
the displayed two-summand presentations have first vanishing homogeneous
degrees **d+1** (secant) and **2d+1** (dense), for d≥2 under the explicit
hypotheses below. The exact F17 graph base has degree **5 to 3**, every target
and all four sign branches. Component indices are computed from the actual
`MvPolynomial.totalDegree` of each original equation.

Headline declarations:
[`DegreeBridge.canonical_strict_reduction`](DegreeBridge.lean) and
[`CanonicalFixture.degree_five_to_three`](CanonicalFixture.lean).
All eleven modules build with warnings treated as failures. The unchanged
repository auditor checked 121 declarations, including generated equation
lemmas: **zero failures, zero trusted native evaluations**. All transitive
axioms lie in `propext`, `Classical.choice`, `Quot.sound`.

See the [illustrated report](reports/formalization.pdf),
[proof diagram](reports/coverage.svg), [build log](verification/build.log),
[axiom audit](verification/axioms.log) and [reproduction instructions](README.md).
This follows merged [PR #3077](https://github.com/isogeny-labs/crypto/pull/3077);
the earlier exponent-only proofs and experimental records remain unchanged.

## Exact mathematical scope

Let K be a field with `2 ≠ 0`, d≥2, a,b≠0. In three variables use

```text
g1 = m² − x − z − r
g2 = xz − r² − rm² − A − 2sm
qx = ax^d + graphX − m(x−r) + s
qz = bz^d + graphZ − m(z−r) + s
```

Require each graph lower term to have total degree below d. Actual signed
degree-d graph equations are the univariate subclass with a=εa₀, b=ηa₀
and corresponding signed lower coefficients. Every sign choice is retained.

The dense reference uses two variables, the actual ordinary short-Weierstrass
S3 formula, and `x^(2d)+memberX`, `z^(2d)+memberZ`, each lower term having
total degree below 2d. Monic membership polynomials for size-2d bases are this
subclass. The different variable counts are part of the stated comparison.

Lean defines `top p = homogeneousComponent p.totalDegree p`, and
`topIdeal generators = Ideal.span (top '' generators)`.
`FirstVanishing J n` means every degree-n homogeneous polynomial belongs to
J and, for **each** k<n, a homogeneous degree-k polynomial does not. A checked
quotient-map equivalence identifies this with vanishing of the homogeneous
image in the actual quotient. No degree, ideal equality or solver success is
assumed as a conclusion-shaped hypothesis.

The degree result is independent of whether a target lies on the curve.
The relation interpretation still uses complete splitting and an on-curve
target in the earlier written geometric argument. This is the precise
highest-form homogeneous quotient convention in the study, not a universal
definition of affine Gröbner solving degree.

## Proof map

Every checked declaration below was built and included in the transitive audit.

| Step | Lean declaration | Hypotheses and status |
| --- | --- | --- |
| Homogeneous image is zero iff its polynomials lie in J | `SecantIdeal.vanishes_iff_quotient_zero` | Checked for actual polynomial ideals |
| Support divisibility ↔ monomial-ideal membership | `SecantIdeal.secant_mem`, `dense_mem` | Checked using Mathlib's exact support theorem |
| Secant first degree d+1, dense first degree b+1 | `secant_first_vanishing`, `dense_first_vanishing` | Checked for d,b≥2; witnesses at every lower degree |
| Actual nonmonomial quadratic ideal first degree3 | `SecantQuadratic.quadratic_first_vanishing` | Checked for arbitrary α,β, `2≠0` |
| Explicit secant top ideals equal the reduced ideals | `SecantTopForms.leadingIdeal_eq_secant`, `leadingIdeal_two` | Checked with nonzero leading coefficients and strict lower-term bounds |
| Actual ordinary S3 component | `DenseTopForms.s3_component` | Checked from the actual formula; equals x²z² |
| Actual total degree of every original generator | `DegreeBridge.g1_degree`, `g2_degree`, `q_degree`, `s3_degree`, `membership_degree` | Checked by vanishing above and nonvanishing at the asserted degree |
| Canonical tops equal the computed components | `DegreeBridge.canonicalSecant_eq`, `canonicalDense_eq` | Checked; component indices are not supplied by the caller |
| Canonical strict degree reduction | `DegreeBridge.canonical_strict_reduction` | Checked under exactly the field/coefficient/degree hypotheses above |
| Exact F17 base and graph intersection | `SecantFixture.base_cardinality`, `nonsingular`, `exact_split`, `graph_intersection_iff` | Checked; primality 17 discharged by kernel `decide` |
| Both signed lifts are curve points | `SecantFixture.signed_lift_on_curve` | Checked for every listed abscissa and either sign |
| Exact membership and signed graph polynomials | `SecantFixture.membership_polynomial`, `signed_graph_polynomial` | Checked polynomial-ring equality, not merely sampled evaluations |
| F17 canonical degree 5→3, all targets and signs | `CanonicalFixture.degree_five_to_three` | Checked over ZMod17, ε,η∈{±1} |
| Secant cubic identity and selected-point curve membership | `SecantIdentity.cubic_identity`, `intersection_factorization`, `selected_point_on_curve` | Checked over arbitrary commutative rings; no division losing diagonals |
| Full elliptic-curve addition/group-relation equivalence | Earlier `SECANT_GRAPH.md` | Written proof; not formalized here |
| Hilbert-series coefficients, actual F4 degree, runtime or independent-row yield | Outside this theorem | No such formal claim |

For d≥3 the reduced secant ideal is `(m²,xz,x^d,z^d)`; dense is
`(x^(2d),z^(2d),x²z²)`. Mathlib's polynomial-ideal support criterion now
connects the old exponent results to actual ideal membership.

For d=2 the proof retains `(m²,xz,x²−αmx,z²−βmz)`. All cubic monomials
belong to this actual nonmonomial ideal. For the lower bound, a ring map
sends m↦u, z↦0, x↦(α/2)u+v in the iterated square-zero extension.
It kills the generators but sends mx to nonzero uv. The formal target is
Mathlib's actual `TrivSqZeroExt (TrivSqZeroExt K K) (TrivSqZeroExt K K)`;
no desired quotient structure or nonmembership is assumed.

## Exact fixture

Field17 and A=B=2 match `icv1-fp5-tm1-a77cc6af`, candidate
`FB1hed5b19904cc8`. Lean checks

```text
h(T) = 4T²+9T+9
h(T)² − (T³+2T+2) = 16(T−5)(T−6)(T−10)(T−16)
F_V(T) = T⁴+14T³+10T+6
```

The cardinality 4, exact intersection for every field element, nonsingularity,
both signed lifts and the actual membership product are checked.
ICV1/FB1 names are dataset provenance; these theorems check the field and
polynomial coefficients, not the storage-hash encoding or group-order 19
certificate.

## Dependencies, checks and remaining scope

Lean 4.19.0; Mathlib v4.19.0 commit
`c44e0c8ee63ca166450922a373c7409c5d26b00b`; all dependencies are locked in
`lake-manifest.json`. Shared `crypto-lean` revision inspected:
`148f5dc389b341401e06d76abe7783c4ccf42f2e` (gate provenance, not a loaded
axiom/dependency). The unchanged local auditor is byte-identical to the
repository tool at merged PR #3077.

Sources checked before implementation:
[Mathlib monomial ideals](https://github.com/leanprover-community/mathlib4/blob/c44e0c8ee63ca166450922a373c7409c5d26b00b/Mathlib/RingTheory/MvPolynomial/Ideal.lean),
[homogeneous components](https://github.com/leanprover-community/mathlib4/blob/c44e0c8ee63ca166450922a373c7409c5d26b00b/Mathlib/RingTheory/MvPolynomial/Homogeneous.lean),
[square-zero extensions](https://github.com/leanprover-community/mathlib4/blob/c44e0c8ee63ca166450922a373c7409c5d26b00b/Mathlib/Algebra/TrivSqZeroExt.lean),
and the parent THEORY/SECANT_GRAPH notes. These are classical algebraic
methods; formal verification does not establish literature-wide novelty.

The binary cache returned proxy CONNECT 403; the pinned library was built
from source without bypassing network policy. The final build and unchanged
transitive auditor passed; the gate's cache-download preflight is not claimed
as passed. `lean4checker` was unavailable, so no `--fresh` result is claimed.
No source uses `sorry`, `admit`, custom axioms, native evaluation or a kernel
bypass. Development failures are preserved separately.

This was proof checking, with no new cryptographic experiment, CAS rerun or
performance measurement. The earlier measured F4 degree 6→3 and the proved
homogeneous degree 5→3 remain distinct. Existing sealed records and historical
figures remain unchanged; this report and diagram add the formal-coverage
record. Canonical priced-pipeline graphs and benchmark tables receive no new
cost row from this proof-only continuation.
