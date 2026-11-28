# Rank-one replacements preserve both missing bridge words

The frozen 3×3 coefficient grid found **three rank-one rational curves for
each** of source-index 85 and 353. Every curve in the 18-cell grid preserves
the selected rational point relation `P+2Q=O` and reduces to the same
order-43 elliptic curve at 47. Simon two-descent completed on all 18 curves;
its six cells with upper bound one also returned exact rank one from a
proof-enabled PARI rank call. Two pinned Lean theorems prove the rational
group relation for the first rank-one curve in each row.

| Source index | Chosen `(u,v)` | Rational curve `[a₁,a₂,a₃,a₄,a₆]` | Simon bounds | Proof-enabled rank |
| ---: | --- | --- | --- | ---: |
| 85 | `(-1,0)` | `[-47,-47/9,0,-7999/9,1345/9]` | 1–1 | 1 |
| 353 | `(0,1)` | `[0,-16591/256,47,172221/128,-2025319/256]` | 1–1 | 1 |

The [protocol](protocol.json) freezes `a₁=47u`, `a₃=47v` for
`u,v∈{-1,0,1}` and derives the other coefficients from the tangent at `Q`.
It also fixes the method order, 25-second caps, and rank-one selection rule
before the run. [`cells.jsonl`](cells.jsonl) retains all 18 Sage results,
including the rank-three and unresolved-interval cells; the complete
rank-one list is in [`summary.json`](summary.json). The independent
[`audit.py`](audit.py) checks every rational equation, tangent and doubled
coordinate using exact fractions, verifies reduction at 47, method-gate
adherence, and exact-rank receipts. [`audit-result.json`](audit-result.json)
is `PASS` and the checked Sage runtime is retained.

## Proof map and scope

| Result | Declaration | Exact scope | Status |
| --- | --- | --- | --- |
| Index-85 selected curve has `P+2Q=O` | `XedniRankOneReplacements.rational_relation_85` | Displayed curve over `ℚ`, `P=(5,20)`, `Q=(8,19)` | Lean checked |
| Index-353 selected curve has `P+2Q=O` | `XedniRankOneReplacements.rational_relation_353` | Displayed curve over `ℚ`, `P=(11,7)`, `Q=(27,14)` | Lean checked |
| Selected curves have rational rank one | — | Displayed curves over `ℚ` | Completed proof-enabled Sage rank and exact Simon bounds; Lean formalization open |
| The six selected reduced rows have rank 20 | — | Fixed order-43 reduction | Sage rank audit in [bridge packet](../xedni_word2_word6_bridge_20261121/README.md); Lean matrix-rank bridge open |

[`formal/Replacements.lean`](formal/Replacements.lean) contains the two
rational group-law proofs. Its pinned build checks the actual module and
records `#print axioms`: only standard `propext`, `Classical.choice`, and
`Quot.sound`. Formalizing the rational rank certificates and reduction maps
in Lean requires definitions and proof infrastructure not yet present here.

This grid supplies rank-one realizations for five of the six bridge rows
when combined with source indices 2, 201, and 511 from the preceding rank
panel. Source index 425 remains the sixth row; its earlier selected rational
curve has an unresolved rank interval. The next exact search is to vary its
free coefficients under the same frozen reduction and relation law.

Replay from the repository root:

```sh
/Volumes/SSD990/cryptanalysis/sage --runtime-info
python3 research/exotic_ecdlp_20261021/xedni_bridge_rank1_replacements_20261121/run.py
python3 research/exotic_ecdlp_20261021/xedni_bridge_rank1_replacements_20261121/audit.py
sh research/exotic_ecdlp_20261021/xedni_bridge_rank1_replacements_20261121/formal/build.sh
```
