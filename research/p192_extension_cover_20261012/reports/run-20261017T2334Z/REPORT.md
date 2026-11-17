# P-192 cross-pivot quartic rectangles and projected-column rank

**Result.** Four split-line cuts through two rational quartic pivots and two
shared selected endpoints give an exact eight-class relation after their
common section class, pivots, and endpoints cancel. After excluding the six
presentations of the original source lines, the three frozen P-192 fans
contain **9,635,113** such rectangle presentations. A compact **6,309-row
independent basis** has rank **6,309** on **22,658** active distinct
projected columns; every implicit row survives private-column peeling.
Including the source-line presentations gives rank 6,314 on 22,672 active
columns. On the first 32 fixed-origin reduced classes from each fan, the
source-excluded 46-row slice has rank **15** on 66 active projected
columns. This is a geometric source of repeated projected columns and
positive factor-base relation rank.

The field prime is
`6277101735386680763835789423207666416083908700390324961279`;
the subgroup order is
`6277101735386680763835789423176059013767194773182842284081`.
All 96 residual quotient classes have the prior frozen fixed-origin
degree-three representatives, with the third fan's 32 classes independently
checked by PARI/GP in the
[previous report](../run-20261017T2119Z/REPORT.md). The present
[rectangle producer](../../line_cross_pivot_rectangles.py) matches every
class to its frozen fan row and checks each of the 48 line-cut equations on
the P-192 subgroup. The [Sage replay](../../verify_cross_pivot_rectangles.py)
reconstructs the curve, all quotient columns, all rows, their ranks, and the
frozen query points independently.

Let $R_{a,u}=X_{a,u}+Y_{a,u}$ be the projected sum of the two residual
classes on the split line through pivot $a$ and selected endpoint $u$.
Every checked line cut has

\[
  a+u+R_{a,u}=S.
\]

Thus for pivots $a,b$ and endpoints $u,v$,

\[
  R_{a,u}+R_{b,v}=R_{a,v}+R_{b,u}.
\]

The four residual-pair sums expand to eight quotient classes. For the
96-class slice, every class also has a frozen fixed-origin degree-three
Jacobian representative. A quotient-column row gets coefficient $+1$ on
the four points on the left and $-1$ on the four on the right. All line
equations and rectangle rows are exact group identities over
$\mathbb F_p$.

For the complete fans, split-cut counts are 5,037, 4,971, and 5,026.
Pairwise selected-endpoint overlaps are 2,574, 2,552, and 2,485; the
triple overlap is 1,295. The 15,034 pivot-endpoint edges touch 8,718
distinct selected endpoints, so the connected incidence graph has
$15,034-(3+8,718)+1=6,314$ independent cycles. Choose endpoint 0 in
the triple overlap as a common anchor. For pivot pairs 000/001 and
000/002, take each other shared endpoint against that anchor; for
001/002, take only endpoints absent from pivot 000. This gives
$(2,574-1)+(2,552-1)+(2,485-1,295)=6,314$ rows. Every other rectangle
is a difference of anchor rows, and a triple-overlap 001/002 anchor row
is the difference of its 000/002 and 000/001 rows. The
[Lean anchor and pivot-triangle identities](../../lean/CrossPivotRectangles.lean)
prove these span reductions for arbitrary additive commutative groups and
arbitrary residual-pair column maps. The
[full-fan producer](../../line_full_cross_pivot_cycle_rank.py) certifies
a column occurring in exactly one row of this compact basis for every
basis row, even after coalescing three duplicate projected-point
presentations. Its
[independent replay](../../verify_full_cross_pivot_cycle_rank.py)
rebuilds the graph, verifies all 6,314 private basis pivots, and checks
another deterministic spread of 111 line cuts and 71 rectangle group
identities. The frozen fan producers checked every cut equation; the
replay samples cross-check those source receipts.

| Complete-fan measure | Excluding source cuts | Including source cuts |
| --- | ---: | ---: |
| Split-cut edges | 15,028 | 15,034 |
| Implicit rectangle presentations | **9,635,113** | 9,652,897 |
| Compact basis rows and quotient-column rank | **6,309** | 6,314 |
| Projected-column presentations / exact distinct columns | 30,056 / 30,056 | 30,068 / 30,065 |
| Active column presentations / exact distinct active columns | 22,658 / **22,658** | 22,674 / 22,672 |
| Minimum active-column degree in the implicit rectangle family | 2,482 | 2,484 |

The full rectangle target has the explicit form
$Q=2S-a-b-U_u-U_v$ for pivot pair $a,b$ and shared endpoints $u,v$.
The [complete-family query lookup](../../line_full_rectangle_query_law.py)
tests each of the frozen 64 uniform and 64 character-guided known-scalar
queries against all 9,652,897 inclusive rectangle presentations. It
uses 7,611 selected-point subtractions per query with one batch inversion
per pivot pair. Both laws have **zero matches**. The
[query receipt](../../type_i_full_rectangle_query_law.json) binds every
frozen scalar list and input by SHA-256; the
[independent x-first replay](../../verify_full_rectangle_query_law.py)
checks the same query set in smaller inversion chunks. The producer's
56.212 s local wall interval is unisolated. The source-excluded target
family is a subset of the inclusive family, so the same zero-hit
observation applies to its 9,635,113 presentations. These are exact
overlap measurements for the declared 128 public query scalars.
The presentation count upper-bounds distinct target support. With the
previously certified accepted-set lower bound
$L=1569275433846670190958947353987675545133315652035359680560$,
the ideal-uniform support fraction satisfies
$9,652,897/L<2^{-166}$; the actual frozen query laws are measured above.

