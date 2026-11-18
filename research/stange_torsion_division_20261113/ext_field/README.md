# Track 1: index calculus on E/F_{p^n} (Weil descent) against matched Pollard rho

Exploratory toy measurement, 2026-11-15. This is not a `bench.record` benchmark. Every number
below comes from a saved run in `runs_n2/` or `runs_n3/` and is collected in `summary.json`
and `analysis.txt`. All numbers are [pcc:unverified].

## Prior work in the repository (reused as a reference, not as code)

- `research/blindspots_20261018/01-fp2-gaudry-k2/` and
  `benchmarks/20261018-blindspots-fp2-gaudry-k2/` already hold a validated Rust study of Gaudry
  k = 2 on E(F_{p^2}) at log2 #E = 24..40. It has 2-torsion folds and one or two large primes,
  and it is measured against matched rho-neg in GAE. Its verdict is REGRESSES (constant): the
  best arms cost 50x and 52x rho at 40 bits (w = 0). This track is an independent Python
  instrument with a different counting unit (F_p multiplications). It also adds n = 3.
- `research/stange_torsion_division_20261113/ic_e2e/` is the generic prime-field
  hash-join IC from earlier in this session.
- `rg -il "weil|gaudry|descent|fp2"` found no reusable Python Weil-descent decomposer. The
  Rust instrument above cannot be imported here, so I wrote the code from scratch.
- The research-kb server was not queried in this session. Nothing below claims novelty.

## Setup

- **n = 2** (`ic_fp2.py`): F_{p^2} = F_p[t]/(t^2+1) with p = 3 mod 4. The curve is
  E: y^2 = x^3 + ax + b with random a, b in F_{p^2}, and #E(F_{p^2}) = r is prime. The order is
  found by baby-step giant-step over the Hasse interval, and that setup is not counted.
  - Factor base: points with x in F_p, about p/2 classes up to sign. A vertex is x, with a
    canonical y.
  - Decomposition R = s1 F1 + s2 F2: write S3(X1, X2, x(R)) in s = X1 + X2 and m = X1 X2, and
    split it over F_p (Weil descent). The imaginary part is linear in m, so m = N(s)/D(s), and
    the real part times D^2 is a quartic in s over F_p.
  - Root finding: gcd(s^p - s, F), then Cantor-Zassenhaus. Then X1 and X2 come from s and m.
    Each X needs f(X) to be a square in F_{p^2} (norm Legendre). The y lifts use F_{p^2}
    square roots, and one point addition fixes the signs.
  - Linear algebra: every relation has weight at most 2, so it is an edge on the x-values.
    A union-find with signed offsets mod r finds the log exactly (the double-large-prime graph).
    `test_solver.py` checks that the union-find finishes at exactly the same relation as a
    rank-based elimination mod r: 300 random graphs, 0 mismatches (`test_solver.out`).
  - Modes, replayed on the same relation stream:
    - `2lp`: every relation kept. The whole of F_p is the vertex set.
    - `1lp:B`: relations with at least one x < B (single large prime). B = p/2 and B = p/4.
    - `full:p/2`: both x < p/2. This is plain Gaudry with a smaller base.
- **n = 3** (`ic_fp3.py`, `make_s4.py`): F_{p^3} = F_p[t]/(t^3 - t - c). E has prime order.
  - S4 = Res_X(S3(x1,x2,X), S3(x3,x4,X)) is symmetrized in e1, e2, e3 (35 monomials, total
    degree 4; `s4_sym.json`, checked by substitution).
  - Weil descent gives 3 equations over F_p in e1, e2, e3. msolve 0.6.5 returns their solutions as a
    rational parametrization. The eliminating polynomial has degree about 64 (the Bezout bound),
    and its F_p roots are found with the counted routine. T^3 - e1 T^2 + e2 T - e3 must have 3
    distinct roots in the factor base. Signs come from point additions.
  - Linear algebra is incremental sparse elimination mod r. Modes: `all`, `2lp:B`, `1lp:B` and
    `full:B` with B = p/2.
- **Rho, matched:** same curve and subgroup. It uses an r-adding walk with 20 multipliers,
  stores every point, and matches on x, so +-R collide. That is the store-all negation
  reference. 3 rho runs per target, each with fresh multipliers.
