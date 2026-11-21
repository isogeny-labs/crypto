# RESULT: LEAD-TRF-kernel-of-reduction-lemma

## Outcome
- status: ran (single run, no retries). exit code 0. wall 33 s (run.status: exit=0 wall=33).
- Files: RESULT.json (one JSON object), run.stderr (empty tail), run.status.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md section 3): "Null arm hits per arm <= mu + 5 sqrt(mu) + 2 with mu = usable/p; chi2 <= 37.7 per curve; positive control hits = usable (100%)." A/A hit counts differ by <= 3. Falsified if any null arm exceeds the bound.

Measured:
- Positive control (anomalous N = p, p = 39887, 14593, 9283): hits 3000/3000 on all three curves (100%). Detector sees the Smart channel.
- Null (6 non-anomalous prime-order curves, 3000 trials per arm): hits_A / hits_A2 = 0/0, 0/0, 1/0, 0/3, 0/0, 0/1. Largest is 3 of 2996 at p = 5557 (mu = 0.54, bound about 6.2). All within bound.
- A/A hit differences: max 3 (p = 5557), within the <= 3 limit.
- Chi-square (15 df, 16 bins) of (r-k) mod p: 20.9, 7.9, 14.6, 8.8, 11.9, 12.0; max 20.93 <= 37.7.
- Flags: pos_ok true, null_ok true, aa_ok true. Controls passed (null, positive, A/A).
- Verdict: consistent_with_prediction.

## Reading
- The toy tests exactly the mod-p^2 first formal-group-layer channel of the Smart ratio. For N != p, psi(N P~) is a random-looking element of F_p because the lift is random and N is invertible mod p. The ratio therefore matches k only at chance level 1/p. The run confirms that and the anomalous case recovers k always.
- run.py's verdict field matches the numbers. No discrepancy found.
- Minor point: the A/A difference sits at the limit of 3 for p = 5557, but with mu about 0.54 such counts are expected under Poisson noise. The comparison is a weak test at small mu.
- This is a sanity check of an algebraic expectation, as PROTOCOL.md section 6 says. It does not touch higher layers, global lifts, heights or xedni/function-field constructions, and it does not prove the lemma for all p. Toy sizes only (p from 5557 to 39887).
- Nothing surprising.

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction: at toy size the first formal-group layer gives no log information for N != p, matching the recorded obstruction, but it is not new evidence beyond the algebra.

## Standing
[pcc:unverified], single local toy run, no independent replay.