The complete-fan [receipt](../../type_i_full_cross_pivot_cycle_rank.json)
retains the compact basis witnesses and private-pivot certificate, exact
duplicate positions, source hashes, and sampled group checks. The
[replay receipt](../../type_i_full_cross_pivot_cycle_rank_replay.json)
independently reports rank 6,314 for all cuts and rank 6,309 after
excluding six source-line presentations. The source-excluded policy has
15,028 cuts, 8,717 distinct endpoints, pairwise overlaps
2,571/2,550/2,483, and a triple overlap of 1,293. Its cycle dimension is
$15,028-(3+8,717)+1=6,309$; the rectangle count is 9,635,113. All
30,056 projected columns in that policy are exact and distinct, and
22,658 are active in its rectangle family. The full basis is a
projected-quotient result: fixed-origin degree-three reductions exist in
the frozen evidence for the first 32 classes of each fan, while the
remaining complete-fan classes still need their individual reduction
certificates.

| Frozen slice | Split-cut edges | Rectangle rows | Matrix rank | Active columns | Peeling core, rows / columns | 64 uniform / 64 character-guided four-sum hits |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Pivot 000 + 001, 32 classes each | 32 | 36 | 8 | 36 | 36 / 36 | 0 / 0 |
| Pivots 000–002, 16 classes each | 24 | 16 | 6 | 30 | 15 / 24 | 0 / 0 |
| Pivots 000–002, 32 classes each | 48 | 67 | **17** | 72 | **67 / 72** | 0 / 0 |
| Pivots 000–002, first 32 each, excluding three source cuts | 45 | 46 | **15** | 66 | **46 / 66** | 0 / 0 |

The 96-class incidence graph has 48 pivot-endpoint edges, 32 vertices (three
pivots and 29 distinct endpoints), and one component, so its cycle-space
dimension is $48-32+1=17$. The 67 generated rectangle rows attain that
dimension. The [producer receipt](../../type_i_cross_pivot_rectangles.json)
retains all edge labels, exact row coefficients, targets, input hashes, and
query hits. Its [independent replay receipt](../../type_i_cross_pivot_rectangles_replay.json)
records ranks 8, 6, and 17. The 67 four-sum rectangle targets are distinct;
neither frozen 64-scalar law hits one. Each tested scalar $a$ uses
$Q=[a](-2G)$, so a hit would retain a known scalar equation.

A separate 4-by-4 additive grid of already reduced seed classes supplies a
control for shared columns. All 16 new sums $C_{ij}=A_i+B_j$ now have
fixed-origin degree-three representatives. Its 16 defining rows plus nine
adjacent rectangle rows form a 25-row, 24-column peeling core of rank 16.
Eliminating the 16 derived columns leaves zero constraints on the eight
seeds. This follows for every additive group: any seed assignment extends
uniquely to all $C_{ij}$. The [grid producer](../../line_additive_grid.py)
and [receipt](../../type_i_additive_grid.json) record the 16 two-cubic cuts;
the [PARI/GP replay](../../verify_additive_grid_native.gp) checks all 16
and reports `ADDITIVE_GRID_NATIVE_PASS cases=16 prime_bits=192` in its
[log](../../type_i_additive_grid_native.log). The checked Sage
[runtime receipt](../../type_i_additive_grid_runtime_info.json) precedes
the unisolated 197.928 s grid stage; that wall time is a local diagnostic.

The grid's 40-class base (the original 16, eight fan seeds, and 16 derived
classes) has 11,480 unordered triples, 8,675 distinct targets, and 2,805
collisions. The independent [collision-elimination
script](../../analyze_additive_grid_rank.py) finds 2,688 rows that reduce to
zero under the grid equations and 117 that reduce to one-dimensional seed
constraints. The combined matrix rank is 17: the additional one is exactly
the cross-pivot relation for the first two endpoints. The independent
[grid coverage replay](../../type_i_additive_grid_replay.json) verifies all
11,480 triples, character cells, collision rows, and query hits. Of the
8,675 targets,
2,156 lie in the frozen `++` character cell; neither 64-query law has a
three-sum support hit. The [rank receipt](../../type_i_additive_grid_rank.json)
retains the first collision witness and its eight-seed coefficient vector.

