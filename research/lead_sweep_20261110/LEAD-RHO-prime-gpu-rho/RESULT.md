# RESULT: LEAD-RHO-prime-gpu-rho

## Outcome
- status: ran (single run, no retries). exit code 0, wall 52 s (run.status; run.py reports 52.65 s).
- run.stderr was empty or uneventful. RESULT.json is a single JSON object.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): all four c values in [0.75, 2.25] => consistent; any outside with valid controls => falsified; invalid controls => inconclusive.

Measured (c_vs_W1, c_dose_slope):
- n=65423 (2^16), 1000 trials, k=2/4: 0.579, 0.782
- n=1048601 (2^20), 600 trials, k=3/5: 0.891, 1.050
Mean steps (SE): 2^16: W1_A 278.8(4.6), W1_A' 279.9(4.5), WW_kA 314.1(4.9), WW_kB 454.9(6.2). 2^20: W1_A 1219.9(25.5), W1_A' 1205.8(24.8), WW_kA 1319.8(28.2), WW_kB 1697.6(30.5).
Unsolved: 0 at both sizes.

Controls:
- Negative (DPs off): 0 of 20 solved, passed.
- A/A: diff 1.08 (SE 6.41) and 14.11 (SE 35.56), within 3 SE, passed.
- Key verification (kP == Q): all verified.
- Dose-response monotone: yes.
- Power check: FAILED (powered=false). My arithmetic: at 2^20, 0.5*(W-1)/theta_A = 60 steps vs 3*SE(A/A) = 107, so the effect under test is below the detectable level. At 2^16, 30 vs 19, which would pass.

Verdict: inconclusive (run.py "verdict" field), because the positive-control power check failed at 2^20. Controls are therefore not all valid.
One estimate (c_vs_W1 at 2^16 = 0.579) lies below 0.75, but the protocol only lets that falsify the prediction when controls are valid. It does not here. It is also a ratio of small differences, with SE roughly 0.1 by my rough propagation (not computed by run.py).

## Reading
- The toy measures counted DP-lag in a W=16 round-robin rho, which is the op-count part of the lead only. It says nothing about GPU or CPU admission or wall clock.
- run.py's verdict agrees with the numbers: inconclusive from the power gate. No discrepancy found.
- Observations: the c values rise with size (0.58 to 0.89 for c_vs_W1) and c_dose_slope exceeds c_vs_W1 at both sizes. The measured lag is at or below the low edge of the theoretical 1.5 band. Possible causes (unverified): DP-lag per collision is smaller than 1/theta rounds, or trail-cap restarts. Not investigated.
- Toy W=16 and n <= 2^20 only; no extrapolation to GPU-scale W.

## Bearing on the sub-sqrt(p) question
none: this is an op-count accounting toy for parallel rho, and the underpowered result neither confirms nor refutes the lag model; it gives no route below sqrt(p).

## Standing
[pcc:unverified], single local toy run, no independent replay.
