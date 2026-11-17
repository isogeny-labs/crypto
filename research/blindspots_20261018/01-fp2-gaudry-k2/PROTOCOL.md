# Item 01 protocol: Gaudry's `k = 2` index calculus on `E(F_{p²})` against matched rho

Frozen 2026-10-18, before any measured run. Source design: section "01" of
`research/blindspots_20261018/EXPERIMENTS.md` and its "Rules every item
follows". Written by the experiment-director lane. The design is tightened in
places and loosened nowhere that the budget does not force; every deviation is
listed at the end.

## Question

On random ordinary curves over `F_{p²}`, how far is the best `k = 2` index
calculus from matched Pollard rho, and does the gap shrink with `p`? Gaudry's
`k = 2` attack is `Õ(p)` and rho is `Õ(√#E) = Õ(p)`: this is the only deployed
family (FourQ, GLS curves) in the tie regime, so the exponent is not obviously
different and the constant decides. Report the gap as distance remaining.

## What already exists, and what is reused

* `src/cryptanalysis/line_oracle.rs`, `gaudry_cubic.rs` (`square_core`,
  Wiedemann, the large-prime eliminator), `glv_invariant_base.rs` (the fold),
  `gls_fp2.rs`, `ic_boundary.rs` (the counted rho) are the audited pieces. The
  instrument ports them; it does not edit them.
* `research/isogeny_fibers_20261010/round4/16_4_zeta_fold_fp2/RESULTS.md`
  measured the same line oracle on `j = 0` curves over `F_{p²}`: 3,000 to
  5,200 `F_p` multiplications per call and a decomposition rate of 0.35 to 0.42
  at `p = 547` to `61363`. This item's oracle must land in that band
  (consistency reference, not a cell: different family, 1,000 targets, no
  complete solve).
* `research/notes/index-calculus/RESEARCH_GLV_INVARIANT_FACTOR_BASES.md`
  section 5.1 and E2/E9: the GLS line folds 4 points a column (column ratio 2
  over negation), and the matched folded rho takes `√2` fewer steps. The GLS
  rows here reproduce those two facts or the instrument is wrong (consistency
  reference). Section 5.1's GLS rows used the `subtract` oracle at
  `log₂ r ≤ 23`, a different arm; no committed result prices a complete
  algebraic-oracle solve on a random `E(F_{p²})` curve, or a GLS curve, from
  `log₂ #E = 24` to `40` against matched rho in GAE. No cell is therefore
  reused; all are measured.
* E16 (`E(F_{p³})` subfield curves, `r ≈ p²/k`): the line base needs `Θ(p)`
  relations, rho costs `Θ(p)`, the ratio does not fall. That is the prediction
  this item tests on the full-group `F_{p²}` setting; it is not a result here.

## The blocker that shaped the instrument

The `crypto_lib` library does not compile at the branch base (25 errors: a bad
merge left `ecbench/workload.rs`, `ecbench/mod.rs` and `gaudry_quartic.rs`
inconsistent; `origin/main` is the same). `cargo build --example` therefore
cannot build an instrument that uses the crate. The instrument
`examples/blindspots_01_fp2_gaudry.rs` is dependency-free (`std` only) and each
card builds it with `rustc`, as item 02 does. It ports the audited pieces named
above, with the differences recorded under "Deviations". Nothing in `src/` is
touched.

## Arms

All index-calculus arms solve `Q = [d]G` for a planted `d` the arm never sees,
in the subgroup `⟨G⟩` of prime order `r`, with cofactor `h`. A relation is
`R = P₁ + P₂` for a target `R = [a]G + [b]Q`; the row is
`Σ coef·x_col − h·b·d ≡ h·a (mod r)`, `x_col` the logarithm of `[h]` of the
column's unit point. The answer is accepted only when `[d]G = Q`.

* **(a) `ic-a`, plain Gaudry–Diem.** Base `{P : x(P) ∈ F_p^*}`, folded by
  `⟨−1⟩` (2 points a column). The decomposition oracle is the Weil-descended
  `S₃(t₁, t₂, x_R) = 0` over the line: two equations over `F_p`, resultant in
  `t₂` (nine evaluations of a 4 × 4 determinant, interpolated), roots of the
  resultant by `gcd(t^p − t, ·)` and Cantor–Zassenhaus, the partner `t₂` from
  the common roots, a lift to signed base points. Sparse linear algebra: the
  filter `square_core`, then sequential Wiedemann over `Z/r`, started on the
  first relations that make the core square (the first-dependency stop of
  `research/ic_improve_20261015/04-d4-2lp-fix`).