| Mathematical statement | Lean declaration | Finite-field input and status |
| --- | --- | --- |
| Four split cuts imply the eight-column rectangle | `cross_pivot_rectangle` in [CrossPivotRectangles.lean](../../lean/CrossPivotRectangles.lean) | The complete frozen producers check all 15,034 cuts; the new full replay samples 111 cuts and 71 basis rectangles. The Lean theorem assumes precisely the four line equations in an additive commutative group. |
| Every same-pair rectangle is a difference of anchor rows; a third-pair anchor row is a difference through the first pivot | `rectangle_anchor_difference`, `rectangle_pivot_triangle` in [CrossPivotRectangles.lean](../../lean/CrossPivotRectangles.lean) | The identities are unconditional for a column map into an additive commutative group. The finite endpoint overlaps, basis selection, coalesced private columns, and resulting rank 6,309 are separately certified by the full-fan replay. |
| Rectangle target formula and known-scalar logarithm equation | `rectangle_target_formula`, `rectangle_known_scalar_log` in [CrossPivotRectangles.lean](../../lean/CrossPivotRectangles.lean) | The full lookup and independent replay test 128 known scalars against all implicit rectangle targets. |
| Full-fan arithmetic consequences of the replayed overlaps | `full_fan_cycle_dimension_arithmetic`, `full_fan_implicit_rectangle_count`, `full_fan_active_edge_count` and their `non_source_*` counterparts in [FullFanCycleArithmetic.lean](../../lean/FullFanCycleArithmetic.lean) | Both finite incidence graphs, connectivity, and every compact-basis pivot are checked by the producer and independent replay. |
| Ideal-uniform accepted-set capacity bound | `full_rectangle_accepted_capacity_bound` in [FullFanCycleArithmetic.lean](../../lean/FullFanCycleArithmetic.lean) | Uses the prior exact lower bound $L$ and the complete-fan presentation count as an upper bound on distinct support. |
| Every additive-grid seed assignment has a unique derived extension | `additive_grid_unique_extension` in [AdditiveGrid.lean](../../lean/AdditiveGrid.lean) | Eight reduced seeds and 16 additional reduced classes; independent PARI replay of 16 cuts. |
| Grid rows admit arbitrary residuals and rectangles follow from the rows | `additive_grid_arbitrary_residual`, `additive_grid_rectangle` in [AdditiveGrid.lean](../../lean/AdditiveGrid.lean) | Exact 25-by-24 P-192 matrix, rank 16 and zero seed constraint rank before three-sum collisions. |

The pinned Lean 4.34.1 toolchain and mathlib revision
`d13f23b723b8a846827a245b89c10fc7d3f11612` are in
[lean-toolchain](../../lean/lean-toolchain) and
[lakefile.toml](../../lean/lakefile.toml). The final build and
`#print axioms` audit are retained in
[the combined Lean log](../../lean/rectangle-lean-build-20261017-combined.log).
The finite-field incidence rank is replayed separately; the Lean structural
theorem takes its four checked cut equations as hypotheses. Lean also proves
the full family lies in the stated anchor-row span. A generic Lean theorem
identifying the finite graph-cycle basis with the coalesced subgroup-column
matrix is still an open formalization bridge.
The builds' `#print axioms` output lists only `propext`,
`Classical.choice`, and `Quot.sound`; it lists no `sorryAx`.
A local `leanchecker --fresh` attempt on `CrossPivotRectangles` was
interrupted after more than 15 minutes on this shared host. The
[CI proof job](../../../../.github/workflows/weak-curve-database.yml)
builds all three modules, runs fresh checks, and audits their axioms;
its result is pending for this branch.

The next collector should reduce more of the 22,658 active non-source
quotient columns to fixed-origin divisors, use the 6,309-cycle row space
directly, and
measure novel rank per ordinary query together with matrix and descent
phases. The 6,309-dimensional row space is fixed-geometry precomputation;
the frozen 64-query overlap cells above are the currently measured target
law for the 96-class slice. The
empirical IC candidate remains `candidate_id: null` until a complete
factor-base and target pipeline is specified and measured.

Reproduce from the `crypto` worktree root:

```sh
python3 -B research/p192_extension_cover_20261012/line_cross_pivot_rectangles.py
python3 -B research/p192_extension_cover_20261012/line_full_cross_pivot_cycle_rank.py
python3 -B research/p192_extension_cover_20261012/verify_full_cross_pivot_cycle_rank.py
python3 -B research/p192_extension_cover_20261012/line_full_rectangle_query_law.py
python3 -B research/p192_extension_cover_20261012/verify_full_rectangle_query_law.py
/Volumes/SSD990/cryptanalysis/sage --runtime-info
/Volumes/SSD990/cryptanalysis/sage -python research/p192_extension_cover_20261012/verify_cross_pivot_rectangles.py
/Volumes/SSD990/cryptanalysis/sage -python research/p192_extension_cover_20261012/line_additive_grid.py
python3 -B research/p192_extension_cover_20261012/prepare_additive_grid_gp.py
gp -s 256000000 -q research/p192_extension_cover_20261012/verify_additive_grid_native.gp
python3 -B research/p192_extension_cover_20261012/verify_additive_grid.py
python3 -B research/p192_extension_cover_20261012/analyze_additive_grid_rank.py
cd research/p192_extension_cover_20261012/lean && lake build AdditiveGrid CrossPivotRectangles FullFanCycleArithmetic
```
