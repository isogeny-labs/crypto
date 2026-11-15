# RESULT: LEAD-EXO-methodology-instruments

## Outcome
Status: ran. Exit code 0. Wall 9 s (run.status: exit=0 wall=9). RESULT.json is a single JSON object, stderr empty. One run, no retries.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): Consistent if FPR in [0.01,0.10], power(q=0.2) >= 0.8 for all families, and min q with power >= 0.8 is <= 0.1 for all families. Falsified if FPR is outside the band or any family has power(q=0.2) < 0.8. Inconclusive otherwise.

Measured (curves p=1511 N=1492; p=2647 N=2676; p=4111 N=2016):
- Null FPR pooled: 0.05 (per curve 0.067, 0.05, 0.033). In band.
- Power by q (0.02, 0.05, 0.1, 0.2):
  - A (window): 0, 0, 0.344, 0.989
  - B (sinusoid, freq 17): 0, 0, 0, 0.172
  - C (comb): 0, 0, 0.156, 0.978
- min q for 80% power: A 0.2, B none, C 0.2. Every family needs q > 0.1; B fails even at q=0.2.
- Verdict (run.py): falsified, because power(B, q=0.2) = 0.172 < 0.8.
- Controls: negative (uniform null) passed, FPR 0.05. Positive control passed for A and C (0.989, 0.978) and failed for B (0.172). A/A (shuffled real labels vs threshold): reject rate 0.0 on all three curves (0 of 180 trials), versus the expected value near 0.05. P(0 of 180 | 0.05) is about 1e-4, so the A/A arm is itself anomalous and is not a clean pass.

## Reading
- The toy tests only a stand-in lead: the idea's real families were unknown, and the planted shapes are mine. It does test whether this max-over-64-frequencies statistic can see these shapes.
- Family B is a density 1+cos(2 pi 17 d/N), which has full amplitude only at the single frequency k=17. The statistic is max over k<=64 on d/sqrt(n). Per-label signal at k=17 is q/2 in expectation, so the shift is about q*sqrt(n)/2, roughly 2 at q=0.2 with n~500. That is comparable to the threshold (about 2.4), so low power is expected. B is a weak signal by construction, not necessarily an instrument bug.
- The A/A discrepancy: real labels are folded, min(d, N-d) in [0, N/2], and the null is uniform on [0, N). The null and the shuffled-real arms are not on the same support. The folded labels give a different (here smaller) statistic distribution than the threshold's null. So the "A/A" arm is not matched to the threshold, and the planted arms inherit folded shuffled labels as their base. This confound could shift power and FPR in unknown directions. I did not rerun or fix it (single-run rule).
- The verdict field agrees with the numbers (falsified follows from B at q=0.2). It does not reflect the A/A anomaly.

## Bearing on the sub-sqrt(p) question
none: this is a calibration check of a toy character-sum instrument on stand-in signals, with no cost mechanism. At most it says this instrument is blind to a weak single-frequency signal at q<=0.2, which weakly supports the recorded "structure instruments are blind" caveat.

## Standing
[pcc:unverified], single local toy run, no independent replay.
