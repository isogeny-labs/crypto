# The shifted rational curves contribute one subgroup relation row

The **264 independently verified rational curves** in the positive `k3`-shift family reduce to only two sign-mirror point-pair rows on the fixed order-43 group `E(F₄₇): y²=x³−x−2`. With all 42 nonidentity group points as a factor base, sign folding gives 21 usable columns. The complete 264×21 relation matrix has **rank 1 modulo 43**, touches two columns, and leaves 19 columns absent. An independent accepted-Sage replay agrees with the exact Python group and matrix calculation.

| Diagnostic | Exact result |
| --- | ---: |
| Rational curves and verified relation records | 264 |
| Distinct rational `j`-invariants in those curves | 132 |
| Reduced point-pair rows before sign folding | 2, each repeated 132 times |
| Sign-folded factor-base columns | 21 |
| Distinct signed folded rows | 2, negatives of one another |
| Matrix rank over `Z/43Z` | 1 |
| Touched / untouched columns | 2 / 19 |

The two reduced rows are `P=(14,7), Q=(17,10)` and their negatives `-P=(14,40), -Q=(17,37)`. Their coefficient vectors in the sign-folded basis are `e₅+2e₈` and `−e₅−2e₈`, where the column order is recorded in [result.json](result.json). The 132 distinct rational isomorphism classes therefore supply the same one-dimensional relation span on this fixed subgroup. Rows alone contain no coefficient for any of the 38 nonidentity points outside these two sign orbits; a target-handling method for those points requires additional relation sources or descent.

## Method and proof map

The [protocol](protocol.json) freezes the 264-row source hash, exact field and curve model, sign-orbit rule, full 42-point base, row word, modulus, and rank boundary. The [runner](run.py) enumerates all `F₄₇` points, checks the group order and every reduced `P+2Q=O` relation, records every duplicate row in [compressed rows](rows.jsonl.gz), and performs modular elimination over 43. The [accepted Sage audit](sage_audit.py) independently constructs the curve and point group, verifies the source order and both reduced point pairs, recomputes all 264 folded rows and Sage's matrix rank, and returns [PASS](sage-audit-result.json). Its [runtime receipt](sage-runtime-info.json) identifies the checked Sage build; stderr is empty.

| Result | Lean declaration or receipt | Exact scope | Status |
| --- | --- | --- | --- |
| Sign mirror | `XedniRelationRank.mirror_row` | Explicit 264×21 matrix with first 132 rows `e₅+2e₈` and last 132 their negatives over `ZMod 43` | Lean checked |
| Row-space structure | `XedniRelationRank.relationMatrix_row_span`, `baseRow_ne_zero` | Entire row span equals the span of a nonzero base row | Lean checked |
| Missing-column obstruction | `XedniRelationRank.uncovered_column`, `row_span_not_full` | Every column other than 5 and 8 is zero in all 264 rows; the row span is a proper subspace of the 21-column space | Lean checked |
| Numeric matrix rank | [result](result.json), [Sage audit](sage-audit-result.json) | All 264 frozen rows, 21 columns, arithmetic modulo 43 | Rank 1 independently replayed |
| Direct Lean `Matrix.rank = 1` declaration | Standard `Matrix.rank_eq_finrank_span_row` and `finrank_span_singleton` composition | Same explicit matrix | Formalization partial: elaboration exceeded 200,000 heartbeats and a subsequent uncapped attempt exceeded two minutes at about 3 GB RSS; the row-span equality and nonzero theorem build cleanly |

The [pinned Lean build](formal/build.sh) uses `v4.35.0-rc4` and Mathlib revision `021c46bdd042f67b538b9f9871d556a93713a9a9`. Its [output](formal/build-output.txt) reports only `propext` and `Quot.sound` for the checked declarations. There is no `sorry`, `admit`, custom research axiom, or native evaluation in the reported modules. The source-table-to-matrix bridge is independently checked by Sage, including the first-132/last-132 order. `lean4checker --fresh` is unavailable on this host.

This is a relation-rank diagnostic for the frozen two-pair construction, not a timing comparison or complete IC candidate. The next experiment should change the source point law or relation word, record the number of novel sign-folded rows and target coverage per query, and stop if the rank gain remains too small for target recovery on the declared one-target workload.

## Replay

```sh
python3 research/exotic_ecdlp_20261021/xedni_relation_rank_20261120/run.py > research/exotic_ecdlp_20261021/xedni_relation_rank_20261120/result.json
/Volumes/SSD990/cryptanalysis/sage --runtime-info > research/exotic_ecdlp_20261021/xedni_relation_rank_20261120/sage-runtime-info.json
/Volumes/SSD990/cryptanalysis/sage -python /Volumes/SSD990-2/crypto-worktrees/exotic-ecdlp-recovery-20261119/research/exotic_ecdlp_20261021/xedni_relation_rank_20261120/sage_audit.py > research/exotic_ecdlp_20261021/xedni_relation_rank_20261120/sage-audit-result.json
sh research/exotic_ecdlp_20261021/xedni_relation_rank_20261120/formal/build.sh
```
