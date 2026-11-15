# Track C: fully counted index calculus on E/F_{p^3} against matched rho

Exploratory toy measurement, 2026-11-15. This is not a `bench.record` benchmark and has no
preregistration, A/A arm or independent replay. Every number below comes from a saved run in
`runs/` (32 curve files, under 400 kB in all) and is collected in `summary.json`,
`analysis.txt` and `component_model.txt`. All numbers are [pcc:unverified].

The goal was to replace the lower bounds in `../ext_field` for n = 3 (137x down to 95x, which
left out msolve and the linear algebra) with a full count, and to check whether the gap shrinks
with size.

## Setup

- **Base:** `../ext_field/ic_fp3.py` and its helpers, copied here unchanged as `ic_fp3_orig.py`,
  `ic_fp2.py`, `make_s4.py` and `s4_sym.json`. The originals were not edited. The curves, the
  field F_{p^3} = F_p[t]/(t^3 - t - c), the symmetrized S4, the Weil descent, the IC walk, the
  factor-base tests, the y lifts, the sign additions and the matched rho are the same code.
- **Counted solver (`counted_gb.py`).** It replaces msolve, so every step of the decomposition
  is charged in F_p multiplications (M) and inversions (I):
  1. F4 in grevlex: normal strategy, Gebauer-Moeller criteria, symbolic preprocessing, and
     column-ordered reduction by the reducer rows, then echelon form of the rest. The matrix
     sizes per round, 3x35, 11x55, 22x76, 30x89, 46x101, 48x106, 48x108 and 46x108, match
     msolve's own `-v 2` log on the same system (`msolve_v2_example_p251.txt`: 3x35 ... 47x108,
     46x108).
  2. Standard monomials, giving the quotient dimension D (64 when generic).
  3. The multiplication matrix by e3. Columns where e3*b is a standard monomial are free. The
     other 12 columns are counted, memoised normal forms.
  4. Wiedemann: s_i = u . M^i [1] for i < 2D, using sparse matrix-vector products, then
     Berlekamp-Massey gives the eliminating polynomial.
  5. F_p roots of that polynomial with gcd(z^p - z, f) and Cantor-Zassenhaus (the same counted
     `proots` the earlier work used).
  6. For each root z, the evaluation functional w^T g(M) with g = f/(T - z), by Horner. It gives
     e1 and e2.

  Counting rule: one elimination step costs nnz(pivot row) - 1 M, and a normalisation costs
  1 I + nnz - 1 M. Products with a zero operand are not charged, which is what a sparse
  implementation would perform. numpy is only the execution vehicle.
- **Replayable by seed.** No external process runs. `replay_check.out` shows reruns of p = 251
  seed 1 and p = 1019 seed 3 identical to the saved JSON, timing fields excluded.
- **Cross-check against msolve** (`test_vs_msolve.py`, output `test_vs_msolve.out`; 180
  systems each at p = 31, 251, 1019 and 4091). Wherever both solvers finish generically, the
  sets of F_p solutions agree on 155/159, 174/175, 180/180 and 180/180 systems. In every
  disagreement (4 at p = 31, 1 at p = 251) this solver found an extra solution. Each extra
  solution satisfied the system, and msolve did not report it. No solution from this solver
  failed the substitution check. msolve is nondeterministic, so these counts come from one draw.
- **Non-generic cases.** In the main runs, 360 of 330,263 solver calls (0.11%) had a minimal
  polynomial of degree below D (`not_cyclic`): 2.9% at p = 31 and 0.03% at p = 4091. The work
  for those calls is charged, and they produce no relation. That inflates the IC trial count
  slightly. No call was positive-dimensional in the main runs.
- **Linear algebra.** Incremental sparse elimination mod r (`Elim`, unchanged) is counted in
  Z_r multiplications. For each finished mode, the run also records a block-Wiedemann estimate
  9 N K: 3N matrix-vector products of 3K nonzeros for N columns and K rows. The primary pricing
  is 1 Z_r mult = 1 M, since every toy value fits a machine word. The sensitivity pricing is
  9 M, which is (log r / log p)^2 for multi-word r ~ p^3.
