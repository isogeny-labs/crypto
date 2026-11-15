# Proof map

Lean 4.34.1 and Mathlib commit `d13f23b723b8a846827a245b89c10fc7d3f11612`
are pinned in the toolchain, Lake file, and dependency manifest. The repository
proof gate builds the module and audits all transitive theorem axioms. The current
latest receipt reports **106 named research theorems, zero trusted native declarations**;
the expanded audit checks 127 declarations including 21 generated helper lemmas. Only standard
`propext`, `Classical.choice`, and `Quot.sound` occur, and some proofs need fewer.
No theorem source contains a placeholder or a custom research axiom.

| Report implication | Lean declaration in `TwistProof` | Exact scope |
| --- | --- | --- |
| Mixed rational sum forces 2B=0 | `mixed_relation_two_torsion` | additive commutative group, additive endomorphism; A fixed, B negated, A+B fixed |
| Coprime annihilators force zero | `coprime_annihilators` | any additive commutative group; two coprime natural annihilators |
| Odd twist contribution vanishes | `mixed_relation_odd` | preceding eigenconditions; r annihilates B and gcd(2,r)=1 |
| Coprime finite orders forbid nonzero homomorphic transport | `finite_hom_coprime` | finite additive commutative groups with coprime cardinalities |
| Trace plus anti-trace equals twice P | `trace_antitrace_sum` | any additive commutative group and additive endomorphism |
| Trace is fixed | `trace_fixed` | the endomorphism is an involution |
| Anti-trace is negated | `antitrace_negated` | the endomorphism is an involution |
| Common odd order divisor divides q+1 | `common_order_divides_q_plus_one` | N+Nt=2(q+1), common divisor r, gcd(r,2)=1 |
| Symmetric S3 formula | `symmetric_s3` | polynomial identity in every commutative ring |
| Vieta twist-parameter cancellation | `vieta_twist_cancellation` | polynomial identity in every commutative ring |
| Cleared isomorphism equation | `twist_equation_identity` | polynomial identity in every commutative ring |
| x-fixing affine twist map | `twist_affine_map` | u²=d and d y²=x³+a x+b; affine equation only |
| Monic normalization of a c y² model | `monic_normalization` | affine equation; map (x,y) to (c x,c² y) |
| Algebraic embedding-degree-two implication | `embedding_degree_two` | in a commutative ring, q+1=0 and 2≠0 imply q²=1 and q≠1 |

The three ring identities have accompanying IDC1 records in `../certificates/`.
Each certificate points to this exact Lean source hash and names its declaration.

`FrobeniusBridge.lean` additionally uses Mathlib's actual nonsingular elliptic
point group, `FiniteField.frobeniusAlgHom` and `Point.map`, not an uninterpreted
endomorphism. `rationalLift` is the actual base-change group homomorphism.
Its four checked declarations are:

| Declaration in `FrobeniusBridge` | Exact statement |
| --- | --- |
| `frobenius_fixes_rational` | q-power Frobenius fixes points lifted from the finite base field |
| `rational_mixed_two_torsion` | an actual rational relation with a Frobenius-negated B forces 2B=0 |
| `rational_mixed_odd` | with an odd annihilator, the same actual-point relation forces B=0 |
| `rational_relation_has_zero_twist_term` | B=0 and the two lifted rational points coincide |

These require a finite base field, a field algebra extension and an elliptic
Weierstrass model; decidable field equality supports Mathlib's group operations.
They derive the plus-eigenspace property of rational lifts. Their abstract minus
hypothesis is now discharged for the explicit short-model images below.

`TwistEmbedding.lean` constructs an injective map on actual nonsingular rational
points, including O, from the monic twist with coefficients d²a,d³b to E(K):
`(X,Y) -> (X/d,Y/(d*u))`, where u²=d. A finite base field, odd characteristic,
a nonsquare d, an elliptic short model, and a field algebra containing u are
explicit assumptions. No assumption that Frobenius negates the image remains.

