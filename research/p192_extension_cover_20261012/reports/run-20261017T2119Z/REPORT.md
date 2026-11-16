# P-192 third-pivot fixed-origin divisors and guided three-sum support

**Result.** The first 32 distinct exact quotient classes from complete
residual pivot 002 now have fixed-origin degree-three representatives on
the smooth P-192 genus-three quartic. An independent PARI/GP replay checks
all 32 two-cubic reductions. Adding 16 of these classes to the frozen
48-class base raises distinct unordered three-sum targets from 19,600 to
45,760 and geometrically admitted `++` targets from 4,926 to 11,531.
Every tested three-sum target is unique, so the internal collision rank is
zero. The frozen 64-query uniform and 64-query character-guided laws have
zero support hits. The certified accepted-set lower bound gives an ideal
uniform guided hit probability below \(2^{-176}\) for the 64-class base.

| Fixed base | Classes | Unordered triples | Distinct targets | Admitted `++` targets | Collision rank |
| --- | ---: | ---: | ---: | ---: | ---: |
| Original | 16 | 816 | 816 | 224 | 0 |
| Original + pivot 000 | 32 | 5,984 | 5,984 | 1,535 | 0 |
| Original + pivots 000 and 001 | 48 | 19,600 | 19,600 | 4,926 | 0 |
| Original + pivots 000, 001, and 002 | 64 | 45,760 | 45,760 | 11,531 | 0 |

The field prime is
`6277101735386680763835789423207666416083908700390324961279`, and
the selected subgroup order is
`6277101735386680763835789423176059013767194773182842284081`.
The [producer](../../line_pivot002_reduced_classes.py) scans pivot 002's
frozen complete fan in row/side order and selects the first 32 distinct
exact projected quotient points. For each point `Q`, it checks subgroup
membership, forms `H=Q/2` in the odd-order subgroup, constructs the two
inverse-cover fibers, and takes two cubic cuts through the six Frobenius
support points and three fixed rational anchors. The resulting degree-three
divisor `D` has the same origin `T` for every class and satisfies
`[D-T]=[B-A]` by the section-divisor identity. The full
[reduction receipt](../../type_i_pivot002_reduced32.json) retains the
quotient coordinates, fiber residual polynomials, cubic coefficients, and
representatives. The checked Sage [runtime receipt](../../type_i_pivot002_reduced32_runtime_info.json)
precedes its 38.238 s local stage interval.

The [PARI/GP replay](../../verify_pivot002_reduced_native.gp) reconstructs
the quartic, verifies both degree-12 resultants and their prescribed factors,
checks each residual point and squarefree cut, verifies rank-nine cubic
interpolation, and confirms every final degree-three divisor is
noncollinear. It reports
`PIVOT002_FIXED_ORIGIN_NATIVE_PASS cases=32 prime_bits=192` in the
[native log](../../type_i_pivot002_reduced32_native.log). The
[selection and provenance replay](../../verify_pivot002_reduced_receipt.py)
checks all 32 recorded row/side choices against the frozen pivot fan and
binds the Sage producer, PARI input, native pass marker, quartic receipt,
and character-density receipt by SHA-256.

The [coverage producer](../../line_three_pivot_guided_coverage.py)
enumerates every unordered triple with repetition on the declared
16/32/48/64-class bases. It labels a target admitted when the two frozen
sextic characters at `P=-Q/2` and `2P` are positive. The 64 uniform-law
scalars contain 11 `++` queries; all 64 character-guided scalars are `++`.
For each scalar `a`, the target is checked as
`Q=-2[a]G=[a](-2G)`, preserving an exact logarithm equation for any hit.
The [coverage receipt](../../type_i_three_pivot_guided_coverage.json)
records exact targets, character cells, collision ranks, query-cell counts,
and hits. Its 37.848 s local stage interval is a shared-host diagnostic.
The [independent integer-curve replay](../../verify_three_pivot_guided_coverage.py)
recomputes all supports, cells, ranks, and known-scalar equations from
frozen coordinates and checks their source digests; its
[receipt](../../type_i_three_pivot_guided_coverage_replay.json) records a
complete pass.

The earlier certified accepted-scalar lower bound is
`L=1569275433846670190958947353987675545133315652035359680560`.
For an ideal uniform scalar from that accepted set, the replayed 11,531
admitted targets give `11531/L < 2^-176`. The
[ThreePivotGuided.lean](../../lean/ThreePivotGuided.lean) module contains
the exact 64-class capacity and probability arithmetic plus the structural
known-scalar logarithm implication. The concrete class count and sextic
cell labels are hypotheses supplied by the finite-field producer and
independent replay; the generic fixed-origin divisor identity is proved in
the existing Lean geometry modules. The pinned
[Lean target build](../../lean/three-pivot-guided-build-20261017.log)
completed with 8,925 jobs. Its `#print axioms` audit lists only `propext`,
`Classical.choice`, and `Quot.sound`; the workflow also runs a fresh-kernel
check of this module.

| Mathematical step | Lean declaration and exact hypotheses | Concrete check |
| --- | --- | --- |
| 64-class unordered-three capacity | `three_pivot_64_class_three_sum_capacity`; natural-number arithmetic | 45,760 distinct targets in the independent integer-curve replay |
| Guided accepted-set bound | `three_pivot_64_class_guided_hit_bound`; exact integer bound at cell count 11,531 and accepted-set lower bound `L` | Frozen character density plus all 45,760 replayed target cells |
| Known-scalar relation | `three_pivot_guided_scalar_relation`; additive groups, a homomorphism `log`, and the stated three-sum hit equation | Frozen scalar list and integer-curve target replay |
| Fixed-origin divisor transfer | Existing `fixed_origin_cubic_class_identity`; its quartic section and divisor hypotheses | 32 PARI/GP two-cubic cut certificates |

This is a fixed-base support and query-law measurement with
`candidate_id: null`. The three complete residual-pivot fans' line rows
still peel through private columns. The next collector experiment needs a
geometry producing nonprivate columns, followed by useful-rank and
ordinary-query cost measurements under a frozen query law.

Reproduce from the `crypto` worktree root:

```sh
/Volumes/SSD990/cryptanalysis/sage --runtime-info
/Volumes/SSD990/cryptanalysis/sage -python research/p192_extension_cover_20261012/line_pivot002_reduced_classes.py
python3 -B research/p192_extension_cover_20261012/prepare_pivot002_gp.py
gp -s 256000000 -q research/p192_extension_cover_20261012/verify_pivot002_reduced_native.gp
python3 -B research/p192_extension_cover_20261012/verify_pivot002_reduced_receipt.py
/Volumes/SSD990/cryptanalysis/sage -python research/p192_extension_cover_20261012/line_three_pivot_guided_coverage.py
python3 -B research/p192_extension_cover_20261012/verify_three_pivot_guided_coverage.py
cd research/p192_extension_cover_20261012/lean && lake build ThreePivotGuided
```