- **Modes, on one relation stream:** `all` (the factor base is every x in F_p), `2lp:B` and
  `1lp:B` with B = p/2. A run stops when every mode finishes, or at 2x the trials of the first
  mode to solve (marked censored).
- **Rho (matched, unchanged from ext_field):** the same curve and subgroup, an r-adding walk
  with 20 multipliers, store-all, matching on x so that +-R collide. It runs 3 times per target
  with fresh multipliers.
- **Unit:** F_p multiplications, squarings counted as M, I = 3 M. One F_{p^3} affine addition is
  42 M + 1 I. The walk setup (20 scalar multiplications) is the same in both arms and is not
  counted. The BSGS point counting that finds the curves is not counted either.
- **Correctness:** every discrete log from every arm was asserted equal to the true w. That is
  64 IC targets x 3 modes (except censored ones) and 192 rho runs, and all passed. Every
  relation is also checked by point addition, since the sign search must reproduce R.
- **Sizes:** p in {31, 67, 127, 251, 503, 1019, 2039, 4091}, so log2 r runs from 14.9 to 36.0.
  There are 4 curves (seeds 1..4) per p and 2 targets per curve.

## Commands

```
./run_all.sh                                  # 32 runs, xargs -P 4; 22 min wall on a shared 4-vCPU container
python3 analyze.py > analysis.txt             # summary.json, gap_fit.{png,svg,pdf}
python3 component_model.py > component_model.txt
python3 test_vs_msolve.py P 7 3 60            # for P in 31 251 1019 4091 -> test_vs_msolve.out (needs SCR scratch dir)
python3 ic_fp3_full.py 251 1 2 3 X.json && python3 replay_check.py runs/p251_s1.json X.json
```

## Results

Means over 4 curves (8 targets) per size. The gap is the ratio of mean IC M-eq to mean rho
M-eq, both in F_p multiplications with I = 3 M and Z_r = 1 M.

| p | log2 r | rho steps | rho M-eq | IC trials | relations | M-eq per trial | IC M-eq | gap `all` | gap `2lp:p/2` | gap `1lp:p/2` | gap `all`, Z_r = 9M | ext_field lower bound |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 14.9 | 155 | 6.96e3 | 141 | 15 | 1.80e5 | 2.54e7 | 3647 | 3818 | censored 3/8 | 3647 | 137 |
| 67 | 18.2 | 485 | 2.18e4 | 246 | 30 | 1.93e5 | 4.74e7 | 2174 | 2455 | censored 1/8 | 2174 | 116 |
| 127 | 21.0 | 1296 | 5.83e4 | 450 | 56 | 2.01e5 | 9.03e7 | 1549 | 1605 | 2377 | 1549 | 107 |
| 251 | 23.9 | 3927 | 1.77e5 | 866 | 109 | 2.07e5 | 1.80e8 | 1017 | 1101 | censored 2/8 | 1017 | 95 |
| 503 | 26.9 | 11394 | 5.13e5 | 1495 | 224 | 2.16e5 | 3.24e8 | 631 | 715 | 1098 | 631 | n/a |
| 1019 | 30.0 | 24949 | 1.12e6 | 2815 | 471 | 2.24e5 | 6.32e8 | 563 | 642 | censored 2/8 | 564 | n/a |
| 2039 | 33.0 | 80252 | 3.61e6 | 5799 | 932 | 2.31e5 | 1.34e9 | 370 | 415 | 665 | 372 | n/a |
| 4091 | 36.0 | 217634 | 9.79e6 | 11281 | 1897 | 2.38e5 | 2.69e9 | 274 | 309 | 489 | 279 | n/a |

Breakdown for mode `all`, as a share of IC M-eq. The second table gives M-eq per trial.

