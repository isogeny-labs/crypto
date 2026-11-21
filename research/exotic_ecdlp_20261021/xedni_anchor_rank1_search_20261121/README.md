# Six rank-one rational curves realize a full-rank reduced word witness

The final source-index-425 coefficient grid found **two exact rank-one
rational curves** carrying `P=(14,7)`, `Q=(17,10)` and `P+2Q=O`. The first is

`E/ℚ: y² = x³ − 47x² + 751x − 3997`.

Together with the five selected curves from the preceding packets, an
independent checked-Sage replay verifies **six distinct rational curves of
rank one**, all reducing to `y²=x³−x−2` over `F₄₇`. Their six coefficient-two
rows raise the coefficient-six matrix rank from 14 to 20 on 21 sign-folded
columns. This is an exact heterogeneous-curve realization of the six-row
algebraic witness.

| Source index | Rational rank | Matrix rank after row | Curve source |
| ---: | ---: | ---: | --- |
| 425 | 1 | 15 | this packet, `(u,v)=(0,0)` |
| 2 | 1 | 16 | direct-lift rank panel |
| 85 | 1 | 17 | rank-one replacement grid |
| 201 | 1 | 18 | direct-lift rank panel |
| 353 | 1 | 19 | rank-one replacement grid |
| 511 | 1 | 20 | direct-lift rank panel |

The [nine-cell protocol](protocol.json) froze `a₁=47u`, `a₃=47v` for
`u,v∈{-1,0,1}`, tangent reconstruction, method gates and caps before the
search. [`cells.jsonl`](cells.jsonl) preserves every Simon two-descent
result, including rank-two, rank-three and rank-four curves. Two cells have
exact 1–1 Simon bounds and proof-enabled PARI rank one; the
[`summary`](summary.json) lists both. The independent exact-fraction
[`audit`](audit-result.json) checks all nine tangent constructions and rank
records.

The separately frozen [six-curve replay protocol](final-audit-protocol.json)
selects one rank-one curve per row and fixes the prior packet hashes. Its
[`final-audit-result.json`](final-audit-result.json) records six proof-enabled
rank-one computations, six rational `P+2Q` checks, common good reduction,
the order-43 reduction, and the rank sequence `15,16,17,18,19,20` rebuilt
directly in Sage. The six curves have six distinct rational `j`-invariants.
The checked Sage runtime receipt and stderr are retained.

## Lean proof map

| Result | Declaration | Exact scope | Status |
| --- | --- | --- | --- |
| Index-425 selected curve has `P+2Q=O` | `XedniRankOneAnchor.rational_relation` | Displayed `E/ℚ`, `P=(14,7)`, `Q=(17,10)` | Lean checked |
| Other five selected rational relations | Declarations in the [direct-lift](../xedni_direct_word2_lifts_20261121/README.md) and [replacement](../xedni_bridge_rank1_replacements_20261121/README.md) proof maps | Their five displayed curves and point pairs over `ℚ` | Lean checked |
| All six curves have rank one | — | Six displayed curves over `ℚ` | Proof-enabled Sage exact ranks, corroborated by Simon bounds for each; Lean rank formalization open |
| Six reduced rows have rank 20 | — | `E(F₄₇)` and 21 sign-folded columns | Independent Sage matrix replay; Lean matrix-rank bridge open |

The pinned [`formal/Anchor.lean`](formal/Anchor.lean) proves the exact
index-425 group relation without a research-specific axiom; retained
`#print axioms` output lists only `propext`, `Classical.choice`, and
`Quot.sound`.

## Algorithmic boundary

The six rational curves differ; the original fixed-lift rank premise is a
statement about **one** rational curve. This heterogeneous construction
therefore does not supply a fixed-curve xedni lift. More directly, each
point pair was selected from the known finite-field equation `P+2Q=O`.
Generating `P` as `−2Q` needs only public group arithmetic, and the lifted
relation then certifies that same equality. The rank-one condition and
matrix span are genuine, but neither measures discovery of relations for a
previously unseen target nor a target-dependent scalar-recovery cost.

The next falsifier is a fixed-rational-curve test: seek two independent
selected bridge words on one curve under a frozen coefficient and coordinate
law, record the curve's exact rank and relation-row gain, then extend only
if that gain survives. A target study must fix the public target before
searching and charge relation discovery through verified recovery against
same-point rho.

Replay from the repository root:

```sh
/Volumes/SSD990/cryptanalysis/sage --runtime-info
python3 research/exotic_ecdlp_20261021/xedni_anchor_rank1_search_20261121/run.py
python3 research/exotic_ecdlp_20261021/xedni_anchor_rank1_search_20261121/audit.py
gtimeout 180 /Volumes/SSD990/cryptanalysis/sage -python "$(pwd)/research/exotic_ecdlp_20261021/xedni_anchor_rank1_search_20261121/final_audit_sage.py"
sh research/exotic_ecdlp_20261021/xedni_anchor_rank1_search_20261121/formal/build.sh
```
