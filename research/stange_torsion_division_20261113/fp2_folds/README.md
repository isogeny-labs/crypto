# Track B: 2-torsion and symmetry folds for index calculus on E/F_{p^2}

Exploratory toy measurement, 2026-11-15. It is not a `bench.record` benchmark and is not
admissible to the leaderboard or the validation gate. Every number below comes from a run saved
in `runs/`, collected by `analyze.py` into `summary.json` and `analysis.txt`. All numbers are
[pcc:unverified].

**Question.** `ext_field/` measured Gaudry index calculus (IC) on E/F_{p^2} at 14x to 27x its
matched store-all +-rho, counted in F_p multiplications. How much of that gap closes with the
standard folds and a cheaper root step, and does any variant reach parity?

**Answer, short.** The best variant (T2) runs at **3.2x rho at log2 r = 36** and 4.1x at 16 bits
(I = 3M; 95% curve-bootstrap interval at 36 bits: 2.7 to 4.0). At I = 1.5 log2(p) M it is 2.6x.
On the same counting, the ext_field oracle (B0) is 27.4x and 17.8x. No variant reaches parity
at any size, and no single curve reaches it either (the lowest per-curve gap is 2.57)
[pcc:unverified]. The best variant has a per-trial cost that does not depend on p
(about 81 M + 4.4 I). Its gap should therefore tend to a constant near 3, not to 1. The
downward slope seen over this range is a finite-size drift in trials per rho step and is not
a forecast [pcc:unverified].

## Prior work used

- `ext_field/ic_fp2.py`, copied unchanged to `ic_fp2_copy.py` (sha256 `47ab458e…` for both). It
  provides the F_{p^2} arithmetic, point addition, the walk, the signed union-find solver and
  store-all +-rho. `ic_fp2_folds.py` imports it and wraps some of its functions; the wrapped
  versions are listed below.
- `research/blindspots_20261018/01-fp2-gaudry-k2/` (Rust, GAE units). From it I took the fold
  set: the 2-torsion family (T = (x0, 0), f'(x0) = c^2, cofactor 4), the line
  x = x0 + c t closed under P -> P + T (t -> 1/t), the column fold <-1, tau_T> (4 points per
  column), and the FGHR symmetrized system (linear in S, P, D, with
  D^2 = (P+4)^2 - 4 S^2, so one F_p quadratic replaces the resultant and gcd(t^p - t, .)).
  Its large-prime arms are not repeated here: ext_field already showed that `2lp` on the whole
  base is the best mode in this graph setting. Its GLS/psi arm is not implemented (see Limits).
- The research-kb server was not reachable: `scripts/research-kb` reports "no executable found".
  Nothing here claims novelty.

## Setup

### Arms and variants

All variants use the 2lp union-find on the whole base. Each arm runs with its own matched
store-all +-rho on the same curves: same subgroup, the r-adding walk with 20 multipliers,
every point stored, matched on x.

**base** uses prime-order curves, `random.Random(seed)`, the same curves and planted logs as
`ext_field/runs_n2`. The decomposition is the ext_field one (x in F_p, a quartic in s, then
gcd(s^p - s, F)). One run gives several pricings:

| Variant | What changes |
|---|---|
| B0 | ext_field as is. This reproduces ext_field: target 0 of all 30 curves matches its trials and M exactly. |
| B1 | Multiplying by s (or s + d) inside s^p mod F is charged as a shift plus the reduction, not as `pmul` by [0, 1]. This is an implementation and accounting correction. |
| B2 | B1, plus every F_p exponentiation (Legendre, square root) costed by the best sliding-window chain instead of binary. |
| B3 | B1, plus every Legendre symbol priced as one Jacobi symbol = 1 I. |
| B4 | B1, plus a square-root table (separate run, same trial stream). |

**tors** uses the 2-torsion family. The curve is y^2 = x^3 + ax + b over F_{p^2}, with random
a and x0, b = -x0^3 - a x0, f'(x0) = c^2 a nonzero square, #E = 4r with r prime (BSGS over
the Hasse interval, checked by N P = O and [r] G = O), and a, b not both in F_p. The vertex is
sigma = t + 1/t. The oracle:

1. Evaluate the four distinct coefficient polynomials g_ij(x_R), normalized by c^-2 at setup.
2. Solve two linear F_p equations in (S, P, D).
3. Solve one quadratic in D.
4. Solve sigma^2 - S sigma + P = 0.
5. Check the validity of each sigma: sigma^2 - 4 must be a square in F_p (so t is in F_p), and
   N(c sigma + 3 x0) must be a square.
6. The point is P(t) = (x0 + ct, ct w_sigma), with w_sigma = sqrt(c sigma + 3x0). It depends only
   on sigma and is memoized per vertex.
7. Signs come from one or two point additions. The relation is taken on [4]-images, using
   [4]P(1/t) = -[4]P(t).

Three runs on identical trial streams. The streams are identical because the oracle uses no
randomness: 180 of 180 targets have the same rho steps and IC trials across the three runs.

| Variant | What changes |
|---|---|
| T0n / T0 | Legendre then square root, both by exponentiation (the ext_field convention). Vertex (sigma, branch) for T0n, which has no T column fold; vertex sigma for T0, which has it. |
| T1 | Square root with check instead of Legendre plus square root. The validity of each sigma is memoized. T1w adds window pricing of exponentiations; T1j adds Jacobi = 1 I. |
| T2 / T2n | A square-root table SQ[z] for all of F_p, built for (p-1)/2 counted squarings in every IC run. All F_p Legendre symbols and square roots become lookups. T2n has no T fold. |

**Positive control (`test_oracle.py`, `test_oracle.out`).** At p = 503 and p = 1019, 3 curves
each and 600 targets in all, the symmetric oracle's set of {sigma1, sigma2} equals exhaustive
enumeration over the line in 600 of 600 cases. All 300 relations reconstruct R exactly with
their signs. [4]P(1/t) = -[4]P(t) is asserted on every relation.

**Every discrete log is checked.** All 450 IC targets (every mode of every variant run) and all
1350 rho runs were asserted equal to the planted log, and all passed.

### Cost unit

- F_p multiplications M, with squarings counted as M. Inversions I are priced at 3M and at
  1.5 log2(p) M. This is the same unit and the same counting code as ext_field. One F_{p^2}
  point addition (a rho step or an IC walk step) is 12M + 1I.
- Lookups are unpriced native work, recorded for both arms and priced w_l in {0, 0.1, 1} M.
  - For IC they are the square-root-table lookups, the sigma-memo probes, one seen-x probe per
    trial and two per relation for the solver.
  - For rho, one table operation per step.
- The square-root table is built in each IC run and charged in full: (p-1)/2 M, about 2.5% of
  T2's cost.
- The mod-r solver work (Zr) is recorded and not priced, as in ext_field. It is tiny.
- Curve generation, BSGS and the 20 walk multipliers are not counted, the same in both arms.

### Sizes

- base: p in {251, 1019, 4091, 16363, 65519, 262139}, so log2 r = 16..36, the same as ext_field.
- tors: p in {503, 2039, 8191, 32719, 131071, 524287}, the largest primes = 3 mod 4 below
  2^{9,11,...,19}. Here r ~ p^2/4, so log2 r = 16..36, matched to base.
- 5 curves per size, 3 targets per curve and 3 rho runs per target.

## Commands

```
python3 test_oracle.py > test_oracle.out   # positive control, ~1 min
./run_all.sh                               # 150 runs, 4 at a time; 20:14-20:31 UTC wall (~17 min)
python3 analyze.py > analysis.txt          # summary.json, gap_vs_size.{png,svg,pdf}
```