- **IC walk:** the same 20-multiplier construction, but the multiplier is drawn by an RNG that
  does not depend on the point, so the walk cannot cycle. A target whose x was seen before is
  skipped, so IC never uses a rho collision; the skipped step's addition is still paid. A first
  version used a point-dependent walk and got free rho collisions, which I found and fixed
  before any reported run (see Errors).
- **Every discrete log** recovered by every arm is asserted equal to the true w. All runs
  passed: n = 2 has 90 IC targets x 4 modes and 270 rho runs; n = 3 has 32 IC targets and
  96 rho runs.

### Cost unit

- F_p multiplications (M), with squarings counted as M. F_p inversions (I) are counted
  separately and priced at I = 3M (batched-inversion level) or I = 1.5 log2(p) M (Fermat).
  Additions, hashing and union-find bookkeeping are not counted.
- Legendre symbols and square roots are exponentiations and are counted multiplication by
  multiplication. Polynomial products are counted as schoolbook.
- One F_{p^2} affine point addition costs 12M + 1I. One F_{p^3} addition costs 42M + 1I
  (mult 11M, sqr 8M, inversion priced 12M + 1I).
- The walk setup (20 scalar multiplications) is the same in both arms and not counted. The
  IC's mod-r solver work is counted separately (Zr). For n = 2 it is tiny: 50.6k Zr mults at
  p = 262139, against 1.05e8 M.
- For n = 3, msolve's Groebner and parametrization work is **not** counted. The n = 3 IC
  numbers are therefore lower bounds. I also report the break-even budget, which is the whole
  per-decomposition cost that would tie rho.
- Wall times are Python on a shared 4-vCPU container running 3-4 processes at once. Read them
  only as a second view.

## Commands

```
python3 make_s4.py                 # s4_sym.json (1.3 s)
./run_n2.sh                        # p in {251,1019,4091,16363,65519,262139}, seeds 1..5, 3 targets, 3 rho/target; 7.0 min wall
./run_n3.sh                        # p in {31,67,127,251}, seeds 1..4, 2 targets, 3 rho/target (SCR = msolve scratch dir)
python3 analyze.py > analysis.txt  # summary.json, gap_vs_size.{png,svg,pdf}
python3 test_solver.py > test_solver.out
```

## Results, n = 2

Ratios are ratios of means over curves (5 curves, 15 targets per size). M-eq uses I = 3M
unless noted.

| p | log2 r | rho steps / sqrt r | rho M-eq | IC trials / p | IC M per trial | IC 2lp M-eq | gap 2lp | gap 2lp (I = 1.5 log2 p M) | gap 1lp:p/2 | gap 1lp:p/4 | gap full:p/2 |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 251 | 15.9 | 0.97 | 3.64e3 | 0.53 | 368 | 5.24e4 | 14.4 | 10.7 | 17.2 | 27.9 | 30.1 |
| 1019 | 20.0 | 0.81 | 1.24e4 | 0.47 | 459 | 2.29e5 | 18.5 | 12.3 | 26.4 | 40.2 | 43.0 |
| 4091 | 24.0 | 0.77 | 4.70e4 | 0.50 | 547 | 1.17e6 | 24.9 | 15.0 | 32.4 | 46.2 | 51.2 |
| 16363 | 28.0 | 0.83 | 2.04e5 | 0.45 | 624 | 4.73e6 | 23.2 | 12.9 | 28.9 | 43.6 | 46.5 |
| 65519 | 32.0 | 0.99 | 9.77e5 | 0.49 | 725 | 2.39e7 | 24.5 | 12.4 | 29.7 | 42.5 | 49.2 |
| 262139 | 36.0 | 0.98 | 3.87e6 | 0.47 | 824 | 1.05e8 | 27.0 | 12.7 | 33.7 | 46.1 | 56.4 |

Fitted slopes of log2(cost) against log2 r (`analysis.txt`):

| Quantity | Slope |
|---|---|
| rho M-eq | 0.508 +- 0.010 |
| IC 2lp M-eq | 0.548 +- 0.005 |
| IC trials | 0.494 +- 0.005 |
| rho steps | 0.508 +- 0.010 |
| gap 2lp | +0.040 +- 0.011 |

