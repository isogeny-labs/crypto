# Actual characteristic-two and characteristic-three embedding controls

Lean checks every general Weierstrass coefficient tuple over F₂ and F₃:
32 and 243 tuples. For each nonsingular model satisfying the retained source
trace predicate, the actual point count belongs to {1,3,5} or {1,4,7},
respectively. Every prime divisor of that actual point count has modular
embedding degree at most six. No general trace classification or Hasse bound
is assumed in these two field cases. [pcc:unverified]

Two actual nonsingular curves directly disprove the unrestricted prime-field
exact-degree-two wording with subgroup primes greater than two:
[pcc:unverified]

| Actual curve | Field | Actual #E | Trace | Actual subgroup prime r | Modular k |
|:--|:--|--:|--:|--:|--:|
| y²+y=x³+x | F₂ | 5 | −2 | 5 | 4 |
| y²=x³−x+1 | F₃ | 7 | −3 | 7 | 6 |

The corrected prime-field degree-two result must retain its characteristic
restriction as well as its subgroup restriction. The earlier p≥5/r>2 proof
remains conditional on Hasse; these examples do not refute that qualified
statement. [pcc:unverified]

## Definitions and exact coverage

`sourceSupersingular` is the original FST finite-field trace definition:
`Int.gcd (p+1−Nat.card E.Point) p ≠ 1`. `embeddingDegree p r` is
`orderOf (p : ZMod r)`. This component does not silently replace it with the
field of rationality of the full r-torsion. Equivalence to a separate geometric
supersingularity definition, higher extension fields F₂ⁿ/F₃ⁿ with n>1, and the
full torsion/cyclotomic field bridge remain separate obligations.
[pcc:unverified]

The census covers all five coefficients a₁,a₂,a₃,a₄,a₆, not just short or
ordinary binary forms. Counts are model tuples, not isomorphism classes.
Singular models are excluded by the theorem's actual discriminant hypothesis.
A generic proved equivalence identifies the executable polynomial-equation
count with actual Mathlib nonsingular-point cardinality. Kernel computation
then proves the closed finite universal statements. This is pure theorem
construction, with no native enumeration, sampling, timing or paid compute.
[pcc:unverified]

## Proof map

| Source / declaration | Obligation |
|:--|:--|
| `PrimeFieldControls.E2_cardinality` / `E3_cardinality` | Complete counts of actual points on the two nonsingular curves |
| `PrimeFieldControls.E2_source_supersingular` / `E3_source_supersingular` | Exact retained trace predicate |
| `PrimeFieldControls.actual_prime_order_point` | Existing Cauchy theorem on the caller's actual point group, with equality choice explicit |
| `PrimeFieldControls.E2_order_five_point` / `E3_order_seven_point` | Actual prime-order subgroups |
| `PrimeFieldControls.E2_embedding_degree` / `E3_embedding_degree` | Exact orders four and six |
| `PrimeFieldControls.prime_field_degree_two_with_p_greater_two_false` | Direct universal negation, using actual E/F₃ and r=7 |
| `AllPrimeFieldModels.actual_cardinality_eq_counted` | Generic executable count to actual Mathlib point-cardinality equivalence |
| `AllPrimeFieldModels.all_binary_models` / `all_ternary_models` | Every coefficient tuple, checked by decide +kernel |
| `AllPrimeFieldModels.actual_binary_count_classification` / `actual_ternary_count_classification` | Actual count sets for all nonsingular source-supersingular models |
| `AllPrimeFieldModels.binary_prime_divisor_bound` / `ternary_prime_divisor_bound` | Bound six for every prime divisor of actual point count |
| `AllPrimeFieldModels.all_prime_field_models_audit_certificate` | All-model statements, explicit actual curve/subgroup witnesses and direct negation |

Namespaces are `SmallCharacteristicPrimeAudit` and
`SmallCharacteristicCensusAudit`. Whole gate: 28 public declarations,
zero failing and zero trusted-native exceptions. Lean 4.35.0 /
Mathlib 56a00585, standard axioms only. No holes, custom axioms,
native_decide exception, linter waiver or added elaboration setting.
Source commit `c1ca2f5f` precedes `PCC-20261110-2661fc`.
Full local verification passes at 10:10:52Z; signed standing remains unverified [pcc:unverified]

```sh
cd research/supersingular_small_characteristic_20261110/lean
lake build
cd ../../..
tools/lean/proof-gate.sh research/supersingular_small_characteristic_20261110/lean
```

See `sources.json`, `verification.json`, losslessly archived proof attempts,
and [the inspected report](reports/proof-20261110/report.pdf).
Sources checked: Freeman–Scott–Teske (2010) Definition 2.1/Remark 2.2,
pp 230–231, and trace-square list p 237; Waterhouse (1969) Theorem 4.1,
pp 536–537, with prior read/visual receipts retained in the census plans.
Actual Mathlib affine point equivalence, finite order and Cauchy declarations
were read. KB/Conductor unavailable; primary/repository/Mathlib fallback,
no novelty claim. The original fourteen-item request remains partial.
[pcc:unverified]
