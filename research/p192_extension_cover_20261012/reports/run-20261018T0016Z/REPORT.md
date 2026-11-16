# A rank-optimal 96-class P-192 triple-pivot selection

**Result.** Selecting the lowest 16 source-free endpoints shared by all
three frozen quartic pivot fans gives 48 split-line cuts and 96 distinct
projected quotient classes. Every class has a checked fixed-origin
degree-three genus-three Jacobian representative. Their 360 eight-column
rectangle presentations are distinct positive targets and have **exact
matrix rank 30** over the P-192 subgroup order; all 96 columns are active
and each occurs in 30 rectangles. An independent PARI/GP replay checks
all 96 two-cubic reductions, and an independent integer-group and modular
matrix replay checks every line cut and rectangle. The selected rank is
maximal among connected three-pivot 48-cut incidence graphs with at most
one cut per pivot-endpoint pair.

The selected endpoints are
`19,30,33,35,48,52,63,66,72,73,75,80,83,91,116,128` in the frozen
9,925-point selected set. The field prime is
`6277101735386680763835789423207666416083908700390324961279` and the
subgroup order is
`6277101735386680763835789423176059013767194773182842284081`.
The [selection and rank screen](../../line_triple_star_screen.py) freezes
the first 16 endpoints in the intersection after removing each fan's two
source-line endpoints. Its [receipt](../../type_i_triple_star_screen.json)
binds the three complete fans and the selected-point source by SHA-256.

| 96-class reduction policy | Retained cuts | Rectangle presentations | Exact rank | Active columns |
| --- | ---: | ---: | ---: | ---: |
| First 32 classes per fan, including source cuts | 48 | 67 | 17 | 72 |
| Same first 96 reductions, excluding three source cuts | 45 | 46 | 15 | 66 |
| **16 shared source-free endpoints, two classes per cut** | **48** | **360** | **30** | **96** |

The first two comparison rows are the independently replayed frozen slice
in the [cross-pivot report](../run-20261017T2334Z/REPORT.md). The third
uses the same **96 reduction slots**, with all selected cuts source-free.
It yields 30/96 independent rows per reduced class, compared with 17/96
for the unfiltered first-32 policy. The new rank is a fixed-geometry
relation-matrix result; the target-query evidence appears below.

For a pivot $a$ and selected endpoint $u$, write the checked line cut as
$a+u+R_{a,u}=S$, where $R_{a,u}=X_{a,u}+Y_{a,u}$ is the sum of its two
projected residual classes. For distinct pivots $a,b$ and endpoints $u,v$,
the four cuts cancel to give

\[
  R_{a,u}+R_{b,v}-R_{a,v}-R_{b,u}=0.
\]

Choose endpoint 19 as anchor. For each of the other 15 endpoints take its
000/001 anchor rectangle and its 000/002 anchor rectangle. These 30 rows
span all 360 presentations: a rectangle at any two endpoints is a
difference of its pair's anchor rows, and a 001/002 anchor row is the
difference of its 000/002 and 000/001 rows. The 30 rows are independent
because each 000/001 row has a private pivot-001 residual column and each
000/002 row has a private pivot-002 residual column. The frozen coordinate
check finds all 96 columns distinct, so those private coefficients are
nonzero modulo the exact subgroup order. A separate modular RREF in the
[independent replay](../../verify_triple_star_rank.py) obtains rank 30
from all 360 rows without using the private-column selection. It also
checks 48 line equations and 360 rectangle group identities directly.

This rank reaches the connected-incidence upper bound for the declared
48-cut, three-pivot budget. Each endpoint has at most three pivot edges,
so 48 edges require at least 16 endpoints. A connected graph with three
pivot vertices and $m$ endpoints has cycle rank
$48-(3+m)+1=46-m\le30$. Each rectangle row is the image of an incidence
cycle under the linear map that replaces one edge by its two residual
columns, so its matrix rank cannot exceed that cycle dimension. The
chosen complete three-by-sixteen incidence
has $m=16$ and rank 30; the independent projected-column certificate
shows that no graph cycle is lost after quotient-coordinate coalescing.

The [Sage producer](../../line_triple_star_reduced_classes.py) uses the
repository's checked Sage launcher and the same two-cubic fixed-origin
construction as the earlier fan reductions. Its
[reduction receipt](../../type_i_triple_star_reduced96.json) records 96/96
successful representatives, exact quotient coordinates, cubic cuts,
the fixed origin, source hashes, and a 443.678-second local unisolated
stage interval. The [runtime receipt](../../type_i_triple_star_runtime_info.json)
was saved before the workload; launcher verification and import startup
are outside that interval. The [PARI input builder](../../prepare_triple_star_gp.py)
serializes every quartic cut, and the
[independent GP script](../../verify_triple_star_reduced_native.gp) checks
the quartic points, both cubic resultant factorizations, interpolation
rank, and final degree-three representative for all 96 cases. Its
[pass log](../../type_i_triple_star_reduced96_native.log) and the
[full replay receipt](../../type_i_triple_star_replay.json) complete the
certificate chain.

