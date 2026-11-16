# RESULT: LEAD-TRF-second-torsion-direction

## Outcome
- Status: ran to completion (single run, no retries). Exit code 0, wall 14 s (run.status; internal 13.4 s).
- RESULT.json: 10783 curves scored (of 16500 drawn; n < 100 skipped), primes 6007, 9001, 14009, seed 20261110.

## Statistic vs preregistered prediction
- Prediction (PROTOCOL.md section 3): "F <= 0.005 -> consistent_with_prediction. F > 0.02 -> falsified (second directions are cheap generically; closure would be wrong). 0.005 < F <= 0.02, or any failed control -> inconclusive."
- Measured: F = 0.05110 (fraction with ord_n(p) <= 12), about 10x the 0.005 bound and 2.5x the 0.02 falsification threshold.
- Controls: positive control passed (40 curves with n | p-1, n | #E all k = 1). A/A halves 0.04744 vs 0.05471, difference 0.0073, about 1.7 binomial sigma (sigma ~0.0042), within 3 sigma, aa_ok = true. Null is the random-curve population itself. All controls OK.
- Verdict: falsified (run.py "verdict" field), consistent with the numbers.

## Reading
- The numbers match the verdict field; no discrepancy.
- The toy tests only the field-of-definition heuristic (k = ord_n(p) small), not Kani/Robert kernels, Richelot gluing or nonequivariant maps.
- The falsification is very likely a small-size artifact. With p <= 14009 the group orders are ~2^12.5..2^14 and the largest prime factor n is often only a few hundred to a few thousand. Then n | p^k - 1 for some k <= 12 happens by chance at a rate on the order of 12^2/n (several percent for n ~ 10^3), which is what is observed. That rate falls as 1/n, so it should vanish at cryptographic n. The protocol's n >= 100 floor is far too low to represent the ECDLP regime the claim is about. The result does not show the lead's closure is wrong at cryptographic size, and it does not confirm it either.
- Not examined: F as a function of n (no stratification by n in the output), so the 1/n scaling is an expectation, not a measurement here. The preregistered criterion was applied as written; it was not changed after seeing the data.

## Bearing on the sub-sqrt(p) question
none: the preregistered threshold was exceeded only in a regime (n ~ 10^2..10^4) where small embedding degree is expected by chance, and the toy says nothing about whether a second torsion direction yields a sub-sqrt(p) transfer at cryptographic n.

## Standing
[pcc:unverified], single local toy run, no independent replay.
