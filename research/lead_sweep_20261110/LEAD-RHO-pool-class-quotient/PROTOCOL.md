# LEAD-RHO-pool-class-quotient

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: open_next_step (last reviewed 2026-10-24); family: generic.
- Title: "Distributed pool v2 walk has no class quotient: add negation (sqrt 2) and Frobenius (sqrt 2m)".
- Next-step excerpt (truncated in leads.json): "Before enabling the negation-class walk (ecc-challenges PR #56, default off): run a few hundred ..."
- Recorded context (atlas_survey.md A5): B3 negation walk 0.700 [0.673,0.728] of v2 steps, 0/1,800 trapped, ~6% per-step overhead, net ~0.744; related rho.negation 1.453 x sqrt(pi/4) vs plain 1.920.
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (grep context only, lines 63-67, 115-118, 253-259); /home/user/crypto/research/ecdlp_subexp_desk_20261101/data/atlas_tally.json (match line only). The full next-step text was not available.

## 2. Hypothesis
In a two-walk, r-adding collision search on a group of prime order n with the +/- class quotient (restart a walk that revisits its own class), the number of steps to the first cross-walk collision is 0.70-0.75 of the unquotiented walk's, i.e. within [0.66, 0.80] of plain at every tested n in 2^16..2^20, with self-cycle (fruitless) events too rare to move the ratio beyond that band.

## 3. Preregistered prediction and falsification criterion
- Statistic: ratio of mean steps neg/plain per size, with 95% bootstrap CI (400 resamples) over T=300 trials.
- Prediction: CI inside [0.66, 0.80] for all three sizes (ideal 1/sqrt2 = 0.707 plus trapping overhead).
- Falsified: for any size the CI lies wholly outside [0.66, 0.80] (the quotient does not give ~sqrt2 on the toy walk; the recorded 0.700 would not reproduce).
- Inconclusive: CI straddles a band edge, or the A/A check fails.
- Controls: A/A arm (plain vs plain, different jump-table and walk seeds): ratio CI must contain 1 (noise floor). Positive control/null reference: plain walk on a group of order ~n/2 (ideal quotient), ratio should be ~0.707 and the neg arm should match it. A separate negative control (non-symmetric walk) is not applicable: the hypothesis concerns a true symmetry.
- Per-step overhead (~6% recorded) is native cost and is not measurable here; step counts only.

## 4. Method
Pure stdlib Python. Group Z_n (n prime: 65521, 262139, 1048573), class of x = min(x, n-x). Jump index = hash(class) mod 32, 32 random jumps per arm. Two interleaved walks from random starts; collision when a step lands on a class last seen by the other walk; a self-repeat counts as a trap and that walk restarts at a random point. Arms: plain, plain (A/A), neg, plain at n/2. Seeds fixed (101/202/303/404 + n; bootstrap seed 7). 300 trials per arm per size. Cost: a few minutes at most, <50 MB.

## 5. Classification: runnable_local
A stdlib toy decides the step-count ratio directly.

## 6. What a run cannot establish
- Not elliptic-curve arithmetic: Z_n is used, so jump and group-law costs, canonicalisation cost and the ~6% overhead are not priced (AGENTS.md section 5a GAE band not applied).
- Not distributed-pool DP behaviour, ecc-challenges PR #56 code, or the Frobenius class (sqrt 2m).
- Not an exponent change (Shoup bound); constant factor only. Results are [pcc:unverified].
