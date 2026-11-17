# P-192 third residual-pivot projected-column and support certificate

**Result.** A third complete residual-pivot fan contributes 5,026 split
line cuts and 10,051 distinct exact and sign-folded subgroup columns.
Its projected columns have no overlap with either earlier residual-pivot
fan. The only overlap with the complete 141-pivot old family is its
original source-line partner. Across all three residual-pivot fans, every
one of the 15,028 non-source rows has two private sign-folded columns;
private-column elimination leaves an empty core. The selected points plus
all three fans contain 39,984 distinct exact quotient classes. Their
unordered three-sum support is at most 10,654,671,159,440 targets, giving
an ideal uniform accepted-query hit probability below \(2^{-146}\).

| Frozen P-192 three-fan quantity | Count |
| --- | ---: |
| Selected quartic and quotient classes | 9,925 |
| New pivot 002 split / nonsplit cuts | 5,026 / 4,899 |
| Three-fan split rows / source-line restatements | 15,034 / 6 |
| Non-source rows with two private signed columns | 15,028 |
| Signed core rows after private-column peeling | 0 |
| Selected-plus-three-fan exact and signed union | 39,984 |
| Unordered three-sum capacity | 10,654,671,159,440 |

The [producer](../../line_residual_pivot_cross_fan.py) uses the checked
Sage launcher to project both rational residual points for every split
line from pivot 002. It computes the first projected class by the
three-step Frobenius trace identity, derives the second from the common
line-section sum, and checks both by direct six-step projection at the
first and every 256th split row. The [runtime receipt](../../type_i_residual_pivot002_cross_runtime_info.json)
was saved before the run. The [fan receipt](../../type_i_residual_pivot002_cross.json)
retains all exact geometry and quotient coordinates, input and source
hashes, 20 spaced direct controls, and complete old-family and pivot 001
key comparisons. It reports zero cross-fan exact or sign-folded hits;
the sole old-family hit is `[2,0,1,[1,1]]`, the source-line partner.

The [independent verifier](../../verify_residual_pivot002_cross.py)
reconstructs every quartic line polynomial from four direct evaluations,
checks all residual coordinates against the quartic, checks each projected
point against the P-192 curve equation, and replays the full group sum
`pivot + selected + residual_1 + residual_2 = S` in prime-field integer
arithmetic. It independently scans all 1,360,948 old-family columns and
both previous complete pivot fans, then peels nonselected signed columns
from the three-fan matrix. Its [receipt](../../type_i_residual_pivot002_cross_replay.json)
finds no overlap with pivot 000 or 001, all 15,028 non-source rows with
two private columns, and zero residual core rows. The existing Lean
`line_private_pairs_extend` theorem in [LineSupport.lean](../../lean/LineSupport.lean)
proves that this private-column condition lets every assignment of the
selected-class logarithms extend to the line equations. The finite-field
distinctness and source-line exceptions are the hypotheses discharged by
the complete producer and independent integer replay.

The [sorted-list union replay](../../verify_three_pivot_union.py) builds the
39,984-class set without using the verifier's hash sets. Its
[receipt](../../type_i_three_pivot_union_replay.json) adds 9,925 selected
classes, then 10,071, 9,939, and 10,049 new classes from the three
fans, with two selected-class overlaps and one within-fan repeat in
each fan. The resulting exact capacity is
\(\binom{39984+2}{3}=10654671159440\). Against the certified accepted
nonzero scalar lower bound
\(L=1569275433846670190958947353987675545133315652035359680560\),
the certificate checks
\(10654671159440\cdot2^{146}<L\). The pinned
[ThreePivotSupport.lean](../../lean/ThreePivotSupport.lean) source checks
both arithmetic theorems with `lake env lean ThreePivotSupport.lean`;
the [axiom audit](../../lean/three-pivot-support-build-20261017.log)
lists only `propext`, `Classical.choice`, and `Quot.sound`. The generic
unordered-image support theorem was already checked in
[UnorderedSupport.lean](../../lean/UnorderedSupport.lean).

The original [two-pivot guided-query test](../run-20261016T2324Z/REPORT.md)
retains its frozen 64 uniform and 64 character-guided known-scalar queries
on a 48-class reduced base. This new result extends the projected-column
criterion to three *complete* residual-pivot fans and tightens the
cardinality screen for the larger prospective base. It does not replace
an ordinary-query relation-yield measurement: the line rows here remain
eliminable through their private columns. The next collector experiment
must generate a different geometry with nonprivate projected columns,
measure the eliminated core's useful rank, and test overlap with a
known-scalar query law on the same frozen base.

Reproduce from the `crypto` worktree root:

```sh
/Volumes/SSD990/cryptanalysis/sage --runtime-info
/Volumes/SSD990/cryptanalysis/sage -python research/p192_extension_cover_20261012/line_residual_pivot_cross_fan.py --pivot 2 --compare research/p192_extension_cover_20261012/type_i_residual_pivot001_cross.json --out research/p192_extension_cover_20261012/type_i_residual_pivot002_cross.json
python3 -B research/p192_extension_cover_20261012/verify_residual_pivot002_cross.py
python3 -B research/p192_extension_cover_20261012/verify_three_pivot_union.py
cd research/p192_extension_cover_20261012/lean && lake env lean ThreePivotSupport.lean
```
