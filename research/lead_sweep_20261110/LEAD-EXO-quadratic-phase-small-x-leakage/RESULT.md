# RESULT: LEAD-EXO-quadratic-phase-small-x-leakage

## Outcome
- Status: ran to completion (single run, no retries). Exit code 0, wall 8 s (run.status; run.py elapsed_s 8.31).
- 8 of 8 curves used, seed 20261110, p in [8000,65000), N from 2459 to 63211. stderr empty/uneventful.

## Statistic vs preregistered prediction
- Prediction (PROTOCOL.md section 3): "NULL persists (n_reject_obs is consistent with Binomial(n, alpha))"; falsified if tail < 0.01 (>= 3 of 8 reject at alpha = 1/16); inconclusive if positive control detects in < 90% of curves.
- Measured: n_reject_obs = 0/8 (observed rank p-values 0.12 to 1.0; smallest 0.118 on two curves); binomial_tail_p = 1.0.
- Positive control: 8/8 detected (every curve p_pos = 1/17 = 0.0588, the minimum attainable).
- A/A: n_reject_AA = 1/8 (p_aa = 0.0588 on the first curve); expected about 0.5 at alpha, so unremarkable.
- Null control: the 16 null draws define the ranks; T_obs sits near null_mean on every curve (e.g. 2.25 vs 2.19, 2.79 vs 2.37).
- Verdict: consistent_with_prediction. Controls passed (positive 8/8 >= 90%, A/A about alpha).

## Reading
- The toy tests the stated reconstruction: a max-over-99-pair quadratic-phase exponential sum on the small-x set versus random negation-closed subsets. It does not re-run the original QUADPHASE instruments (statistic unknown; PROTOCOL section 6).
- run.py verdict agrees with the numbers. No discrepancies found. Note params report n_pairs = 100 (99 grid pairs plus the planted pair in the positive arm), consistent with the protocol text.
- Caveats: only 16 nulls, so rank p-values are coarse (minimum 1/17) and power is low for weak effects. The positive control is a strong planted effect (a0*d^2 in [0,N/4)), so it shows only that large effects are visible. No weak-class (anomalous / small embedding degree) positive control. Sizes are toy (N <= 2^16). Run took 8 s versus the 1-6 min estimate, which is only a speed difference.
- Nothing surprising: the null extends to 8 more random curves.

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction: a further null (0/8 rejects) for quadratic-phase bias on the small-x set at N <= 2^16, matching the earlier QUADPHASE nulls, though with low power and no relevance to cryptographic sizes.

## Standing
[pcc:unverified], single local toy run, no independent replay.