| Declaration in `TwistEmbedding` | Exact statement |
| --- | --- |
| `nonsquare_ne_zero` | a nonsquare in a field is nonzero |
| `frobenius_root_neg` | over a finite odd base field, u²=d with nonsquare d implies u^q=-u |
| `twist_image_equation` | the explicit normalized twist coordinates satisfy the actual target equation |
| `frobenius_lift_neg` | actual q-power Frobenius negates every mapped twist point, including O |
| `lift_injective` | the explicit map on actual nonsingular points is injective |
| `rational_twist_relation_two_torsion` | A+lift(P)=R with A,R rational forces 2*lift(P)=0, with no assumed hB |
| `rational_twist_relation_odd` | an odd annihilator of that image forces P=O and equal rational lifts |
| `frobenius_sum_lift_neg` | every finite sum of explicit twist images is Frobenius-negated |
| `rational_twist_sum_odd` | an odd-annihilated sum of twist images in a rational relation is zero |
| `cleared_monic_twist_identity` | universal cleared-map polynomial identity; source-bound IDC1h9dc35b198cc2fa02 |

Those original statements keep the odd-annihilator hypotheses on the image or
sum. Their source and source-bound certificate are unchanged. The two new modules
below prove addition compatibility, transport source annihilators, and identify
the whole short-model minus eigenspace. The sum conclusions kill the aggregate;
they do not make each summand zero.

## Group law and full short-model eigenspaces (2026-10-23)

`TwistGroupMap.lean` proves the existing map preserves addition in every case,
including O, inverse pairs, tangents and two-torsion. For the group-map statements,
the hypotheses are fields k,K, an algebra k→K, d≠0, u²=d, and ellipticity of
`short a b`; finiteness, nonsquareness and odd characteristic are not needed
until the Frobenius obstruction. `liftHom` is the actual `AddMonoidHom`, and
`liftEquivRange` is its injective isomorphism onto its image.

| Declaration in `TwistGroupMap` | Exact statement |
| --- | --- |
| `slope_scale` | secant/tangent/vertical slope scales by 1/u, with d≠0 and u²=d; all zero-denominator cases retained |
| `addX_scale`, `addY_scale` | actual addition-coordinate formulas scale by 1/d and 1/(du) |
| `cleared_tangent_identity`, `cleared_addX_identity`, `cleared_addY_identity` | universal commutative-ring identities for the polynomial parts, with IDC1 records |
| `mapped_slope_scale` | the slope statement commutes with the actual base-field embedding |
| `lift_add` | psi(P+Q)=psi(P)+psi(Q) for actual nonsingular points |
| `lift_nsmul`, `lift_zsmul` | natural and integer scalar compatibility |
| `source_annihilator` | rP=O implies r psi(P)=O |
| `lift_sum` | every finite sum commutes with psi |
| `lift_annihilator_iff` | r psi(P)=O iff rP=O, using the proved injection |
| `rational_relation_source_odd` | rational A+psi(P)=R and odd r with rP=O imply P=O and equal rational lifts |
| `rational_sum_source_odd` | an odd annihilator on the source aggregate suffices for the same aggregate conclusion |
| `rational_sum_common_source_odd` | a common odd source annihilator on every summand suffices; individual summands need not vanish |

`TwistEigenspace.lean` derives the fixed field rather than assuming it. It uses
the complete root set of X^q−X, and injectivity of k→K, to show z^q=z iff z
comes from k. No finite-extension or degree-two assumption on K is hidden:
K can be any field algebra containing the specified square root. The minus
theorem uses finite odd k, nonsquare d, u²=d and the explicit elliptic short model.

| Declaration/construction in `TwistEigenspace` | Exact statement |
| --- | --- |
| `frobenius_fixed_iff` | q-power fixed field in K is exactly the image of the finite base field |
| `twist_delta` | the actual discriminant scales by d^6; source-bound IDC1 record |
| `twist_elliptic` | d≠0 and ellipticity of E imply ellipticity of the monic twist |
| `lift_range_iff_neg` | Q is an explicit rational twist image iff actual Frobenius sends Q to −Q |
| `rational_range_iff_fixed` | for any elliptic Weierstrass model, Q comes from a rational base-change point iff Frobenius fixes Q |
| `minus`, `liftEquivMinus` | actual additive minus subgroup and group isomorphism from the rational monic twist onto all of it |

The inverse existence proof descends X=d·x and Y=d·u·y from a minus point,
checks the actual twist equation and nonsingularity, and reconstructs Q.
`liftEquivMinus` is not merely an isomorphism onto an unspecified image.
It is also not a claim that every point of E(K) is in the minus subgroup, or
that E(K) is a direct sum at two-torsion. The retained nonzero two-torsion
controls explain why oddness cannot be dropped from the relation obstruction.

