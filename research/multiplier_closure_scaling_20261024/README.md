# Multiplier-closure bases: counted-S scaling vs their incumbents

**Date.** 2026-10-24. **Protocol.** [PROTOCOL.md](PROTOCOL.md) (frozen in commit 8795a8e1fd; amendment 1 CPU-time cap and
shared target dir, 01ebdd3f05; amendment 2 point cap for compute budget, fdfce25b61; all before any recorded run; the one
abandoned 24-bit probe under the unamended grid is unrecorded and unused). **Amendment 2 cut the admissible multiplier grid to three cells (prime: `cm-multiplier` size=256 norm=4 or 8, size=512 norm=4; binary: size=256 mult=2 or 4, size=512 mult=2; i.e. N <= 8, M <= 4) and dropped the 40-bit size, so `R_adm` cannot test larger-norm or larger-multiple bases and says nothing about them.** **Status.** Unreviewed, unvalidated, local branch
`research/multiplier-closure-scaling-20261024`; not an admitted benchmark.
**Raw.** `raw/*.jsonl` (3,768 lines: one per configuration x repeat), `analysis.md`/`analysis.json` from `analyze.py`.
Run: `runall.sh` (calls `run.py`). Binary: `ic` sha256 59a70e5a...8c68b417 (shared target
`cargo-target-ic-followthrough-20261024`, origin/main 5cb553810e). Metric: counted `S = total_gae / sqrt(r)`, mean of 4 targets;
oracle `mitm:negation_folded=1,m=3`; every row `verified` (0 failures; PROTOCOL.md rule also asks for `correct`, but the raw rows carry no `correct` field and `analyze.py` checks only `verified`, which is the bench's own end-to-end logarithm check); wall time not used (load 25-150).

## Result (frozen decision rule applied)

**Frozen-rule outcome.** Slope of `log2(S_mult/S_inc)` against `log2 r` is negative in every primary fit. H is formally
**falsified** (95% interval entirely below 0) for the prime regime (pooled per-instance R_all -0.47 [-0.56, -0.38], R_adm
-0.24 [-0.31, -0.16]) **and for binary** (primary per-instance fit R_all -0.203 [-0.247, -0.158], R_adm -0.126 [-0.183, -0.069]).
The only intervals that span 0 are secondary fits (binary R_adm size-means, n = 4; j0 R_adm size-means).

**Reading.** No scaling or exponent claim. The slopes are grid-placement artefacts: the admissible multiplier minimiser is
the single smallest admissible cell in 33 of 33 instances (`cm-multiplier` size=256 norm=4 on prime; `binary-multiplier`
size=256 mult=2 on binary) and the admissible incumbent minimiser is on a grid edge in 29 of 33 (`glv-orbit` size=400 in 17 of
21 prime instances, size=800 in the other 4; plain K=256 in all 12 binary), so `R_adm` is a fixed-cell ratio, not best-vs-best.
"The multiplier base scales better" is not supported, for three reasons the numbers show:

1. **Low-r rows are the other way round.** At r <= 2^26 the best multiplier base is *worse* than the incumbent on j0
   (0.25x at 24 bit, 1.5x at 28 bit) and the 10-20x of the READMEs appears only at 2^32..2^34 (6.3x, 12.8x on j0; 11.7x j1728).
2. **The gain lives in the below-256-column regime.** The best all-rows multiplier cell is nearly always K = 4 columns
   (5 relations) with ~960 points, i.e. a baby-step/giant-step table over multiples (S ~ 10 flat in r over 2^21..2^34).
   Restricted to bases with >= 256 columns on both sides (`R_adm`), the multiplier base is *worse* than the incumbent
   (gain 0.26x..0.49x) at every size except j0 36 bit (2.07x) and j1728 32 bit (1.39x). Only 99 of 750 multiplier cells
   (13%) have >= 256 columns; 106 of 192 incumbent cells do.
3. **Grid-edge confounds.** The multiplier minimiser sits on the smallest-points edge of its grid in 22 of 33 instances (never on the upper edge),
   and the incumbent at its smallest edge in 9 of 33 (small-r rows; never the upper edge); those S values are upper bounds (the K*N >= 256 and
   points-cap rules limit how small a multiplier base can be). The unflagged subset (post-hoc, n = 1..6) is too small to
   fit on prime and gives slope +0.10 [-0.41, +0.60] pooled prime (n = 4), -0.27 binary (n = 6).
   The slope is therefore a statement about where each grid's optimum falls in r in [2^18, 2^34], not about exponents:
   both are the same table pipeline and the exponent is unchanged; a flat S ~ 10 for a 960-point table cannot persist
   as r grows (relation cost ~ r / (pairs * points) per relation rises).

Admissible rows (>= 256 folded columns): 205 of 942 distinct configuration cells (106/192 incumbent, 99/750 multiplier);
admissible instance pairs 33 of 33, but `R_adm` fits are testable (>= 4 sizes) for prime j0, pooled prime and binary only
(j1728 has 3 sizes: not testable).

Binary at r in 2^18..2^24 (degree 20..26): all-rows gain 1.4x, 2.3x, 2.6x, 3.8x (README claimed 3-7x at degree 26-28).

## Per-size table (geometric mean over seeds of R = S_mult/S_inc; lower R = bigger multiplier gain)

| regime | size | seeds | log2 r | best inc S (cols,pts) | best mult S (cols,pts,rows) | R_all (gain x) | adm pairs | R_adm (gain x) | boundary-flagged |
|:--|--:|--:|--:|:--|:--|--:|--:|--:|--:|
| binary random | 20 | 3 | 18.0 | 18.6 (64,128) | 13.6 (8,128,8) | 1.38x | 3 | 0.26x | 3/3 (mult-lo:3) |
| binary random | 22 | 3 | 19.0 | 21.8 (64,128) | 9.5 (4,128,5) | 2.29x | 3 | 0.26x | 3/3 (inc-lo:1 mult-lo:2) |
| binary random | 24 | 3 | 22.0 | 27.8 (128,256) | 10.6 (4,256,5) | 2.63x | 3 | 0.29x | 0/3 (-) |
| binary random | 26 | 3 | 24.0 | 32.8 (256,512) | 8.6 (4,256,5) | 3.80x | 3 | 0.49x | 0/3 (-) |
| prime j0 | 24 | 3 | 20.9 | 36.7 (50,300) | 151.6 (32,960,25) | 0.25x | 3 | 0.27x | 3/3 (inc-lo:3 mult-lo:3) |
| prime j0 | 28 | 3 | 25.7 | 52.1 (100,600) | 33.9 (16,960,16) | 1.54x | 3 | 0.27x | 3/3 (inc-lo:3 mult-lo:3) |
| prime j0 | 32 | 3 | 31.9 | 68.4 (400,2400) | 10.5 (4,960,5) | 6.34x | 3 | 0.45x | 3/3 (inc-lo:1 mult-lo:3) |
| prime j0 | 36 | 3 | 33.7 | 132.7 (800,4800) | 10.2 (4,960,5) | 12.82x | 3 | 2.07x | 2/3 (mult-lo:2) |
| prime j1728 | 24 | 3 | 21.7 | 35.0 (100,400) | 75.9 (64,768,50) | 0.46x | 3 | 0.27x | 3/3 (inc-lo:1 mult-lo:3) |
| prime j1728 | 28 | 3 | 26.9 | 78.6 (200,800) | 29.8 (16,768,16) | 2.67x | 3 | 0.31x | 3/3 (mult-lo:3) |
| prime j1728 | 32 | 3 | 30.8 | 106.6 (400,1600) | 9.0 (4,784,5) | 11.73x | 3 | 1.39x | 0/3 (-) |

## Fits: log2(S_mult/S_inc) = a + slope * log2 r  (95% t-interval)

| regime | statistic | basis | n | slope | 95% interval | verdict |
|:--|:--|:--|--:|--:|:--|:--|
| prime_j0 | R_all | per_instance | 12 | -0.478 | [-0.574, -0.382] | slope<0: gain grows, H falsified |
| prime_j0 | R_all | size_means | 4 | -0.503 | [-0.666, -0.341] | slope<0: gain grows, H falsified |
| prime_j0 | R_adm | per_instance | 12 | -0.227 | [-0.336, -0.117] | slope<0: gain grows, H falsified |
| prime_j0 | R_adm | size_means | 4 | -0.236 | [-0.696, +0.224] | slope<0 but interval spans 0: not distinguishable from constant |
| prime_j1728 | R_all | per_instance | 9 | -0.550 | [-0.712, -0.388] | slope<0: gain grows, H falsified |
| prime_j1728 | R_all | size_means | 3 | -0.583 | [-1.887, +0.721] | slope<0 but interval spans 0: not distinguishable from constant |
| prime_j1728 | R_adm | per_instance | 9 | -0.301 | [-0.414, -0.189] | not testable as factor-base measurement (<4 sizes with admissible pairs) |
| prime_j1728 | R_adm | size_means | 3 | -0.319 | [-1.668, +1.031] | not testable as factor-base measurement (<4 sizes with admissible pairs) |
| prime_pooled | R_all | per_instance | 21 | -0.472 | [-0.563, -0.380] | slope<0: gain grows, H falsified |
| prime_pooled | R_all | size_means | 7 | -0.496 | [-0.667, -0.325] | slope<0: gain grows, H falsified |
| prime_pooled | R_adm | per_instance | 21 | -0.238 | [-0.312, -0.164] | slope<0: gain grows, H falsified |
| prime_pooled | R_adm | size_means | 7 | -0.248 | [-0.420, -0.077] | slope<0: gain grows, H falsified |
| binary | R_all | per_instance | 12 | -0.203 | [-0.247, -0.158] | slope<0: gain grows, H falsified |
| binary | R_all | size_means | 4 | -0.200 | [-0.365, -0.035] | slope<0: gain grows, H falsified |
| binary | R_adm | per_instance | 12 | -0.126 | [-0.183, -0.069] | slope<0: gain grows, H falsified |
| binary | R_adm | size_means | 4 | -0.128 | [-0.373, +0.117] | slope<0 but interval spans 0: not distinguishable from constant |

## Post-hoc (NOT part of the frozen verdict): fit on instances whose all-rows minimisers are both interior to their grids

* prime_j0: n=1 instances interior-both; too few
* prime_j1728: n=3 instances interior-both; slope +0.386 [-5.671, +6.442]
* prime_pooled: n=4 instances interior-both; slope +0.096 [-0.407, +0.598]
* binary: n=6 instances interior-both; slope -0.267 [-0.379, -0.154]

Admissible instance pairs (both sides >=256 cols): 33 of 33 instances; multiplier configurations with >=256 columns: 99 of 750 cells.

## Verdict
Per the frozen rule (slope >= 0 => constant; interval wholly < 0 => H falsified): **H formally falsified for the
prime regime** (pooled per-instance and size-mean fits, R_all and R_adm) **and for binary** (primary per-instance fits, R_all and R_adm);
only secondary fits (binary R_adm size-means, n = 4) span 0. Reading: slopes are grid-placement artefacts, not scaling. Not a scaling claim: toy sizes (r <= 2^34 prime, 2^24 binary,
3 seeds), grids capped at ~8,000 points, minimisers on grid edges, per-instance fits pseudo-replicate seeds within a
size (use the size-mean rows as the conservative interval). The ratio is an internal pipeline comparison, not a comparison
with rho or any published index calculus (rho reference S ~ 1 in the same unit).

## Suspicious behaviour / bugs seen
* `ic bench --sweep` with only `configurations` (empty `matrix`) fails: `expand_matrix` of an empty matrix returns one empty
  config, which is appended to `configurations` and aborts with "a configuration needs a `factor_base`" (bench.rs ~line 925).
  Worked around with the `matrix` form.
* `cm-multiplier`/`binary-multiplier` silently drop points that collide with an earlier point (`seen` check), so reported
  points per column can be below the nominal count; the columns count and weights stay consistent (unit tests pass).
* With K <= 4 columns the base is a BSGS table (known, in the READMEs); the framework's "folded columns" count is then not
  a factor-base size, which is why the 256-column rule removes almost all multiplier cells.
* Sandbox note: the first worktree checkout was killed by the tool timeout and left a stale `index.lock`; the work uses a
  fresh worktree `crypto-worktrees/multiplier-closure-scaling-20261024-w` (the original directory is a partial leftover that could not be removed).

## Figure
[gain_vs_log2r.svg](gain_vs_log2r.svg) (from `plot.py`): gain vs log2 r, all rows (solid) and admissible rows (dashed). No PDF/PNG render or
report PDF was produced (the study is unvalidated and not entering `benchmarks/`; omission recorded here).

## Problem records
[`problems/ic-bench-sweep-empty-matrix.md`](../../problems/ic-bench-sweep-empty-matrix.md),
[`problems/ic-multiplier-base-silent-collision-drop.md`](../../problems/ic-multiplier-base-silent-collision-drop.md).

## Benchmarks row (not added to `benchmarks/README.md`)
The validation gate (docs/VALIDATION_GATE.md) admits a row only with a replay certificate, a validator report and a matched
rho reference; none exists for this study, so adding the row would be refused by CI. Row text for when it is validated:

| 2026-10-24 | Counted-S scaling of cm-multiplier / binary-multiplier vs incumbent glv-orbit / binary-subspace+plain, `mitm:m=3`, 33 instances (j0 r 2^21..2^34, j1728 2^22..2^31, random binary 2^18..2^24, 3 seeds): frozen rule formally falsifies H; slopes are grid artefacts (admissible minimisers on grid edge 33/33); no scaling claim; admissible (>=256 columns) gain only 0.26x..2.1x; unvalidated | counted ops, host-independent | [study](../research/multiplier_closure_scaling_20261024/README.md) |