`run_all.sh` writes `runs/<arm>_<variant>_p<p>_s<seed>.{json,log}`. A single run is
`python3 ic_fp2_folds.py tors:table 524287 1 3 3 out.json`.

## Results

Gap = (mean over curves of IC cost) / (mean over curves of matched rho cost), in F_p M-equivalents.
I = 3M and w_l = 0 unless stated. Full tables for every pricing are in `analysis.txt`.

| log2 r | B0 ext_field | B1 shift | B2 +window | B4 +table | T0n tors, no T fold | T0 +T fold | T1 sqrt-check, memo | T1j +Jacobi | T2 +sqrt table | T2 at I = 1.5 log2 p M | T2 at w_l = 1 |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 15.9 | 17.8 | 15.6 | 15.2 | 13.2 | 11.8 | 8.8 | 7.2 | 6.9 | 4.08 (3.66-4.60) | 3.57 | 4.21 |
| 20.0 | 18.3 | 15.9 | 15.3 | 13.1 | 13.7 | 10.0 | 7.9 | 7.6 | 3.92 (3.72-4.12) | 3.36 | 4.05 |
| 24.0 | 21.3 | 18.3 | 17.6 | 14.9 | 14.8 | 10.5 | 7.4 | 6.8 | 4.00 (3.62-4.42) | 3.37 | 4.14 |
| 28.0 | 26.4 | 22.8 | 21.8 | 18.5 | 16.1 | 11.4 | 8.7 | 8.2 | 3.58 (3.04-4.19) | 2.97 | 3.70 |
| 32.0 | 25.9 | 22.0 | 21.0 | 17.8 | 14.8 | 10.7 | 7.1 | 6.5 | 3.31 (2.87-3.74) | 2.71 | 3.43 |
| 36.0 | 27.4 | 23.2 | 21.9 | 18.4 | 15.8 | 11.2 | 7.2 | 6.6 | 3.17 (2.73-4.01) | 2.57 | 3.28 |

The values in brackets are 95% curve-bootstrap intervals (2000 resamples, 5 curves). Per-curve
T2 gaps at 36 bits: 2.64, 4.98, 2.76, 3.39, 2.84.

Ingredients at log2 r = 36:

| Quantity | base | tors (T2) | rho |
|---|---|---|---|
| Trials or steps per sqrt r | 0.449 | 0.471 | 0.93 to 0.94 |
| Relations per trial | 0.50 | 0.50 | - |
| M per trial (B0 to B4; T0 to T2) | 824 to 545 | 320 to 81 | 12 per step |
| I per trial | 7.9 | 4.4 | 1 per step |
| Lookups per trial | 2.0 to 6.8 | 9.8 | 1 per step |

The table variant costs a flat 79-82 M per trial at every p. Every exponentiation variant grows
with log p.

### Fold contributions (ratio of gaps, I = 3M, at 16 bits / 36 bits)

| Step | Factor | What it is |
|---|---|---|
| B0 to B1, s-multiplication as shift | 1.14 / 1.18 | Accounting and implementation correction. Not an algorithmic improvement. |
| B1 to B2, sliding-window powering | 1.03 / 1.06 | Exponent chains only. s^p with base s gains nothing from windows, because multiplying by s is already a shift. |
| B1 to B4, square-root table on the base oracle | 1.19 / 1.26 | Limited, because s^p mod a quartic still dominates. |
| B0 to T0n, torsion family + FGHR symmetric oracle (quadratic instead of quartic) | 1.51 / 1.74 | Compared through each family's own matched rho. |
| T0n to T0, T column fold <-1, tau_T> | 1.34 / 1.41 | Fewer trials: nofold needs 1.39x to 1.49x the trials. Note that this is not 2x. |
| T0 to T1, square root with check + sigma-validity memo | 1.23 / 1.54 | Removes most exponentiations (Legendre calls per trial: about 4.8 down to about 0.6). |
| T1 to T1j, Jacobi = 1 I | 1.04 / 1.10 | A pricing assumption. |
| T1 to T2, square-root table | 1.76 / 2.28 | Removes every remaining F_p exponentiation. This is the largest single step at 36 bits. |
| T2n to T2, T fold with cheap square roots | 1.36 / 1.45 | |
| B4 to T2n, oracle change with cheap square roots on both sides | 2.4 / 4.0 | |
| **B0 to T2, total** | **4.4 / 8.6** | |

