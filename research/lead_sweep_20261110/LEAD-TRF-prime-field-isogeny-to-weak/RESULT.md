# RESULT: LEAD-TRF-prime-field-isogeny-to-weak

## Outcome
- status: ran; exit code 0; wall 2 s (run.status: exit=0 wall=2). stderr empty.
- Single run of run.py, no retries, PROTOCOL.md and run.py untouched.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): M = 0 and W = 0 in both seeds; positive control flags >=50% of independent-curve pairs as order-differing; weak curves exist in the sample; A/A (seed 20261111) also M=W=0.

Measured:
- Main arm (seed 20261110): 2370 isogenous pairs, order_mismatch M=0, weak_status_mismatch W=0; anomalous curves 8, MOV k<=6 curves 280.
- A/A arm (seed 20261111): 2328 pairs, M=0, W=0; anomalous 4, MOV 293.
- Control (independent curves): 1185/1200 order-differing (98.75%) in each arm.
- positive_control_detects = true.

Verdict: consistent_with_prediction (run.py field). Null control, positive control and A/A all passed.

## Reading
- The toy does test what it claims: Tate's theorem (equal #E on F_p-isogenous curves) and hence constancy of order-determined weak flags, on 2-isogeny walks. It re-derives the recorded obstruction at toy size and does not touch descent-friendliness.
- Discrepancy: run.py used 8 primes (1009..1297), not the 12 described in PROTOCOL.md s4. The "every 6th prime in [1000,1300)" wording gives about 8 primes (the range holds about 50), so the 12 looks like a protocol miscount. It does not change the result.
- Odd but unexplained: control_order_differs is exactly 1185 in both arms. This may be a coincidence, a shared control seed or a derived seed that does not depend on the arm. I did not inspect run.py's code in depth, so this is unresolved. The control only needs >=50%, so the verdict is unaffected.
- The isogenous-pair count is below the 2400 maximum (8*150*2 ... walk discards of singular or rootless steps), as expected.
- Weak curves exist in the sample (anomalous and MOV counts nonzero), so the special set is nonempty in the field; only the isogeny class is closed.

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction: zero order and weak-status mismatches in 4698 pairs matches Tate's theorem, so rational isogeny cannot turn a non-weak prime-field curve into an anomalous or low-embedding-degree one.

## Standing
[pcc:unverified], single local toy run, no independent replay.
