# PROTOCOL: extension-field index-calculus crossover model, n = 3 (2026-11-19)

Frozen before any measurement of this lane. Predictions are never edited
afterwards; deviations are recorded in README.md. Every number this lane
produces is [pcc:unverified].

Rule applied: one target per instance, charged cold. No multi-target or
batch-of-targets method is used (each run solves one DLP on one curve).

## Question

Index calculus on `E(F_{p^n})`, `n >= 3` (Gaudry decomposition with the
subspace factor base `F = {P : x(P) in F_p}`, optionally with double large
primes), has exponent `q^{2-2/n}` against rho's `q^{n/2}` (`q = p`). At what
group size, if any, does it cross below matched rho, with intervals?

## Tooling found (searched before registering)

- `n = 3`: `src/cryptanalysis/gaudry_cubic.rs` + `examples/gaudry_cubic_bench.rs`.
  Gaudry's O(1) symmetrised-S4 solve (`--groebner`), Wiedemann or dense LA
  (`--sparse-la`), double large primes (`--lp-rule`, small base `|F|^{2/3}`),
  and a rho run on the same group (`run_rho3`). Not an `ecbench` method:
  `ecbench methods` lists no Gaudry-cubic oracle, so `ecbench band` / `verify`
  cannot be applied to n = 3 rows; only the bench's own counters exist.
- `n = 4`: `ic.k4_gaudry`, `gaudry-d4` (research/gaudry_quartic_d4_cost_20261014),
  already measured and crossover-modelled there. Re-measuring at `p = 29`
  costs about 7,300 s per run; not repeated. Desk citation only.
- `n = 5`: Joux-Vitse (n-1) (research/joux_vitse_n1_20261015), desk citation only.
- Committed legacy n = 3 data: `experiments/21_gaudry_cubic*.json` (p = 271..2083).
  Read before registering (stated here for honesty): the Groebner path costs
  about 9e5 F_p mults per residual at p = 271..2083, and the LP arm needed
  roughly 1.7x (p=271) to 4x (p=2083) the residuals of the no-LP arm.

## Cost model (registered)

Unit: group-addition equivalents (GAE), bench convention: affine addition in
`E(F_{p^3})` = 63 F_p mults; one mod-n LA operation = 9/63 GAE. `S = GAE/sqrt(n)`.
`n` = prime subgroup order, about `p^3`.

    IC(p) = R(p) * (C3/63 + g)  +  9*LA(p)/63  +  setup(p)
      R(p)   residuals (trials) until the target logarithm is pinned
      C3     F_p mults of one S4 solve (per residual), model: flat in p
      g      group ops per residual (relation checks, cofactor), model: small
      LA(p)  mod-n operations of the linear algebra
    arm NL (no large primes): R = a_NL * p,              base |F| ~ p/2, LA ~ p^2 (Wiedemann)
    arm LP (double large primes, |F'| = |F|^{2/3}): R = a_LP * p^{e}, e = 4/3, LA ~ p^{4/3}
    rho(p) = kappa * sqrt(n), kappa in {0.627 (negation map), 0.886 (plain, theory)}; the
             bench's own measured rho run is reported beside it (single runs, noisy).

Fits: log R, log C3, log LA against log p by least squares on per-run points,
95% intervals by bootstrap over runs (10,000 resamples). Crossover `p*`:
smallest p with IC(p)/rho(p) <= 1 using the fitted terms; group bits
`log2(p*^3)`. Reported for (i) fitted exponents, (ii) exponents forced to the
theoretical values with fitted constants, (iii) both kappa.
Sensitivity (labelled what-if, not in the interval): C3 divided by 3 and by 10.
Native-work weights `w = 0, 0.1, 1`: `ecbench band` has no n = 3 arm, so the
band cannot be produced by the tool. Substitute: report the IC/rho ratio when
rho is charged 1, 1.1 and 2 GAE per step (an upper bracket on its unpriced
hash/canonicalisation work); the crossover moves by the 6th power of that
factor and is reported per factor.

## Measurements (single thread, RAYON_NUM_THREADS=1)

`target/release/examples/gaudry_cubic_bench --groebner --sparse-la [--lp-rule] --p P --seed S --json ...`
- arm NL: p in {523, 1039, 2083, 4003}, seeds 1, 2.
- arm LP: p in {523, 1039, 2083, 4003}, seeds 1, 2; p = 8101 seed 1 if time remains.
- total wall budget about 40 minutes; a run past its wall cap is recorded as
  unfinished, not retried with other settings.
Raw JSON goes under `raw/` (gitignored); SHA-256 recorded in README.md.

## Predictions (falsifiable)

- P1  C3 is flat in p: slope of log(oracle F_p mults / residual) against log p
  over the measured p lies in [-0.05, +0.15].
- P2  Decomposition rate (decompositions / residuals) lies in [0.10, 0.22] at
  every measured run (theory 1/6).
- P3  NL arm: R / |F| lies in [4, 8] at every p (rank needs |F| independent rows).
- P4  LP arm: fitted exponent e of R against p lies in [1.20, 1.50].
- P5  LP arm needs more residuals than NL at every p, ratio increasing in p,
  ratio at the largest p at least 2.5.
- P6  Measured IC S exceeds rho's theoretical S (0.886) at every measured p
  (IC loses at every runnable size), for both arms; ratio at the largest p
  at least 100 for NL and at least 100 for LP.
- P7  LP arm: S_IC/S_rho decreases with p with log-log slope in [-0.45, -0.05].
- P8  NL arm: the registered model has no crossover at any p: its minimum
  ratio IC/rho over p (LA term ~p^2 against rho p^{1.5}) is at least 10.
- P9  LP arm: the crossover group size under the fitted model exceeds 2^150,
  and the 95% interval's lower end exceeds 2^100.
- P10 The asymptotic gap is small: with the fitted constants, the LP ratio
  falls by less than a factor 4 per 40 bits of group size beyond p = 2^15
  (theory: p^{-1/6}, i.e. 2^{-40/18} ~ 4.7 per 40 bits in group size is the
  limiting rate; registered as "at most 5x per 40 group bits").
