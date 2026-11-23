# NC-2 Stage 0 results (scalar Monte Carlo, zero group operations)

Status: [pcc:unverified]. No pcc claim id. Record: `../ideas/NONCOLLISION-20261108.md`, NC-2. Tool:
`src/bin/nc2_stage0.rs` (design table, model and guard are documented in its header; 8 unit tests).

## Commands and seeds

```
cargo build --release --bin nc2_stage0
cargo test  --release --bin nc2_stage0          # 8 passed
sh research/curve25519_autolab_20261104/nc2/run.sh   # runs the three commands below, from this directory
```

Exact lines (`run.sh`, all with `--seed 20261108`; held-out samples use seed ^ 0xAAAA0001, training samples
seed ^ 0x55550002, search seed ^ 0x33330003, randomised-e seed ^ 0x77770004):

```
B=../../../target/release/nc2_stage0
COMMON="--c 1,2 --n 2000 --seed 20261108 --train 1000 --restarts 12 --steps 50"
$B --r 131203,131317,131111      --d1 0.5,4  $COMMON > out-k20.txt
$B --r 2097349,2097559,2097523   --d1 1.0,16 $COMMON > out-k24.txt
$B --r 33558163,33554761,33558059 --d1 1.0,8 $COMMON > out-k28.txt
$B --r R --c 0,1,2 --n 20000 --seed 20261108 --no-d3 > out-controls-R.txt     # R in 131203, 2097349, 33558163
```

r values are the first siblings of `c1/siblings-k{20,24,28}-10.json` and two members of `c1/confirm-curves.json`
(131203 is the first). `E` is in sqrt(r) units: mean of T / sqrt(r) over 2000 samples of (d, s) uniform in Z_r,
plus or minus the standard error. T = points generated (stored points plus lookup-only probes). Cap 10 sqrt(r).
D3 budget (fixed before the run): 12 random restarts x 50 hill-climbing steps = 612 evaluations on 1000 training
samples; the winner by training E is re-scored on 2000 disjoint fresh samples. Only the held-out value counts.

## Control status (read this first)

Poisson floors: 0.886 (C=0), 1.329 (C=1), 1.662 (C=2).

* The engine is right: **D0i** (every point a fresh uniform vector, not a walk) gives 0.887, 1.327 to 1.330 and
  1.661 to 1.667 at n = 20000 (SE 0.0034), on all three sizes.
* **D0 (16-jump random walk) sits above its stated control intervals' upper edges, not inside them.** At n = 20000:
  C=1: 1.381 (k20), 1.382 (k24), 1.377 (k28) against [1.28, 1.38]; C=2: 1.728, 1.720, 1.722 against [1.60, 1.72];
  C=0: 0.918 to 0.923 against 0.886. SE is 0.0035. At the pre-registered n = 2000 the means are within about 1 SE of
  the edges (C=1: 1.368 to 1.398; C=2: 1.684 to 1.740). The excess over the iid value is 3.5 to 4.5 percent and is
  a property of a walk with 16 jump types (pair differences are sums of few jumps, not uniform), not of the engine.
  I did not tune D0 to land inside the interval. By the task's rule this is a control outside its interval at
  n = 20000; the caller decides whether it stops Stage 0. All D3 comparisons below are against D0 as defined.
* D0h (jump chosen by a hash of the scalar, the literal wording) is not oblivious: trails merge, duplicate rows are
  dependent, and 50 to 75 percent of samples hit the cap (E 5.7 at C=1, 7.9 at C=2). It is reported in the raw
  outputs only; D0 uses a jump stream independent of the point (as `wwalk` does).

## E in sqrt(r) units (n = 2000, held-out for D3)

| k | r | C | D0 | D0i (iid) | D1 wwalk | D3 held-out | D3 train | D3/D0 | D0/D3 | CE (rand. e) |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 20 | 131203 | 1 | 1.380 (0.011) | 1.336 | 1.767 (0.018) | 1.375 (0.011) | 1.340 | 0.996 | 1.004 | 1.380 |
| 20 | 131317 | 1 | 1.390 | 1.326 | 1.772 | 1.371 | 1.344 | 0.986 | 1.014 | 1.380 |
| 20 | 131111 | 1 | 1.375 | 1.328 | 1.771 | 1.394 | 1.347 | 1.014 | 0.986 | 1.386 |
| 20 | 131203 | 2 | 1.727 (0.011) | 1.685 | n/a | 1.727 (0.011) | 1.689 | 1.000 | 1.000 | 1.728 |
| 20 | 131317 | 2 | 1.740 | 1.677 | n/a | 1.719 | 1.681 | 0.988 | 1.012 | 1.724 |
| 20 | 131111 | 2 | 1.724 | 1.655 | n/a | 1.742 | 1.686 | 1.010 | 0.990 | 1.745 |
| 24 | 2097349 | 1 | 1.368 | 1.325 | 1.609 (0.011) | 1.373 | 1.340 | 1.003 | 0.997 | 1.369 |
| 24 | 2097559 | 1 | 1.398 | 1.323 | 1.617 | 1.382 | 1.350 | 0.988 | 1.012 | 1.380 |
| 24 | 2097523 | 1 | 1.372 | 1.341 | 1.634 | 1.366 | 1.343 | 0.996 | 1.004 | 1.374 |
| 24 | 2097349 | 2 | 1.728 | 1.653 | n/a | 1.712 | 1.687 | 0.991 | 1.009 | 1.724 |
| 24 | 2097559 | 2 | 1.724 | 1.663 | n/a | 1.717 | 1.683 | 0.996 | 1.004 | 1.721 |
| 24 | 2097523 | 2 | 1.684 | 1.654 | n/a | 1.720 | 1.677 | 1.021 | 0.979 | 1.730 |
| 28 | 33558163 | 1 | 1.385 | 1.319 | 1.625 (0.012) | 1.381 | 1.341 | 0.998 | 1.002 | 1.354 |
| 28 | 33554761 | 1 | 1.391 | 1.334 | 1.629 | 1.406 | 1.346 | 1.011 | 0.989 | 1.373 |
| 28 | 33558059 | 1 | 1.376 | 1.329 | 1.632 | 1.413 | 1.347 | 1.027 | 0.974 | 1.371 |
| 28 | 33558163 | 2 | 1.711 | 1.648 | n/a | 1.730 | 1.684 | 1.011 | 0.989 | 1.718 |
| 28 | 33554761 | 2 | 1.740 | 1.658 | n/a | 1.704 | 1.684 | 0.979 | 1.021 | 1.727 |
| 28 | 33558059 | 2 | 1.738 | 1.658 | n/a | 1.723 | 1.685 | 0.991 | 1.009 | 1.730 |