| p | walk | build (descent) | F4 | mult. matrix | Wiedemann + BM | root finding | eigenvector (e1, e2) | post (cubic, lifts, signs) | LA (counted) | LA, Wiedemann estimate |
|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 0.0% | 0.9% | 22.7% | 0.5% | 55.5% | 1.5% | 18.8% | 0.1% | 0.00% | 0.01% |
| 251 | 0.0% | 0.8% | 20.9% | 0.3% | 49.4% | 9.3% | 19.1% | 0.1% | 0.00% | 0.06% |
| 1019 | 0.0% | 0.7% | 19.5% | 0.3% | 45.6% | 14.3% | 19.3% | 0.2% | 0.01% | 0.32% |
| 4091 | 0.0% | 0.7% | 18.4% | 0.3% | 43.0% | 18.8% | 18.4% | 0.2% | 0.02% | 1.23% |

| p | F4 | Wiedemann | root finding | eigenvector | rest of decomposition | walk | LA counted | LA Wiedemann estimate |
|---|---|---|---|---|---|---|---|---|
| 251 | 43.4k | 102.4k | 19.4k | 39.5k | 2.6k | 45 | 7 | 124 |
| 4091 | 43.7k | 102.4k | 44.8k | 43.8k | 2.8k | 45 | 556 | 2928 |

Fits of log2(quantity) against log2 r, OLS over 32 curves, with 95% t intervals
(`analysis.txt`):

| Quantity | Slope |
|---|---|
| gap `all` | -0.172 (-0.195 .. -0.149) |
| gap `all`, p >= 251 only (20 curves) | -0.150 (-0.186 .. -0.114) |
| gap `2lp:p/2` | -0.167 (-0.190 .. -0.144) |
| rho M-eq | +0.495 (+0.480 .. +0.510) |
| IC trials | +0.305 (+0.290 .. +0.321) |
| IC M-eq per trial | +0.018 (+0.016 .. +0.019) |
| LA, counted | +0.684 (+0.646 .. +0.721) |

Findings [pcc:unverified]:

- **Fully counted, IC on E/F_{p^3} is slower than store-all +- rho at every tested size: 3647x at
  15 bits down to 274x at 36 bits (mode `all`).** That is 10x to 27x above the ext_field lower
  bounds at the sizes both cover. The Groebner/FGLM step (F4, Wiedemann and the eigenvector
  together, about 190k M-eq) dominates every trial. The rho break-even budget per trial is
  225 M-eq at p = 251 and about 870 M-eq at p = 4091 (rho M-eq / IC trials).
- **The gap shrinks with size at about r^(-1/6).** The fitted slope is -0.172 per bit
  (95% CI -0.195 .. -0.149). This matches the decomposition-dominated regime: the cost per
  trial is almost constant (slope +0.018), trials grow about like p = r^(1/3) (about 2.8 p trials,
  with one relation per about 6 trials), and rho grows like r^(1/2). Most of the small growth per
  trial is root finding, which is linear in log2 p.
- **Parity by naive extrapolation.** The straight-line fit of the gap would reach parity at
  log2 r = 83, with a bootstrap 95% interval of 76 .. 91 (resampling curves within sizes). On
  p >= 251 alone the fit gives 90 (79 .. 106). This is **not a credible forecast**, for the
  next reason.
- **The linear algebra grows faster than rho and stops the trend [pcc:unverified].** Counted LA grows with
  slope 0.68 (Wiedemann estimate 0.67; theory 2/3 for a full base of size about p/2). It is
  still only 0.02% to 1.2% of the cost at 36 bits. A structured model, fitted on p >= 251 and
  bootstrapped over curves (`component_model.txt`), uses trials = 2.82 p, non-LA M-eq per
  trial = 1.51e5 + 7.3e3 log2 p, LA = 9 N K with N = 0.468 p and K = 0.460 p, and
  rho = 37.9 sqrt(r). For the `all` arm it gives a **minimum gap of 66 (95% CI 60 .. 72) near
  log2 r = 56 and no parity at any size**: 0 of 2000 resamples reached parity. With
  Z_r = 9 M, the minimum is 189 (173 .. 208) near log2 r = 46. The bootstrap intervals cover
  only curve-to-curve noise, not the choice of model.
