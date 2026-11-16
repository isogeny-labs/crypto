# Growth exponents of prime-field decomposition (closing the prime-field line)

Question: does any decomposition tested in this study have a per-target cost
that grows more slowly than n, the factor-base size? In the cost model below,
that is the only route by which prime-field index calculus (IC) could improve
its exponent [pcc:unverified].

Exploratory analysis. This is not a bench.record run and not an ecbench
measurement. Every number is [pcc:unverified]. The wall-clock unit is seconds
(msolve 0.6.5 single-threaded wall time minus a trivial-system baseline, or
in-process Python time). The exact unit is point additions, counted for the
hash-join arms.

## Setup

- **Saved data (not rerun).** Medians over 8 targets from
  `../sym_m4/m4_p*.json`, `../sym_m4/split/*.json` and `../sym_m4/speed/*.json`.
  Arms D, Bn, Bs, Ds, Dy, By, Dp, M and Mt, with n = 8, 12, 16, 20, 24, 30 and
  p ~ n^3/3 (p = 193 ... 9001). These files keep only the median, min and max
  per cell, not per-target times, so their intervals bootstrap over sizes. That
  is a cluster bootstrap: draw distinct n with replacement and keep every
  replicate cell (file) at each drawn n. Draws with fewer than 3 distinct n are
  rejected. Arms with only 3 sizes (Ds) get no bootstrap interval, because
  every admissible resample is the original set. 4000 replicates, seed
  20261115.
- **Per-target replicate (new, small).** `m4_pertarget.py` uses the same
  curve, factor bases, encodings and timing rule as `../sym_m4/m4_sym.py`. It
  adds three things: per-target times and relation counts are saved, point
  additions are counted for M and Mt, and 16 targets are used instead of 8.
  Arms D and Bs ran at n = 8-20 only, as small cells; the big D/Bs grid at
  n = 24, 30 was not rerun. Arm Dp ran at n = 8-50. Arms M and Mt ran at
  n = 8-60. Intervals bootstrap over targets, resampling the 16 targets within
  each n and recomputing the median.
- **sp04 (m = 2, S3).** msolve arms at n ~ sqrt p (`../sp04/ms_p*.json`) and
  sympy arms at n = 4-16 pooled over 4 primes (`../sp04/gb*.out`).
- **Fits.** Least squares of log(time) on log(n), giving the slope alpha. The
  table also reports the OLS t-interval and the slope between the two largest
  sizes. Per relation means median time x targets / relation multisets for
  saved cells, and the sum of times / relation multisets for the rerun. The
  relation sets are dense multisets for D, Ds, Dy, M and Mt, binomial multisets
  for Bn, Bs and By, and partitioned multisets for Dp.
- **Noise-floor exclusions** (all listed in `fits.json`, `excluded_cells`). A
  msolve cell is dropped if its min is <= 0 or its median is < 3 ms. The 3 ms
  cut is about 3x the largest spread of the 7-run trivial-system baseline seen
  in the rerun (0.29-0.63 ms per run in the saved files; up to 0.90 ms in the discarded first run, printed but not saved). An
  in-process Python cell is dropped if its median is < 50 us. Dropped cells:
  - Dp at n = 8 and 12, saved and rerun.
  - M at n = 8, saved and rerun.
  - Mt at every saved n (all under 42 us) and at rerun n = 8-30.
  - sp04 msolve arms E, I and B at every n.
  - sp04 msolve arm P at n = 33.
  - sp04 at p = 106277, an empty file from an msolve segfault.
- **Checks asserted on every target in the rerun.**
  - Each msolve basis is [1] iff brute force finds no solution.
  - M and Mt return exactly the brute-force set of valid relation multisets.
  - The fast brute-force evaluator agrees with direct S4 evaluation on 5
    random triples per target.
  - The Newton-sum identities hold numerically.
  - No discrete logs are solved in this track [pcc:unverified]. The decomposition answers are
    checked against brute force instead.

## Commands

