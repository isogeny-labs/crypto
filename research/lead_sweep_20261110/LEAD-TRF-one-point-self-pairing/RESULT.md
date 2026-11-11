# RESULT: LEAD-TRF-one-point-self-pairing

## Outcome
- status: ran; exit code 0; wall 33 s (run.py reports 32.19 s). Single run, no retries.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md section 3): over random prime-order curves, p in [2^12, 2^16), r > 1000: (i) 100% have gcd(r,p-1)=1; (ii) median k/(r-1) >= 0.1; (iii) at most 5% have k <= 20. Positive control: k=1 curves, >= 90% nontrivial, T(P,2P)=T(P,P)^2 in every case, n >= 30. A/A halves within 0.15. Verdict "inconclusive" if positive control fails, A/A fails, or n_null < 100.

Measured:
- n_null = 240; trivial pairing target (gcd(r,p-1)=1): 240/240.
- median k/(r-1) = 0.5; min k = 21; curves with k <= 20: 0/240.
- A/A halves medians: 0.5 and 0.5 (difference 0, passes).
- Positive control: n = 16 (protocol asked n = 60 attempted, n >= 30 required); 16/16 nontrivial (frac 1.0); bilinearity 16/16.
- Null-population criteria (i)-(iii) are all met, so the claim is not falsified.
- run.py verdict: inconclusive. Controls_ok = false, because the positive control reached only n = 16, below the preregistered n >= 30.

## Reading
- The null-population numbers match the recorded obstruction: no curve had a nontrivial self-pairing target, and k was large (median exactly 0.5, the typical value when ord_r(p) is (r-1)/2 or (r-1)).
- The verdict field is consistent with the numbers. The only failure is the positive-control sample size (16 < 30). The detector worked on every case it saw (16/16 nontrivial, bilinear), but the preregistered n floor was not met. The generator for k=1 curves (prime r < 2^13, p = m*r+1, m in {2,4,6,8}, cofactor and r^2 conditions) yielded fewer than 60 usable instances. This is a protocol-level shortfall that I did not fix.
- Nothing surprising otherwise. The toy tests only the Tate self-pairing at toy sizes; the minimum k of 21 sits just above the threshold of 20, so criterion (iii) has little margin at this sample size, though it passes.
- The result is the preregistered "inconclusive", not a pass, even though the null evidence agrees with the theorem.

## Bearing on the sub-sqrt(p) question
closes_or_confirms_obstruction: the null population agrees with the recorded theorem (trivial target, large k), so it gives no sign of a one-point self-pairing route. The formal verdict stays inconclusive because of the positive-control n.

## Standing
[pcc:unverified], single local toy run, no independent replay.
