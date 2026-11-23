# RESULT: LEAD-RHO-batched-inversion-shared-pm

## Outcome
- status: ran (single run, no retries). exit=0, wall=21 s (run.status). 120 rows (4 primes x 5 W x 6 seeds). Solve rate 1.0.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): (a) |M/step - (2+3(W-1)/W)|/formula < 2% at every W; (b) S/step ~1, M+S never < 6 M-eq;
(c) I/step within a few % of 1/W; (d) solve rate > 90%; (e) W=1 does not beat Jacobian mixed at I=50M, W=256 does.
Any of (a)-(c) failing => falsified.

Measured (M/step vs formula, rel dev; I/step):
- W=1:   2.000 vs 2.000, 0.0%;   I 1.0000
- W=4:   4.258 vs 4.250, 0.19%;  I 0.2509 (1/W = 0.25)
- W=16:  4.874 vs 4.8125, 1.27%; I 0.0639 (0.0625)
- W=64:  5.163 vs 4.953, 4.23%;  I 0.01673 (0.01563)
- W=256: 5.591 vs 4.988, 12.08%; I 0.004694 (0.003906)
- S/step = 1.000 at all W. (b) holds: min inversion-free-equivalent cost 6.5 M-eq at I=20 (W=64), above 6.
- (d) solve rate 1.0 holds. (e) holds: W=1 loses at I=20/50/100; W=256 wins at all three.
- Steps to solve (mean): W=1 293.5, 4 345.4, 16 385.2, 64 463.1, 256 665.8. Ratio W=256/W=1 = 2.27.

Verdict (run.py): falsified. Criterion (a) fails at W=64 and W=256 (max rel dev 12.1%). (c) is off by 7% at W=64 and 20% at W=256 relative to 1/W.
Controls: positive_control_ok = true, negative_control_ok = true. The A/A expectation (steps ratio near 1) was NOT met (2.27); bands are descriptive only.

## Reading
- The verdict field matches the numbers: (a) fails at large W under the preregistered 2% tolerance.
- Likely reason (my inference, not tested here): at toy sizes (N about 2^14..2^17) a run ends within a few hundred steps, so the final rounds
  are partial (fewer active walks, so per-step overhead 3(W-1)/W and I/W are not amortised the same way) and degenerate x-equal fallbacks matter more.
  The step counts also grow with W because lockstep walks overshoot after the first collision, so total steps per solve rise (2.27x at W=256),
  contrary to the "steps independent of W" claim as measured by total steps. The batching formula holds exactly when W is small (W<=16, within 1.3%).
- The toy does test the batch-inversion bound, but only in a regime where the finite-run edge effects dominate at large W. It does not falsify the asymptotic
  1I/n + 3M cost per full round, only the preregistered per-run average at W=64/256 and the "unchanged walk statistics" claim in its total-steps form.
- Does not test x(P-Q) trick, wall clock, or any cheaper inversion-free step (PROTOCOL s6).

## Bearing on the sub-sqrt(p) question
none: this is constant-factor accounting for rho at toy sizes, and it neither reopens nor changes the closed_known lead.

## Standing
[pcc:unverified], single local toy run, no independent replay.
