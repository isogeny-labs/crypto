# RESULT: LEAD-TRF-pairing-target-field-recost

## Outcome
- status: ran; exit code 0; wall 0 s by run.status (integer-second clock; the toy is tiny). One run, no retries.
- run.stderr empty.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): "(a) and (b) both hold. Consistent_with_prediction if both hold and the controls pass. Falsified if either fails with controls passing."
Statistic: log2 SexTNFS(c=1.526) cost minus log2 matched rho.
- BN254 (rho 2^126.83): w=0 cost 2^99.0 (beats rho, -27.8 bits); w=0.5 2^123.74 (still beats, -3.1 bits); w=1 2^148.49 (loses, +21.7 bits). Claim (a) HOLDS (flip inside the band, between w=0.5 and 1).
- BLS12-381 (rho 2^127.33): w=0 2^119.27 (8.06 bits BELOW rho); w=0.5 2^149.09 (loses); w=1 2^178.91 (loses). Claim (b) has two parts: "within 4 bits of rho at w=0" FAILS (gap is 8.06 bits, and it beats rho at w=0); "does not beat rho at w=0.5" holds.
- MexTNFS (c=1.923) is shown for context only: it loses to rho for both curves at every w (BN254 w=0 is 2^127.54 vs 2^126.83, 0.7 bits above).
- Controls: positive control (toy rho / sqrt(pi r/4) in [1,6]) passed, ratios 3.32 to 3.81 at all four sizes. A/A passed, max |ln ratio| 0.117 (< 0.5). Null: not applicable (deterministic model).
- Verdict (run.py): falsified, with controls passing.

## Reading
- run.py's verdict agrees with the numbers: statistic.BLS12_381_near_rho_then_loses is false because of the 4-bit proximity clause. Only (b) failed; (a) held.
- The toy is GF(p^2) torus rho. It only certifies the step counter and sqrt(r) scaling. It does not test the exTNFS cost model at all.
- The model is a single-anchor L_Q(1/3,c) with o(1) dropped (anchor offset -10.74 bits); the BLS gap of 8 bits versus the 4-bit threshold is well within the several-bit disagreement between published estimates, and results are very sensitive to c (MexTNFS reverses the sign at w=0). The "falsification" of (b) is of a preregistered numeric threshold in an uncalibrated model, not of a real attack cost.
- Memory charge T*sqrt(T)^w is a stand-in, not a measured model nor 5a GAE pricing.

## Bearing on the sub-sqrt(p) question
none: the lead concerns pairing target-field DLP cost modelling and says nothing about ECDLP on non-pairing prime-field curves; toy run only re-confirms the rho sqrt(r) scale.

## Standing
[pcc:unverified], single local toy run, no independent replay.
