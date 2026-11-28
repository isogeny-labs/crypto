# A wider xedni coefficient box admits eight more strict pairs

On the frozen 1,722-pair input law, expanding the rational coefficient multipliers from `kᵢ∈[−1,1]` to `kᵢ∈[−2,2]` increases strict two-prime admission from **12 to 20 ordered pairs**. The strict selected reduced-relation count remains two, so its exact conditional rate changes from `1/6` to `1/10`. Against the full-law rate `1/41`, these are enrichment factors `41/6` and `41/10` for the two specified budgets. The weak order-7 gate at 11 increases from four to twelve selected pairs; none of those twelve has the reduced relation.

| Coefficient multiplier bound | Constructed candidates | Strict passing choices / pairs | Strict reduced events | Weak passing choices / pairs | Weak reduced events |
| ---: | ---: | ---: | ---: | ---: | ---: |
| `K=1` | 45,360 | 12 / 12 | 2 | 4 / 4 | 0 |
| `K=2` | 210,000 | 20 / 20 | 2 | 12 / 12 | 0 |

The [protocol](protocol.json) freezes the same ordered-pair source, centered lift, Weierstrass reconstruction, simultaneous `#E(F₇)=4` and `#E(F₁₁)∈{6,7}` gates, and nested coefficient boxes before measurement. For different centered x-coordinates, `a₁,a₂,a₃=47(k₁,k₂,k₃)` and the two point equations determine `a₄,a₆`. The 42 same-x pairs remain explicit zero-choice rows. A budget's selected curve is the first passing triple in lexicographic order within that budget. All 210,000 maximum-budget choices are exact arithmetic checks, with both auxiliary-prime statuses accumulated even when a candidate fails an earlier gate.

The [result](result.json) retains source, protocol, code, raw-row, compressed-row, and complete candidate-stream digests. The [compressed per-pair rows](pair-rows.jsonl.gz) retain every pair's failure statuses, gate counts, selected coefficients, and rational and local relation flags. The [accounting audit](audit.py) verifies every row and restores the exact `K=1` counts and first selections from the prior full-law packet. It finds eight newly admitted pairs in each arm. A separate [checked Sage replay](sage_replay.py) reconstructs every selected `K=2` rational curve, recomputes its orders at 47, 7 and 11, and verifies the point sums for all 32 selected arm rows. Both [audit](audit-result.json) and [Sage replay](sage-replay-result.json) pass; the [runtime receipt](sage-runtime-info.json) precedes the Sage work and stderr is empty. This is an exact finite census, not a timing comparison.

The larger box raises admission but adds no reduced-event pair in this input law. The next curve-selection experiment should move from a centered cube to a frozen CRT construction over auxiliary primes, including coefficient-search effort and every failed candidate. A higher-field, higher-point family is needed to test the uniform rank and height assumptions of the existing Lean obstruction. This packet introduces no new mathematical theorem.

Replay from this checkout:

```sh
python3 research/exotic_ecdlp_20261021/xedni_coefficient_budget_20261120/run.py > research/exotic_ecdlp_20261021/xedni_coefficient_budget_20261120/result.json 2> research/exotic_ecdlp_20261021/xedni_coefficient_budget_20261120/run.stderr
python3 research/exotic_ecdlp_20261021/xedni_coefficient_budget_20261120/audit.py > research/exotic_ecdlp_20261021/xedni_coefficient_budget_20261120/audit-result.json
/Volumes/SSD990/cryptanalysis/sage --runtime-info > research/exotic_ecdlp_20261021/xedni_coefficient_budget_20261120/sage-runtime-info.json
/Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/xedni_coefficient_budget_20261120/sage_replay.py > research/exotic_ecdlp_20261021/xedni_coefficient_budget_20261120/sage-replay-result.json 2> research/exotic_ecdlp_20261021/xedni_coefficient_budget_20261120/sage-replay.stderr
```
