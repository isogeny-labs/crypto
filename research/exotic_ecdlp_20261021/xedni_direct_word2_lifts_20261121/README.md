# Direct tangent construction realizes all five rank bridges over ℚ

Five rational elliptic curves constructed from the selected coefficient-two
point pairs each reduce to `E/F_47: y²=x³−x−2` and each carry the exact
rational relation `P+2Q=O`. With the existing source-index-425 relation,
their six reduced rows raise the 21-column coefficient-six word matrix from
rank 14 to rank 20. The five new rational relations are checked by Sage, an
independent exact-fraction audit, and five Lean 4 group-law theorems.

| Source index | P | Q | Rational curve `[a₁,a₂,a₃,a₄,a₆]` | Max coefficient bits |
| ---: | --- | --- | --- | ---: |
| 2 | (2,2) | (5,27) | `[0,517/9,0,-1795/9,1486/9]` | 11 |
| 85 | (5,20) | (8,19) | `[0,-188/9,0,1166/9,1345/9]` | 11 |
| 201 | (8,19) | (43,19) | `[0,-94,0,2537,-14431]` | 14 |
| 353 | (11,7) | (27,14) | `[0,-16591/256,0,169589/128,-2051639/256]` | 21 |
| 511 | (15,16) | (23,4) | `[0,-235/4,0,2207/2,-25811/4]` | 15 |

The frozen [`protocol.json`](protocol.json) takes the five finite-field pairs
from the rank-bridge packet and uses their least nonnegative coordinate
representatives. For each pair, let
`λ=(yP−yQ)/(xP−xQ)`, choose `a₁=a₃=0`, and set

```text
a₂ = λ² − 2xQ − xP
a₄ = 2yQ λ − 3xQ² − 2a₂ xQ
a₆ = yQ² − xQ³ − a₂ xQ² − a₄ xQ.
```

The tangent at `Q` has slope `λ`, and duplication gives `2Q=−P`.
The coefficient reductions are `[0,0,0,−1,−2]` at 47; each constructed
curve has nonzero reduced discriminant and the reduction has order 43.
[`result.json`](result.json) records every coefficient, point, status, and
the Sage folded-rank sequence `15,16,17,18,19,20`.
[`audit-result.json`](audit-result.json) independently checks the rational
curve equations, tangent, doubled coordinates, and coefficient reductions
using Python exact fractions.

## Proof map

| Mathematical result | Lean declaration | Exact scope | Status |
| --- | --- | --- | --- |
| Source-index-2 rational relation | `rational_relation_2` | Displayed `E₂/ℚ`, `P=(2,2)`, `Q=(5,27)` | Checked |
| Source-index-85 rational relation | `rational_relation_85` | Displayed `E₈₅/ℚ`, `P=(5,20)`, `Q=(8,19)` | Checked |
| Source-index-201 rational relation | `rational_relation_201` | Displayed `E₂₀₁/ℚ`, `P=(8,19)`, `Q=(43,19)` | Checked |
| Source-index-353 rational relation | `rational_relation_353` | Displayed `E₃₅₃/ℚ`, `P=(11,7)`, `Q=(27,14)` | Checked |
| Source-index-511 rational relation | `rational_relation_511` | Displayed `E₅₁₁/ℚ`, `P=(15,16)`, `Q=(23,4)` | Checked |
| Shared reduction, order 43, and rank sequence | — | The five displayed rational curves, fixed reduction at 47, 21 folded columns | Sage and exact-fraction audited; Lean bridge open |

[`formal/Lifts.lean`](formal/Lifts.lean) constructs each rational curve and
proves `P+2•Q=0` in Mathlib's affine elliptic-curve point group. The pinned
[`formal/build.sh`](formal/build.sh) checks the toolchain and dependency
manifest and compiles the theorem module. The retained `#print axioms`
output lists only `propext`, `Classical.choice`, and `Quot.sound`.

## Input dependency and next cost experiment

This construction takes pairs already known to satisfy `P+2Q=O` in the
finite group. Creating each `P` from a chosen `Q` by scalar multiplication
is therefore easy, and the resulting rational relation reflects that known
equation. The measured contribution here is exact rational realizability
and six-row span on this fixed subgroup. To test a point-decomposition
algorithm, the next experiment must choose previously unseen target points
under a frozen input law, account for discovering useful relations without
prescribing `P=−2Q`, and charge all target-dependent work through verified
scalar recovery against same-point rho.

Reproduce from the repository root:

```sh
/Volumes/SSD990/cryptanalysis/sage --runtime-info
/Volumes/SSD990/cryptanalysis/sage -python "$(pwd)/research/exotic_ecdlp_20261021/xedni_direct_word2_lifts_20261121/run_sage.py"
python3 research/exotic_ecdlp_20261021/xedni_direct_word2_lifts_20261121/audit.py
sh research/exotic_ecdlp_20261021/xedni_direct_word2_lifts_20261121/formal/build.sh
```
