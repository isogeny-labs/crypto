# L6: a large-prime variation of the cover attack does not pay at p <= 12k (2026-10-18)

Lead L6 of #2216 ("the only lead that changes the exponent"). **Verdict: negative for the cover decomposition path in the range that matters (p <= 12k).** The model gives at most a 0.02 % saving with one large prime, 0.27 % with two, and 0.56 % for an information bound that no scheme is known to reach. The 10 % gate fails by a factor above 18. No change to the attack was made; the only code is opt-in logging in the dlp path and a model example.

Status labels: **measured** (a recorded run), **derived** (computed from measured values), **proposed** (not built). No wall-clock claims (shared box). No new cross-method comparison with rho is made, so the AGENTS.md §5a band does not apply; the rho figures quoted from other studies are theirs and are lower-bound comparisons.

Files: [`REPORT.pdf`](REPORT.pdf) (source `REPORT.typ`), [`COSTMODEL.md`](COSTMODEL.md) (derivation), [`run.sh`](run.sh), [`collect.sh`](collect.sh), raw records in [`raw/`](raw/), graphs in [`figs/`](figs/) (SVG + PNG, source `plot.py`), Lean in [`lean/`](lean/), model code `examples/jv_lp_model.rs`, problem record [`problems/jv-cover-subbase-restricted-test.md`](../../problems/jv-cover-subbase-restricted-test.md).

## The reason

