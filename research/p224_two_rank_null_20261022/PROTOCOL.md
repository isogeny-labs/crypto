---
title: "Protocol: is a class-group 2-rank of 6 unusual for the CM discriminant of a prime-order curve?"
date: "2026-10-22"
version: 1
---

Frozen and committed before any data of the real run. A dry run at N = 40 per
arm on scratch output preceded it (section "Dry run"). Skill:
`.claude/skills/controlled-experiment/SKILL.md`.

# Question and claim classes

PR #2408 found, for NIST P-224, a class-group 2-rank of 6 for the CM field K
(`D_K` squarefree part of `t^2 - 4p`, 7 distinct primes) and 4 for P-256.
Is that unusual for a prime-order curve of this size?

| sentence | class |
|:--|:--|
| 2-rank of `Cl(D_K)` = `omega(D_K) - 1` (genus theory) | theorem |
| `omega(D_K)` of P-224 is 7 and of each other target is as measured | exact computation (where the factorisation is complete), else bounds |
| "P-224's omega is (not) unusual among prime-order curves" | comparative: this protocol |
| null distribution shape beyond the sampled arms | out of scope |

Genus theory applies to the fundamental discriminant. For a curve of prime
order `N`, `t = p + 1 - N` is odd, so `Delta = t^2 - 4p` is `1 mod 4` and
negative, and `omega(D_K)` = number of primes with **odd exponent** in
`|Delta|` (primes with even exponent lie in the conductor). This is the metric
`omega`. No other quantity (class number, 4-rank, ring-class group of
`Z[pi]`) is tested.

# Targets

P-224, P-256 (tested); brainpoolP224r1, brainpoolP256r1 (tested); P-384,
P-521, brainpoolP384r1 (descriptive); secp224k1, secp256k1 (descriptive and
structural). The two Koblitz curves have `j = 0`, so `Delta = -3 v^2` and
`D_K = -3` with `omega = 1` by CM theory for every such curve. They are not
members of the generic null (the generic null does not contain `j = 0`
curves), so no percentile is claimed for them; the measured `omega = 1` is
reported as a check of the instrument on a square-heavy input.
Parameters come from `CurveParams::*` in `src/ecc/`; `t = p + 1 - h n`.

# Null populations (negative control)

Point counting over a 224-bit field (SEA) is not available in this repository,
so random prime-order curves cannot be generated. **Substitution (disclosed
deviation from "random curves"):** sample the group order `N` uniformly among
odd integers in the Hasse interval `[p + 1 - floor(2 sqrt p), p + 1 + floor(2
sqrt p)]` subject to `N` prime (Miller-Rabin on 20 bases, the repository's
`is_probable_prime`); `t = p + 1 - N`. This is the null for "Frobenius
discriminant of a prime-order curve" with `t` uniform. It is not the
distribution of traces of random curves, which is Sato-Tate / Deuring
(density proportional to `sqrt(1 - t^2/4p)`). The semicircle variant is run as
a pre-registered **sensitivity** arm, not as a family member. Neither
substitutes for a genuine point-counted sample; that is a stated limit.

| arm | field prime `p` | trace | N | role |
|:--|:--|:--|--:|:--|
| R224u | random 224-bit prime per sample | uniform | 1000 | primary null for P-224, brainpoolP224r1 |
| F224u | `2^224 - 2^96 + 1` fixed | uniform | 1000 | primary null for P-224 (fixed-prime arm requested) |
| R256u | random 256-bit prime | uniform | 1000 | primary null for P-256, brainpoolP256r1 |
| R224s, F224s, R256s | as above | semicircle | 1000 each | sensitivity |
| R384u, R521u | random 384-/521-bit | uniform | 100 each | descriptive only (fails the power check; no verdict) |

Primes: random with top and bottom bit set, tested by the same Miller-Rabin.
Per-sample RNG: `SmallRng::seed_from_u64(splitmix(splitmix(seed ^ arm_id) ^
idx))`, seed `20261022`, so results do not depend on thread count. `N = 1000`
is chosen by the power check below.

# Instrument (identical on every arm and target)

`two_rank_null::omega_of(|Delta|, trial_bound = 10^6, rho_budget = 2^21)`,
which calls `p224_hypotheses::factor` (trial division, Miller-Rabin, Pollard-Brent
with at most `2^21` iterations in total per number) and then counts odd-exponent
primes. Unsplit composite cofactors are the censored part. For each number:

* `omega_small`: odd-exponent primes `<= 10^6`. **Exact; never censored.**
* `omega_lo` (rigorous): odd-exponent primes found + 1 per unsplit cofactor that
  is not a perfect square (perfect squares contribute 0, exactly).
* `omega_hi` (rigorous): found + `max(2, floor(log2 c / log2(10^6+1)))` per such
  cofactor (all its primes exceed the trial bound).
* `omega_mid` (imputation, a model not a bound): found + 2 per such cofactor.
* `overlap` flag if a cofactor shares a prime with a found prime or another
  cofactor; counted and reported, not corrected.

Targets and controls use the same call. A target is never factored with a
stronger tool than a control. P-224's discriminant happens to factor completely
under this budget (its second-largest prime is 34 bits); that is a property of
the number, not of extra effort. Published factorisations are used only to
cross-check (unit test `nist_p224_has_omega_seven`).