```
cd research/stange_torsion_division_20261113/growth_fit
export GF_SCRATCH=<scratch dir>
for pn in "193 8" "577 12" "1361 16" "2741 20"; do set -- $pn
  python3 m4_pertarget.py $1 $2 --arms=D,Bs,Dp,M,Mt --targets=16 > runs/pt_p$1_n$2.json 2> runs/pt_p$1_n$2.err; done
for pn in "4657 24" "9001 30" "21401 40" "41651 50"; do set -- $pn
  python3 m4_pertarget.py $1 $2 --arms=Dp,M,Mt --targets=16 > runs/pt_p$1_n$2.json 2> runs/pt_p$1_n$2.err; done
python3 m4_pertarget.py 71941 60 --arms=M,Mt --targets=16 > runs/pt_p71941_n60.json 2> runs/pt_p71941_n60.err
python3 fit.py > fit.out      # writes fits.json, fit_table.md, growth_fit.{png,svg,pdf}
```

Rerun compute was about 2.5 minutes. Per-n medians, relation counts and
addition counts are in `runs_summary.txt`.

## Results: fitted exponents alpha (cost ~ n^alpha)

Selected rows. The full table, including per-relation fits for every arm, is
`fit_table.md`. All values are [pcc:unverified].

| arm (encoding) | data | n range | alpha per target | 95% interval | alpha per relation | 95% interval |
|---|---|---|---|---|---|---|
| D dense S4 | saved | 8-30 | 4.30 | [4.12, 4.59] sizes | 4.36 | [3.96, 4.76] sizes |
| D dense S4 | rerun | 8-20 | 4.09 | [4.00, 4.17] targets | 4.34 | [3.81, 4.79] targets |
| Bn binomial | saved | 8-30 | 4.27 | [4.05, 4.67] sizes | 4.20 | [2.03, 5.17] sizes |
| Bs binomial symmetrized | saved | 8-30 | 4.33 | [4.03, 4.77] sizes | 4.54 | [2.25, 5.17] sizes |
| Bs binomial symmetrized | rerun | 8-20 | 4.06 | [4.00, 4.11] targets | 4.82 | [3.48, 5.88] targets |
| Ds dense symmetrized | saved | 8-16 | 3.90 | OLS t [3.71, 4.09] | 4.45 | OLS t [3.61, 5.28] |
| Dy APS split | saved | 8-30 | 4.63 | [4.49, 4.92] sizes | 4.88 | [4.36, 5.08] sizes |
| By APS split binomial | saved | 8-30 | 4.65 | [4.55, 4.90] sizes | 5.27 | [3.00, 5.74] sizes |
| Dp partitioned bases | saved | 16-30 | 4.43 | [4.14, 4.75] sizes | 3.70 | [3.25, 3.89] sizes |
| Dp partitioned bases | rerun | 16-50 | 4.42 | [4.38, 4.47] targets | 4.12 | [3.56, 4.76] targets |
| sp04 sympy plain S3 (m=2) | saved | 4-16 | 2.13 | [2.05, 2.39] sizes | - | - |
| sp04 sympy pulled-back (m=2) | saved | 4-16 | 3.15 | [2.83, 3.28] sizes | - | - |
| sp04 msolve P pulled-back (m=2) | saved | 101, 1001 | 2.30 | none (2 sizes) | - | - |
| M hash join, seconds | rerun | 12-60 | 1.88 | [1.78, 2.01] targets | 1.87 | [1.64, 2.09] targets |
| M hash join, point additions | rerun | 8-60 | 1.87 (exactly n^2 + 3n) | exact | - | - |
| Mt amortized hash join, seconds | rerun | 40-60 | 0.76 | [-0.05, 0.98] targets | 1.04 | [0.02, 2.13] targets |
| Mt amortized hash join, point additions | rerun | 8-60 | 1.00 (exactly 2n) | exact | - | - |
| Mt one-time pair table, point additions | rerun | 8-60 | 1.95 (n^2 + n) | exact | - | - |

Plot: `growth_fit.png` / `.svg` / `.pdf`. The left panel shows per-target
time vs n on log-log axes with the fitted lines, the noise floors and excluded
cells drawn hollow. The right panel shows the exact addition counts for M and
Mt.