* **(b) `ic-b`, plus the FGHR 2-torsion symmetry.** On a curve with a rational
  2-torsion point `T = (x₀, 0)` and `c² = f'(x₀)`, `c ∈ F_{p²}`, the line
  `x ∈ x₀ + c·F_p^*` is closed under `P ↦ P + T` (`t ↦ 1/t`); the base is folded
  by `⟨−1, τ_T⟩` (4 points a column, checked by `Closure::Strict`'s port).
  The decomposition system is symmetrised as well (FGHR at `m = 2`): the
  descended polynomial `G(t₁, t₂)` of the shifted line satisfies
  `G(t₁, t₂) = t₁²t₂² G(1/t₁, 1/t₂)` (checked on every call, resultant solver as
  the counted fallback), so it is a polynomial in `σᵢ = tᵢ + 1/tᵢ` and
  `D = (t₁ − 1/t₁)(t₂ − 1/t₂)`; with `S = σ₁ + σ₂`, `P = σ₁σ₂` the two `F_p`
  equations are linear in `(S, P, D)` and `D² = (P + 4)² − 4S²`, so the system
  is a line cut by a quadric: one `F_p` quadratic, then square roots, in place
  of the degree-8 resultant and its `gcd(t^p − t, ·)`. Applies to the torsion
  family below.
* **(c) `ic-c1`, `ic-c2`: (b) with single and double large primes.** The small
  base is a pseudo-random `⌈|columns|^e⌉` subset of the columns, `e = 0.75` for
  one large prime and `e = 0.5` for two; the rest of the line is the large-prime
  domain. Relations with more large primes than allowed are discarded; the rest
  go through the graph eliminator (`LargePrimeEliminator`) and the same
  filter/Wiedemann on the small base.
* **(d) `rho-neg`, matched rho.** Van Oorschot–Wiener on the classes `{P, −P}`:
  r-adding walk (16 jumps below `2^30`, 32 above, jump exponents below
  `2^⌈log₂ r / 2⌉`), distinguished points with `⌊log₂ r / 4⌋` bits, one
  addition per walk start (starts `Z₀ + k·S`), the short-cycle escape by one
  doubling, the answer verified. On GLS curves also **`rho-psi`**: classes
  `{±P, ±ψP}` (order-4 GLS endomorphism, 4-fold).
* **(e) `ic-gls-neg`, `ic-gls-psi`.** On GLS curves, the line `x ∈ u·s·F_p^*`
  folded by `⟨−1⟩` (control) and by `⟨−1, ψ⟩` (arm e: IC folded the same way as
  rho-psi).

### Instruments for the instances

* **torsion family (6 curves a size, `idx` 0 to 5).** `a, x₀` random in
  `F_{p²}`, `b = −x₀³ − a x₀`, `f'(x₀)` a nonzero square, `#E = 4r`, `r` prime
  (order by Mestre/BSGS in the Hasse interval, certified on four more points).
  Cofactor 4 is the design's "cofactor ≤ 4". It is forced, not chosen: with
  `f'(x₀)` a square (needed for `c ∈ F_{p²}`), `T` lies in the image of the
  2-descent map's kernel, so `4 | #E`; a development probe on 2,017 random such
  curves at `p = 3989` found `v₂(#E) ≥ 2` on every one, against 1,001 of 1,983
  curves with `f'(x₀)` a non-square at `v₂(#E) = 1`. Arms (a), (b), (c1), (c2),
  (d).
* **prime family (3 curves a size, `idx` 0 to 2).** Random `a, b`, prime
  `#E`. Arms (a), (d). Arm (b) does not apply (no 2-torsion).
* **gls family (3 curves a size, `idx` 0 to 2).** `E : y² = x³ + ax + b` over
  `F_p`, the twist over `F_{p²}` by a non-square `u`, `#E' = (p − 1)² + t²`
  with `h ≤ 4`, `ψ` verified on a random point (on the curve, `ψ² = −1`) and
  its eigenvalue `λ` decided on `G`. Arms (e) with (d) `rho-neg` and
  `rho-psi`.
* Sizes: `log₂ #E` nominal `{24, 28, 32, 36, 40}`, `p` a random prime in
  `[0.72, 1) · 2^{bits/2}`; the fit uses each curve's actual `log₂ r`.

## Unit and pricing

* **Priced work, in group-addition equivalents (GAE).** Index-calculus arms
  count every `F_p` multiplication and inversion they perform (field
  arithmetic, curve additions, the oracle's polynomial algebra, root
  finding, base lifts and square roots, the walk, scalar multiplications,
  verification), inversions at `--inv-cost 4` multiplications, and divide by
  `c_add`, the measured multiplication equivalents of one affine addition of
  two random distinct points of the same curve (about 24 at inv-cost 4;
  `c_add` at inv-cost 2 and 8 is recorded in every row). Rho counts group
  additions and doublings, one GAE each, starts, table and verification
  included.
* **Unpriced native work (AGENTS.md §5a).** Listed in every row by name and
  count, never folded into the priced figure, and priced at 0, 0.1 and 1 GAE per
  operation to give three totals `w0`, `w0.1`, `w1`:
  index calculus `la_modr_ops` (Wiedemann and Berlekamp–Massey multiply-adds mod
  `r`), `elimination_merge_modr_ops`, `filter_nnz_touches_lower_bound` (a lower
  bound: the library filter does not count its passes; an implementation gap),
  `abscissa_lookups`, `walk_step_hashes`, `target_dedup_probes`,
  `base_columns_hashed`; rho `canonicalisations_uncharged`,
  `negations_examined_uncharged`, `partition_hashes_uncharged`,
  `dp_table_ops_uncharged`, `psi_maps_uncharged`. Every arm, rho included, is
  priced the same way. Two arms are ranked only where the order holds at all
  three weights. No calibrated cost is compared with an uncalibrated one: the
  pinned calibration table is not used.
* **Metric.** `S = cost / √r` per arm and weight; the exponent `α` in
  `cost ∝ r^α` by least squares of `log₂ cost` on `log₂ r`; the gap to rho per
  size (ratio of the mean over curves) with a 95 % cluster-bootstrap interval
  (clusters are curves, 2,000 resamples, targets stay with their curve).

## Targets, controls, cross-checks

* **Targets.** Index-calculus arms: targets 0 and 1 of every curve at every
  size. Rho: targets 0 to 7 of every curve at
  every size. A target is a planted `d` drawn from the curve and the target
  index; the arm sees only `G` and `Q = [d]G`. Targets are walk points
  `R = [a]G + [b]Q` (one addition each, a fresh random start every 64 targets);
  a target whose abscissa was seen before is a rho collision, not a relation,
  and is skipped (counted).
* **Positive control.** Every solved row must recover the planted `d`
  (`verified`); the `smoke` run at 24 bits on every arm and family checks the
  same before the cards, and the summary reports solved and verified counts.
  An unverified or exhausted row is reported, excluded from the fits, and an
  arm with more than 5 % unverified rows in a cell is INCONCLUSIVE there.
* **Oracle cross-check.** On the first `K` trials of every `ic-a`, `ic-b`,
  `ic-gls-*` run the oracle's answer (a decomposition exists or not) is
  compared with exact enumeration over the base (uncounted): `K` = all trials
  at 24 bits, 400 at 28, 60 at 32, 30 at 36, 10 at 40. Zero disagreements
  required.
* **Negative control.** The arm (a) pipeline on a base of the same number of
  columns whose points are drawn from the whole curve (no line structure),
  decomposed by exact enumeration, charged one addition per base point per
  target: full solves at 24 and 28 bits (torsion curves 0 to 2, target 0); the
  decomposition rate only (`control-shuffled-rate`, 200 or 300 targets) at
  32, 36, 40 bits (torsion curves 0 to 2). The null: the shuffled base has the
  same decomposition rate as the line base (it is a property of the base size
  `|F|²/#E`), at a cost per target of `2·columns` additions instead of about
  150. The control passes if its rate lies within 0.08 of arm (a)'s on the same
  curve and, at the two full sizes, `S_control / S_a ≥ 10`.

## Decision rule (frozen)

The mechanical read-out is `summarize`'s `summary.json`. Per index-calculus arm,
per weight `w ∈ {0, 0.1, 1}`, against the matched rho of its family (`rho-neg`
for `ic-a`, `ic-b`, `ic-c1`, `ic-c2`, `ic-gls-neg`; `rho-psi` for `ic-gls-psi`,
and `rho-neg` as well):

1. **Exponent level.** `ADVANCES (exponent)` if the arm's `α` 95 % interval lies
   below rho's at every `w`; `REGRESSES (exponent)` if it lies above at every
   `w`; `MATCHES (exponent)` if the intervals overlap at every `w`;
   `INCONCLUSIVE` if the classification changes inside the band. The all-size
   fit decides. The fit on the three largest sizes and the analytic rho
   asymptote (reported alongside) are the bias check: rho's all-size `α` is
   pulled below 1/2 by its fixed set-up at small `r`, so an all-size
   `REGRESSES` that the top-three fit does not reproduce is recorded as
   `INCONCLUSIVE (exponent)` with that reason.
2. **Constant level (distance remaining).** The per-size gap is reported, not
   gated. For the vocabulary: at the largest size, gap interval entirely above 1
   at every `w` is `REGRESSES (constant)`, entirely below 1 is
   `ADVANCES (constant)`, otherwise `INCONCLUSIVE`.
3. **Extrapolated FourQ gap.** At `log₂ r = 246` (`p = 2^127 − 1`, FourQ's
   246-bit prime subgroup), from the two fitted lines with the bootstrap
   interval propagated; reported either way, labelled extrapolation. The
   sensitivity with rho at its analytic `√(π r / 2A)` steps is also reported.
4. **Item verdict.** The best arm's level pair, e.g. `MATCHES (exponent),
   REGRESSES (constant)`. A `TRADES` needs one level better and one worse. The
   README states the verdict, the weights at which it holds, and the
   extrapolated gap; nothing is promoted beyond the cells.

## Budget

* One thread per job, `prlimit --as=4294967296`, one `rustc` build a card (about
  10 s). Every card's command timeouts sum to at most 540 s.
* Measured work expected about 0.35 CPU-hour (limit 2): at 40 bits the four
  torsion-family arms and rho take about 80 s a curve for two targets; the control's full
  solves are cut at 28 bits because their cost grows as `p²` (a 32-bit full
  control took 236 s in a development probe).
* No GPU, no external solver; msolve, Magma and Sage are not used (the
  decomposition is the repository's Weil-descent line oracle, not a Gröbner
  basis).

## Pre-protocol runs (not cited)

Development runs of the instrument before the cards were frozen, all
unregistered: the `smoke` run at 24 bits and 28 bits; timing probes at
torsion `b24 i1`, `b28 i0 i1`, `b32 i0 i1`, `b36 i0`, `b38 i0`, `b40 i0`, gls
`b40 i0`, prime `b40 i0` (their outputs were read, so the same cells are
re-run in the cards and only the card outputs are cited); and probes on
curves outside the cell set (`idx ≥ 100`). They found and fixed two defects in
this instrument: (1) the target walk can merge with an earlier walk at
`~√r` targets, after which every target repeats and the filter sees duplicate
rows (a rho collision counted as a relation); the walk now restarts every 64
targets and a target whose abscissa repeats is skipped; (2) after a failed
Wiedemann attempt the same square core was retried on every later relation; an
identical selection is now not re-run. Neither fix was chosen to move a verdict:
both remove cost the library implementation would also avoid. (3) The
FGHR system symmetrisation of arm (b) was added after those probes had shown
the resultant solver at about three quarters of the oracle's cost: arm (b) as
the design states it includes the symmetrisation, and the first version of the
instrument had only its base lever. The symmetric solver agrees with the
resultant solver and with exact enumeration on 300 of 300 targets (smoke test,
24 and 28 bits) at 862 against 4,958 multiplication equivalents a call; the
cards bind the instrument that has it. This protocol file was edited for it
before any card ran.

## Deviations from the design (and why)

Tightenings:

1. Targets: 2 a curve (IC) and 8 a curve (rho) at every size,
   against the design's one: the extra targets are cheap and reduce the noise
   of a first-dependency stop. The curve stays the unit of replication.
2. Curves: 6 torsion-family (cofactor 4) + 3 prime-order + 3 GLS a size. The
   design's "6 random curves of prime order (or cofactor ≤ 4)" is met by the 6
   torsion curves; the 3 prime-order curves add arm (a) on the other shape.
3. Cross-check and positive control as above (all trials at 24 bits).
4. The `top-three` fit, the analytic-rho sensitivity, and the three-weight band
   are added to the design's read-out.

Forced by the budget or by the repository state (each reduces coverage):

5. The library does not compile; the instrument is a `std`-only port
   (above). Differences from the audited code: the `S₃` oracle, the fold and the
   linear algebra are re-written from the library's source, and the symmetric
   solver of arm (b) is new code (no library counterpart at `m = 2`); `F_p`
   multiplications are counted inside every routine (the library counts them
   in the oracle and in the group, and prices the framework's `solver` term by
   wall time, which this item does not use); rho is a fresh Van Oorschot–Wiener
   implementation (starts by stride, jump exponents below `2^⌈log₂ r / 2⌉`,
   cycle escape by doubling) whose steps over `√(π r / 2A)` lies near 1.0 to
   1.1 on development probes (not the library's `rho_reference`).
6. The negative control is a full solve only at 24 and 28 bits (3 curves each);
   at 32, 36 and 40 bits only the decomposition rate is measured, because the
   full solve costs `2·columns` additions a target (`∝ p²`).
7. The oracle cross-check is exhaustive only at 24 bits; beyond it a prefix of
   each run is checked.
8. Large-prime parameters `e = 0.75` (single) and `e = 0.5` (double) are fixed
   before the measurement, not tuned per size.
9. FourQ extrapolation uses `log₂ r = 246` for every family.

## Outputs

`raw/<family>-b<bits>-*.jsonl` (one record per arm and target, write-once),
`raw/instances-*.txt` (the cell's curves), `raw/summary.json`,
`raw/receipt-card-NN.json`, then, by the assess stage, `README.md` (verdict by
the frozen rule), the SVG and PNG graph, `REPORT.pdf`, and
`benchmarks/20261018-blindspots-fp2-gaudry-k2/` with its row in
`benchmarks/README.md`; the integration step regenerates the leaderboard
sections that cite it.