# Censoring (pre-registered handling)

Samples with an unsplit cofactor are **kept** and reported, with their number
and a Clopper-Pearson interval per arm. They are never dropped or resampled.
For a test of target value `x` (bounds `x_lo <= x_hi`, equal when the target is
fully factored) against controls with bounds `(c_lo, c_hi)`, add-one tails are
computed three ways:

* `p_mid`: `omega_mid` for both.
* `p_max` (most adverse to a verdict of "unusual"): for the upper tail count
  controls with `c_hi >= x_lo`; for the lower tail count controls with
  `c_lo <= x_hi`.
* `p_min` (most favourable): upper tail `c_lo >= x_hi`; lower tail `c_hi <= x_lo`.

Each is `min(1, 2 min(upper, lower))` with `upper = (1 + count) / (N + 1)`.
Verdict rule: **unusual** if `p_max < alpha/m`; **within the control
distribution** if `p_min >= alpha/m`; otherwise **inconclusive: censoring**.
`omega_small` has `p_min = p_mid = p_max`.

# Decision rule, family and power

`alpha = 0.05`. Tested family, two metrics (`omega`, `omega_small`) times five
pairs: (P-224, R224u), (P-224, F224u), (P-256, R256u), (brainpoolP224r1, R224u),
(brainpoolP256r1, R256u), so `m = 10`, threshold `alpha/m = 0.005` (Bonferroni).
Two-sided, as above. Reported beside each verdict: the mid-rank percentile
`(#{c < x} + 0.5 #{c = x}) / N`, upper-tail and lower-tail fractions each with an
exact Clopper-Pearson 95% interval, and the effect size (target value minus
control median).

Power check: smallest attainable two-sided p is `2/(N + 1) = 2/1001 = 0.0020 <
0.005`. Required `N + 1 >= 4m/alpha = 800`; `1001 >= 800`. Passes for all
tests. R384u/R521u (N = 100) give `2/101 = 0.020 > 0.005`: they fail, and are
descriptive only. Sensitivity arms are reported with the same rule but are not
part of `m`.

# Positive control (detector power), run first

(a) Population: 300 fresh R224u samples (index offset `10^6`, disjoint from the
null) with the smallest `k` primes `>= 5` not dividing `|Delta|` multiplied in
(`k = 2` and `k = 4`), through the same `omega_of`. Compared with the R224u
null by tie-corrected Mann-Whitney (two-sided). Detected if `p < 0.001` and the
mean `omega_mid` shift is within 0.5 of `k` (the shift can exceed `k` slightly
if planting changes which cofactors split; that is a finding, not corrected).

(b) Target: P-224's `|Delta|` times the first `k` such primes, `k = 0..12`, ranked
in R224u by the full rule above, and the same on `omega_small`.

Pass: (a) detected for both `k`, and (b) flagged ("unusual", `p_max < 0.005`) at
some `k <= 10` on **both** metrics. If the positive control fails, every
verdict in this study is reported as invalid, not as "within the control
distribution". Expected: planted ω of 15 to 17 lies far above the null's 97.5th
percentile (about 8 to 9).

# What does not count

Changing `N`, `trial_bound`, `rho_budget`, the metric, the null or the thresholds
after seeing real-run data. Any such change is a new labelled protocol version
and both are reported. Adding arms or targets is likewise a new version.

# Pre-flight runtime estimate

Dry run (N = 40/arm, 8 per large arm, 20 per positive-control set, 4 threads
shared with a concurrent compile): 1.8 to 2.7 CPU-s per 224- to 256-bit sample,
2 to 4.5 CPU-s per 384-/521-bit sample, 2 CPU-s per target (fixed cost 18
CPU-s). Estimate for the real run: `18 + (6000 + 600) * 2.0 + 200 * 3.5 =
about 14000 CPU-s`, so about 60 minutes on 4 threads. The actual time is
recorded in the report.

# Dry run (disclosure)

Run on scratch output before freezing (not committed). It showed: 50 to 70 %
of 224/256-bit samples keep an unsplit cofactor under this budget, so
`omega_hi` is loose (median 10 to 13) and most verdicts on `omega` will be
decided by `p_min` ("within") or "inconclusive", and the strongest
statement the study can make about `omega` for a censored target is a bound;
P-224 (7) and brainpoolP224r1 (5) are fully factored; P-256 is `4..13`; P-384,
P-521 and brainpoolP384r1 are heavily censored; secp224k1/secp256k1 give 1.
At N = 40 the P-224 `omega_small` (5) sat near the 97th percentile, which N = 40
cannot resolve. The only change made after the dry run was adding the
`omega_small` column to positive control (b), because the dry run showed
the sweep depended on `p_max` of the censored metric only. No `N`, budget or
threshold was changed.

# Outputs

`run/rows.jsonl` (one row per sample: arm, index, seed, `p`, `t`, bounds, cofactor
bits, rho iterations, milliseconds), `run/config.json`, and, recomputed from the
rows by `analyze`: `run/summary.json`, `run/summary.md`, `run/omega_null.svg`.