Four [new native IDC1 records](../validation/group-map-20261023/certificates/receipt.json)
bind the exact new Lean sources, replay 32+16 points each, reject false plus-one
identities arithmetically, and reject false source hashes. They certify only the
polynomial identities; the checked Lean proofs establish division, actual-point
addition and Frobenius consequences. That 2026-10-23 axiom receipt covers
57 declarations in the then-current five modules and remains frozen.

## Positive pairing transfer (2026-10-24)

`PairingFixture.lean` constructs the computable field `QuadraticAlgebra (ZMod 43) 2 0`
and actual nonsingular points on E:y²=x³+x. The rational generator is P=(31,18),
Q=(13,24), and the monic twist point T=(35,12) maps to S=(39,3u).
The explicit `weil11` function uses line functions and a linear Miller chain;
support collisions give `none`. It is not defined by a logarithm lookup.
Kernel evaluation, including `decide +kernel`, uses no trusted native axiom.

| Declaration in `PairingFixture` | Exact finite statement |
| --- | --- |
| `explicit_twist_source` | the existing actual twist map sends T to S |
| `source_frobenius`, `twist_frobenius` | P is Frobenius-fixed and S is negated |
| `source_order`, `twist_order` | 11P=11S=O |
| `source_known_scalar` | 7P=Q on the actual Mathlib group |
| `target_nontrivial`, `target_order` | g=2+25u is nontrivial and g^11=1 |
| `target_known_scalar`, `target_norm_one` | g^7=11+19u and g^43*g=1 |
| `pairing_generator`, `pairing_target` | explicit Miller evaluations give g and g^7 |
| `pairing_table` | every one of the 121 generator pairs has value g^(a*b), for a,b in Fin11 |
| `target_powers_injective` | the eleven powers are distinct |
| `logarithm_transport` | the concrete value on ([a]P,S) equals g^b iff a=b in Fin11 |

`PairingTransfer.lean` separately proves the generic implication with its
homomorphism/nondegeneracy hypotheses explicit:

| Declaration | Exact scope |
| --- | --- |
| `character_scalar` | an additive-to-multiplicative homomorphism transports natural scalars to powers |
| `character_order` | prime r annihilating P and a nontrivial image imply exact image order r |
| `character_log_recovery` | under those hypotheses, exponents in Fin r agree iff their target values agree |
| `norm_product_identity` | universal quadratic norm multiplication polynomial; native source-bound IDC1h23c0696a12358b08 |

These 19 new statements bring the total to 68 named theorems. The
[unmodified pairing-round gate](../validation/pairing-20261024T0442Z/lean-gate-final/)
audits 85 declarations in seven modules, including 17 generated helpers, with
zero failing/extra trusted axioms. The [finite native census and report](../reports/pairing-20261024T0442Z/README.md)
independently check 4,616 auxiliary points in 274 shared-prime cases and all
eleven F43 IC certificates. Native IC relation collection/solving is not formalized.
The finite Miller table does not prove equality with the geometric divisor
definition or general Weil bilinearity/nondegeneracy. The generic character
theorem does not assume those properties have been proved for every curve.

## Relation yield, row rank and conditional solver-degree costs (2026-10-24)

`RelationYield.lean` defines finite accepted integer coefficient sets by actual
zero sums of the indexed factor-base points. For every injective additive map,
the accepted coefficient set is identical before and after transport. Raw yield,
the same useful-row predicate's yield, and the actual coefficient span/rank over
any field are therefore equal. `twist_relationRows`, `twist_rawYield` and
`twist_rowRank` instantiate that result on the actual short-model map into E(K).
It does not identify E(k) with E_d(k).

For uniformly sampled entire finite groups, `completeRelationEquiv` gives the
unique last point completing any prefix to zero. `full_relation_count` proves
N^m accepted tuples; `full_factor_base_yield` proves raw rate1/N;
`full_yield_ratio` proves twist/main rate ratio N/Nt, and
`full_yield_comparison` gives higher raw rate for a smaller group. Arbitrary
small factor-base distributions and useful independent rows are outside that
whole-group rate theorem.

`RelationCost.lean` proves finite-batch work/useful normalization, exact overhead
thresholds, monotonicity (and strict monotonicity under n,omega>0) of the explicit
binomial(n+D,n)^omega work model, and sufficient yield/degree budget inequalities.
The supplied D is a relevant solver-degree parameter. No actual polynomial
ideal's regularity, regularity-to-solving-degree bridge or F4/F5 work bound is
silently formalized. Independently justified actual work bounds and a genuine
scalar-preserving source-to-destination reduction remain required for an actual
main-DLP speed claim. `scalar_solution_iff` and `actual_faster_of_bounds` expose
those separate bridges.