The 360 positive four-sum targets are all distinct. For the 64 frozen
uniform and 64 frozen character-guided known-scalar queries, each target
point is $Q=[a](-2G)$ for a recorded scalar $a$; neither law hits any of
the 360 targets. This is an exact membership result for those 128 inputs.
Using the earlier accepted-set lower bound
$L=1569275433846670190958947353987675545133315652035359680560$,
the ideal-uniform fraction is $360/L<2^{-181}$. The local frozen query
laws are measured separately from that capacity calculation. The
complete 9,652,897-presentation rectangle family was also checked
against the same 128 scalars in the previous report and had zero hits.

| Mathematical statement | Lean declaration | Concrete bridge and status |
| --- | --- | --- |
| Four line cuts give the eight-column row; anchor and third-pivot identities span every rectangle | `cross_pivot_rectangle`, `rectangle_anchor_difference`, `rectangle_pivot_triangle` in [CrossPivotRectangles.lean](../../lean/CrossPivotRectangles.lean) | The independent replay checks the 48 selected cuts and all 360 group identities. Lean proves the identities for arbitrary additive commutative groups. |
| A private nonzero column in each row forces independent coefficients | `private_column_coefficients_zero` in [LineLeafRank.lean](../../lean/LineLeafRank.lean) | The producer checks 30 distinct private pivot columns after exact-coordinate deduplication; independent modular RREF returns rank 30. |
| The 16-endpoint counts, connected 48-cut upper bound, and ideal-uniform capacity | `triple_star_16_cut_count`, `triple_star_16_rectangle_count`, `triple_star_16_cycle_dimension_arithmetic`, `connected_48_cut_three_pivot_cycle_upper_bound`, `triple_star_ideal_uniform_support_bound` in [FullFanCycleArithmetic.lean](../../lean/FullFanCycleArithmetic.lean) | Endpoint incidence, connectivity, distinct coordinates, and the accepted-set lower bound are frozen computational inputs. |

The new Lean declarations build under the pinned Lean 4.34.1 toolchain
and mathlib revision in [lean-toolchain](../../lean/lean-toolchain) and
[lakefile.toml](../../lean/lakefile.toml). The
[build and axiom audit](../../lean/triple-star-lean-build-20261018.log)
lists only standard `propext`, `Classical.choice`, and `Quot.sound` uses.
The exact finite incidence and coordinate certificate is replayed in
Python and PARI; a Lean encoding of that concrete 48-edge certificate
and its reduction to the generic private-column theorem remains open.
The focused CI job reruns the selection, GP cuts, and independent rank;
the existing proof job runs fresh checks on the Lean module.
Older frozen manifests still retain their original hashes for the
repository workflow, `.gitattributes`, and Lean package list. The CI
replays omit only those superseded entries when checking historical
manifests; this run's [artifact manifest](ARTIFACTS.sha256) pins the
current workflow, package list, and P-192 attributes alongside the new
evidence.

The relation policy remains `candidate_id: null`: the 443.678-second
interval measures fixed-origin construction on an unisolated local host,
not a target-dependent online phase. For this frozen selection, the
measured target-query overlap is zero. A next collector experiment must
expand target support while retaining independently reduced columns and
charge its ordinary-query yield, useful rank, matrix work, and descent.

Reproduce from the `crypto` worktree root:

```sh
python3 -B research/p192_extension_cover_20261012/line_triple_star_screen.py
/Volumes/SSD990/cryptanalysis/sage --runtime-info > research/p192_extension_cover_20261012/type_i_triple_star_runtime_info.json
/Volumes/SSD990/cryptanalysis/sage -python research/p192_extension_cover_20261012/line_triple_star_reduced_classes.py > research/p192_extension_cover_20261012/type_i_triple_star_reduced96.log 2>&1
python3 -B research/p192_extension_cover_20261012/prepare_triple_star_gp.py
gp -s 256000000 -q research/p192_extension_cover_20261012/verify_triple_star_reduced_native.gp > research/p192_extension_cover_20261012/type_i_triple_star_reduced96_native.log
python3 -B research/p192_extension_cover_20261012/verify_triple_star_rank.py
```
