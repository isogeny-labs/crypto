# Protocol: six weak points of EC index calculus, measured on Weil-descent systems (frozen 2026-11-02, before data)

Tool: `research/ic_weakpoints_20261102/tool` (Rust, std only), commit `1a572f0607`.
Family: E: y^2 = x^3 + a x + b over F_{p^n}, a, b uniform, n in {2,3}; factor base {P : x(P) in F_p}
(Gaudry/Diem descent). Decomposition system: S_{n+1}(x_1..x_n, x_R), symmetrised to e_1..e_n,
Weil-restricted to n equations over F_p. All outputs are exact counts; no wall time is reported.
Seeds: base 20261102, per-(experiment, p, index) derived (`field::derive`).

## Arms and sizes
| id | command | p | instances |
|---|---|---|---|
| A3 | `algebra --n 3 --inst 8 [--enum if p<=127]` | 31 61 127 251 509 1021 4093 65521 1048573 16777213 1073741789 | 8 targets/p, fresh curve each |
| A2 | `algebra --n 2 --inst 8 [--enum if p<=127]` | same | 8 |
| Y  | `yield --curves 3 --targets 2000` | 61 127 251 509 | 3 curves x 2000 random targets |
| P  | `pipeline --curves 3 --rho 30` | 31 61 127 251 509 | 3 curves; rho 30 runs/curve |

## Predictions and falsifiers (rows of the weak-point table)
1. Solving degree (Macaulay, grevlex, Caminata-Gorla definition) of the n=3 system is 10 = 3(4-1)+1
   for every p >= 61, quotient dimension 64 = Bezout. Falsified if the median degree rises with p.
   Matrix at the solving degree (rows x cols, dense elimination multiplications) = c(3), constant in p.
5. Zero-dimensional for every instance; no positive-dimensional line in the enumeration; singular rational
   solutions rare. Mean F_p-rational solutions per target ~1 while genuine decompositions ~ lambda ~ 1/6:
   most solver outputs are spurious (non-split cubic). Falsified if genuine/rational > 0.5.
3. Per-target count of decompositions vs Poisson(lambda_pred = F^3 / (6 N)): report mean / lambda_pred,
   P(0), dispersion index var/mean with 95% bootstrap CI (1000 resamples). Poisson rejected if the CI
   excludes 1. Heavy tail: report max and top-1% share.
2/7. Factor-base fraction phi in {1, 1/2, 1/4, 1/8}: full-relation yield / phi^3 and one-large-prime
   yield / (3 phi^2 (1 - phi)) both ~ yield(1). Falsified if either ratio leaves [0.7, 1.3] at p >= 251.
4. Rank of the over-collected matrix (unknowns + 20 rows) = unknowns - empty columns; log Q determined and
   correct on every curve. Controls: cofactor scaling by h gives the same k; dropping signs makes the
   system inconsistent (positive control for the detector); 10 duplicated rows leave rank unchanged.
6/8. Cost in group-addition equivalents: IC = walk additions + w (trials * c(3) + LA), the decomposition
   oracle's own additions excluded and replaced by the measured Groebner price c(3); rho measured
   (r=20 adding walk, Brent) and analytic negation-map sqrt(pi l / 4), rho comparisons priced at w.
   Band w in {0, 0.1, 1}. Prediction: IC above rho at w = 0.1 and 1 at every toy size; order resolved
   only if it holds across the band.

Scope: toy sizes (group <= 2^28), exact computation on one 4-core container; not an ecbench session,
not a leaderboard row, not admitted by the validation gate. A derived crossover is a model output.

## Amendment 1 (2026-11-02, after the frozen arms, before any follow-up data)
Observed in the frozen arms: pooled (p >= 251) full-relation yield at phi = 1/2 is 0.85x its exact
hypergeometric expectation, 95% CI [0.73, 0.97], while one-large-prime yield is 1.04x [0.96, 1.11].
The counts at phi = 1/4, 1/8 are nested subsets of phi = 1/2, so this is one ~2.3 sigma deviation.
Follow-up arm Y2, fresh seed base 20261103 (independent curves and targets):
`yield --p 251 --curves 4 --targets 6000 --seed 20261103` and the same at p = 509.
Decision rule: the deficit is real if the pooled Y2 full ratio at phi = 1/2 has a 95% CI excluding 1
in the same direction; otherwise the frozen-arm deviation is recorded as noise. Y and Y2 are reported
separately, then pooled.

## Amendment 2 (2026-11-02, before any data): large-prime variation, arm L
Tool subcommand `dlp` (`tool/src/dlp.rs`). One relation stream per curve; factor base FB_r = the first
ceil(F p^(r-1)) x-classes (F = all x-classes in F_p, so |FB_r| ~ p^r / 2), r in {1, 5/6, 3/4, 2/3, 1/2}.
Variants on the same stream: 0LP (full relations only), 1LP (merge relations with one point outside
FB_r), 2LP (also two, eliminated as graph cycles). Each variant stops at |FB_r| + 21 full rows, which are
solved mod ell; log Q checked against the planted k. Trial cap 40 p (censored cells reported as such).
Arms: `dlp --p P --curves 2 --seed 20261104` for P in 127 251 509 1021; P = 2039 as two processes
(`--c0 0 --curves 1`, `--c0 1 --curves 1`).
Predictions: (a) 0LP trials T ~ p^(3-2r) (from the measured phi^3 yield); (b) at r = 2/3 the 2LP
trial exponent is <= 1.4 (theory 4/3, Gaudry-Thome-Theriault-Diem), below rho's 1.5, while 0LP is 5/3;
(c) every reached cell recovers log Q; (d) merged 2LP rows are heavier than 0LP rows but by < 3x.
Falsified: (b) if the fitted 2LP exponent at r = 2/3 exceeds 1.5; (c) by any wrong or undetermined k.
Exponents are least-squares slopes of log T on log p over reached cells (curves averaged in log).
