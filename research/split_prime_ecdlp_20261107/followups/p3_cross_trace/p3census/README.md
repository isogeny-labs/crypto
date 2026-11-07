# p3census: native instrument for P3 (`IDEA-20261107-SEED-NEGATIVE-CONTROL`)

Question (idea-generator.md, P3): across random prime-order curves over one
fixed prime p, do counted ECDLP outcomes depend on Frobenius-field covariates
(the split-prime statistic of `../../tools/splitcount.rs`, partial L(1,chi),
3- and 5-division root counts) after conditioning on n? Expected: null.

This directory is the instrument only. **No claim cell has been run.** Every
number below is from the smoke test (p = 262111 < 2^18, 50 curves, 32 targets per
run, one core, `nice -n 10`), which has no evidential standing for the P3 claim
[pcc:unverified]. Std-only Rust, no dependencies, standalone workspace
(precedent: `research/isogeny_class_ic_yield_20261102/p2_prime_analogues/p2tool`).

```
CARGO_TARGET_DIR=<scratch>/targets/p3 cargo build --release     # then run the binary directly
p3census selftest
```

## CLI

All subcommands write JSON lines (`kind` = curve | summary | cov | rho | yield | analysis);
`--out FILE` or stdout. p < 2^32 (arithmetic is u64 with u128 only for scalars mod n).

| command | arguments | output |
|---|---|---|
| `curves` | `--p P --count C --seed S` | `curve` lines: `idx, draw, p, a, b, n, t, gx, gy, j`; last line `summary` with `draws` and a `rejected` map (`singular`, `j_0`, `j_1728`, `composite_order`, `anomalous_n_eq_p`, `duplicate_j`) |
| `covariates` | `--curves F` | per curve: `E_c, z_c, logL1_c` for the 12 cutoffs of splitcount.rs (100 ... 200000), `exp_691, sd_691, primes_691`, `psi3_roots, psi5_roots`, `n, t` |
| `rho` | `--curves F --targets T --seed S --run A\|B [--targets-out CSV]` | per curve: `T, verified, mismatch, unsolved, mean_ops, sd_ops, mean_over_sqrt_n, se_over_sqrt_n, mean_walk_ops, mean_setup_ops, escapes_per_target, abandoned_total`; CSV (optional) one row per target: `idx,target,k,ops_total,ops_setup,ops_walk,escapes,verified` |
| `yield` | `--curves F --N N` | per curve: `distinct_x, size, coverage, lambda = 2N^2/n, cov_norm = coverage/lambda, secs` |
| `analyze` | `--curves F --cov F --rho-a F [--rho-b F] --yield F [--seed S --perms 1000 --alpha 0.05 --planted 0.02 --covariates a,b,... --family-size K]` | one `analysis` JSON line (stdout/`--out`) and a table on stderr |
| `selftest` | none | PASS/FAIL list, exit 1 on failure |

**Curve population.** (a, b) uniform over F_p^2 from `derive(seed, p)`; rejected:
singular, a = 0 (j = 0), b = 0 (j = 1728), #E composite, #E = p, repeated j (an
isomorphic curve already kept). Exact order by baby-step giant-step over the whole Hasse
interval on a random point: if the candidate list holds a prime c then ord(P) = c
forces #E = c (the interval is shorter than c); none prime means #E composite.
A kept curve is re-verified (`n * P = O` on 5 fresh points; G is the last of them).
Every draw is counted; the smoke prime accepted 50 curves from 1088 draws (1034 composite,
3 anomalous, 1 duplicate j).

## Covariates

