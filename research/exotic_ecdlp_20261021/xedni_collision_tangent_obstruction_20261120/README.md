# Tangent divisibility closes the auxiliary-x-collision source stratum

For the frozen 336 ordered source pairs with distinct rational x coordinates and an auxiliary x collision, the specified `P + 2Q = O` rational relation is excluded throughout the rational elliptic-curve family with **integral `a1,a2` and prescribed good reduction to `y²=x³−x−2` at 47**. The fixed-prime group law excludes 328 pairs. The other eight pass that necessary finite-field relation, but each has `x₁−x₂ ∤ (y₁−y₂)²`; a checked Lean theorem derives this divisibility as a necessary consequence of the rational point-group relation for any elliptic Weierstrass curve through the two points with integral `a1,a2`. The exact 336-row census and an independent Sage replay agree on every case.

| Source indices | `x₁−x₂` | `y₁−y₂` | `(y₁−y₂)² mod |x₁−x₂|` | Obstruction |
| --- | ---: | ---: | ---: | --- |
| 273, 315 | 28 | −22, 22 | 8 | Tangent divisibility |
| 663, 703 | 7 | 11, −11 | 2 | Tangent divisibility |
| 1160, 1202 | −33 | 7, −7 | 16 | Tangent divisibility |
| 1500, 1542 | 11 | 10, −10 | 1 | Tangent divisibility |
| Remaining 328 pairs | varies | varies | recorded per row | Fixed-prime reduction |

The six relation-eligible pairs admitted by the auxiliary local gates are among these eight. The earlier 16,200-curve height-box census found zero rational relations; this theorem supplies an arithmetic obstruction across **all integral `a1,a2` choices** for those eight exact point pairs, independent of the height of `a3,a4,a6`. The two remaining finite-field-positive pairs, indices 663 and 703, are covered by the same theorem even though their auxiliary gate was empty.

## Mathematical proof

Write the two integer points as `P=(x₁,y₁)` and `Q=(x₂,y₂)` on a nonsingular rational Weierstrass equation

`y² + a1·x·y + a3·y = x³ + a2·x² + a4·x + a6`.

If `P+2Q=O`, then `2Q=−P`, so the tangent at `Q` is nonvertical and its third intersection with the cubic is `P`. Let `D=x₁−x₂` and `Y=y₁−y₂`. The tangent slope is `Y/D` and also satisfies

`slope·(2y₂+a1x₂+a3) = 3x₂²+2a2x₂+a4−a1y₂`.

Subtracting the two point-incidence equations and eliminating the tangent slope and `a4` yields the exact identity

`D²(a2+x₁+2x₂) − a1·D·Y − Y² = 0`.

When `a1,a2,x₁,x₂,y₁,y₂` are integers, the first two terms are multiples of `D`, so `D ∣ Y²`. The four nonzero residues in the table refute that necessary condition for all eight ordered cases. The proof uses the full Mathlib affine point group law: it derives `2Q=−P`, rules out a vertical tangent, extracts the doubled point's x and y coordinates, obtains the tangent line and derivative equations, and then proves the integer divisibility consequence.

For the other 328 pairs, a rational relation would reduce to the same relation on the fixed good-reduction curve over `F₄₇`. Their finite-field sums are nonidentity. The [previous Sage audit](../xedni_collision_reduction_screen_20261120/README.md) checked the model reduction, exact point incidence, and denominator units for all 18,576 centered constructed curves. The present classification applies the standard good-reduction homomorphism to every curve in the declared family satisfying those same reduction conditions.

## Proof map and evidence

| Result | Lean declaration or receipt | Exact assumptions | Status |
| --- | --- | --- | --- |
| Tangent algebra | `XedniCollisionTangent.obstruction_of_tangent` | Two rational point-incidence equations, tangent line through `P,Q`, derivative slope equation | Lean checked |
| Point-group to tangent | `XedniCollisionTangent.obstruction_of_point_relation` | Elliptic `W/ℚ`; nonsingular affine points `P,Q`; `P+Q+Q=0` | Lean checked |
| Integer necessity | `XedniCollisionTangent.no_relation_of_nondvd` | Integral `a1,a2` and point coordinates; `D ∤ Y²` | Lean checked |
| Eight exact ordered pairs | `XedniCollisionTangent.no_relation_for_eight_source_pairs` | Explicit `Fin 8` coordinate table; any elliptic `W/ℚ` with integral `a1,a2` containing its two points | Lean checked |
| Remaining 328 pairs | `XedniCollisionReduction.no_relation_of_reduced_sum` in the preceding packet | Good-reduction homomorphism to fixed `E/F₄₇`, exact point reduction, nonidentity reduced sum | Generic Lean implication checked; concrete map instantiation open |
| Complete input classification | [result](result.json) and [Sage audit](sage-audit-result.json) | Frozen 1,722-row source; 336 selected collision pairs; exact `D,Y` and finite-field sums | 328 reduction, 8 divisibility, audit PASS |

The [formal build](formal/build.sh) checks pinned Lean `v4.35.0-rc4` and Mathlib revision `021c46bdd042f67b538b9f9871d556a93713a9a9`. Its [output](formal/build-output.txt) lists only `propext`, `Classical.choice`, and `Quot.sound` for every declaration, with empty stderr; the module has no `sorry`, `admit`, custom claim axiom, or native evaluation. `lean4checker --fresh` is unavailable on this host. The eight-case point-level theorem is fully checked. Formalizing the concrete rational-curve reduction map for the 328 other cases remains a separate bridge; their finite-field arithmetic and the required model reductions have independent Sage receipts.

The [protocol](protocol.json), [census runner](census.py), and Lean source were pushed before census execution as `b68cfe5f5d2`. Selection uses the independently computed fixed-prime relation bit from the preceding packet, whose result SHA-256 is frozen in the protocol. The [result](result.json) retains every one of the 336 classifications and exact arithmetic residues. The [Sage replay](sage_audit.py) independently recomputed all finite-field sums and integer residues; its [runtime receipt](sage-runtime-info.json) records the checked Sage build.

## Replay and next proof obligation

```sh
python3 research/exotic_ecdlp_20261021/xedni_collision_tangent_obstruction_20261120/census.py > research/exotic_ecdlp_20261021/xedni_collision_tangent_obstruction_20261120/result.json
/Volumes/SSD990/cryptanalysis/sage --runtime-info > research/exotic_ecdlp_20261021/xedni_collision_tangent_obstruction_20261120/sage-runtime-info.json
/Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/xedni_collision_tangent_obstruction_20261120/sage_audit.py > research/exotic_ecdlp_20261021/xedni_collision_tangent_obstruction_20261120/sage-audit-result.json
sh research/exotic_ecdlp_20261021/xedni_collision_tangent_obstruction_20261120/formal/build.sh
```

The next mathematical check is the source law with **distinct x at both auxiliary primes**, where the preceding CRT census recorded eight rational relations. For this collision stratum, changing the rational point lifts, permitting nonintegral `a1,a2` under explicit local-integrality rules, or changing the relation word defines a new construction with its own proof and cost obligations.
