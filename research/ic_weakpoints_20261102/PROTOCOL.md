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