Findings [pcc:unverified]:

- **IC is slower than rho at every size, and the gap does not shrink.** Measured gap to rho:
  14x to 27x at I = 3M and 11x to 15x at I = 1.5 log2 p M. Fitted in log2 r, the gap grows
  slowly (+0.04 per bit). That is the log p factor of root finding.
- **IC needs fewer trials than rho needs steps.** A solve needs about 0.47 p trials: about 0.23 p
  relations, which is about half the vertices that appear, the point where a component with two
  cycles forms. Rho needs about 0.9 p steps, so rho takes about 1.9 steps per IC trial.
- **Each trial is expensive.** A trial costs 368 to 824 M plus about 7.9 I. Most of that is
  s^p mod the quartic, which takes log2 p squarings mod a quartic. A rho step costs 12M + 1I.
  For parity, a trial would have to cost about 2 rho steps, about 30 M-eq. Checking whether a
  quartic has an F_p root in about 30 M-eq is not possible with gcd(s^p - s, F) once
  log2 p > 2.
- **The large-prime variants did worse in this setup.** A smaller base with one large prime
  (`1lp`) or none (`full`) only throws relations away. The weight-2 graph already makes linear
  algebra almost free, so the reason for a smaller base in Gaudry's analysis is absent here.
  `2lp` with every x in F_p is the best arm. This matches the asymptotics: for n = 2, double
  large primes give q^(2-2/n) = q, the same exponent as rho (q^(n/2) = q).
- Same exponent, about 20x constant. This agrees in kind with the Rust blind-spot study,
  which measured 50x in GAE at 40 bits [pcc:unverified]. The two use different units and
  instruments, so the numbers are not comparable one to one.
- Wall time agrees: 27.7 s (IC) against 1.38 s (rho) at p = 262139.

## Results, n = 3

Means over 4 curves and 8 targets per size. I = 3M. IC numbers leave out msolve and are lower
bounds.

| p | log2 r | rho steps | rho M-eq | IC trials | IC relations | counted M per trial | gap lower bound, all | gap lower bound, 2lp:p/2 | gap lower bound, 1lp:p/2 | full:p/2 | break-even M-eq per decomposition | msolve per call | IC wall | rho wall |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 31 | 14.9 | 139 | 6.25e3 | 193 | 14 | 4398 | 137 | 154 | 245 | censored 6/8 | 32 | 5.0 ms | 1.23 s | 0.002 s |
| 67 | 18.2 | 488 | 2.20e4 | 279 | 29 | 8974 | 116 | 129 | 207 | censored 7/8 | 79 | 6.0 ms | 2.35 s | 0.011 s |
| 127 | 21.0 | 1384 | 6.23e4 | 436 | 56 | 15106 | 107 | 115 | 152 | censored 1/8 | 143 | 6.8 ms | 4.70 s | 0.046 s |
| 251 | 23.9 | 4176 | 1.88e5 | 835 | 106 | 21179 | 95 | 106 | 159 | censored 4/8 | 225 | 6.6 ms | 9.44 s | 0.123 s |

A censored entry means the run stopped at 4x the trials of its first solving mode before that
mode finished. Means over the uncensored targets alone would be biased low, so none is given.

Findings [pcc:unverified]:

- **The gap lower bound shrinks with size: 137x to 95x.** Its fitted slope is -0.057 per bit of
  r (`fits_n3.txt`). IC trials grow far more slowly than rho steps: slope 0.23 against 0.54 over
  this range.
- **This does not show an approach to parity.**
  - The decomposition counts only root finding (about 64-degree eliminating polynomial,
    log2 p squarings). It leaves out the Groebner step, which costs 5 to 7 ms of wall time per
    call. A whole rho run at p = 251 takes 0.12 s.
  - The rho budget per decomposition (225 M-eq at p = 251) is already below the counted root
    finding alone (21k M).
  - Linear algebra grows like the square of the base size; Zr mults scale with slope 0.50.
    It is uncounted in M and would dominate at large p unless the base is shrunk with
    large primes. Shrinking the base raises the trial count, as the `1lp` and `full` columns
    show.
  - Extrapolating the counted lower bound to parity would need r around 2^135. That rests on a
    four-point fit and leaves out the two costs that grow fastest, so it is not a forecast.
