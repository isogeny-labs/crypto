# Summary (coded rules, PROTOCOL.md)

- **O1** (criterion ⇒ constant): 1287 criterion lines, 0 not constant: **PASS**
- **O2** (⇐ where Weil decides: (101,2), (101,3)): 3520 lines failing the criterion, 0 constant: **PASS**
- **O3** (descent lines: criterion, σ^(n/2)-stable, F ∈ K₀[t], all-lift iff χ_K(u³) = 1): 1280 lines, 0 exceptions: **PASS**
- **O4** (dead set predicted exactly; pooled fraction in [0.35, 0.65]): 520 decidable lines, 405 mismatches, fraction rule ok = true: **FAIL**
- **O5** (n·r/s integer, Σ = 3n): 0 exceptions: **PASS**
- Descent curves: 32 (j ∈ K₀ on 32; order check failed on 0; ℓ on both sides on 3). Generic curves with j ∈ K₀ (n even): 1. σ^(n/2)-stable root sets among random lines: 7.

## Post hoc: direction of the O4 mismatches

| p | n | mismatches with predicted dead, observed alive | observed dead, predicted alive | expected (l-side x)/l^v |
|--:|--:|--:|--:|--:|
| 13 | 2 | 0 | 80 | 224.0 |
| 13 | 4 | 0 | 15 | 22.9 |
| 101 | 2 | 0 | 240 | 332.7 |
| 101 | 4 | 0 | 70 | 68.0 |

## Criterion vs constancy per cell

| p | n | Weil decides | lines | criterion true | constant | criterion false but constant | criterion true, not constant |
|--:|--:|:--|--:|--:|--:|--:|--:|
| 13 | 2 | false | 2240 | 327 | 327 | 0 | 0 |
| 13 | 3 | false | 1600 | 0 | 1 | 1 | 0 |
| 13 | 4 | false | 2240 | 320 | 320 | 0 | 0 |
| 101 | 2 | true | 2240 | 320 | 320 | 0 | 0 |
| 101 | 3 | true | 1600 | 0 | 0 | 0 | 0 |
| 101 | 4 | false | 2240 | 320 | 320 | 0 | 0 |

## Dead points on all-lift descent lines

| p | n | decidable lines | lifted x | dead observed | dead predicted | mismatches | dead fraction |
|--:|--:|--:|--:|--:|--:|--:|--:|
| 13 | 2 | 160 | 2080 | 1280 | 1200 | 80 | 0.615 |
| 13 | 4 | 120 | 1560 | 840 | 825 | 15 | 0.538 |
| 101 | 2 | 120 | 12120 | 7160 | 6920 | 240 | 0.591 |
| 101 | 4 | 120 | 12120 | 6116 | 6046 | 70 | 0.505 |

