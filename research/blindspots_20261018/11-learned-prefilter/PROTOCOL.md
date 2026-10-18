# Item 11 protocol: a learned necessary-condition prefilter on x_R for subspace bases

Frozen 2026-10-18, before any measured run. Source design: section "11" of
`research/blindspots_20261018/EXPERIMENTS.md` and its "Rules every item
follows". Written by the experiment-director lane. The design is tightened in
places and loosened nowhere; every deviation is listed at the end.

Class: **stage diagnostic** (a necessary-condition filter in front of an exact
decomposer, on toy fields). It computes no `S`, no rho ratio and no GAE, so the
section 5a cross-method ranking does not apply to its own rows. Its cost
columns are nevertheless priced the §5a way: the field multiplications and
squarings of the filter and of the decomposer are the counted unit (weight 1),
and every unpriced native word operation (F_2-linear maps, bit tests, popcounts,
model arithmetic) is priced at 0, 0.1 and 1 field operations, the same for the
filter and the decomposer, and a conclusion is drawn only where it holds at all
three weights. The gap to matched rho is not changed by this item and is
reported as the unchanged distance remaining.

## Question

Is there a cheap function of the target abscissa `x_R` that predicts "no
`m`-decomposition of `R` over this subspace base" better than the trace-zero
baseline?

## What already exists, and what is reused

