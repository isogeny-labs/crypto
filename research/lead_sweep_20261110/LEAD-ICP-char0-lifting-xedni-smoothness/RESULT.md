# RESULT: LEAD-ICP-char0-lifting-xedni-smoothness

## Outcome
- status: ran; exit code 0; wall 2 s (run.status: exit=0 wall=2). run.stderr empty.
- Faster than the 1-3 min estimate in PROTOCOL.md section 4 (trial division exits early); not a crash.
- One run only, no retries, no edits to PROTOCOL.md or run.py.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md section 3): R ~ 1.00; consistent if |R-1| <= 3 SE + 0.03; falsified if R - 3 SE > 1.10; A/A within 3 SE of 1; planted positive control R > 3.

Measured (N = 150,000 per arm, B = 300, k in 1..63, p in {65521, 1048573, 4194301}):
- smooth rates: point-lift 0.062607, null 0.062473, A/A 0.062193, planted 1.0
- R = 1.00213, SE = 0.01417; threshold 3 SE + 0.03 = 0.0725, |R-1| = 0.0021. Consistent.
- A/A: R_aa = 0.99552, SE = 0.01410; |R_aa-1| = 0.0045 < 3 SE (0.0423). Pass.
- Positive control: R = 16.0 (> 3). Pass.
- Null control (uniform x, seed 202) is the comparison arm; its rate matches A/A.
- Controls passed: true. Verdict: consistent_with_prediction.

## Reading
- The numbers agree with run.py's verdict field; no discrepancy.
- The toy tests only the smoothness-of-integer-lift leg: x-coordinates of random curve points lifted as x + kp are no more B-smooth than uniform integers of the same size, at toy sizes. The 95% resolution is about +/- 3% (SE 1.4%), so a bias below roughly 5% would not be seen.
- It does not test xedni rank-drop, Cheon 2-isogeny lifts, heights/discriminants or asymptotics (PROTOCOL.md section 6). H-XEDN-4a5b50 is not touched.
- Nothing surprising. Possible quirk: the sampler is a fixed 3 primes, so results are pooled across sizes; no per-size breakdown was examined.

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction: for this one leg, the lifted x-coordinates behave like random integers, matching the random-image model; it says nothing about the dependency-certification leg.

## Standing
[pcc:unverified], single local toy run, no independent replay.
