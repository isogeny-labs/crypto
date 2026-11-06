# Known-scalar query cubics and 12-column line relations on the P-192 cover

**Result.** On the frozen P-192 Type I plane quartic, a known-scalar query can
be converted to a rational cubic section and then to an explicit line-fan
relation. Of 128 query scalars under two frozen input laws, 27 distinct
positive query cubics split into three rational quartic points. Using their
81 roots as reciprocal endpoints yields **370 verified 12-column relations
of exact rank 181**, with 184 projected columns shared across rows at
different endpoints. The known right-hand side is
`[-4h(a_i-a_j)]G`, where `h` is the recorded cofactor and each query point
is `Q_i=[-2a_i]G`. Independent PARI and integer-group replays check the
roots, line cuts, scalar equations, and modular rank.

The prime field has
`p=6277101735386680763835789423207666416083908700390324961279`; the
subgroup has prime order
`r=6277101735386680763835789423176059013767194773182842284081`.
The [query-cubic census](../../type_i_known_scalar_query_cubic_roots.json),
[transfer screen](../../type_i_known_scalar_query_transfer.json), and
[reciprocal relation screen](../../type_i_known_scalar_query_cross_fan.json) pin
the input laws, source hashes, exact coordinates, and row coefficients.
All 27 query scalars are distinct. The fixed-endpoint comparator uses the
16 endpoints of the
[character-guided selection](../run-20261018T1929Z/REPORT.md).

| Stage | Frozen inputs | Measured outcome | Unisolated stage interval |
| --- | ---: | --- | ---: |
| Positive and half query cubics | 128 scalars, 256 cubics | 90 with no rational root, 113 with one, 53 with three; 272 point appearances, 247 distinct | 34.155 s |
| Query pivot to selected endpoint | 16 distinct pivot scalars, 256 lines | 146 split lines, 292 distinct projected columns, 1,542 homogeneous rectangles of rank 115 | 6.603 s |
| Positive cubic quotient transfer | 27 distinct fully split query cubics | One shared constant for `sum(P_i)+[2h]Q_i` | 3.650 s |
| Three-cut query bundles | 27 cubics, 16 endpoints, 1,296 lines | 651 split lines, 49 complete bundles, 72 known-scalar rows of rank 33 | 42.902 s |
| Reciprocal query-root bundles | 27 cubics, 81 endpoints, 6,561 directed lines | 3,194 split lines, 258 complete bundles, 370 known-scalar rows of rank 181 | 150.549 s |

The intervals above exclude checked Sage startup, runtime inspection,
fixture preparation, GP replay, and independent Python replay. They are
algorithmic stage diagnostics from this local host, with no isolated CPU
timing claim. The [runtime receipts](../../type_i_known_scalar_query_cubic_runtime_info.json),
[pivot runtime receipt](../../type_i_known_scalar_query_pivot_runtime_info.json),
[transfer runtime receipt](../../type_i_known_scalar_query_transfer_runtime_info.json),
and [relation runtime receipt](../../type_i_known_scalar_query_relation_runtime_info.json)
and [reciprocal runtime receipt](../../type_i_known_scalar_query_cross_fan_runtime_info.json)
pin the accepted Sage installation before each producer ran.

The PARI [root replay](../../verify_known_scalar_query_cubics_native.gp)
checks all 256 cubic resultants and rational roots. It reports 272 rational
point appearances across the cases. Eleven scalars occur in both query laws;
those account for all 25 repeated quartic points. No point repeats between
distinct scalars, and none lies in the existing 9,925 selected quartic
points. The [independent census replay](../../verify_known_scalar_query_cubics.py)
checks the query scalars and subgroup points, while the
[pivot replay](../../verify_known_scalar_query_pivot_screen.py) checks
quartic and line equations, group identities, and exact modular rank.

## Conditional line-fan theorem

Let `P_{i,1},P_{i,2},P_{i,3}` be the three rational roots of query cubic
`i`, and let `q_i` be the corresponding quotient point. Work in the
additive subgroup containing their projected classes. Suppose both cubic
divisors obey