The ratios multiply only approximately, because B and T are on different curve families with
different rho references.

### Slopes of log2(gap) against log2 r (OLS over 6 sizes, +- standard error)

| Variant | I = 3M | I = 1.5 log2 p M |
|---|---|---|
| B0 | +0.035 +- 0.006 | +0.003 +- 0.006 |
| B4 | +0.029 +- 0.006 | -0.001 +- 0.006 |
| T0 | +0.015 +- 0.005 | -0.012 +- 0.004 |
| T1 | -0.001 +- 0.008 | -0.022 +- 0.006 |
| T2 | -0.019 +- 0.003 | -0.025 +- 0.003 |

T2's straight line would cross 1 near log2 r = 123 (I = 3M) or 91 (I = 1.5 log2 p M). I do not
read that as a crossover forecast [pcc:unverified]:

- T2's per-trial cost is flat in p (80-82 M + 4.4 I).
- Its negative slope comes entirely from trials per rho step falling from 0.66 to 0.50
  (`analysis.txt`). That is IC trials/sqrt r falling (0.56 to 0.47) while rho steps/sqrt r
  rises (0.85 to 0.94). Both are finite-size and noise effects.
- In the 2lp random-graph regime, trials/sqrt r tends to a constant, and rho tends to
  sqrt(pi/4) ~ 0.886. The expected limit is about 0.5 x (81 + 3 x 4.4) / 15, which is about 3.

Findings [pcc:unverified]:

- **The folds close most of the gap but not all of it.** At 36 bits the gap goes from 27.4x
  to 3.2x (I = 3M), and from 12.9x to 2.6x at I = 1.5 log2 p M. The intervals stay above 2.7
  at 36 bits.
- **The decisive lever is removing the log p root step.** The 2-torsion symmetric oracle turns
  root finding into one quadratic plus square roots. Once square roots are lookups, IC's
  per-trial cost no longer depends on p. The quartic oracle cannot get there: B4 keeps about
  545 M per trial at 36 bits, because of s^p mod F.
- **A remaining floor of about 3x is structural in this design.** IC uses about 0.5 trials per
  rho step. Each trial pays the same walk addition as a rho step (15 M-eq) plus about 69 M of
  oracle and 3.4 extra inversions. Parity would need a trial to cost about 2 rho steps, i.e.
  about 30 M-eq, including the walk step.
- **The same direction as the Rust study, with a smaller residual gap.** The Rust study found
  7.7x from the T fold plus the symmetric system at 40 bits, with GAE in a different
  instrument. Here B0 to T0 is 2.5x with exponentiation square roots, and B0 to T2 is 8.6x.
  The units differ (M-eq here, GAE there), so the numbers are not directly comparable.

## Limits

- Toy sizes: log2 r 16 to 36, 5 curves per size, 3 targets per curve. Python instrument;
  operation counts, not time. Wall time was not used.
- **The square-root table is a memory-for-time trade.** It needs p words, the same order as
  IC's own vertex set and as store-all rho's table (~0.9 sqrt r ~ 0.45 p points). It is
  charged at (p-1)/2 M, with lookups at 0 to 1 M. Lookups are not free on real hardware for
  large p; at w_l = 1 the T2 gap at 36 bits is 3.28.
- **Jacobi = 1 I is a pricing assumption.** At I = 3M that is a batched-inversion price, which
  a lone Jacobi symbol does not get. T1j is a sensitivity row; T2 does not depend on it.