* **The baseline is a theorem, not a hypothesis.** For a binary curve
  `E_{a,b}: y^2 + xy = x^3 + a x^2 + b` over `F = GF(2^n)`, the map
  `delta(P) = Tr(x(P) + a)`, `delta(O) = 0`, is a homomorphism
  `E(F) -> F_2` (its kernel is `2E(F)`). It is stated in
  `research/sat_factor_base_review_20260908/MATHEMATICS.md` (line 111: "sum_i
  Tr(x_i) = Tr(x_R) + (m+1) Tr(a)"), used as a draw-consistency check in
  `research/ic_rr_norm_ladder_20260930/RESULTS.md` (line 132) and
  `research/ic_gb_ladder_20261003/PREREGISTRATION.md` (line 37), and listed as a
  "Trace and linear-part prefilter" lead (candidate 3) in
  `research/decomposition_programme_20261016/PLAN.md`. No committed result
  measures how many targets it refutes, nor whether anything beats it. This item
  is the first measurement; nothing is skipped because of an earlier result.
  The instrument nevertheless re-proves the identity numerically (self-test) and
  counts false refutations of arm (b) in every cell (it must be 0).
* **The exact decomposer is built here.** The repository has Semaev/Groebner
  decomposers (`src/cryptanalysis/ic_framework`, `binary_semaev*.rs`), but none
  gives a labelled `m`-decomposability bit per target at `n` up to 29 with a
  cost counter, and `cargo build --release --lib` fails at the branch base (25
  errors in `ecbench` and `gaudry_quartic`, as items 01, 02, 04, 07 recorded),
  so no code that links the crate can run. The decomposer is a point-addition
  scan (m = 2) and a meet-in-the-middle scan against a bitset of all pair sums
  (m = 3), validated against brute force (Controls).
* No cell or arm is already measured; no earlier result is reused as a cell.

## Objects

* `F = GF(2^n)`, `n` odd, with the repository's sparse irreducible polynomial
  (same candidate order as `find_irreducible_sparse`). `E_{0,b}:
  y^2 + xy = x^3 + b`, `b` a seeded non-zero element (Deviation 3: `a = 0`).
* The base for an `F_2`-subspace `V <= F` of dimension `l`:
  `B = { P in E(F) : x(P) in V }`, closed under negation, containing the
  2-torsion point `(0, sqrt b)`.
* **Label.** `R` is `m`-decomposable over `B` iff `R` lies in the `m`-fold
  sumset `B + .. + B` (finite summands, repeats and degenerate sums such as
  `P - P + Q` allowed). This is the exact zero set of `S_{m+1}(x_1..x_m, x_R)`
  over `V^m` with the usual degenerate branches. It is decided exactly:
  * `m = 2`: for every `x`-class `P` in `B`, test `x(R + P)` and `x(R - P)` for
    membership in `V` (`R = +-P` handled: the only finite candidate is `2R`);
  * `m = 3`: a bitset over `F` of all `x(P + Q)`, `P, Q in B` (including
    `x(2P)`), then for every `P` test `x(R +- P)` in the bitset; `x_R in V`
    (then `R = P + Q - Q`) is positive.
  Batch inversion (Montgomery) is used, so a scan costs about 5 multiplications
  and 2 squarings per class.
* **Target populations** (the design is silent on the distribution; both are
  measured, as separate datasets with separate classifiers):
  * **U**: `R` uniform over the affine points of `E(F)`. The trace filter can
    refute here: half of all targets have `delta(R) = 1`.
  * **S**: `R` uniform over `2E(F)` minus `O` (`delta(R) = 0`). This is where a
    real index-calculus target lives, because a target `kP` with `P` of odd
    prime order is a double. The trace filter is vacuous on S by construction
    (it can only refute through the base, never through the target), so every
    refutation a learned filter earns on S is new.
* **Bases.** Ten bases per cell: four of type **T** (`V` a random subspace of
  the trace-zero hyperplane, so arm (b) applies), three of type **G** (generic
  random subspaces with a trace-1 element, so arm (b) is vacuous) and three of
  type **P** (geometric `V = <1, beta, .., beta^(l-1)>`, the programme's
  incumbent base; `Tr(1) = 1`, so arm (b) is vacuous). 2,000 targets per base
  and population, so **20,000 labelled targets per cell and population**.
* **Folds, split by base.** Five folds, two whole bases each:
  `{T0,G0} {T1,P0} {T2,G1} {T3,P1} {G2,P2}`. Every target is tested exactly
  once, by a classifier that never saw its base. On the other eight bases a
  deterministic 75/25 split of targets gives a fit set and a calibration set.

## Arms

* **(a) no filter**: refutes 0 targets; the decomposer pays its full scan on
  every negative.
* **(b) trace filter where it applies**: `D = { delta(P) : P in B }` is computed
  exactly from the base; the `m`-fold sumset `D^(+m)` is the set of values
  `delta(R)` can take if `R` is `m`-decomposable; `R` is refuted iff
  `delta(R) not in D^(+m)`. A trace-zero base gives `D = {Tr(a)}`; any base
  with elements of both `delta` values gives the whole group and refutes
  nothing. Zero false refutations is a theorem; the instrument counts them.
* **(c) learned classifier**, two models, both written in Rust:
  logistic regression (standardised features clipped to +-8, Adam, 30 epochs,
  batch 128, lr 0.02, L2 1e-4) and gradient-boosted trees (depth 3, 100 rounds,
  eta 0.1, lambda 1, 16 quantile bins, minimum child hessian 2). Features of
  `x_R` (`4n + 48` columns), by group:
  1. polynomial-basis bits (`n`);
  2. normal-basis bits (`n`; a normal basis found by seeded search);
  3. absolute traces `Tr(x^k)` and `Tr(x^-k)` for odd `k = 1..31` (32);
  4. Hilbert 90 indicators `[Tr(z) = 0]` (i.e. `z in {w^2 + w}`) for `z = x`,
     `x + a`, `1/x`, `b/x^2`, `x + a + 1/x`, and `x + a + b/x^2` (6; the last
     is the point-existence condition, constant 1 on real targets);
  5. shifted-subspace membership `[x + s in V]`, `s in {0, 1, a, a+1}`, and the
     Hamming weight of `x`, `x + 1`, `x + a` reduced modulo `V` (7);
  6. half-trace bits of `x` and of `x + a + b/x^2` (`2n`; the latter is the
     `y/x` coordinate of `R` up to sign);
  7. context (3): the compatibility bit of arm (b) (1 = not refuted by (b)),
     `[Tr|_V = 0]`, `Tr(a)`.
  Group 7 hands arm (b)'s verdict to the learner, so that a learned filter can
  at least reproduce (b); beating (b) then needs genuinely new structure.
  The score is the model margin; **a target is refuted when its score is below
  the minimum score of any positive in the calibration set** (zero false
  refutations on the calibration set by construction).
* **(d) label-permutation null**: 20 permutations of the labels **within each
  base** (so base-level positive rates survive), the identical pipeline and
  folds, for both models. The null band of a statistic is its maximum over the
  20 permutations.
* **(e) positive controls** (planted labels, same pipeline and folds, each model):
  * `planted_trace`: label `[Tr(x_R) = 0]` on U (the design's planted label);
    on S `Tr(x_R) = Tr(a)` is constant, so the planted label is
    `[Tr(1/x_R) = 0]` there (Deviation 5). The classifier must recover it:
    AUC >= 0.99 and an oracle-threshold refutation rate >= 0.9.
  * `planted_selector`: label = `Tr(x^3) = 0` if `Tr(1/x) = 0` else
    `Tr(x^5) = 0`, an interaction a tree finds and a linear model only
    approximates; the gradient-boosted model must reach AUC >= 0.95 (checked by
    the self-test; reported per cell).

## Metrics (per cell, population, model and base type: T, G, P, and ALL)

* **AUC** on held-out bases, Mann-Whitney with tie correction, averaged over
  the folds that contain both classes.
* **Refutation rate** `rate_c`: the fraction of negative held-out targets with
  score below the calibration threshold, **credited only if the pooled number
  of false refutations (decomposable held-out targets with score below the
  threshold) is 0; otherwise `rate_c` is credited as 0** and the false-refutation
  count is reported. `roc0` is the same fraction with the threshold set at the
  held-out minimum positive score (an oracle, optimistic, reported to show what a
  perfectly calibrated threshold would give; never used for the rule's credit).
* **rate_b**: the refutation rate of arm (b) on the same negatives.
  `delta = rate_c - rate_b` in absolute points, with a 95% paired bootstrap
  interval (1,000 resamples of the negatives).
* `rate_union`: refuted by (b) or by (c), descriptive only.
* **Cost** (counted per target): field multiplications and squarings and word
  operations of the features, model operations (logistic regression `2d`,
  trees `rounds x (depth + 1)`), field operations and membership/bit tests of
  the decomposer, separately for negatives and positives. The `verdict`
  subcommand prints the net work at the three weights.

## Controls

* **Instrument self-test** (`selftest`, run in the first card and as the smoke
  test): field identities for `n` = 11, 17, 19, 23, 29 (multiplication by carry-less
  multiply equals the shift-and-add reference, inverse, trace, half-trace,
  normal-basis map bijective); group-law identities, `N P = O`, Hasse bound,
  `delta` additive, `{delta = 0} = 2E(F)` on `E(F_{2^11})` for `a = 0, 1`; **the
  exact decomposer equals a brute-force `m`-fold sumset on every point** of
  `E(F_{2^11})` and `E(F_{2^13})`, for `m = 2, 3`, all three base types and both
  `a`, with arm (b) never refuting a decomposable point; the learners recover
  a planted label, give AUC in [0.45, 0.55] on permuted labels, and the trees
  recover the planted selector.
* **Negative control**: the within-base permutation null (d).
* **Positive control**: (e), per cell, population and model. A model whose
  `planted_trace` control fails in a cell contributes no pass from that cell.

## Cells and the choice of `l`

`n in {17, 19, 23, 29}`, `m in {2, 3}`, populations `U` and `S`: sixteen
cell-populations. `l` is the transition size, found by measurement and not
guessed: the Poisson heuristic `lambda = 2^(m l) / (m! 2^n)` gives the centre
`l_0` nearest `lambda = ln 2`; the cell then runs a **pilot** (400 targets on
two generic bases, not reused) at `l_0 - 1, l_0, l_0 + 1` and takes the `l`
whose positive fraction is closest to one half (ties to the smaller `l`). The
pilot lines are in the output. `plan` prints the heuristic.

Seeds: `cell_seed = 20261018 ^ n<<40 ^ m<<32` (xor `1<<24` for S). The curve
(`b`) is seeded without the population, so U and S of a cell share the curve;
their pilots, `l`, bases and targets are independent draws. Results are
deterministic given the binary and these seeds.

## Decision rule (frozen; the `verdict` subcommand applies it mechanically)

A (model, base type) pair **passes in a cell and population** iff all of:
the cell's `planted_trace` control for that model passed; pooled false
refutations 0; `delta >= 0.05` (5 points) and the lower end of its 95%
interval `> 0`; `roc0` and AUC both above the maximum of the 20-permutation null
band.

* **ADVANCES (primitive)** iff some (model, base type incl. ALL) passes in **at
  least two distinct (n, m) cells of the same population** (tightening:
  replication). Any such feature is then proven algebraically before any use,
  and the proof is a separate item.
* **INCONCLUSIVE** if the planted control fails everywhere in a population, or
  if the rule is met in exactly one cell for some arm (a single hit is not
  replication).
* **REGRESSES** if on the trace-applicable bases (U, type T, `rate_b > 0`) the
  learned filter alone refutes more than 5 points fewer targets than (b) in
  every cell and gains nowhere.
* **MATCHES** otherwise: nothing beats (b) by the rule.
* The overall verdict is the best of the two populations in the order
  ADVANCES > INCONCLUSIVE > MATCHES > REGRESSES (an ADVANCES on S is the
  deployable finding; U alone would be a re-derivation of structure the trace
  already carries on a population that real targets do not populate, and the
  assess step must say which one it is).

## Budget and execution

One thread, `prlimit --as=4294967296` per job. Peak memory is the `2^n`-bit pair
table at `n = 29, m = 3` (64 MiB) plus the feature matrix (about 130 MiB at
`n = 29`). Measured budget at most 2 CPU-hours; the expected total is a small
fraction of that (see the card plan). Each card finishes in under nine minutes.
Build products go to `target/blindspots-11/<card>/` and are removed at the end of
each card. Nothing in `src/` or `Cargo.toml` is touched.

## Deviations from the frozen design

1. **`rustc`, not `cargo build --example`**: the crate does not build at the
   branch base (25 errors), so the example is std-only and compiled by `rustc
   --edition 2021 -C opt-level=3 -C codegen-units=1 -C target-cpu=native` in the
   first card. Same as items 04 and 07.
2. **Two target populations (U and S)**, an addition: the design does not say
   how targets are drawn, and the trace baseline behaves oppositely on the two.
   Both are measured, 20,000 labelled targets each per cell.
3. **`a = 0`** (Koblitz-form family) in every measured cell. With `a = 1` and
   odd `m` the trace-zero bases have `delta(R) = 0` outside the sumset, so no
   subgroup target decomposes over them and population S would have a class
   with no positives. The self-test covers `a = 1`.
4. **Ten bases of three types per cell**, with folds split by base: the design
   says "class-split by base"; this is the reading chosen (held-out bases,
   the stricter one). A within-base split is not run.
5. **Planted label on S** is `Tr(1/x_R)` parity, since the design's `Tr(x_R)`
   parity is constant there. The selector control is an addition.
6. **Group 7 (context) features** include arm (b)'s own verdict. The design's
   feature list is features of `x_R`; the base descriptor is needed so that a
   pooled classifier can reproduce (b) at all.
7. **`l` by pilot**, not by a formula alone (the design says "the transition
   size").
8. **Replication across two cells** is required for ADVANCES (design: one
   cell would do).
9. Zero false refutations is **calibrated** on unseen targets of seen bases and
   **verified** on unseen bases; any false refutation voids that arm's credit
   (the design says "at 0 false refutations on held-out targets", which an
   oracle threshold would trivially meet; `roc0` is reported but not credited).

## Outputs

`research/blindspots_20261018/11-learned-prefilter/`: `PROTOCOL.md`,
`card-NN.yaml` (frozen, linted), `raw/` (JSON-lines per cell and population,
`selftest.txt`, `verdict.txt`, receipts), then `README.md` (verdict by the
frozen rule), an SVG and PNG graph, and `REPORT.pdf` after the runs. No leaderboard
row and no `benchmarks/` entry is warranted (a stage diagnostic with no
speed claim); the integration step decides.

## Card plan (appended 2026-10-18, before any measured run; adds no design change)

Instrument: `examples/blindspots_11_learned_prefilter.rs` (std-only Rust,
compiled by `rustc`, Deviation 1). Smoke test, at the smallest cell with
reduced sizes (n = 17, m = 2, population U, 300 targets per base, 3
permutations, 100 pilot targets; not a measured result, not used by the
verdict): `smoke/smoke-n17-m2-U-per300-perms3.jsonl`, and the instrument
self-test `smoke/selftest.txt` (0 failures; decomposer equals brute force on
every point of `E(F_2^11)` and `E(F_2^13)`; planted-label positive controls
pass for both models; permuted labels give AUC in [0.45, 0.55]). The smoke
run's `planted_trace` control passed for both models (AUC 1.0, refutation
rate 1.0). Timings from reduced runs (single thread) set the split below.

Every measured cell uses `--per-base 2000 --perms 20 --pilot 400` (the code
default of 10 permutations is overridden to the frozen 20) and lets the pilot
choose `l`. One thread, `prlimit --as=4294967296` on every job. Each card
builds its own binary into `target/blindspots-11/card-NN/` and removes it at
the end (a binary that lingers in the tree would fail the card linter's
"names a repository file that binds does not pin" rule on the next card).

| card | contents | estimated run time |
| --- | --- | --- |
| 01 | build, `selftest`, (n,m,pop) = (17,2,U) (17,2,S) (17,3,U) | about 4 min |
| 02 | build, (17,3,S) (19,2,U) (19,2,S) (19,3,U) | about 5 min |
| 03 | build, (19,3,S) (23,2,U) (23,2,S) (23,3,U) | about 6.5 min |
| 04 | build, (23,3,S) (29,3,U) (29,3,S) | about 6 min |
| 05 | build, (29,2,U) (29,2,S) | about 5 min |
| 06 | build, `verdict --dir raw` (reads every `raw/*.jsonl`), remove binary | seconds |

Total measured about 27 CPU-minutes, far under the 2 CPU-hour budget. The
sixteen cell-populations of the frozen cell list are each run exactly once;
no cell is dropped, merged or re-run. Raw outputs: `raw/n<N>-m<M>-<pop>.jsonl`
(stdout) and `.log` (stderr), `raw/selftest.txt`, `raw/verdict.txt`,
`raw/receipt-card-NN.json`.

### Card plan, final split (supersedes the table above; same sixteen cells)

Seven cards, each with a timeout sum under nine minutes (at most 525 s) and an
estimated run time of 4 to 6.5 minutes. Card 01: build, `selftest`, (17,2,U)
(17,2,S) (17,3,U). Card 02: (17,3,S) (19,2,U) (19,2,S). Card 03: (19,3,U)
(19,3,S) (23,2,U). Card 04: (23,2,S) (23,3,U) (23,3,S). Card 05: (29,3,U)
(29,3,S). Card 06: (29,2,U) (29,2,S). Card 07: `verdict --dir raw` only.
Every card builds its own binary (`target/blindspots-11/card-NN/b11`) and
removes it at the end. Total measured about 27 CPU-minutes.

## Repair pass (appended 2026-10-18, after the first run; no change to the question, arms, cells, metric or decision rule)

First-run outcome: cards 01, 02, 04, 06, 07 completed with receipts. Cards 03 and 05 returned
early (the executor used the 2-minute shell timeout and moved the run to the background), so
neither has a receipt; of their five cells, only the first command of each finished
(`raw/n19-m3-U.jsonl`, `raw/n29-m3-U.jsonl`, complete but unreceipted). Cells (19,3,S), (23,2,U)
and (29,3,S) were never measured. Card 07 completed but saw 13 files, so its `verdict.txt` is
stale. There were no refused cards. No instrument bug was shown by the first run: the instrument
is unchanged (sha256 e6f23662...), no deviation to the instrument is made, and every
completed card's numbers stand.

Revision cards (same arguments as the frozen cells, outputs at new `-r2` paths so nothing from
the first run is overwritten; every card builds and removes its own binary and has the explicit
10-minute shell timeout in its notes):

| card | contents | expected wall time |
| --- | --- | --- |
| `card-03-r2` | build, (19,3,S), (23,2,U): two never-measured cells | about 4 min |
| `card-05-r2` | build, (29,3,S) never measured, (29,3,U) re-run | about 5 min |
| `card-03b-r2` | build, (19,3,U) re-run | about 2.5 min |
| `card-07-r2` | staged: frozen by `make_card_07_r2.py` only after the three cards above have run and their outputs are committed; builds, copies the sixteen cell files into a throwaway directory, runs `verdict` there | about 1 min |

The two re-runs (19,3,U) and (29,3,U) are not new cells: they have the same arguments and seeds,
so the run is deterministic and should reproduce the unreceipted first-run files; they exist to
give those cells a hash-attested receipt, and the `-r2` files replace the unreceipted first-run
files in the verdict (the assess step reports whether the numbers are identical to the first-run
files; this is not interpreted here). `card-07-r2` copies exactly sixteen files (eleven receipted
first-run files plus the five `-r2` files) because `verdict` reads every `*.jsonl` of a directory
and would otherwise count the two re-run cells twice. The 13-file `raw/verdict.txt` is kept as
history. Smoke test of the unchanged instrument at the smallest draw count (`--per-base 100 --perms 1
--pilot 40`) on all five re-run cells: exit 0, one `done` line, no panic, 69 MiB peak at n = 29.
Cards that run from the same host should not overlap more than two at a time.
