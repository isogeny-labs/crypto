# RESULT: LEAD-STR-automorphism-quotient-rho

## Outcome
- status: ran (single run, no retries). exit code 0, wall 3 s (run.status: exit=0 wall=3).
- run.py verdict: "inconclusive"; controls_ok = false. run.stderr empty.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): R_k = steps(k)/steps(1) about 1/sqrt(k) (up to 1.4x from fruitless cycles) at N ~ 2^16, 2^18, 2^20.
Falsified if R_k + 1.96 SE < 0.9/sqrt(k) with controls OK. Consistent if R_k - 1.96 SE >= 0.9/sqrt(k) and R_k <= 1.4/sqrt(k), controls OK. Control failure makes the verdict inconclusive.

Measured R_k (SE), 400 trials, [theory 1/sqrt(k)]:
| bits | k=2 [.707] | k=4 [.500] | k=6 [.408] |
| 16 | 0.724 (.026) | 0.536 (.019) | 0.448 (.016) |
| 18 | 0.598 (.024) | 0.458 (.017) | 0.372 (.014) |
| 20 | 0.571 (.024) | 0.451 (.017) | 0.368 (.013) |
Mean steps k=1 real: 325, 701, 1455 (16/18/20 bits); k=1 ideal: 330, 631, 1293.

Controls:
- Null (k=1 vs itself): R=1 by construction, trivially OK.
- A/A (second k=1 real arm): means 325.4/334.1, 701.3/695.9, 1454.6/1427.5; all within 3 combined SE. PASS.
- Positive control (ideal arm within 12% of 1/sqrt(k), compared against k1_real per run.py line 79): FAIL at one cell by my recomputation. At 20 bits, k=2, ideal/k1_real = 885.9/1454.6 = 0.609 vs 0.707, 13.9% off (>12%). Other cells pass by my arithmetic (nearest: 20-bit k4 11.6%, k6 11.0%). Measured against k1_ideal instead, all ideal arms are within about 6% of 1/sqrt(k).

Verdict: inconclusive (controls_ok false). Not falsified, not confirmed.

## Reading
- The positive control fails because the k=1 real (r=32 adding walk) baseline is slower than the k=1 ideal random mapping by 11% at 18 bits and 12% at 20 bits (1455 vs 1293), while the 16-bit gap is absent. The detector compares ideal k-arms to the real k=1 arm, so a baseline slowdown of the real walk at k=1 trips it. This is a property of the control design (mixed baseline), not evidence about the quotient.
- Consequence for the ratios: R_k drifts below 1/sqrt(k) at 18 and 20 bits (k=2 at 20 bits: 0.571 vs 0.707). R_k + 1.96 SE at 20 bits, k=2 is 0.618, below 0.9/sqrt(2) = 0.636. So that one cell meets the numeric falsification condition, but controls failed, so it does not count. Other cells (18-bit k2: 0.645; k4, k6 at 18/20 bits) are above their thresholds. The real k=2 arm is actually close to its own ideal arm (831 vs 886 at 20 bits), so the apparent "extra gain" is mostly the k=1 real baseline being inflated (the real r-adding walk at k=1 is worse than a random mapping; negation/automorphism symmetry perhaps removes part of that excess). A real-vs-ideal comparison within the same k would show no gain beyond sqrt(k).
- No excess fruitless-cycle loss: no R_k exceeds 1.4/sqrt(k). The 16-bit k=2 value 0.724 is only 2% above ideal.
- Does the toy test the lead? Only the step-count ceiling on Z_N logs; no curve arithmetic, no canonicalisation cost kappa, no GAE pricing (PROTOCOL s6). It cannot test kappa < 1.
- Surprising: R_k trending down with size for k=2 (0.72, 0.60, 0.57), which warrants a control with matched baseline, not a claim.

## Bearing on the sub-sqrt(p) question
none: the run is inconclusive by its own controls, and even the uncontrolled numbers sit at or near the sqrt(k) ceiling (the below-ceiling cells are explained by the baseline artifact), so it neither reveals a sub-sqrt gain nor touches the kappa condition.

## Standing
[pcc:unverified], single local toy run, no independent replay.
