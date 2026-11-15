# P3: cross-trace covariate test of the split-prime statistic (protocol, frozen 2026-11-07)

Proposal `IDEA-20261107-SEED-NEGATIVE-CONTROL` (`../../ideas/idea-generator.md`
lines 228–290). Class: **covariate test (seed falsifier)**. No algorithm is priced.

## Question

The lab measured index-calculus yield flat **within** an isogeny class
(`research/isogeny_class_ic_yield_20261102`), where the Frobenius field is fixed.
The seed statistic (split primes ℓ ≤ 691 in Q(√(t² − 4p))) varies only **across**
classes. Across random prime-order curves over one fixed prime, do counted ECDLP
outcomes depend on Frobenius-field covariates? Expected: no (proposer's probability
of a non-null effect 0.02).

## Instances

- Primes: p24 = 16,777,213 (largest prime below 2^24), p28 = 268,435,399 (largest
  prime below 2^28).
- Curves: K random prime-order curves per prime from seed `P3/curves/<p>`
  (j ∉ {0, 1728}, exact order checked); K, the targets per curve T and the base
  size N are pinned in Addendum A from the builder's measured throughput, with
  K ≥ 1,000 per prime, T ≥ 64, and a total budget of about 4 CPU-hours.
- Base for the yield outcome: the first N points with prime abscissa, N = round
  (√(0.015 n)), so the m = 2 coverage is about 3 %.

## Covariates (the frozen family)

z_691 (split count of odd ℓ ≤ 691, ℓ ∤ n, against the conditioned GL2 baseline of
`../../tools/splitcount.rs`), log of the partial Euler-product ratio over odd
ℓ ≤ 200,000 (same code), the number of F_p-roots of ψ3, the number of F_p-roots of
ψ5. Nuisance: log n.

## Outcomes

- Y1: mean rho operations to a checked solution divided by √n, over T targets per
  curve (Pollard rho, negation map, the instrument pinned in Addendum A). Two
  independent runs per curve (A and B, disjoint seeds); A is the outcome, B is the
  A/A control.
- Y2: exact m = 2 coverage |{±B_i ± B_j}| / n of the prime-abscissa base, divided by
  2N²/n. Exact, no sampling noise.

## Analysis (frozen)

Per prime, per outcome, per covariate: ordinary least squares of the outcome on the
standardized covariate with log n as nuisance; effect = slope per covariate SD as a
fraction of the outcome mean, with 95 % CI and p-value. Family: 4 covariates × 2
outcomes × 2 primes = 16 tests; Bonferroni α = 0.05/16 = 0.003125.

## Controls

- Positive (analysis): a copy of each outcome with a planted +2 % effect per SD of
  z_691; the analysis must detect it in at least 90 % of 200 planted replicates
  (re-noised by resampling curves), else that outcome is "not informative".
- Negative: 1,000 permutations of the covariates across curves; the observed
  |effect| is placed in the permutation distribution.
- A/A: correlation and slope of run B on run A for Y1.

## Decision rules (frozen)

1. **Seed becomes a coordinate lead:** some covariate has |effect| ≥ 1 % and is
   Bonferroni-significant with the same sign at **both** primes, for the same outcome.
2. **Null at tested sizes:** no covariate meets rule 1 and the positive control is
   informative for that outcome; report the largest |effect| and its CI.
3. **Not informative:** the positive control fails for an outcome.
Descriptive, decides nothing: the percentile of each published curve's z_691
(from `../../data/recount.json`) within the census z_691 distribution at each prime.

## Sources checked

KB unavailable. `../../README.md` §4 and §6, `../../ideas/idea-generator.md` P3,
`research/isogeny_class_ic_yield_20261102/p2_prime_analogues/RESULTS.md`,
`research/nist_skip_criterion_20261025/strict_elkies_20261029/REPORT.md`.

## Addendum A: instrument and pre-data amendments (pinned before any claim cell)