`YieldFixture.lean` checks actual F5 main/twist points: one versus three zero
relations on n=0,...,8, but all twist rows are zero modulo3. Its two synthetic
budget examples separately show possible model improvement and reversal by
overhead, without attributing those costs to the F5 curves. The [source-linked
report](../reports/yield-20261024/README.md) maps all 38 new named theorems.
The [unmodified final gate](../validation/yield-20261024/lean-gate-final/)
audits127 declarations in ten modules, zero failing/extra trusted. Source-bound
IDC1h1fac46ceb41801ac covers only the cleared-budget polynomial; Lean separately
checks positivity, counts, row rank and inequalities. All previous sources and
certificate bindings remain unchanged.

## Actual S3 systems, degree certificates and F43 reduction (2026-10-24)

`S3System.lean` adds23 named theorems. It proves the actual S3 leading component
is x^2*y^2, the degree2/3 base-polynomial leading components, and the actual
leading ideals <x^2*y^2,x^b,y^b>. Monomial membership and the sharp first-empty
degree b+1 are checked. This refutes a strict homogeneous-regularity gain for
this specific encoding, not other descent systems. Its actual F43 point map
phi(x,y)=(41x,11y) is injective/additive and preserves arbitrary scalar solutions.
The actual first-target main base has no signed decomposition; the twist base
has two independent rows, uniquely determining logs5 and7.

`S3DegreeFixture.lean` adds15 named theorems. An explicit dual functional
annihilates all13 degree4 main input multiples but maps1 to1. An explicit
degree5 combination derives1. Both are checked on the actual bound S3/base
polynomials; native closure degree6 is a separate, stronger matrix criterion.
The module also proves that pulling the twist base back to main reproduces
the accepted relation sets for every finite integer coefficient candidate set,
with the target included. It checks both actual pulled-back point relations.
The universal cleared unit identity has its own source-bound IDC1 record.

The [report and inspected PDF](../reports/s3-yield-20261024/README.md) distinguish
homogeneous first-empty degree, finite vanishing-space closure degree, actual
refutation degree and unmeasured F4/F5 work. The [unmodified current gate](../validation/s3-yield-20261024/lean-gate-final/)
audits185 declarations in twelve modules, including144 named research theorems,
zero extra trusted; only standard propext, Classical.choice, Quot.sound occur.
The independent native audit checks124 comparisons /1,904 systems. The F43
first-target closure4 versus6 is target/basis dependent, with equal full-target
means and exact inverse-transport equality. No source/receipt in earlier dated
rounds is changed, and native matrix replay is not silently called a Lean proof.

**Remaining bridges.** The explicit short-model image's Frobenius-negation
property, additive isomorphism onto the entire minus subgroup, and rational
fixed-point identification are checked under the hypotheses above. General-model
normalization with compatible group maps remains open, as do
S3/S4 lifting including exceptional cases, general geometric Weil-pairing
correctness/nondegeneracy and the uniform DLP reduction. The explicit F43
Miller transfer is now checked as stated above. Its abstract implications are fully proved;
the complete historical geometric theorems are only partially formalized.
Finite enumerations check the original fixtures, not those universal bridges.
The trace-zero counting theorem and general GHS construction likewise remain open.
The finite S3 useful-row/closure example and actual F43 source reduction are now
checked above. Uniform useful yield, a lower relevant degree in a different
encoding and compatible complete solver work bounds remain unestablished.
No measured speedup is claimed.
The [latest build and axiom receipts](../validation/s3-yield-20261024/)
include all twelve modules. The earlier 14-,18-,28-,49-,68- and106-theorem gate receipts and
their exact Lake configurations are retained unchanged.

Reproduce from the repository root:

```sh
tools/lean/proof-gate.sh --out /tmp/twist-proof-receipts research/quadratic_twist_evidence_20261021/lean
```

`#print axioms` appears for every reported declaration. `lean4checker` was not
available for an additional external replay. The first failed elaboration/build
and its source are retained under `../validation/proof-attempt-1/`; its provisional
axiom printouts are not proof receipts. The passing gate record is authoritative.
