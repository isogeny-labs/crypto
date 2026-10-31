# Character-guided P-192 triple-star selection at fixed rank and class budget

**Result.** A deterministic search among the first 32 endpoints common to
three complete source-free pivot fans selected 16 endpoints whose 360
four-sum presentations contain **128 targets in the `++` character cell**.
The previous first-16 selection contained 93. The new selection retains 48
line cuts, 96 distinct projected quotient classes, 360 distinct positive
targets, and exact matrix rank **30**. All 96 classes were converted to
fixed-origin degree-three genus-three Jacobian representatives; independent
PARI/GP and integer-group replays checked the cuts, cells, and rank.

The selected endpoint indices in the frozen 9,925-point set are
`48,52,63,66,73,80,83,130,136,156,157,195,197,198,216,221`. The
field prime is
`6277101735386680763835789423207666416083908700390324961279`, and
the subgroup order is
`6277101735386680763835789423176059013767194773182842284081`.
The [selection program](../../line_triple_star_character_selection.py)
pins the three complete fans, the initial [triple-star screen](../../type_i_triple_star_screen.json),
the exact selected points, and the sextic character polynomial by SHA-256
in its [deterministic receipt](../../type_i_triple_star_character_selection.json).

| 16-endpoint policy | `++` | `+-` | `-+` | `--` | Distinct targets | Reduced classes | Rank |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| First 16 common source-free endpoints | 93 | 92 | 94 | 81 | 360 | 96 | 30 |
| Character-guided selection within the first 32 | **128** | 82 | 77 | 73 | 360 | **96** | **30** |

The first row's rank and reductions are from the [prior frozen
run](../run-20261018T0016Z/REPORT.md). The character cells for both rows
and the second row's matrix rank were independently recomputed from group
coordinates, rather than accepted from the search program. The gain is 35
admitted presentations, or 37.6% relative to 93, at the same 96-class
budget. The search scored
all three pivot-pair targets for each of the 496 unordered pairs in the
first-32 universe and ran deterministic best-improvement one-swap ascent
from 19 frozen starts. It establishes the reported selection, not a global
optimum over all 1,293 shared endpoints.

For pivot $a$ and endpoint $u$, the checked line equation is
$a+u+R_{a,u}=S$, where $R_{a,u}$ is the sum of two projected residual
classes. For pivots $a,b$ and endpoints $u,v$, four cuts yield the known
scalar equation

\[
R_{a,u}+R_{b,v}-R_{a,v}-R_{b,u}=0,
\qquad
Q=R_{a,u}+R_{b,v}=2S-a-b-u-v.
\]

The search evaluates the exact character cell of $Q$ and its unique half
$[2^{-1}]Q$ in the prime-order subgroup. Its objective is the number of
presentations with cell `++`, the cell used by the frozen character-guided
query law. The four character-cell counts sum to all 360 presentations;
all 360 positive targets are distinct for both endpoint selections.

The [checked Sage producer](../../line_triple_star_character_reduced.py)
constructed 96/96 fixed-origin degree-three representatives in a
179.995-second local unisolated stage interval. Its [runtime
receipt](../../type_i_triple_star_character_runtime_info.json) was saved
before the workload, and the [reduction
receipt](../../type_i_triple_star_character_reduced96.json) retains exact
quotient points, cubic sections, source hashes, and the fixed origin. The
[PARI input builder](../../prepare_triple_star_character_gp.py) and
[native checker](../../verify_triple_star_character_reduced_native.gp)
independently replayed every quartic cut, resultant factorization,
interpolation rank, and final representative. The [native pass
log](../../type_i_triple_star_character_reduced96_native.log) records all
96 cases. The [group and rank replay](../../verify_triple_star_character_rank.py)
checked 48 line equations, 360 rectangle identities, 96 distinct active
columns, exact rank 30 modulo the subgroup order, and the 128 `++` cells.
Its [receipt](../../type_i_triple_star_character_replay.json) pins those
outputs and the native input and log.