1. **The test cannot be restricted to a sub-base (argued).** A decomposition test solves six quadrics over all of `F_p`; every solution is a full relation over `F`, at the same `C_cov` whatever `F'` is. A sub-base only filters solutions afterwards, so shrinking `F'` never makes a relation cheaper. In Gaudry-type large-prime attacks the trade is a smaller linear algebra against more relations; here relations are 99.99 % of `S` and cost the same per solution.
2. **The linear algebra has nothing to trade against (derived from measured constants).** LA = 419 `u²` multiplications (measured: 26.2 Wiedemann operations per `u²` at row weight 7, 16 multiplications each). Relations = `N·T·C_test` with `T = 779 ± 33` tests per decomposition (measured, 14 runs, 2139 decompositions; the heuristic is 720) and `C_test = 1.22M` (measured, #2182). LA share of `S`: 2.2e-5 at p = 101 (matches #2182's 0.0045/194), 5.4e-5 at 251, 2.2e-4 at 1009, 8.7e-4 at 4096, **2.5e-3 at 12000**; 1 % at p ≈ 4.9e4, 10 % at p ≈ 5.5e5, 50 % at p ≈ 5.1e6.
3. **Even a free linear algebra saves little (derived, checked by Monte Carlo).** Any elimination of the points outside `F'` removes exactly the rank it needs, so it needs at least as many solutions as the plain method (`c* = 0.9975`, versus `N+1`). The saving is at most the LA share plus 0.25 %. Single and double large-prime graphs need more solutions than that bound: at f = 0.5 about 8 N and 2.8 N.

## Table (derived): best S(α)/S_full over the sub-base size, N = p/2

| p | LA share (plain) | 1 large prime | 2 large primes | information bound |
|--:|--:|--:|--:|--:|
| 101 | 2.2e-5 | 0.9995 | 0.9981 | 0.9980 |
| 251 | 5.4e-5 | 0.9991 | 0.9978 | 0.9944 |
| 1009 | 2.2e-4 | 0.9997 | 0.9973 | 0.9966 |
| 4096 | 8.7e-4 | 0.9999 | 0.9973 | 0.9957 |
| 12000 | 2.5e-3 | 0.9998 | 0.9975 | 0.9944 |
| 1e5 | 0.020 | 0.9997 | 0.9969 | 0.977 |
| 1e6 | 0.166 | 0.996 | 0.980 | 0.832 |
| 1e7 | 0.661 | 0.916 | 0.778 | 0.34 |
| 1e8 | 0.950 | 0.542 | 0.378 | 0.050 |
| 1e9 | 0.995 | 0.264 | 0.160 | 0.0054 |
| 1e12 | 1.000 | 0.029 | 0.013 | 7e-6 |

Rows at p ≤ 12k are within 0.56 % of 1 (differences at the 0.1 % level are Monte-Carlo noise). The graph and the merged rows' extra weight are priced at 0 and at the plain weight, which favours large primes. The 10 % gate is first passed between p = 1e6 and 1e7 with two large primes and between 1e7 and 1e8 with one; the bound column at f < 0.05 is extrapolated. Asymptotically (single large prime) `S ∝ N^{2 - 4/13} = N^{1.69}` against `N^2`: the exponent does change, but only in the LA-dominated regime p ≳ 1e6. That regime lies two to three orders of magnitude above the p at which the plain method's S/rho crosses 1 in #2216's projection (1.7k–12k), so it does not move the crossing.

## Measured against modelled yields

Real decompositions (opt-in `JV_LP_LOG=1`, the dlp path unchanged; control: seeds 1, 2 at p = 101 and seeds 1, 2 at p = 251 reproduce the earlier records' residual counts and multiplication totals exactly, and every run's discrete logarithm verified): 322 at p = 101 (6 seeds), 254 at p = 251 (2 seeds).

- The six abscissae are uniform over the base: per-seed chi-square z between -1.5 and +0.6 (p = 101: -0.7 to +0.6); all six distinct in every decomposition.
- Share of decompositions with 0, 1, 2, ≥3 points outside a sub-base of fraction f = 0.1..0.9 against the hypergeometric model: largest |z| = 2.6 over 72 cells, no systematic bias ([`figs/fig_yields`](figs/fig_yields.svg)).
- Relations that the real decompositions give each scheme on random sub-bases equal the uniform model's (for example at p = 101, f = 0.5: 1.10 real against 1.07 modelled single-prime relations, 27.67 against 27.71 for the bound; `raw/yields.json`, `relation_counts`). At X ≈ N solutions the real data gives about 1 relation of the 26 needed at f = 0.5.

## What would change the exponent (ranked; numbers are in the repo)

1. **The sieve path (`src/cryptanalysis/jv_sieve.rs`), with a large prime in it (proposed).** Its relation search walks the factor base itself, so a sub-base does shrink the work per line, and the linear algebra is no longer negligible there: **la share of S measured 0.04–0.07 % at p = 251, 0.8–2.4 % at 1009, 4.7–8.8 % at 1511 and 1777, 8.5–9.9 % at 1823** (`experiments/32_*, 35_*, 38_*`). Relation cost there is 2.1–2.7M multiplications at p = 1823 (m = 9) against 0.9G for 779 cover tests (derived: 779 × 1.22M). Recorded S/rho (lower bound, as recorded): 0.039–0.069 at 1009 and 0.0030–0.0037 at 1823. Whether a counter at m−1 (the last root is free) helps is not modelled here: the next study.
2. **A different number of points m.** In the sieve m is a free parameter (m = 11 at p = 101, 10 at 251, 9 from 1009; `m_final`); in the decomposition path m = 6 is fixed by the dimension of the Jacobian over `F_p`, which is why `T ≈ 6!` and `C_cov` multiply.
3. **A cheaper test: a constant, not an exponent.** L1–L4 of #2216 reach about 1.4× on `C_cov`.
4. **Large primes in the decomposition path:** at most 0.56 % for p ≤ 12k (this study).

## Open and not done

- Proposition 1 (no sub-base-restricted test is cheaper) is argued, not proved ([problem record](../../problems/jv-cover-subbase-restricted-test.md)).
- The rank bound holds for generic coefficients; the actual rows have ±1 entries. Lean: only the conditional arithmetic step is proved (`lean/LpBound.lean`, axioms `propext, Quot.sound`); the rank bound itself needs linear algebra and is open (no Mathlib).
- Monte Carlo at N ≤ 6000 (N-independence checked 484 → 6000 within 3 %); p ≥ 1e6 rows use that curve.
- Seeds at p = 251: two (254 decompositions); a third was started and stopped for time.
- No `benchmarks/` entry or leaderboard change: no speedup is measured or claimed.
- `cargo test --release --lib jv_cover`: see REPORT.pdf, "Correctness".