Saved and rerun fits agree within their intervals for D, Bs and Dp. The rerun
D and Bs slopes cover only n <= 20 and run slightly lower. In every Gröbner
arm the slope between the two largest sizes (4.4-5.1 per target) is above the
global slope, so growth is not flattening at these sizes [pcc:unverified].

## Verdict

- No algebraic (Gröbner) decomposition has an exponent whose upper bound is
  below 1 [pcc:unverified]. Every msolve S4 arm (three factor-base points per
  relation) has a per-target alpha of about 3.9-4.7, with 95% lower bounds of
  3.7 or more. The m = 2 arms (sp04) have alpha of about 2.1-3.2. Symmetrizing
  (Ds, Bs), using a binomial base (Bn, Bs), splitting variables (Dy, By) and
  partitioning bases (Dp) change the constant, not the exponent.
- The only arm with alpha <= 1 is the generic amortized hash join Mt. Its
  exact cost is 2n additions per target, so alpha = 1.00. One fitted row has
  an upper bound below 1: Mt wall time over n = 40-60, alpha 0.76 with interval
  [-0.05, 0.98]. That is an artifact, not sublinear decomposition
  [pcc:unverified]:
  - The fit uses 3 sizes over a 1.5x range of n.
  - Each call costs 60-85 us, where fixed Python call overhead is a large
    share.
  - The unfiltered slope over n = 8-60 is 0.975 (`mt_unfiltered.txt`).
  - The per-relation interval [0.02, 2.13] contains 1.
  - The counted additions are exactly linear.
- Mt is a generic-group algorithm, so Shoup's generic lower bound applies
  whatever its constant.

## What this means for total IC cost against p^(1/2)

The model below extends the end-to-end model in `../ic_e2e/README.md`. Take n
= p^beta factor-base points and k points per relation. Here k = 3 for every S4
arm and the hash joins, and k = 2 for sp04. The decomposition probability per
target is about n^k / p, so beta <= 1/k. About n relations are needed. With
per-target cost n^alpha, the relation phase costs about p n^(alpha+1-k).
Sparse linear algebra, and for Mt the pair table, cost about n^2.

- If alpha <= 1, the balance gives a total of p^(2/(k+1-alpha)).
- If 1 < alpha < k-1, n is pinned at p^(1/k), and the total is p^((1+alpha)/k).
- If alpha >= k-1, larger n never helps, and the relation phase stays at
  order p or more. That is no better than exhaustive search.

The model rests on one assumption: per-target cost at fixed n depends only
weakly on p. The sp04 sympy timings support it. At n = 16 they are flat
(0.059-0.076 s) from p = 1009 to 10^6 [pcc:unverified].

Applied to the measurements [pcc:unverified]:

- **Mt** (k = 3, alpha = 1): p^(2/3). This matches the measured end-to-end
  plain IC slope. Refitting `../ic_e2e/runs/` here (best n per p, 3 primes)
  gives total additions ~ p^0.62 (OLS se 0.02), against p^0.52 (se 0.02) for
  Brent rho on the same curves (`fit.out`).
- **Gröbner S4 arms** (k = 3, alpha ~ 4): alpha >= k-1 = 2, so even before
  constants these arms put prime-field IC at cost ~ p or worse. That is far
  above both the Mt route and rho's p^(1/2).
- **sp04 m = 2 arms** (k = 2, alpha ~ 2-3): alpha >= k-1 = 1, which gives the
  same conclusion.
- **Hypothetical arms.** With k = 3, even a hypothetical alpha = 0 gives only
  p^(1/2), and alpha < 1 would only move IC from p^(2/3) toward p^(1/2). To go
  below p^(1/2) the model needs more points per relation, with alpha < k-3
  (k = 4: alpha < 1; S5 has degree 8 per variable) or alpha < k/2 - 1. The
  measured trend goes the other way: alpha grows from about 2 at k = 2 to about
  4 at k = 3. A generic hash join cannot supply it either, because its table
  term is about p^(1/2) (Shoup).