The frozen 64 uniform and 64 character-guided known-scalar queries each
missed the 360 selected positive targets. This is an exact overlap count
for those 128 queries. With the earlier accepted-set lower bound
$L=1569275433846670190958947353987675545133315652035359680560$, the
ideal-uniform admitted-support fraction is $128/L<2^{-182}$, as checked
in Lean. The stage interval above measures class construction; a
target-dependent relation collector and its full one-target cost are
still separate measurements. This selection remains `candidate_id: null`.

| Mathematical statement | Lean declaration | Concrete bridge and verification |
| --- | --- | --- |
| Four line cuts give each eight-column rectangle, and anchor rectangles span the 30-dimensional cycle space | `cross_pivot_rectangle`, `rectangle_anchor_difference`, `rectangle_pivot_triangle` in [CrossPivotRectangles.lean](../../lean/CrossPivotRectangles.lean) | The replay checks every selected line equation and all 360 group identities. |
| Private nonzero columns make the 30 anchor coefficients independent | `private_column_coefficients_zero` in [LineLeafRank.lean](../../lean/LineLeafRank.lean) | The replay checks distinct projected coordinates and independently performs modular RREF, obtaining rank 30. |
| The connected 48-cut rank bound and 16-endpoint counts | `connected_48_cut_three_pivot_cycle_upper_bound`, `triple_star_16_cut_count`, `triple_star_16_rectangle_count` in [FullFanCycleArithmetic.lean](../../lean/FullFanCycleArithmetic.lean) | The receipt fixes 16 endpoints and 48 cuts; the replay checks rank and all 96 active columns. |
| Ideal-uniform admitted-support fraction is below $2^{-182}$ | `character_star_ideal_uniform_support_bound` in [FullFanCycleArithmetic.lean](../../lean/FullFanCycleArithmetic.lean) | The finite search and independent replay establish the numerator 128; the accepted-set lower bound is an earlier frozen input. |

The pinned Lean toolchain and mathlib revision are in
[lean-toolchain](../../lean/lean-toolchain) and
[lakefile.toml](../../lean/lakefile.toml). The
[build and axiom audit](../../lean/character-star-lean-build-20261018.log)
records the theorem checks; the generic group and rank lemmas retain their
earlier audited builds. As in the prior report, the exact 48-edge coordinate
certificate is replayed in Python and PARI; a Lean encoding of that finite
certificate remains an open formalization bridge. The new CI job reruns
the deterministic selection, GP cuts, rank, cells, and artifact hashes.
The prior run's manifest still pins its workflow and Lean module revisions,
so its CI replay omits only those two superseded entries; this run's
[manifest](ARTIFACTS.sha256) pins their current revisions.

Reproduce from the `crypto` worktree root:

```sh
python3 -B research/p192_extension_cover_20261012/line_triple_star_character_selection.py
/Volumes/SSD990/cryptanalysis/sage --runtime-info > research/p192_extension_cover_20261012/type_i_triple_star_character_runtime_info.json
/Volumes/SSD990/cryptanalysis/sage -python research/p192_extension_cover_20261012/line_triple_star_character_reduced.py > research/p192_extension_cover_20261012/type_i_triple_star_character_reduced96.log 2>&1
python3 -B research/p192_extension_cover_20261012/prepare_triple_star_character_gp.py
gp -s 256000000 -q research/p192_extension_cover_20261012/verify_triple_star_character_reduced_native.gp > research/p192_extension_cover_20261012/type_i_triple_star_character_reduced96_native.log
python3 -B research/p192_extension_cover_20261012/verify_triple_star_character_rank.py
cd research/p192_extension_cover_20261012/lean && lake env lean FullFanCycleArithmetic.lean
lake build FullFanCycleArithmetic
lake env leanchecker --fresh --verbose FullFanCycleArithmetic
```