`p_{i,1}+p_{i,2}+p_{i,3}+[k]q_i=C`,

with the same `C` and `k`, and that all six lines through one endpoint `u`
split into two residual classes `x_{i,j}` whose sum obeys

`p_{i,j}+u+x_{i,j}=S`.

Then the difference between the two three-cut residual sums is

`(x_{1,1}+x_{1,2}+x_{1,3})-(x_{2,1}+x_{2,2}+x_{2,3})=[k](q_1-q_2)`.

Indeed, summing the three line equations for each query gives
`sum_j x_{i,j}=3S-3u-sum_j p_{i,j}=3S-3u-C+[k]q_i`.
Subtraction cancels the section and endpoint constants. For this frozen
cover the 27 checked cubic sums obey `k=2h`; with `q_i=[-2a_i]G`, the
right-hand side is `[-4h(a_1-a_2)]G`. The
[transfer replay](../../verify_known_scalar_query_transfer.py) independently
finds the same constant at all 27 quotient sums and checks the cofactor
coefficient. The [relation replay](../../verify_known_scalar_query_relation_screen.py)
recomputes all 651 quartic cuts, 49 complete bundles, 72 scalar equations,
and rank 33 using independent integer-group and dense modular arithmetic.

The 72 fixed-endpoint rows use exactly 12 nonzero columns each. Across all 651 split
lines there are 1,302 distinct residual columns and **no repeated column**;
none is among the fixed 96 columns. Their rank equals
`sum_u(max(b_u-1,0))=33`, where `b_u` is the number of complete bundles at
endpoint `u`. Thus these rows have no cross-endpoint column coupling.

The reciprocal experiment makes that coupling explicit. Its endpoint set is
the 81 rational roots of the 27 query cubics. Each split line through two
different roots is traversed once in each direction. The
[reciprocal replay](../../verify_known_scalar_query_cross_fan.py) checks
that all 3,194 directed split edges have a reversed edge with the same two
residual points, and that every one of the 3,194 distinct projected columns
occurs exactly twice among 6,388 appearances. It also replays all 258
complete three-cut bundles, all 370 scalar equations, and rank 181. Of the
1,316 columns active in the relation rows, **184 appear in rows based at
different endpoints**; none belongs to the fixed 96-class base. Exactly 77
of the 81 endpoints support a complete bundle, so the endpointwise rank
ceiling is `258-77=181`, attained by the measured rows. This is cross-row
column reuse under a known-scalar query law, with a 1,316-column active
support for 181 independent equations.

The first reciprocal producer completed its line-fan sweep but failed
receipt serialization when Sage field elements were written as JSON. Its
[failure receipt](../../type_i_known_scalar_query_cross_fan_first_attempt.json)
records that status. The corrected producer converted those coordinates to
integers and reran the same 27-query policy through the checked launcher.

| Mathematical statement | Lean declaration | Concrete bridge and status |
| --- | --- | --- |
| Two cubic-sum identities and six line equations imply the known-scalar difference equation | `query_cubic_line_difference` in [QueryCubicLineRelation.lean](../../lean/QueryCubicLineRelation.lean) | Complete conditional additive-group proof. `#print axioms` reports only standard `propext`; `leanchecker --fresh` succeeds. |
| Each selected query cubic has the common quotient-sum constant with coefficient `2h` | Open structural bridge | Verified for the 27 frozen cubics by the checked Sage projection and independent group-coordinate replay. A general proof through the cubic divisor class, conorm, norm, and quotient homomorphism has not yet been formalized. |
| Concrete line cuts, reciprocal column reuse, and ranks 33 and 181 | Finite certificates, not encoded in Lean | The Sage producers and independent Python replays check coordinates, reversed edges, scalar equations, and modular RREF. |