Per odd prime l <= 200000 the model is copied from `splitcount.rs` `classify`: Frobenius mod l uniform
in GL2(F_l) with determinant p, conditioned on l not dividing n, giving P(split | p mod l).
Observed: Legendre symbol of Delta = t^2 - 4p mod l (t = p + 1 - n) equals +1.
`z_c = (E_c - sum P) / sqrt(sum P(1-P))` over odd l <= c; `logL1_c = log(prod (1 - chi(l)/l)^-1 / exp E[log prod])`
(the file's "L1_ratio", logged). `E_691` is the strictly split count of the task; for fixed p and no l | n,
`z_691` is an affine function of `E_691`, so "standardised E_691" and `z_691` are the same regressor.
Differences from splitcount.rs, forced by small p: p, n are integers not hex, and an l dividing p is skipped
(only possible for p <= 200000). Equivalence is a selftest: the reference `classify` is pasted in the test and the
`split` counts, `z` and prime counts agree at all 12 cutoffs on 3 random curves over p = 4194319
(max |dz| = 0, 124 odd primes at 691).

`psi3_roots`, `psi5_roots`: number of F_p-roots of psi_3 = 3x^4 + 6a x^2 + 12 b x - a^2 and psi_5 (degree 12,
psi_5 = 32 f^2 g - psi_3^3 with f = x^3 + ax + b, g = psi_4/(4y)), as deg gcd(psi, x^p - x) with native
polynomial arithmetic mod p. Selftest: degree 12 and leading coefficient 5, squarefree, gcd count equals root
count by evaluation at every x, and for every x with f(x) a square, psi_l(x) = 0 iff [l](x, y) = O.

**Degenerate covariates (finding).** A root of psi_l in F_p is the abscissa of an F_p-rational l-torsion point
of E or of its twist. #E + #E' = 2(p + 1), so if p = -1 (mod l) and l does not divide the prime n, l does not
divide #E' either: both counts are identically 0 on prime-order curves. `smoke/psi_degeneracy.txt`: at
p = 262139 (= 2 mod 3, = 4 mod 5) all 50 curves have 0 roots of both polynomials; at p = 262111 they are
`{0: 21, 1: 26, 4: 3}` and `{0: 39, 2: 11}`. `analyze` reports zero-variance covariates under
`degenerate_covariates` and drops them from the family. **The protocol primes must have p = 1 (mod 3) and
p mod 5 in {1, 2, 3}** (or the two covariates are dead). They are also mechanical functions of Frobenius mod 3
and mod 5, so they overlap the l = 3, 5 terms of E_691 and are not independent tests.

## `rho`: the exact walk

**Lab implementation not linked.** The lab's walk is `ic_wide::rho_negation<G: CountedGroup, S>`
(`src/cryptanalysis/ic_wide.rs`) with `PrimeCurve { p, a, b }` (`ic_boundary.rs:4463`, `impl CountedGroup` at 4749),
which takes arbitrary (p, a, b, generator, r, seed) with no spec file. But `crypto_lib` does not
resolve its dependencies in this sandbox (`cargo metadata --offline`: `no matching package named aws-credential-types`),
and it is a ~900k-line crate that would have to be built as a path dependency. This walk is a re-implementation of the same
family, differing as listed at the end of this section. The ecbench harness (`methods.rs`, `rho.negation`) was not run.

Walk on <G> of prime order n, target Q = [k]G with k uniform in [1, n-1], seeds `derive(seed, run, curve idx, target)`
(run A and run B use different tags, so disjoint streams):

1. Canonical representative of {P, -P}: the one with y <= (p-1)/2; key = x. Every point carries (a, b) with P = aG + bQ mod n; flipping negates both.
2. r = 32 jumps M_j = u_j G + v_j Q, (u_j, v_j) 32 distinct pairs from [0,16)^2 minus (0,0), built from the tables iG, iQ (i < 16); index = (mix64(x) >> 40) mod 32 (depends on x only, so P and -P use the same jump).
3. Step: P <- canon(P + M_j); a += u_j, b += v_j.
4. Fruitless cycles (Bernstein-Lange-Schwabe 2011): window of the last 16 canonical points of the walk. If the new x is in the window, the cycle is the window tail from that entry; double the member with the smallest x (same point for every walker entering the cycle), canonicalise, clear the window, count one escape. If the next escape of the same walk picks the same minimum, the walk is abandoned (counted).
5. Distinguished point: `mix64(x) & (2^d - 1) == 0`, d = floor(log2 n / 4) (min 1); length cap 20*2^d + 64 (counted as capped).
6. Walk starts: S_0 = aG + bQ and D = dG + eQ with a, b, d, e uniform in [0, n); walk i starts at S_0 + i D (one addition).
7. Collision at a stored DP with b != b': k = (a - a')/(b' - b) mod n, checked by [k]G = Q (counted); `verified` is that check, `mismatch` counts k different from the planted one (always 0 so far).
8. Counting: each non-trivial addition or doubling is 1 operation (no field-multiplication weighting; the lab's GAE weights were not matched). `ops_total` = setup (tables, jumps, S_0, D) + starts + walk (steps, look-aheads none, escape doublings) + verification. Per-target cap `60 sqrt(n) + 20000` ops; hitting it is `unsolved`.

Differences from the lab's `rho_negation`: 32 jumps (lab: 4/8/16 by bit length), jump coefficients < 16 from tables instead of 12-bit joint multiplications,
no look-ahead on flipped jumps, DP bits from floor(log2 n) rather than bit_len, GAE not used.

**A design bug that the smoke test caught (fixed).** The first draft took the start stride D from small coefficients
(< 16). Walks from S_i and S_j then reached the same point with the identical (a, b) (useless collisions, ~400 in 4 runs) and
the walk cost grew with n (1.09, 1.36, 1.47, 1.61 sqrt(n) over p = 2^14 ... 2^18). With full-size D it is flat, 0.97 - 1.10 sqrt(n).

## `yield`: exact m = 2 coverage

B = the first N points with abscissa x = 2, 3, 5, 7, ... on E (one y each). S = {+-B_i +- B_j : i <= j}, O included
(B_i - B_i). S is closed under negation and n is odd, so |S| = 2 * #(distinct x of the non-identity elements) + 1; the
x-coordinates of B_i +- B_j come from one batched inversion per row. `coverage = |S|/n` is the exact fraction of targets
Q = [k]G with a single-attempt m = 2 decomposition, with no sampling noise. Memory ~ 4N^2 bytes. Selftest: size equals the size of the set of
group elements built with the generic `add` (p = 1009, N = 20, 3 curves).

## `analyze`: model and controls

Outcomes per curve: `rho` = mean of (A mean, B mean) of ops/sqrt(n) (A alone if no run B), `cov_norm`.
For each outcome y and covariate z (standardised, sample SD): OLS `y = b0 + b1 z + b2 w`, w = standardised ln n (n-conditioning).
Report `slope_frac = b1 / mean(y)` (per covariate SD, fraction of the outcome mean), 95% t interval (CI of mean(y) ignored), two-sided p,
Bonferroni over `family` = outcomes x live covariates (`--family-size` overrides, e.g. 24 across primes), `reject_null` if p < alpha/family,
and `ci_within_pm1pct` (the P3 falsification threshold is a replicated effect >= 1% of the outcome).
Default covariates: `z_691, z_5000, z_50000, logL1_200000, psi3_roots, psi5_roots`.

* (a) planted positive control: y' = y + 0.02 mean(y) * z_E691 into a copy; reports the recovered slope (it adds to the unplanted one), detection at Bonferroni, the analytic power at 2% and the minimum detectable effect at 80% power.
* (b) permutation negative control: covariate shuffled across curves (n and y fixed), 1000 permutations per (outcome, covariate): null mean, sd, 2.5/97.5% quantiles, permutation p of the observed slope, false-positive rate at 0.05 and at the Bonferroni level.
* (c) A/A: two rho runs per curve with disjoint seed streams: Pearson r of the curve means (Fisher CI), slope of B on A (with ln n), paired mean difference, and `var_ratio` = between-curve variance of the means / mean sampling variance (1 = curves differ by noise only); plus the covariate slopes on A and on B separately (`replication_rho_A_vs_B`).

## Smoke results (exact commands: `../smoke/run_smoke.sh`; logs in `../smoke/`)

`BIN=<scratch>/targets/p3/release/p3census ../smoke/run_smoke.sh`, p = 262111 (prime, 1 mod 3, 1 mod 5; above 200000 so no l | p), seed 1, 50 curves, rho T = 32 per run, seed 7, yield N = 150 (lambda = 0.17).

* `selftest`: 10/10 PASS (sqrt_mod, is_prime, BSGS vs brute-force Legendre count on 160 curves at p = 1009 ... 65537, psi3/psi5, equivalence to splitcount.rs, rho recovery, coverage vs brute force, t quantile 2.228139, normal, OLS).
* Curves: 50 from 1088 draws. Rho: 3200 targets, 0 unsolved, 0 mismatches, mean S = ops/sqrt(n) = 1.50 (walk 1.09 sqrt(n) + setup 154 ops + starts + verification); per-target SD/mean 0.43; 8.5 escapes per target at 2^18.
* Yield: mean cov_norm 0.9201, between-curve relative SD 0.29%.
* Regression table (`analysis.txt`): all 12 (outcome, covariate) tests are null at Bonferroni 4.2e-3; the smallest uncorrected p is 0.037 (rho on `logL1_200000`, slope -1.4%, CI [-2.7%, -0.1%]). Yield slopes are within +-0.06% of the mean, CIs about +-0.09%.
* Planted +2% control: recovered +2.63% for rho (unplanted +0.6%), p = 1.5e-4, detected; analytic power at m = 50 and T = 32 is only 0.60 and MDE(80%) = 2.4%, as expected for this tiny sample; for yield MDE(80%) = 0.14%, power 1.
* Permutation control: null SD of the rho slope 0.7% and of the yield slope 0.04%; false-positive rates at 0.05: 0.042 - 0.055 for rho, 0.038 - 0.056 for yield, at Bonferroni 0.001 - 0.009.
* A/A: r(A,B) = -0.18 [-0.43, +0.11], slope of B on A -1.2% (p = 0.26), mean B - A = +0.09% (t = 0.04), var ratios 0.98 and 0.81. Consistent with noise only at this sample size (one realisation, not a calibration).

## Throughput (one core, smoke, p <= 2^18 only; not measured at the protocol primes)

| p | rho ops/s | S = ops/sqrt(n) | walk/sqrt(n) | setup/sqrt(n) |
|---|---:|---:|---:|---:|
| 16411 | 7.0e6 | 2.23 | 0.97 | 1.02 |
| 65521 | 7.6e6 | 1.75 | 1.04 | 0.56 |
| 131071 | 6.5e6 | 1.63 | 1.10 | 0.41 |
| 262139 | 6.6e6 | 1.49 | 1.09 | 0.30 |

(run `smoke/throughput.txt`; 20 curves x 32 targets each; the 50-curve run above gave 5.7e6 ops/s). Coverage: about 105 - 115 ns per pair (i,j), i.e. 0.0013 s per curve at N = 150, 0.47 s at N = 3000, flat in N.

**Extrapolation to the protocol primes (not measured, extended-Euclid inversions cost ~ log p):** ~5.5e6 ops/s at 2^24 and
~4.5e6 ops/s at 2^28; ops per target = 1.09 sqrt(n) + ~200: about 4.7e3 (2^24) and 1.8e4 (2^28).

Sizing for at most about 4 CPU-hours in total (2 primes, 2000 curves each, rho runs A and B of T = 256 targets, yield at lambda = 0.2, N = ceil(sqrt(0.1 n))):

| prime | rho (2 x 256 targets x 2000 curves) | yield | total |
|---|---|---|---|
| ~2^24 (N = 1296) | 5.1e5 targets x 4.7e3 ops = 2.4e9 ops = 0.15 h | 2000 x 0.09 s = 0.05 h | ~0.2 h |
| ~2^28 (N = 5182, ~110 MB) | 1.0e6 targets x 1.8e4 ops = 1.8e10 ops = 1.1 h | 2000 x ~1.5 s = 0.85 h | ~2.0 h |

About 2.2 CPU-hours in all, plus minutes for `curves` (about 50 draws per accepted curve at 2^28) and `covariates` (about 2 ms per curve). Expected resolution (the 1% falsification line):
MDE(80%, family 24) ~ 3.9 x 0.45 / sqrt(512) / sqrt(2000) = 0.17% of the mean for rho and ~0.03% for yield. The proposal's 1024 targets per curve with A/A at 2^28 would be 2 x 1024 x 2000 x 1.8e4 / 4.5e6 = 4.6 CPU-hours for rho alone; 2^32 (~7e4 ops per target, N = 20k) is out of reach at 2000 curves.
