# RESULT: LEAD-ICE-charged-toy-ic-exponent-vs-rho

## Outcome
- Status: ran to completion. Exit code 0, wall 36 s (run.status). One run only, no retries.
- run.stderr was empty. RESULT.json is a single JSON object.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md section 3): R_1(b) > 1 at all five sizes (16, 20, 24, 28, 32 bits). Consistent if both gates pass. Falsified if some R_1(b) <= 1 with gates passing.

Measured, median IC cost / median rho cost over 12 seeds:
- w=1: 5.23 (16), 10.02 (20), 5.79 (24), 15.03 (28), 24.48 (32). All > 1.
- w=0.1 (unranked band): 1.89, 2.98, 1.50, 3.21, 4.91.
- w=0 (unranked band): 1.52, 2.25, 1.02, 1.93, 2.63. The 24-bit point is close to 1 but still above it.

Slopes (log2 cost per bit, bootstrap 95% CI), w=1:
- rho 0.537 [0.483, 0.581]
- IC 0.662 [0.575, 0.748]
- IC slope above rho slope; CIs overlap only marginally in [0.575, 0.581].

Controls:
- Positive control (rho slope within 0.1 of 0.5): passed, 0.537 (diff 0.037).
- A/A (odd vs even seeds): passed. Rho slope diff 0.003 (< 0.15), IC slope diff 0.041 (< 0.3).
- Null/wrong-answer assertion G^s == y on every solve: no assertion failed (the run completed).

Verdict: consistent_with_prediction. Both gates passed.

## Reading
- run.py's verdict field matches the numbers: all w=1 ratios exceed 1 and the gates pass.
- The ratio is not monotone (24-bit dips below 20-bit, 28-bit jumps), so the median-of-12 is noisy per size. The slope CIs are wide.
- The reported linear-fit "crossover_bits" values (-2.6 at w=1, -0.5 at w=0, 4.2 at w=0.1) are not meaningful. They do not order with w, as they should if cost rose with w. They extrapolate separate noisy fits. With IC slope above rho slope at w=1, the ratio grows with size and the fit puts any crossing below the ladder, so there is no crossover to the right. The protocol's phrase "crossover beyond 32 bits" is not what the fit reports. I do not read these as a crossover estimate.
- The toy does test the proxy form of the lead's claim (empty crossover on a short ladder, exponent poorly identifiable). It does not test the EC backend, and Z_p^* index calculus is subexponential, so a higher IC slope here is a ladder-width and constant effect, not an asymptotic statement.
- Surprise: IC slope (0.66) exceeds rho's (0.54) on this ladder, as expected from the fixed-B heuristic being tuned for larger p. The w=0 24-bit ratio of 1.02 shows IC nearly ties rho when trial division is free.

## Bearing on the sub-sqrt(p) question
Closes or confirms the recorded obstruction (in the proxy only): at w=1 IC never beats rho on the ladder and the exponent is not cleanly identifiable from five sizes. It says nothing about EC groups.

## Standing
[pcc:unverified], single local toy run, no independent replay.