- **B1 (the shift) is an accounting correction to how ext_field charged s^p mod F**, not a new
  algorithm. It is listed so that later rows do not inherit the overcharge.
- **Not implemented:**
  - the GLS/psi (Frobenius-like) fold. The torsion curves are random over F_{p^2} and have no
    such endomorphism. In the Rust study psi helped both IC and rho, and the matched gap stayed
    above 100x.
  - Ferrari/Cardano root finding for the base quartic.
  - A cheaper (8M + 1I) addition with 2-torsion points to give a second target per walk step.
  - Batching several targets' inversions below 3M.
- **Fold contributions are compared across curve families** (prime order against cofactor 4),
  each with its own matched rho. "T fold" and "table" are paired on identical trial streams;
  "oracle change" is not.
- No preregistration, A/A arm or independent replay. msolve is not used, so every run is
  deterministic and replayable from its seed.

## Errors hit

- **Variant streams diverged at first (fixed before the sweep).** The table square root returns
  the smaller root, but exponentiation can return either. That swapped the order of
  (sigma1, sigma2), and with it the nofold vertex labels. The nofold solver then finished at a
  different trial, and the RNG streams split. Fixed by ordering sigma1 <= sigma2. After the
  fix, all 180 tors targets pair exactly across variants.
- **Table installed before curve generation (fixed).** The first table smoke run crashed because
  curve generation called the table before it was built. Curve generation now always uses
  exponentiation square roots, and the variant is switched on afterwards, so all variants see
  the same curves.
- **The `time` wrapper of `run_all.sh` was killed** (exit 143) while `xargs` and the jobs carried
  on. All 150 runs finished, and no log contains a traceback. Wall time is taken from the
  file timestamps instead.
- **research-kb CLI not installed** in this container. The knowledge-base query was not done.

## Files

- `ic_fp2_folds.py`: the instrument.
- `ic_fp2_copy.py`: the unchanged ext_field copy.
- `test_oracle.py`, `test_oracle.out`: the positive control.
- `run_all.sh`: the sweep.
- `runs/`: 150 JSON and log files, 1.2 MB in all.
- `analyze.py`, `analysis.txt`, `summary.json`: the analysis.
- `gap_vs_size.{png,svg,pdf}`: the plot.

## Independent review notes (2026-11-15)

A separate reviewer reran cells and recounted from the saved outputs. Verdict: holds with caveats [pcc:unverified].

Corrected headline: the 3.2x (36 bits) applies only to the cofactor-4 2-torsion curve family, with its own rho. On the prime-order curves of `../ext_field` the best measured variant is B4 at 18.4x. The B0 to B1 step (1.14-1.18x) is an accounting correction, not an improvement, so the baseline for gains is B1 (23.2x at 36 bits). T2 relies on an O(p) square-root table with lookups priced at 0, which only works at toy p. A t-interval at 36 bits is 2.33-4.46, so parity is still excluded, but the quoted 2.73-4.01 is too narrow. The plateau near 3x is an untested argument. [pcc:unverified]

### Corrections to the text above

Where these conflict with the text above, these take precedence [pcc:unverified].

- The README says 'The square-root table is built in each IC run and charged in full: (p-1)/2 M'. That is false for B4 (base arm): B.ic_run resets the counters after build_table(), so the table is not in B4's M, and analyze.py does not add table_M back.
- The README says the table costs 'about 2.5% of T2's cost'. The saved runs give 1.9 to 2.2 % (I=3M).
- The README says 'the lowest per-curve gap is 2.57', next to 'At I = 1.5 log2 p M the best gap is 2.6x'. The 2.57 per-curve minimum holds only at I=3M, and it comes from the 32-bit size (p=131071), not 36 bits. At I=1.5 log2 p M, per-curve T2 gaps reach 2.11 (32 bits) and 2.13 (36 bits).
- The README lists 'the sigma-memo probes' among the IC lookups. Only the sigma-validity probes in sigma_ok (memo/table variants) are counted. The vertex-memo probes in vertex() are never counted, and naive-variant memo probes are not counted at all.
- The README says 'The intervals stay above 2.7 at 36 bits'. That holds only for the I=3M bootstrap. At I=1.5 log2 p M the 36-bit bootstrap interval is 2.21 to 3.25, and a t-interval at I=3M gives 2.33 to 4.46.
- The README gives the T2 slope SE as '+- 0.003'. That is the residual OLS SE; propagating the per-size uncertainty gives about +- 0.006.

