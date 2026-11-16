# K13 rootfinding scan: signed-pair denominator sharing

**Status:** source-level implementation review, 2026-11-15 PST. The
336-execution study has not run. The [editable diagram](pair-scan-products.dot),
[SVG](pair-scan-products.svg) and [PDF](PAIR_SCAN_REVIEW_20261115.pdf) show
derived arithmetic paths, not measured performance.

## Question, scope and sources

Can the separate `rootfind-negfold-batch` candidate avoid duplicate field
work while preserving the exact K13 relation stream? The registered curve
is Koblitz E0 over `F_(2^13)`,
`icv1-f2m13-t181-515ee569`, subgroup order 2003, with the 33-point
dimension-five standard-subspace base. The fixed study has eight public
targets and six measured rounds, but no outcome row yet.

**Sources checked:** the supplied primary manuscript
`/Users/adamburan/Downloads/rootfinding_decomposition.pdf` (§3 and the
binary complete-solve table); `papers/rootfinding-decomposition/`
in this repository; `src/cryptanalysis/ic_boundary.rs` for signed-base
construction and the folded table; `src/cryptanalysis/semaev_decomp.rs`
for `Gf2::batch_inv`; and `src/cryptanalysis/koblitz_fast.rs` for the
existing lazy and SIMD paired kernels. KB revision
`9141e386254340a9bdba3036f2bb706a` returned
`source:883ab40565c1f261353efc03ba04473e` at
`crypto:research/notes/ecc2k130/RESEARCH_KOBLITZ_INDEX_CALCULUS.md`
for the established Koblitz collector and
`source:4aeae310bd8926130ee38d08ce3d9da5` at
`crypto:docs/perf/RESULTS.md` for previous batched-arithmetic tests.
The index predates this branch's source; current local files were read.
The scan-window method and existing x86 paired kernel already use related
ideas. This review makes no novelty claim.

## Derived arithmetic change

The binary base stores each ordinary signed pair adjacent: `(x,y)` then
`(x,x+y)`. For fixed `R`, both additions use the denominator `x_R+x`.
The K13 base has one self-negative point first and 16 signed pairs. Fixed
eight-point blocks cross pair boundaries: each full block contains three
complete pairs and two singletons. The previous lazy path sends eight
denominators through `Gf2::batch_inv`. The new
`FastCurve::add_many_lazy_paired_x` sends five, reusing each paired
inverse for both slopes. Both paths have one field inversion per ordinary
block. The existing SIMD path is retained where available. The
current four-lane `Gf2::batch_inv` performs `3k+9` field products when
all `k` inputs are nonzero: one forward and two backward products per
input plus nine fixed products to combine and invert the lane totals.
The portable denominator chain therefore changes from 33 products for
eight inputs to 24 for five, a derived difference of nine per nonexceptional
full K13 block. The documented
`3(k-1)` formula describes the ideal Montgomery chain, not these exact
kernel calls. This excludes slope, squaring, point, lookup and setup
work; it is not a complete-operation or wall estimate.

The candidate preserves both lazy sums, their order, and the table's
first-witness policy. A singleton gets its own denominator; infinity and
blocks with no adjacent equal abscissae use the existing path, while
equal-abscissa exceptional sums use ordinary point addition. The K13 point set and first-witness
comparison in the focused tests are the correctness gate. The original
F4/F5/F6 study and its `rootfind-negfold` arm are unchanged.

## Next check and claim boundary

[Amendment 4](AMENDMENT-4.md) binds this source change before data and
retains [Amendment 3](AMENDMENT-3.md)'s L2/A/A whole-process wall decision.
Before execution, publish a new source receipt including both amendments
and the source files. The monthly EKS budget gate must then clear. The
run must retain every incomplete row and answer audit; a performance
assertion also needs independent second-host replay and a progress-fidelity
record. The [prior cost-model review](COST_MODEL_REVIEW_20261115.md)
remains scoped to the earlier source revision. No canonical quantitative
graph changes: the paper and leaderboard graphs checked here describe
other sources or workloads, and this study has no run data.
