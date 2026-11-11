# RESULT: LEAD-ICB-multi-target-and-portfolio

## Outcome
- Status: ran to completion (single run, no retries). Exit code 0, wall 3 s (run.status: exit=0 wall=3). run.stderr empty.
- q = 1048889 (P = 2q+1), K in {1,4,16,64}, reps = 60.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): controls pass (|S2|<=0.15, A/A within 0.1, r=0.1 gain < -0.2); S1 in [-0.65,-0.35]; for r=1.0 and 1.5 the best portfolio ratio-1 exceeds -2SE-0.01; verdict consistent_with_prediction.

Measured:
- ops/target shared: 1571, 1113, 595, 424 (K=1,4,16,64). S1 = -0.328 (just outside the band, edge -0.35).
- ops/target independent null: 1747, 3315, 3638, 6344. S2 = +0.286, so |S2| > 0.15: NULL CONTROL FAILED.
- ops/target A/A: 1564, 1025, 542, 533. S3 = -0.279. |S3-S1| = 0.049 <= 0.1: A/A passed.
- S4 positive control (r=0.1): -0.629 (SE 0.079) < -0.2: positive control passed.
- S4 r=1.0: -0.217 (SE 0.057, w=0.8, no restart cutoff). -2SE-0.01 = -0.124, so the gain exceeds 2 SE.
- S4 r=1.5: -0.151 (SE 0.048, w=0.8, no restart). -2SE-0.01 = -0.106, so the gain exceeds 2 SE.
- run.py checks: positive_control true, aa_ok true, null_flat false, share_slope_in_band false, obstruction_holds false.
- Verdict: inconclusive (the null control failed). Controls: null FAIL, positive PASS, A/A PASS.

## Reading
- run.py's verdict "inconclusive" agrees with the numbers and the protocol rule (any failed control gives inconclusive).
- Null arm is broken as a null: independent K=1 solves should give a flat per-target cost, but the null rises about 3.6x from K=1 to K=64. The protocol's null arm runs K independent single-target solves with reps/3 repetitions each. The measured medians change with K, so a K-dependent artefact is likely (for example, different seeds or sample sizes per K, or a cost accounting difference between the shared and null arms). I did not investigate, since the script may not be edited or rerun.
- The shared-arm slope (-0.33, A/A -0.28) is near the sqrt band edge but flattens at K=64 (595 to 424, 533 in A/A), consistent with the tame table cost not yet being amortised away. Not a clean K^(-1/2) test at this size.
- Portfolio: for r=1.0 and 1.5 the best of 30 settings beats pure rho by more than 2 SE, with w=0.8 and no restart. If the controls had passed, this would count as falsifying the closure in the toy model. Likely cause: the pure-rho cost samples are the measured K=1 costs, which are themselves heavy-tailed and drawn from the same sample whose mean mu defines the IC floor. An IC floor of r*mu then beats any rho draw above mu/w-ish. The gain mainly reflects the toy's cost model (the IC cost has a floor but lower variance than the rho draws) and best-of-30 selection, not a real IC advantage. This is not evidence against the recorded obstruction for real IC cost laws, which the toy cannot test.
- The toy does not test the lead's real claim (batch IC set-up amortisation, real IC attempt-cost distributions), as PROTOCOL.md s6 states.

## Bearing on the sub-sqrt(p) question
none: the run is inconclusive (failed null control) and the synthetic cost law cannot say anything about real IC or sub-sqrt(p) behaviour.

## Standing
[pcc:unverified], single local toy run, no independent replay.