- **Large primes did not help here.** `2lp:p/2` costs 1.04x to 1.13x more than `all` at every
  size, and `1lp:p/2` costs 1.5x to 1.8x more where it finished. At these sizes the LA is
  negligible, so shrinking the base only discards relations. Gaudry's double-large-prime
  exponent q^(4/3), which is r^(4/9) against rho's r^(1/2), comes from balancing the base
  against the LA. That only matters past the crossover above, about log2 r of 45 to 56, which
  these runs do not reach. An order-of-magnitude reading: a slope of -1/18 starting from a gap
  of about 66 would need about 110 more bits, which puts parity near log2 r = 165 for this
  decomposition cost. That is an argument, not a measurement.
- Wall time agrees in direction. At p = 4091 an IC target took 205 s and a rho run took 7.3 s
  (Python, a shared container, other jobs running).

## Limits

- **The constant depends on the implementation.** The counted solver is a plain Python F4 with
  sparse FGLM. The Wiedemann sequence costs 102k M per trial: 2D = 128 matrix-vector products
  over 12 dense non-trivial columns. The eigenvector step costs 44k M per root. A tuned trace
  (an F4 trace, a smaller projection, or batching the e1/e2 parametrization) could plausibly cut
  the per-trial cost by a small factor. That would shift the gap curve down but not change its
  slope. Even a 10x cut would leave about 27x at 36 bits [pcc:unverified]. Charging msolve's
  own work was not attempted. Its F4 matrices are the same size as this solver's, so its counts
  should be of the same order.
- The `all` arm uses a factor base of every x in F_p (about p/2 classes up to sign). No
  2-torsion or symmetry folds were used. No balanced double-large-prime base (size about
  p^(2/3)) was tried. Its trial count would rise and its LA would fall.
- Toy sizes: log2 r from 15 to 36, 4 curves per size. The crossover where LA dominates (log2 r
  of 45 to 56) and any parity point are extrapolations from models.
- The Z_r pricing (1 M or 9 M) and I = 3 M are conventions. Additions, hashing and Python
  bookkeeping are not charged.
- The censored `1lp` entries stopped at 2x the trials of `all`. Their means are not reported.
- The research-kb server was not queried in this track. Nothing here claims novelty.

## Errors hit

- **Missing eigenvector component (fixed before the runs).** Horner with w equal to the unit
  vector at '1' can give lambda[1] = 0. The first version then dropped a valid root: one
  system at p = 251 lost a solution that msolve found. Fix: retry with a random w, counted at
  D extra M per Horner step. After the fix there were 0 cases where msolve found a solution
  this solver missed.
- **Duplicate pivot row in F4 (fixed before the runs).** The first version sent both rows of a
  selected pair to the matrix as rows to reduce. Symbolic preprocessing then added a third copy
  as the reducer. It now uses the first pair row as the pivot of its lcm column, as standard F4
  does.
- **The original msolve output parser crashed (handled).** In the cross-check at p = 31, msolve
  produced output that the original `solve_sys` parser could not read (TypeError). The test now
  records that case as `parse_error` (1 system).
- msolve is nondeterministic. Two cross-check runs at p = 251 gave 175 agreements in one and
  174 + 1 disagreement in the other (RNG consumption follows msolve's output). The saved file
  holds the last run.

## Independent review notes (2026-11-15)

A separate reviewer reran cells and recounted from the saved outputs. Verdict: holds with caveats [pcc:unverified].

Corrected headline: the walk setup (21 pairs of scalar multiplications) was not counted. It costs about 6x the whole counted rho at p = 31. With setup counted in both arms the gap goes from about 526x (15 bits) to 271x (36 bits), and the steep -0.172 slope largely disappears. The 'parity at log2 r = 83' straight-line extrapolation should not be relied on. The component-model conclusions depend on the model chosen: the structured model's 'never parity' follows from assuming linear algebra grows like p^2 (Gaudry's full-base n = 3 result), and the power-law model gives a minimum of 72.6x near 62 bits, dominated by root finding. Absolute gaps measure this solver, not index calculus in general. [pcc:unverified]

