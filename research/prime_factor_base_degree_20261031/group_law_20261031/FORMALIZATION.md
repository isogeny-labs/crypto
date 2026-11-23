# Exact group-law bridge: proof map and argument

## Statement

Let K be a field and W the short Weierstrass curve

    y² = x³ + A x + B.

For nonsingular finite points P=(x,y), Q=(z,v), R=(r,s), define

    g1 = m² - x - z - r,
    g2 = x z - r² - r m² - A - 2 m s,
    qx = y - m(x-r) + s,
    qz = v - m(z-r) + s.

The checked theorem `SecantGroupLaw.finite_target_iff` states

    (there exists m with g1=g2=qx=qz=0) iff P + Q = R,

where + is Mathlib's addition on `WeierstrassCurve.Affine.Point`.
This is the existing mathematically verified abelian group, including its
associativity theorem, rather than a newly defined coordinate operation.

For target O the exact theorem is

    P + Q = O iff x=z and y=-v.

No finite-slope branch represents O; it has its own complete inverse-pair
characterization. Points in the graph factor base are finite, so inputs O
are outside its domain. The existing group also supplies its identity laws.

## Proof map

Every declaration below compiled in the successful standalone build03.
The published sources were recovered after the outage; a fresh published-byte
replay and full repository declaration census are pending.

| Mathematical obligation | Lean declaration | Exact hypotheses |
| --- | --- | --- |
| Short-model equation and nonsingularity | `SecantGroupLaw.equation_iff`, `nonsingular_iff`, `negY_eq` | Field K; A,B in K |
| Extract line ordinates | `SecantGroupLaw.system_ordinates` | Four residuals vanish |
| Repeated root is tangent | `SecantGroupLaw.diagonal_tangent` | g1,g2 vanish with z=x; line ordinate |
| Residual slope equals Mathlib's canonical slope | `SecantGroupLaw.canonical_slope` | Nonsingular P; four residuals |
| Soundness | `SecantGroupLaw.system_implies_add` | Nonsingular P,Q,R; residuals |
| Secant slope difference including diagonal | `SecantGroupLaw.slope_difference` | Nonsingular P,Q; pair nonvertical |
| Completeness | `SecantGroupLaw.add_implies_system` | Nonsingular P,Q,R; actual P+Q=R |
| Finite-target equivalence | `SecantGroupLaw.finite_target_iff` | Nonsingular P,Q,R |
| Identity target | `SecantGroupLaw.identity_target_iff` | Nonsingular P,Q |
| Exact graph-intersection membership | `SplitFactorBase.graph_intersection_iff` | Field; CompleteSplit identity; a≠0 |
| Every lift has an allowed sign | `SplitFactorBase.every_lift_is_signed` | Same split; x in V; curve equation |
| Curve equation gives nonsingular point | `SplitFactorBase.nonsingular_of_equation` | Discriminant Δ≠0 |
| Raw selector roots iff actual base-pair relation | `SplitFactorBase.selector_iff_group_relation` | CompleteSplit; a≠0; Δ≠0; nonsingular R; e,f in {1,-1} |
| Completeness of union of four branches | `SplitFactorBase.all_four_branches_complete` | CompleteSplit; a≠0; nonsingular P,Q,R; x,z in V |
| Signed-base identity target | `SplitFactorBase.identity_signed_iff` | Nonsingular signed graph points |
| Universal cubic identity and root soundness | Three declarations in copied `SecantIdentity` | Commutative ring; residual hypotheses exposed |

## Soundness, including the diagonal

The line equations give y=m(x-r)-s and v=m(z-r)-s. If x≠z, subtracting
them identifies m with Mathlib's secant slope. If x=z, y=v, and the explicit
linear combination

    (2x-r)g1 + g2 + 2m(y-m(x-r)+s)

gives the tangent equation 2my=3x²+A. If 2y=0, that equality forces
3x²+A=0, contradicting nonsingularity. Thus the tangent denominator is
nonzero and m is the canonical tangent slope. This also excludes a vertical
pair for a finite-target system.

