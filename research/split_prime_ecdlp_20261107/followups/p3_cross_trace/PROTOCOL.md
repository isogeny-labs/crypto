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

## Addendum A: instrument (pinned before any claim cell)

Pending the builder's report.