### Caveats

- Framing: the headline '27.4x to 3.2x' compares two curve families. B0 runs on prime-order curves with their own rho. T2 runs on a special cofactor-4 family (a rational 2-torsion point T=(x0,0) with f'(x0) a square, a and b not both in F_p), with a different rho. On the prime-order family that B0 measures, the best measured variant is B4 at 18.4x (36 bits), not 3.2x. The 3.2x applies only to curves that have this 2-torsion structure, so it does not improve IC on the baseline target class. The README admits the cross-family comparison under Limits, but the headline does not say it.
- Accounting is counted as improvement: the B0-to-T2 total (8.6x at 36 bits) includes B0-to-B1 (1.18x). The README calls B0-to-B1 an accounting and implementation correction. Under the success criterion it is not an improvement, so the headline's starting point should be B1 (23.2x at 36 bits, 15.6x at 16 bits), not 27.4x / 17.8x.
- T2's advantage rests on a square-root table over all of F_p, with lookups priced at 0 by default. The table needs O(p) memory and only works at toy p. The w_l=1 sensitivity (3.28) is shown, but the headline uses w_l=0. The p table writes are not priced as lookups either.
- The intervals are too narrow. The 95% CI is a percentile bootstrap over only 5 curves, which is anti-conservative at n=5. A t-interval on the log per-curve gaps at 36 bits is 2.33 to 4.46, against the reported 2.73 to 4.01. The conclusion that parity is excluded still holds, but the quoted CI and the line 'intervals stay above 2.7' overstate the precision.
- The slope standard errors are understated. The OLS SE comes from residuals with 4 degrees of freedom and ignores per-size noise. Weighting by the per-size SE gives -0.0175 +- 0.0063 for T2 at I=3M, against the reported -0.019 +- 0.003.
- The forecast that the gap levels off near 3x is an untested theoretical argument (trials/sqrt r assumed to tend to a constant). The measured T2 slope is negative at roughly 3 SE over 16 to 36 bits, and the README attributes it to finite-size drift without data at larger sizes to support that. It is labelled [pcc:unverified], but the headline states the plateau as an expectation.
- B4 (square-root table on the base arm) does not charge the table. base_ic_run calls build_table() and then B.ic_run, which zeroes C[0]/C[1]/C[2] at its start. table_M is recorded in the JSON, but it is neither in M nor added back in analyze.py. The effect is negligible (0.2 to 0.3 % of IC cost) but contradicts the README.
- Lookups are undercounted. The vertex-memo probes (orc.memo.get in vertex(), twice per relation) are never added to lk. In the naive variant (T0/T0n) the sigma_ok memo probes are not counted either. Only the valid-dict probes in sigma_ok count in memo/table mode. The effect at w_l=1 is about 1% of T2 cost, but the README says the sigma-memo probes are counted.
- The positive control (test_oracle.py) exercises only the naive variant: it never calls set_variant('memo'/'table'). The memo and table oracle paths are covered only indirectly, by identical trial streams and log asserts. Of the '600 of 600' agreements, 331 are cases where both oracle and enumeration return the empty set; only 269 targets were decomposable.
- Toy evidence: 5 curves per size and 3 targets per curve, with no A/A arm, preregistration or independent replay. It is not a bench.record, so it cannot support a leaderboard or success claim (the README acknowledges this).
