# Proof map

Lean 4.34.1 and Mathlib commit `d13f23b723b8a846827a245b89c10fc7d3f11612`
are pinned in the toolchain, Lake file, and dependency manifest. The repository
proof gate builds the module and audits all transitive theorem axioms. The current
latest receipt reports **18 theorems, zero trusted native declarations**; only standard
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
They derive the plus-eigenspace property of rational lifts. The minus property
of B remains an explicit hypothesis, rather than an assumed twist theorem.

**Remaining bridge.** Construct the actual twist-point embedding and prove its
Frobenius negation property, identify the full eigenspaces with the required
rational point sets, formalize
S3/S4 lifting including exceptional cases, or prove Weil-pairing nondegeneracy
and the complete DLP reduction. Its abstract implications are fully proved;
the complete historical geometric theorems are only partially formalized.
Finite enumerations check the original fixtures, not those universal bridges.
The trace-zero counting theorem and general GHS construction likewise remain open.
The [latest gate receipt](../validation/lean-bridge-gate/) includes both modules;
the earlier 14-theorem gate and its exact Lake configuration are retained.

Reproduce from the repository root:

```sh
tools/lean/proof-gate.sh --out /tmp/twist-proof-receipts research/quadratic_twist_evidence_20261021/lean
```

`#print axioms` appears for every reported declaration. `lean4checker` was not
available for an additional external replay. The first failed elaboration/build
and its source are retained under `../validation/proof-attempt-1/`; its provisional
axiom printouts are not proof receipts. The passing gate record is authoritative.
