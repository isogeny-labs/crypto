# LEAD-ICE-relation-graph-rank-large-prime

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: open_next_step (last reviewed 2026-10-30); family ic-engine; next_step_cost medium.
- Title: Relation-stage rank accounting, large-prime / cycle-basis enrichment and rank-aware collection.
- Next-step excerpt (truncated in leads.json): "Extend the honest 2LP summation-multigraph run (EXP-RELN-c5a377 v4 generator, L = q^(1/5)) to la..."
- Why it might beat sqrt(p): "2LP delta > 1/4 with whole-solve gain beyond null; genus-3 analogue was 2.31x costlier".
- Survey line (atlas_survey.md l.46): 2LP cycle-basis enrichment 9/9 cells inconclusive (EV-RELN-3c9a24); genus-3 2LP analogue LA exponent 0.495->0.420 but whole solve 2.31x costlier.
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (grep context, ~80 lines); /home/user/crypto/docs/session-logs/2026-10-26.md (grep context). Not read: EXP-RELN-c5a377 generator, EV-RELN-3c9a24 (not located; work is from leads.json plus the mechanism).

## 2. Hypothesis
In a toy integer-smoothness index-calculus relation stage over F_p* (p ~ 2^18 and 2^20, factor base primes <= B = 1.5 p^(1/4), large primes <= L = B^2.5), with trials counted until the usable (factor-base-only) relation rank reaches |FB|: (a) the 2LP cycle-basis arm needs at most 0.8 times the candidate trials of the full-smooth-only arm (median over paired seeds, upper 95% bootstrap bound <= 0.8), and (b) that gain equals the gain of a configuration-model null (large-prime labels permuted among partial relations), i.e. the real/null trial ratio is within 5% of 1. So 2LP enrichment at toy size is explained by graph degree statistics alone, and gives no gain beyond null.

## 3. Preregistered prediction and falsification criterion
- Statistic: per-seed T_arm = trials to usable rank |FB|; R2 = median(T_2LP/T_full), R1 = median(T_1LP/T_full), Rn = median(T_2LP_real/T_2LP_null); 95% bootstrap CIs over 20 paired seeds per size (400 resamples, seed 7).
- Prediction: R2 upper CI <= 0.8 and |Rn - 1| <= 0.05 (or CI of Rn contains 1) at both sizes.
- Falsified if R2 lower CI >= 0.95 (no 2LP gain), or if R2 gain holds and Rn is clearly different from 1 (structure beyond null).
- Inconclusive otherwise, or if the positive control fails, or fewer than half the seeds complete.
- Positive control: 1LP arm must beat full-smooth-only (R1 upper CI < 1), a known effect.
- Negative control/null: configuration-model relabelling of large-prime slots (preserves degree multiset, breaks specific coincidences).
- A/A arm: two disjoint halves of the full-smooth-only arm; reported ratio of medians (expected near 1; reported, not gating, with 10 seeds each it is noisy).
- Decision: consistent verdict supports "2LP gain is null-explained"; falsified verdict reopens the lead as having real cycle enrichment at toy scale.

## 4. Method
- For each size bits in {18, 20}: p = next prime above 2^bits; sieve smallest-prime-factor table to p. Sample a uniform in [1,p-2], v = 2^a mod p, factor v exactly. Relation classes: full (all prime factors <= B), lp1 (one large prime power <= L), lp2 (two large primes <= L), else discarded.
- Rows are exponent vectors over columns (FB primes, large primes), reduced mod 2^31-1. Incremental sparse Gaussian elimination pivoting on the highest column, large-prime columns first, so a stored pivot with column < |FB| is a usable relation (cycle-basis rank). Arms: maxlp 0, 1, 2 over the same trial stream (paired), plus null arm.
- Seeds: 20261110 + 1000*bits + s, s = 0..19; null permutation seed = seed xor 0x5EED. Trial cap 120000 per seed.
- Cost: sieve ~ seconds, 20 seeds x 2 sizes, pure Python; estimated 1 to 4 minutes, memory < 100 MB. No network, no files.

## 5. Classification: runnable_local
A stdlib toy decides the relation-count question directly; the whole-solve and genus-3 pricing parts of the lead are not run.

## 6. What a run cannot establish
- Not the honest 2LP summation-multigraph (Semaev/EC decomposition) generator; this is classical integer smoothness in F_p*, a mechanism analogue only.
- Counts candidate trials only; linear-algebra cost, cycle-finding cost and the whole-solve ratio (the 2.31x penalty) are not priced, and no matched-rho comparison or 0/0.1/1 GAE band is made.
- L = B^2.5 rather than q^(1/5); sizes 2^18..2^20 are far from asymptotic regimes.
- A null-explained gain says nothing about the whole-solve exponent. All results are [pcc:unverified].