Conclusion [pcc:unverified]:

- Within the tested encodings, the prime-field line is closed. No
  decomposition shows sublinear per-target growth. The only linear one (Mt) is
  generic and already reproduces the measured p^(2/3) end to end.
- None of these measurements beats rho, and none narrows its exponent gap [pcc:unverified].
  The distance remaining is the measured end-to-end gap in `../ic_e2e/README.md`
  (plain IC 7.9-12.9x Brent rho; LP-IC about 2.2-2.4x store+negation rho).
- This track adds no new improvement over the IC baseline [pcc:unverified].

## Limits

- One curve (y^2 = x^3 + 5x + 7) for all sym_m4 and rerun cells.
- n <= 30 for the msolve S4 arms (n <= 20 in the rerun for D and Bs).
- Single-threaded msolve on a shared host.
- At most 6 sizes per fit. The bootstrap over sizes on 4-6 points is coarse;
  OLS t-intervals are shown next to it in `fit_table.md`.
- The saved files hold only medians, so the target-level bootstrap is
  available only for the rerun arms.
- Wall time is not group operations. Exponents carry over only if the cost per
  field operation is constant over the tested p (all p < 2^17).
- p grows with n (p ~ n^3/3) in the sym_m4 and rerun grids, so alpha mixes
  n-growth with any weak p-dependence.
- The cost model is a heuristic count, not a proof. It ignores the
  linear-algebra constant, relation weight and the log factors in msolve.
- Arms not covered:
  - S5 (k = 4).
  - Other Gröbner engines (Magma F4).
  - APS with genuinely disjoint smaller-degree bases beyond Dp.
  - Extension fields. Those are where IC exponents are known to improve [pcc:unverified], and
    they are out of scope here.

## Errors hit

- The first rerun replaced Tonelli-Shanks with an O(p) square-root search. That
  added about p ~ n^3 work to every M and Mt call and gave a spurious Mt slope
  of 2.4. It was caught because the time slope disagreed with the exact count
  (alpha = 1.00). The first rerun was discarded, the original Tonelli-Shanks
  was restored, and every rerun cell was run again. All reported numbers come
  from the second run.
- msolve 0.6.5 segfaulted (rc = -11) on the Dp system at n = 60 (p = 71941).
  The log is `runs/FAIL_Dp_p71941_n60.err`. It comes from the first run, but the square-root change does not touch the msolve input. The second run did not retry Dp at n = 60, so n = 60 has only M and Mt. This
  matches the earlier sp04 segfault at p = 106277. Cause not investigated.
- With 3 sizes, the first fit version reported a degenerate bootstrap interval
  for Ds ([3.90, 3.90]). Bootstrapping over sizes now needs at least 4 sizes,
  and Ds reports only the OLS t-interval.

## Independent review notes (2026-11-15)

A separate reviewer reran p = 577/n = 12 and p = 1361/n = 16 for all arms. All
assertions passed, and timings and exact addition counts matched. They flagged
these limits on the headline [pcc:unverified]:

- The conclusion is about the encodings tested, not about prime-field algebraic decomposition in general. Every Gröbner arm enters the factor base as a degree-n univariate (dense product or X^n = 1), so the ideal degree grows with n by construction. Factor bases with a low-degree description, such as the PKM 2016 composed-map bases, were not tested.
- The fitted n-exponent is confounded with p, because the grids use p ~ n^3/3.
- The end-to-end exponents rest on 3 primes. With df = 1 the 95% t-intervals are p^0.62 [0.38, 0.86] for plain IC and p^0.52 [0.31, 0.72] for Brent rho, and they overlap, so the exponent gap is not established. Only the measured ratios (7.9-12.9x) are.
- "Mt gives p^(2/3)" is the plain-IC route only. The large-prime variant in `ic_e2e/` measured about p^0.48, 2.2-2.4x behind strong rho.
- Not every 95% lower bound is at least 3.7. That holds for the bootstrap intervals, but the OLS t lower bounds go down to 3.60. Units are wall seconds, not counted operations, except for M and Mt.