Standard errors: 0.011 to 0.012 on every 2000-sample cell (0.015 to 0.016 on the 1000 training cells); all rows are in
`out-k*.txt`. D2 (serpentine box tiling) reaches rank K in 0 to 9 percent of samples before the cap
(E >= 9.5 at C=0, >= 9.98 at C=1 and 2: censored lower bounds), see `out-controls-*.txt`; this is the redundant
coverage the record predicts for tiling, and not a competitive design.

## Readings [pcc:unverified]

* D3 held-out versus D0: ratios 0.974 to 1.027 over 18 (k, r, C) cells, centred at 1.00, every cell within about
  2.5 SE of 1.00. The best held-out cell is 0.979 (C=2, r=33554761); the worst is 1.027. No cell reaches the claim's
  0.92, none is below 0.97. The record's falsification test (best D3 within 3 percent of D0 at C = 1 and 2, at
  k = 20 and 28) holds in the sense of a null result: the best D3 is within 2.1 percent (C=1, k20: 0.986; k28:
  0.998; C=2, k20: 0.988; k28: 0.979, taking the best of three r in each case).
* Training E of the winners (1.34 to 1.35 at C=1, 1.68 to 1.69 at C=2) is 2 to 4 percent below D0; held-out E is
  back at D0. That gap is the selection effect of 612 noisy evaluations (training SE 0.016), not structure.
  The winners are mostly 14 to 16 random-stream jumps with uniform or small coordinates, i.e. a random walk.
* The randomised-e control returns to D0 (CE within about 1 SE of D0 for the winners, 1.35 to 1.39 at C=1 and 1.72
  to 1.75 at C=2): consistent with the winners having no seed-axis structure to lose. This control cannot
  show structure was removed when none was found.
* The walk-correlation excess (D0 over D0i, 3.5 to 4.5 percent) is the only effect with a clear size. A designed
  walk that removed it entirely would reach about 1.33 and 1.66; no search winner got below D0 on held-out data.
* K = 1 cross-check (C=0): D0i 0.886 to 0.887 matches NC-1's Poisson value; D0 gives 0.918 to 0.923.

## D1 (`wwalk` C=1) against the c1 README

D1 is the pre-registered arm of each size: k20 `x=0.5, jumps=4`, k24 `x=1.0, jumps=16`, k28 `x=1.0, jumps=8`.

| k | D1 E (mean of 3 r) | above Poisson floor 1.329 | c1 README S/sqrt(r) (C=1 arm) | README above floor |
|--:|--:|--:|--:|--:|
| 20 | 1.770 | +33 percent | 2.227 | +68 percent |
| 24 | 1.620 | +22 percent | 1.814 | +36 percent |
| 28 | 1.629 | +23 percent | 1.711 | +29 percent |

The README S includes set-up and verification group operations that Stage 0 does not count (jump scalar
multiplications, doubled multiples, row verification); the differences between S and E (0.46, 0.19, 0.08 sqrt(r))
shrink with k as set-up does in sqrt(r) units. The README's ordering (excess shrinking with k, 68 to 16 percent over
k20 to k32) is reproduced in direction by E (33 to 22 percent over k20 to k28) but not in size.

## Approximations

* D1: single walk of T = round(x sqrt(r)) entries, jumps `(gamma, 0, alpha)` with alpha uniform in
  `[1, round(1.2 sqrt(T))]` and gamma uniform in `[1, r)`, extension up to 2T while no free row exists (grow=1), then
  probes `Q + kG` (k = 0, 1, ...), lookup-only, one point each. Not modelled: the identity check on `Q + kG`, the
  probe guard, `seen` bookkeeping, set-up and verification operations, the exact `u` integer (taken mod r; the real
  coefficient is below 2^32 and may exceed r at k = 20). Jumps are redrawn per sample (the oracle seeds per run).
  Stop rule: rank 2 on (db, de) with the guard; the oracle's own stop rule was not re-read beyond `grow`.
* D0 draws its 16 jumps and start uniformly in Z_r^(2+C) per sample; D3 and CE are fixed designs (jumps and stream
  fixed across samples), so D3 is a single design while D0 averages over designs.
* D2 is a serpentine tiling of a (C+2)-dimensional box with strides m^k; a first draft (a = 0 everywhere) had every
  row proportional to (1, -d/s), rank 1 forever, and was replaced before use. The tiling the record describes
  (SYNTHESIS 1.1) is not specified precisely enough to say this is the only reading.
* D3's conic-like family is the quadratic-in-jump-index coordinate rule (h^2, h, 1 patterns across jumps), not a
  conic of one-addition-per-point points; exact finite-difference designs (3 additions per point) are out of scope.
* Search budget is small (612 evaluations per cell, 1000 training samples): structure that raises the gain by less
  than about 3 percent, or sits in a region the random restarts do not reach, would not be found.