Mathlib's addition coordinates now yield x(P+Q)=m²-x-z=r and
y(P+Q)=s. No step requires x-r or z-r to be nonzero, so the same proof
includes x=r, z=r, x=z, and x=z=r flex configurations.

## Completeness without cancelling target-abscissa differences

Finite P+Q excludes the vertical pair. Define m to be Mathlib's canonical
slope. Its addition coordinates yield g1=qx=qz=0, on and off the diagonal.

For g2, use `Affine.addPolynomial_slope`: the full intersection cubic
factors with the addition point as third root. Via `Cubic.toPoly_injective`
compare the linear coefficients of the cubics, obtaining

    2x m² - 2y m + A = xz + xr + zr.

Combining this with the first residual and the line equation gives g2=0.
This coefficient argument preserves multiplicity and avoids cancelling any
target-abscissa difference.

## Exact split factor base

`CompleteSplit A B h V a` means the actual polynomial equality

    h(X)² - (X³ + A X + B) = a² ∏[t in V] (X-t).

Assume a≠0. Evaluation at u and the field's zero-product law prove

    h(u)² = u³+A u+B iff u in V.

For any curve ordinate y over u in V, factor
(y-h(u))(y+h(u))=0 to obtain y=h(u) or y=-h(u).
These are proved consequences of the polynomial identity, not assumed
membership and sign oracles.

For each e,f in {1,-1}, the raw four-equation system with ordinates
e h(x), f h(z) therefore holds exactly when x,z are in V and their
corresponding actual curve points sum to R. With Δ≠0 all extracted curve
points are nonsingular. Taking the four sign choices covers every ordinate
lift, including overlapping choices at two-torsion.

## Relationship with the canonical degree theorem

Merged #3082 proves `DegreeBridge.canonical_strict_reduction` for actual
highest homogeneous forms and the F17 specialization
`CanonicalFixture.degree_five_to_three`. This continuation proves the
group semantics of the same displayed secant equations. The copied
`SecantIdentity.lean` is byte-identical to that sealed proof package.

This project does not yet provide a single theorem importing the old F17
fixture and bundling its concrete CompleteSplit identity, group equivalence
and canonical degree statement. An attempted relative-dependency layout was
not built before worker loss, and is not the published configuration.

The degree notion remains the prior study's first vanishing degree of the
canonical homogeneous quotient. It is separate from affine solving degree,
and no new numerical degree or runtime measurement is asserted here.

## Foundations and sources

The successful build printed transitive axioms for twelve central
declarations: only `propext`, `Classical.choice` and `Quot.sound`.
The copied ring identities use `propext` and `Quot.sound`.
There are no proof placeholders or custom axioms in these source files.
A whole-module audit, warning-as-error replay and `lean4checker --fresh`
are not claimed for this continuation.

Sources read before proof development:

- Angdinata and Xu, *An Elementary Formal Proof of the Group Law on
  Weierstrass Elliptic Curves in Any Characteristic*, ITP 2023,
  [DOI 10.4230/LIPIcs.ITP.2023.6](https://doi.org/10.4230/LIPIcs.ITP.2023.6).
  The paper supplies the verified group-law architecture reused here.
- Pinned [Mathlib Affine.lean](https://github.com/leanprover-community/mathlib4/blob/c44e0c8ee63ca166450922a373c7409c5d26b00b/Mathlib/AlgebraicGeometry/EllipticCurve/Affine.lean):
  nonsingularity, secant/tangent slope, addition cubic and point constructors.
- Pinned [Mathlib Group.lean](https://github.com/leanprover-community/mathlib4/blob/c44e0c8ee63ca166450922a373c7409c5d26b00b/Mathlib/AlgebraicGeometry/EllipticCurve/Group.lean):
  actual abelian group law.
- [Existing secant/graph argument](../SECANT_GRAPH.md) and
  [merged canonical degree proof](../proof/FORMALIZATION.md).

No literature-wide novelty claim is made for the group law or the bridge.
