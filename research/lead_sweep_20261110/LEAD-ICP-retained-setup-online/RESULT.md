# RESULT: LEAD-ICP-retained-setup-online

## Outcome
- status: ran; exit code 0; wall 0 s as logged by run.status (the protocol estimated 1-3 min; the run was far faster than estimated, no stderr output). One run only, no retries.

## Statistic vs preregistered prediction
Prediction (PROTOCOL s3): (a) T*S/N in [0.6,1.6] at all four sizes; (b) OLS slope of log T on log S in [-1.15,-0.85]; (c) rho_steps/online_steps divided by sqrt(pi N/2)*S/N in [0.7,1.4] at all sizes; controls pass -> consistent_with_prediction; any miss with controls passing -> falsified.

Measured:
- (a) T*S/N = 0.943, 0.906, 0.805, 0.826 (N=16411, 65537, 262147, 1048583): PASS.
- (b) slope = -1.083: PASS.
- (c) rho_over_online / formula_ratio = 5.74/6.32=0.91, 9.72/7.96=1.22, 4.90/10.03=0.49, 10.19/12.63=0.81. N=2^18 is outside [0.7,1.4]: FAIL.
- Controls: positive (S=N/4) mean T = 3.06, 3.11, 2.96, 2.91, all in [2.5,6]: pass. Negative (permuted labels) wrong rate 1.0, 0.987, 1.0, 1.0 (>0.9): pass; honest wrong_rate 0 everywhere: pass. A/A ratios 1.18, 0.815, 0.7004, 0.949, all in [0.7,1.4] (N=2^18 is at the edge, 0.7004).
- run.py verdict: falsified (controls_ok true).

## Reading
- The online side behaves as predicted: T ~ N/S with T*S/N about 0.8-0.94 and slope -1.08. That is the substance of the recorded BL-at-equal-advice law.
- The (c) failure is driven by the empty-table rho reference, not the online side. Mean first-revisit steps are 137, 355, 253, 855 for N=2^14..2^20, non-monotone (253 at 2^18 is below 355 at 2^16), against sqrt(pi N/2) of about 160, 321, 642, 1283. Only 150 trials per size and heavy early-cycle behaviour of the 32-multiplier r-adding walk make this reference noisy and biased low, particularly at 2^18 (0.39x of formula) and 2^20 (0.67x). So the "falsified" label reflects the toy's rho reference deviating from the idealised formula, not a failure of the retained-table cost law. I did not check the others for sampling noise (no extra runs permitted).
- The strict preregistered rule gives falsified; I report it as such. The A/A ratio at 2^18 sitting at 0.7004 is a marginal pass, which shows the noise floor is comparable to the thresholds.
- The 0-second wall time is much less than the protocol's estimate; I did not investigate (not allowed to edit or rerun). Results are internally consistent, so I do not suspect a truncated run, but this is unverified.
- The toy is a generic Z_N table, not index calculus, so it cannot speak to the n61/n83 rows.

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction: the online cost scaling T ~ N/S that makes "ratio vs empty-table rho" a measure of advice size was reproduced (a, b pass); the (c) miss comes from the noisy rho reference, not from any gain.

## Standing
[pcc:unverified], single local toy run, no independent replay.
