# Summary (coded rules, PROTOCOL.md)

- **C1** (n = 4: exactly 6 lines on ≥ 90 % of curves, none > 6; k = n/2 gives 0 on ≥ 90 %): 31/32 with 6, 1 over 6, 32/32 with 0 from k = n/2: **FAIL**
- **C2** (criterion, lone root in K₀, pair conjugate, constant, not σ^(n/2)-stable; j ∉ K₀): 877 lines, 0 exceptions, 0 curves with j ∈ K₀: **PASS**
- **C3** (n = 2: 0 lines): 202 lines: **FAIL**
- **C4** (dead ≤ E + 3√E + 3 per cell): **PASS**
- **C5 / C5+** (collision pairs ≤ E₂ + 3√E₂ + 3; planted control exact): **PASS**
- Orders: BSGS failures or [#E]P ≠ O on 0 curves; BSGS ≠ enumeration on 0. All-lift constructed lines: 449 of 877.

## Lines per curve (k ≠ n/2; and from k = n/2)

| p | n | lines per curve | k = n/2 lines per curve |
|--:|--:|:--|:--|
| 13 | 2 | [0, 0, 0, 0, 0, 0, 0, 0] | [0, 0, 0, 0, 0, 0, 0, 0] |
| 101 | 2 | [0, 0, 0, 0, 0, 0, 0, 0] | [0, 0, 0, 202, 0, 0, 0, 0] |
| 13 | 4 | [6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 19, 6, 6, 6, 6, 6] | [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] |
| 101 | 4 | [6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6] | [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] |
| 13 | 6 | [168, 168, 168, 168] | [0, 0, 0, 0] |

## Dead points and column collisions (n = 4, all-lift lines)

| p | n | all-lift lines | lifted x | dead | E dead | collision pairs | E pairs | planted control |
|--:|--:|--:|--:|--:|--:|--:|--:|:--|
| 13 | 4 | 53 | 689 | 10 | 7.59 | 75 | 81.05 | ok |
| 101 | 4 | 42 | 4242 | 3 | 1.17 | 101 | 116.21 | ok |

