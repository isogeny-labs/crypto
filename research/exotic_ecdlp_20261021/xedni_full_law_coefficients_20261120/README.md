# Full-law coefficient census admits 12 strict two-prime pairs

The frozen 27-choice coefficient rule admits **12 of 1,722 ordered point pairs** through the simultaneous auxiliary gates `#E(F₇)=4` and `#E(F₁₁)=6`. Two of the 12 have the target relation `P₁+2P₂=O` on the fixed order-43 group over `F₄₇`. The exact conditional reduced-event rate is `2/12=1/6`, compared with `42/1722=1/41` in the full input law, an enrichment factor of `41/6` under this finite selection rule. The selected rational lifts have zero such relations; for each of the two reduced-event pairs, reduction at both 7 and 11 gives a concrete nonidentity certificate.

| Gate on each reconstructed curve | Ordered pairs in full law | Constructed choices | Passing choices | Selected pairs | Reduced events among selected | Rational events among selected |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| `#E(F₇)=4`, `#E(F₁₁)=6` | 1,722 | 45,360 | 12 | 12 | 2 | 0 |
| `#E(F₇)=4`, `#E(F₁₁)=7` | 1,722 | 45,360 | 4 | 4 | 0 | 0 |

The [protocol](protocol.json) fixes all 1,722 distinct nonidentity ordered pairs on `E(F₄₇): y²=x³−x−2`, centered coordinate lifts, and `a₁,a₂,a₃=47(k₁,k₂,k₃)` for `kᵢ∈{−1,0,1}`. For each pair with different lifted x-coordinates, the two point equations uniquely determine `a₄,a₆`. The 42 same-x pairs are retained as unconstructed rows. For each of the other 1,680 pairs, all 27 choices are checked for integrality, good reduction, two-row incidence rank, and prescribed curve order at 7 and 11. The first lexicographic passing choice is selected separately for each order arm. All coefficients reduce to the source curve at 47. The two reduced-event strict selections are source pair indices 353 and 395, corresponding to prior 20-pair panel indices 2 and 7.

The [per-pair rows](pair-rows.jsonl.gz) preserve 1,722 source statuses, construction counts, failed-prime statuses, passing-choice counts, chosen coefficients, and relation flags. The gzip stream losslessly restores the exact raw SHA-256 in [result metadata](result.json), which also fixes the full candidate-stream digest and aggregate counts. The arithmetic uses Python exact fractions and finite-field enumeration; the [audit](audit.py) verifies every pair count, result hash, and exact containment of the previous 20-pair screen. A separate [checked Sage replay](sage_replay.py) reconstructs all 16 selected arm rows, checks their rational and finite-field point sums, and independently recomputes orders at 47, 7 and 11. Both [audit](audit-result.json) and [Sage replay](sage-replay-result.json) passed, with empty stderr. The [Sage runtime receipt](sage-runtime-info.json) was saved before replay. This is an exact finite census, not a wall-time comparison.

The finite `{-1,0,1}³` coefficient box is one explicit selection rule. The full CRT coefficient-choice space and the larger-field, higher-point lift family remain separate questions. A next experiment should freeze a wider coefficient budget or a CRT construction cost cap before inspection, then carry the same source law and local relation certificates. To test the checked Lean height obstruction against selected rational curves, the curve-selection law must also supply rank and canonical-height envelopes across its full support.

Replay from this checkout:

```sh
python3 research/exotic_ecdlp_20261021/xedni_full_law_coefficients_20261120/run.py > research/exotic_ecdlp_20261021/xedni_full_law_coefficients_20261120/result.json 2> research/exotic_ecdlp_20261021/xedni_full_law_coefficients_20261120/run.stderr
python3 research/exotic_ecdlp_20261021/xedni_coefficient_choice_20261120/package_rows.py verify
python3 research/exotic_ecdlp_20261021/xedni_full_law_coefficients_20261120/audit.py > research/exotic_ecdlp_20261021/xedni_full_law_coefficients_20261120/audit-result.json
/Volumes/SSD990/cryptanalysis/sage --runtime-info > research/exotic_ecdlp_20261021/xedni_full_law_coefficients_20261120/sage-runtime-info.json
/Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/xedni_full_law_coefficients_20261120/sage_replay.py > research/exotic_ecdlp_20261021/xedni_full_law_coefficients_20261120/sage-replay-result.json 2> research/exotic_ecdlp_20261021/xedni_full_law_coefficients_20261120/sage-replay.stderr
```

This packet introduces no new mathematical theorem; the Lean obstruction and its explicit arithmetic hypotheses are recorded in the adjacent formalization packets.