- **The large-prime variants were also worse here.** `2lp:p/2` and `1lp:p/2` cost more than `all`
  at every size, by trial count. Gaudry's q^(2-2/n) = q^(4/3) < q^(3/2) gain with double large
  primes comes from linear-algebra savings at large q. These sizes do not reach that regime,
  and the Groebner step is not counted.

## Limits

- Toy sizes: log2 r is 16 to 36 for n = 2 and 15 to 24 for n = 3. 5 or 4 curves per size.
  Python instrument. Ranges are means; no confidence intervals.
- The unit is an operation count. The constants depend on my formulas: schoolbook polynomial
  products, F_{p^2} Karatsuba, and inversion pricing. A tuned decomposition might cut the
  per-trial M by a small factor. It cannot cut it by the 13x to 27x needed for n = 2, because
  root finding costs at least log2 p squarings mod a quartic, about 20 to 30 log2 p M.
- Rho is a store-all reference in Python with an r-adding walk. Its steps/sqrt(r) of 0.77 to
  0.99 sits near the random-walk value sqrt(pi/4) = 0.886. A distinguished-point rho would add
  a small overhead. Rho does not use the GLS/Frobenius automorphism, which these random curves
  do not have, since a and b are not in F_p.
- n = 3 excludes the Groebner (msolve) cost and linear algebra from M, so its gaps are lower
  bounds. About 2.4% of the roughly 48k msolve calls (7.9% at p = 31) were non-generic (no
  parametrization in the chosen variable) or had a parametrization that did not verify, and 18
  calls exited with an error. Those trials
  were counted as producing no relation, which slightly inflates IC trials.
- No 2-torsion or other symmetry folds were used. The Rust study found them worth about 7.7x
  at 40 bits for n = 2. Even 7.7x would leave IC 2lp at about 3.5x rho at p = 262139
  (27.0 / 7.7). That is an estimate, not a measurement [pcc:unverified].
- No preregistration, A/A arm or independent replay. Not admissible to the leaderboard or the
  validation gate.

## Errors hit

- **Point-dependent IC walk (fixed).** The first IC version reused rho's point-dependent
  r-adding walk. Once it hit a repeated point it cycled, so later relations were duplicates,
  and duplicate edges gave the log by a rho collision. Several modes then finished at the
  same trial. Fixed by RNG-chosen multipliers plus skipping repeated x. The reported runs use
  only the fixed code; the stray test outputs stayed in the scratch directory.
- **msolve `-c 2` (removed).** `msolve -c 2` crashed (SIGSEGV) on these systems, so I removed
  the flag. Without it, msolve exits nonzero on 18 of about 48k calls (0.04%), and those are recorded as
  `msolve_error`.
- **Off-by-sign in BSGS (fixed).** The first BSGS point-counting step had an off-by-sign in the
  giant/baby match. I fixed it before any run. Every curve's order is checked by r*P = O on a
  random point, and r is prime.

## Independent review notes (2026-11-15)

A separate reviewer reran p = 251 s1 and p = 4091 s2 (byte-identical apart from
timing) and recomputed every n = 2 gap from the raw JSON, and all matched.
Verdict: the result holds with these caveats [pcc:unverified]:

- The n = 3 runs cannot be replayed exactly. msolve output is nondeterministic, so the same seed gives different trial counts (p = 31 s1 target 0: saved 62, reruns 101, 102, 66, 66). The saved n = 3 numbers are one draw. [pcc:unverified]
- Whether the n = 2 gap grows depends on pricing. At I = 3M the slope is +0.040 +- 0.011 over 6 non-monotone points. At I = 1.5 log2 p M the gap is flat at about 11-15x. The "never beats rho" conclusion holds with a margin above 10x. The trend claims are statistically weak (no confidence intervals, wide per-curve spread).
- The n = 3 "lower bound" is relative to this implementation (schoolbook root finding; msolve and linear algebra excluded), not a bound on index calculus in general.
- The claim that a tuned decomposition cannot cut cost by the needed 13-27x is an argument, not a measurement.
