# RESULT: LEAD-RHO-bsgs-family-memory

## Outcome
- status: ran; exit code 0; wall 1 s (run.status: exit=0 wall=1). Single run, no retries.
- run.py verdict field: "inconclusive". run.stderr empty.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): BSGS at m = n^(1/3) has slope in 2/3 +- 0.07 with CI lower bound > 0.58, and exceeds the rho slope by more than 0.08. Falsified if the CI upper bound < 0.58. Inconclusive if controls fail or neither criterion is met.

Measured (log2 ops vs log2 n, n = 2^12..2^22, 40 targets/size):
- m = n^(1/3): slope 0.651, CI [0.617, 0.679]. In 2/3 +- 0.07 band; CI lower > 0.58. Not falsified.
- m = sqrt n (positive control): 0.500, CI [0.491, 0.509]. Pass (0.5 +- 0.06).
- m = sqrt n A/A rerun: 0.496, CI [0.489, 0.505]; difference 0.004 < 0.03. Pass.
- m = n^(1/4) (dose-response, not decisive): 0.727, CI [0.700, 0.757]; expected 0.75. Monotone with memory.
- rho with DPs (null / low-memory reference): slope 0.937, CI [0.884, 0.999]; mean stored DPs about 34-55.
Criterion "BSGS(1/3) slope exceeds rho slope by > 0.08": FAILS (0.651 - 0.937 = -0.29).
Verdict: inconclusive. Positive and A/A controls passed. The null control did not behave as the intended sqrt-time reference (see Reading), so the protocol's comparison clause is not met.

## Reading
- The BSGS arms test the lead's core claim directly and cleanly: ops slope follows 1-b with small finite-size bias (0.651 vs 0.667, 0.727 vs 0.75, 0.500 vs 0.5). The m = n^(1/3) numbers agree with the recorded generic memory-time tradeoff.
- The verdict is "inconclusive" only because of the rho comparison clause. The rho arm has slope 0.94, far above the expected 0.5. Mean ops go 76, 617, 2895, 11058, 27444, 52737. At small n the walk is dominated by DP-startup and restart overhead, and the DP mask is near sqrt(n)/32, so DP overhead is large relative to sqrt n. This is the finite-size bias PROTOCOL s6 warned about, and it is large. The "ops > rho slope + 0.08" criterion is therefore uninformative: the rho arm is not a valid sqrt-time reference at these sizes. Its slope should not be read as rho scaling worse than 1/2.
- Rho absolute ops exceed BSGS(sqrt) ops by 16x at n = 2^22 (52737 vs 3200), a constant-factor gap rather than a scaling statement.
- The toy uses an abstract additive Z_n with trivial logs, so it says nothing about elliptic-curve constants or the claw/negation/Frobenius tables.
- Nothing surprising in the BSGS arms. The rho null failure is the one discrepancy worth noting. The verdict field is consistent with the numbers.

## Bearing on the sub-sqrt(p) question
closes_or_confirms_obstruction: sub-sqrt-memory BSGS (m = n^(1/3)) shows slope about 0.65, well above 0.5, which is consistent with the recorded generic memory-time obstruction. This is a toy generic-group result only.

## Standing
[pcc:unverified], single local toy run, no independent replay.