### Corrections to the text above

Where these conflict with the text above, these take precedence [pcc:unverified].

- 'The rho break-even budget per trial is 225 M-eq at p = 251': summary.json gives rho M-eq / IC trials = 1.767e5 / 866 = 204 (the mean of the per-curve ratios is 213). 225 is the break-even value from ../ext_field, not from these runs.
- Breakdown table, 'LA (counted)' column: p = 1019 is listed as 0.01% but is 0.022%, and p = 4091 is listed as 0.02% but is 0.23% (analysis.txt itself shows 0.2%).
- 'It is still only 0.02% to 1.2% of the cost at 36 bits': at 36 bits counted LA is 0.23% and the Wiedemann estimate is 1.23%. The 0.02% figure is wrong.
- '2lp:p/2 costs 1.04x to 1.13x more than all at every size': the data range is 1.036x to 1.141x, with 1.141 at p = 1019 and 1.133 at p = 503.
- F4 matrix sizes are said to 'match msolve's own -v 2 log', but the row counts differ from round 4 on: counted solver 30x89, 46x101, 48x106, 48x108 against msolve 29x89, 43x101, 46x106, 47x108. Only the column counts and the first three rounds agree.
- Per-trial table: 'LA counted 7' at p = 251 is 7.67, which rounds to 8.
- The headline ties 'linear algebra (slope about 0.68)' to the component model, but the structured model prices LA as 9NK (slope 2/3 by construction), not the counted 0.68 slope. In the power-law model the dominant phase at the minimum is root finding, not LA.

### Caveats

- Uncounted setup changes the small-size headline and the slope. The walk setup (21 pairs of scalar multiplications) costs 41.4k M-eq at p = 31, about 6x the whole counted rho cost there (6.96k). With it counted in both arms, the 15-bit gap is about 526x instead of 3647x, and the gap barely moves across sizes (526x to 271x) instead of slope -0.172. The README says the setup is 'the same in both arms and is not counted', which is true in absolute terms but hides that it dominates the rho arm at small p. Leaving it out is defensible asymptotically, but '3647x at 15 bits' and the size trend both depend on that choice, and it should be stated.
- Headline description of the component model is misleading. The structured model that gives '66x near log2 r = 56, never parity' does not use the measured LA slope of about 0.68. It uses the block-Wiedemann estimate 9NK with N, K proportional to p, so LA grows as p^2 by assumption. 'Never reaches parity' then follows from p^2 against rho's p^1.5, which is Gaudry's full-base result for n = 3. It is not something the data established, so '0/2000 resamples reach parity' adds almost no evidence.
- Model choice matters more than the stated intervals. The power-law component model in the same file (component_model.txt) gives a minimum of 72.6x at log2 r about 62, with root finding, not LA, dominant. Its Wiedemann-estimate variant gives 109x at 53. The structured model gives 66x at 56 with LA at only 48% at the minimum, so 'LA taking over first' is not shown in either model. The quoted CI (60..72, location 56..56) covers only curve resampling. The location interval is degenerate. The README admits this in the body, but the headline does not.
- The per-trial constant depends on the implementation (plain F4, 102k M of Wiedemann over 12 dense columns, about 44k M per root for the eigenvector). The README discloses this, but it means the absolute gaps say only that this solver is slower, not that IC is.
- Counting conventions affect the gap level: with I = 10 M the gap is 0.87x the reported value, and with I = 30 M it is 0.64x, because rho performs one F_{p^3} inversion per step. The slope is unchanged. I = 3 M slightly favours rho.
- The 1lp:B fit (slope -0.152, n = 25) and its per-size means use only targets that finished. The censored targets are the expensive ones, so these numbers are biased low (selection on the outcome).
- Evidence standing: this is not a bench.record, has no preregistration, no A/A arm and no independent replay. The cross-check against msolve uses a single nondeterministic draw. The results are correctly labelled [pcc:unverified]; none of this can reach the leaderboard or validation-gate standing.
