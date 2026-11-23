# LEAD-ICP-generic-table-decomposition

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: closed_obstructed (atlas_tally_20261101; last_reviewed 2026-10-30). Family: ic-prime.
- Title: Enumeration, subtraction and meet-in-the-middle pair-table decomposition for prime-field Semaev IC (arity 2 to 7).
- next_step_excerpt: empty in leads.json. From the survey: reopen only with a non-generic oracle with T_dec << p^(1/4) at m=4, or a sub-linear decomposition test.
- Recorded obstruction: S*T^2 >~ N (Corrigan-Gibbs-Kogan), T >~ sqrt(N) (Shoup); measured exponents mitm 0.60-0.75 vs rho 0.50.
- Files read: research/lead_sweep_20261110/leads.json; research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 15, 137-138); docs/session-logs/2026-11-01.md (lines 7871-7877). README.md and atlas_tally.json were hits but not read.

## 2. Hypothesis
For arity 2 on prime-order E(F_p), p = 3 mod 4, group order N in 2^12..2^18, the cheapest relation phase of a pair-table IC (build table of x(+-Fj+-Fk) for a random base of size B, then walk until B+1 hits), minimised over B in {N^0.25..N^0.5}, costs (group additions, build plus walk, linear algebra excluded) scaling as N^e with e > 0.55 (bootstrap 95% lower bound). Heuristic expectation: B^2 + N/B gives e = 2/3. The recorded obstruction (generic table cannot reach 0.5) is thereby re-derived as a lower bound that already exceeds rho.

## 3. Preregistered prediction and falsification criterion
- Statistic: OLS slope of log2(cost) on log2(N), 7 sizes x 5 reps, 300-resample bootstrap CI.
- consistent_with_prediction: IC CI lower bound > 0.55 and all controls pass. falsified: IC CI upper bound < 0.55 with controls passing (a generic table toy would then be sub-rho-like in exponent, contradicting the closure). Otherwise inconclusive.
- Negative control / null: Pollard rho (16-way walk, birthday on visited set, steps counted); slope CI must contain 0.5.
- Positive control: same IC but table build not charged (free oracle); detector must see slope gap IC minus free > 0.08.
- A/A arm: second IC arm with different derived seeds; |slope difference| < 0.1.
- If any control fails the verdict is inconclusive.

## 4. Method
Python 3 stdlib. Per bit size b in 12..18: random prime p = 3 mod 4 (seed 1000+b), random curve, point count by Legendre sum until N prime; P random, Q = tP. Factor base: B random points uV+vQ-type (random combos). Table: set of x of Fj+Fk and Fj-Fk (affine adds counted, about B^2). Walk: R += F[x(R) mod B], one counted add per step, restart (charged 3 log N adds) every sqrt(N) steps, hit when x(R) in table and not in FB. Stop at B+1 distinct hits; cap 40N steps. B = round(N^e), e in {0.25,0.3,0.35,0.4,0.45,0.5}; cost is min over e. 5 reps per size. Rho cost capped at 50N. Cost estimate: about 1-3 minutes, <100 MB. No linear algebra, so cost is a lower bound for full solve.

## 5. Classification: runnable_local
A stdlib toy decides directly whether the pair-table relation phase already exceeds the rho exponent at toy sizes.

## 6. What a run cannot establish
- Nothing about arity >= 3, 4, 5, Groebner/non-generic oracles, or sizes beyond 2^18; slopes over 7 sizes are noisy and constant-factor effects (log N factors) bias the exponent.
- Linear algebra, discrete log recovery and signed/negation optimisations are not run; native costs are unpriced (no 0/0.1/1 GAE band), so no cross-method ranking under AGENTS.md 5a.
- Does not test the theorem S*T^2 >~ N itself. Results are [pcc:unverified].