Lean is pinned to `leanprover/lean4:v4.34.1` and mathlib commit
`d13f23b723b8a846827a245b89c10fc7d3f11612` in the
[toolchain](../../lean/lean-toolchain) and
[manifest](../../lean/lake-manifest.json). The
[build and axiom log](../../lean/query-cubic-line-lean-build-20261018.log)
records `lake build`, `#print axioms`, and the fresh checker. The conditional
theorem formalizes exactly the cancellation step; the geometric transfer
hypothesis is stated openly and is a separate proof obligation.

The row count and rank measure the frozen known-scalar query law. The
1,316 active columns and rank 181 show the exact scale of the remaining
closure problem for this reciprocal graph. A complete candidate requires
an explicit column-closure policy, unseen-target query yield, final
relation-matrix solving, target descent, and paired one-target timing;
accordingly the receipts retain `candidate_id: null`. The next experiment
should search for endpoint/query choices that repeatedly hit the same
active columns while adding rank, including the one-root query cubics whose
other two roots form a rational degree-two divisor.

Reproduce from the `crypto` worktree root:

```sh
/Volumes/SSD990/cryptanalysis/sage --runtime-info > research/p192_extension_cover_20261012/type_i_known_scalar_query_cubic_runtime_info.json
/Volumes/SSD990/cryptanalysis/sage -python research/p192_extension_cover_20261012/line_known_scalar_query_cubics.py > research/p192_extension_cover_20261012/type_i_known_scalar_query_cubic_roots.log 2>&1
python3 -B research/p192_extension_cover_20261012/prepare_known_scalar_query_cubics_gp.py
gp -s 256000000 -q research/p192_extension_cover_20261012/verify_known_scalar_query_cubics_native.gp > research/p192_extension_cover_20261012/type_i_known_scalar_query_cubic_roots_native.log
python3 -B research/p192_extension_cover_20261012/verify_known_scalar_query_cubics.py
/Volumes/SSD990/cryptanalysis/sage --runtime-info > research/p192_extension_cover_20261012/type_i_known_scalar_query_pivot_runtime_info.json
/Volumes/SSD990/cryptanalysis/sage -python research/p192_extension_cover_20261012/line_known_scalar_query_pivot_screen.py > research/p192_extension_cover_20261012/type_i_known_scalar_query_pivot_screen.log 2>&1
python3 -B research/p192_extension_cover_20261012/verify_known_scalar_query_pivot_screen.py
/Volumes/SSD990/cryptanalysis/sage --runtime-info > research/p192_extension_cover_20261012/type_i_known_scalar_query_transfer_runtime_info.json
/Volumes/SSD990/cryptanalysis/sage -python research/p192_extension_cover_20261012/line_known_scalar_query_transfer.py > research/p192_extension_cover_20261012/type_i_known_scalar_query_transfer.log 2>&1
python3 -B research/p192_extension_cover_20261012/verify_known_scalar_query_transfer.py
/Volumes/SSD990/cryptanalysis/sage --runtime-info > research/p192_extension_cover_20261012/type_i_known_scalar_query_relation_runtime_info.json
/Volumes/SSD990/cryptanalysis/sage -python research/p192_extension_cover_20261012/line_known_scalar_query_relation_screen.py > research/p192_extension_cover_20261012/type_i_known_scalar_query_relation_screen.log 2>&1
python3 -B research/p192_extension_cover_20261012/verify_known_scalar_query_relation_screen.py
/Volumes/SSD990/cryptanalysis/sage --runtime-info > research/p192_extension_cover_20261012/type_i_known_scalar_query_cross_fan_runtime_info.json
/Volumes/SSD990/cryptanalysis/sage -python research/p192_extension_cover_20261012/line_known_scalar_query_cross_fan.py > research/p192_extension_cover_20261012/type_i_known_scalar_query_cross_fan.log 2>&1
python3 -B research/p192_extension_cover_20261012/verify_known_scalar_query_cross_fan.py
cd research/p192_extension_cover_20261012/lean
lake build QueryCubicLineRelation
lake env lean QueryCubicLineRelation.lean
lake env leanchecker --fresh --verbose QueryCubicLineRelation
```