**Instrument.** `p3census/` (standalone Rust, no dependencies; sha256 of the
concatenated `src/*.rs` in name order
`2e24f15166813fb99ac0155f8d432a49ccbb8e8c34638ed696717da6202cbbf7`). `selftest`
10/10, including equivalence with `../../tools/splitcount.rs` (z and split counts
equal at all 12 cutoffs over p = 4,194,319). Smoke only at p = 262,111 (< 2^18).
CLI and the exact rho walk: `p3census/README.md`.

Commands, per prime P with the seeds below:

```
p3census curves     --p P --count 2000 --seed SC --out curves.jsonl
p3census covariates --curves curves.jsonl --out cov.jsonl
p3census rho        --curves curves.jsonl --targets 256 --seed SA --run A --out rhoA.jsonl
p3census rho        --curves curves.jsonl --targets 256 --seed SB --run B --out rhoB.jsonl
p3census yield      --curves curves.jsonl --N NP --out yield.jsonl
p3census analyze    --curves curves.jsonl --cov cov.jsonl --rho-a rhoA.jsonl --rho-b rhoB.jsonl \
                    --yield yield.jsonl --seed SZ --perms 1000 --alpha 0.05 --planted 0.02 \
                    --covariates COVS --family-size 16 --out analysis.json
```

| P | SC | SA | SB | SZ | NP | COVS |
|---|---|---|---|---|---|---|
| 16,777,213 | 11787384277891755733 | 11424068243091161927 | 16739699506839864087 | 9456636048090206450 | 502 | z_691, logL1_200000, psi3_roots, psi5_roots |
| 268,435,399 | 9196315568214002056 | 6471924370547007479 | 13467469337155996127 | 14287345612908892213 | 2007 | z_691, logL1_200000, psi3_roots |

Seeds are the shared rule applied to `P3/curves/<p>/0`, `P3/rho/A/<p>/0`,
`P3/rho/B/<p>/0`, `P3/analyze/<p>/0`. NP = round(√(0.015 · 2^k)) for k = 24, 28.

**Pre-data amendments (from the build, before any claim cell):**

1. **ψ5 is dropped at p28.** For a prime-order curve, #E + #E' = 2(p + 1) forces
   the ψ3 (resp. ψ5) root count to be identically 0 when p ≡ −1 (mod 3)
   (resp. p ≡ −1 mod 5); the builder confirmed all-zero counts at a test prime.
   p24 ≡ 1 (mod 3) and 3 (mod 5): both kept. p28 ≡ 1 (mod 3) and 4 (mod 5): ψ5 is
   degenerate there and is not tested. The Bonferroni family stays 16 (α = 0.003125,
   now conservative). ψ3 and ψ5 re-encode Frobenius mod 3 and 5 and so overlap the
   ℓ = 3, 5 terms of z_691; they stay as separate regressors as frozen.
2. **z_691 and E_691 are the same regressor** at fixed p (z is affine in the count).
3. **K = 2,000 curves and T = 256 targets per run (A and B) at both primes.** The
   proposal's 1,024 targets would cost about 4.6 CPU-hours at p28 for rho alone;
   T = 256 keeps the run near 2.2 CPU-hours (builder's estimate, extrapolated from
   p ≤ 2^18). Expected 80 %-power minimum detectable effect about 0.17 % of the mean
   for rho and about 0.03 % for yield, well under the 1 % decision line.
4. **The positive control is analytic power plus one planted copy.** `analyze`
   reports the power to detect a +2 % effect per SD and whether a planted +2 % copy
   is detected; an outcome is "not informative" if that power is below 0.9 or the
   planted copy is missed. This replaces the 200-replicate resampling, which the
   instrument does not implement.
5. **The rho walk is this study's own**, not the lab's `ic_wide::rho_negation`,
   because the main crate's dependencies cannot be resolved offline here. Same
   family: r = 32 additive walk, negation map, fruitless-cycle escape by doubling
   the cycle minimum (window 16), distinguished points; additions and doublings count
   one operation each (no GAE weighting). Smoke: 3,200 targets, 0 unsolved,
   0 mismatches; walk cost 0.97–1.10 √n.
6. **The correlation prediction |ρ| < 0.03 is not used:** with 2,000 curves its
   standard error is about 0.022. The decision is the 1 % effect line, as frozen.
