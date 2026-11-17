# Statement, hypotheses and exact proof map

Let F=ZMod 17 and W be Mathlib's affine short Weierstrass curve with A=B=2.
Let h(X)=4X²+9X+9 and V={5,6,10,16}. For signs e,f∈{1,-1}, define the
actual polynomial set G(r,s,e,f)⊂F[x,z,m] by

    g1 = m²-x-z-r
    g2 = xz-r²-rm²-2-2ms
    qx = e(4x²+9x+9)-m(x-r)+s
    qz = f(4z²+9z+9)-m(z-r)+s.

`generators` uses these very `SecantTopForms` definitions from the earlier
canonical-degree project. `coordinates x z m = ![x,z,m]` supplies the
evaluation homomorphism. `BaseRelation` means x,z∈V and the corresponding
actual nonsingular points (x,e h(x)) and (z,f h(z)) sum to R=(r,s).

For every nonsingular finite target R and every sign branch, the headline
theorem is the following conjunction:

1. For every x,z, existence of m annihilating every p∈G is equivalent to
   `BaseRelation` in Mathlib's actual nonsingular-point group.
2. The canonical ideal `DegreeBridge.topIdeal G` has first vanishing
   homogeneous degree 3.
3. The existing dense comparator `canonicalDense 4 2 2 r ...` has first
   vanishing homogeneous degree 5.

The actual degree of each generator selects its highest homogeneous
component. The theorem does not accept a caller-supplied component index.

## Declaration map

All new declarations are in `FixtureGroupDegree.lean` and were built, checked
with warnings as errors and included in the unchanged transitive audit.

| Declaration | Exact conclusion and assumptions |
| --- | --- |
| `graph_eval` | The polynomial h evaluates to the inherited graph function, for every t∈F |
| `complete_split` | Polynomial equality h²-(X³+2X+2)=4²∏[v∈V](X-v); no hypothesis |
| `discriminant_ne_zero` | Mathlib's actual W.Δ is nonzero, proved by kernel reduction |
| `target_nonsingular` | Every r,s satisfying s²=r³+2r+2 is nonsingular |
| `eval_g1`, `eval_g2`, `eval_q` | Evaluation of original polynomials equals the scalar residual, for every variable assignment |
| `zeroes_iff_system` | Every generator evaluates to zero iff all four secant residuals vanish |
| `canonical_top_ideal` | `topIdeal generators` is definitionally the inherited `canonicalSecant` |
| `polynomial_roots_iff_relation` | For each e,f∈{1,-1} and nonsingular R, raw roots iff actual base-pair relation; x,z need not be assumed in V |
| `exact_relations_and_degree` | The root equivalence and canonical 3/5 degree statements in one conjunction |
| `all_lifts_polynomial_complete` | For x,z∈V and nonsingular P,Q,R, actual P+Q=R iff one of the four polynomial branches supplies their ordinate lifts |
| `identity_relations_iff` | For signed nonsingular finite inputs, P+Q=O iff x=z and their ordinates are negatives |

## Why the bridges are faithful

The split identity is proved in the polynomial ring itself: expand the
finite product, normalize the ring expression, unfold the F17 abbreviation
and reduce coefficients modulo 17. No inference from equality of functions
on a finite field is used. Nonzero a=4 and Δ are discharged in the concrete
field rather than introduced as unverified fixture assumptions.

Each generator's evaluation is separately proved. Thus the exact set used
on the canonical top-ideal side also defines the roots on the group side.
The existing generic selector theorem then derives membership x,z∈V from
the split polynomial identity and proves both directions of actual point
addition. Its tangent and cubic-coefficient argument retains repeated
abscissae, including x=r, z=r and flexes. All ordinate lifts are covered by
the four branches. Infinity is handled by its exact inverse-pair theorem.

The degree part applies the already checked `CanonicalFixture` theorem
after the definitional `canonical_top_ideal` equality. This is a semantic
integration of that degree result, not a newly measured degree value.
The semantic equivalence here is for the secant presentation. A new
independent formal equivalence for the dense S3 presentation is outside
this theorem and is not silently assumed in its root statement.

## Foundations and dependency scope

Lean 4.19.0; Mathlib commit
`c44e0c8ee63ca166450922a373c7409c5d26b00b`; all nine locked Git revisions
were checked against the native checkouts. The two sealed prior projects are
repository-relative dependencies. Their shared identity module is identical.
No old source, workflow, audit program or native-trust manifest is modified.

The unchanged auditor imports all fourteen modules and checks every theorem
declared there, including generated equation declarations and imported
support definitions declared in those source files. It reports **159
declarations, zero failures, zero native trust**. All transitive axioms are
among propext, Classical.choice and Quot.sound. There are no proof
placeholders or custom axioms. `lean4checker` was unavailable.

The exact published generic group sources also rebuilt and passed a complete
22-declaration census and warnings-as-errors, closing the prior outage gap.
The now-recovered original source hashes match the published copies exactly.
Old sealed pending-status notes remain historical; new receipts supersede
those specific verification gaps without rewriting them.

## Sources and prior art

- [Existing concrete fixture](../secant/fixture.json), curve
  `icv1-fp5-tm1-a77cc6af`, base `FB1hed5b19904cc8`; inherited unregistered
  status retained. Field and model identities are exact. The new theorem
  does not establish the fixture metadata's group-order or registry claims.
- [Earlier canonical degree proof](../proof/CanonicalFixture.lean) and
  [highest-form construction](../proof/DegreeBridge.lean).
- [Generic split-base group bridge](../group_law_20261031/SplitFactorBase.lean).
- Angdinata and Xu, *An Elementary Formal Proof of the Group Law on
  Weierstrass Elliptic Curves in Any Characteristic*, ITP 2023,
  [DOI 10.4230/LIPIcs.ITP.2023.6](https://doi.org/10.4230/LIPIcs.ITP.2023.6),
  together with the pinned Mathlib Affine.lean and Group.lean.
- Pinned Mathlib `Algebra/MvPolynomial/Eval.lean` and polynomial evaluation
  definitions provide the evaluation homomorphisms used in the new bridge.
- [Existing identity certificate](identity.json),
  `IDC1h621ed1a51b5f3ca8`, copied unchanged from the secant study. Its
  original evaluated-tier claim is distinct from the Lean proofs here.

The original chord/tangent identity and Mathlib group law are established
prior work. No external novelty, affine solving-degree, runtime or yield
claim is added by this formalization.
