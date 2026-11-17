# RESULT: LEAD-RHO-r-adding-walk-mixing

## Outcome
Status: ran. Wall 212 s. Exit code 0 (run.status: exit=0 wall=212). run.stderr empty/no errors.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): hash r=4 ratio >= 1.10; hash r=64 ratio in [0.95, 1.07]; every arm ratio >= 0.95. Falsified if any arm < 0.95, r=64 > 1.10, or r=4 < 1.02.

Measured (ratio of mean steps to first repeat to sqrt(pi N/2)=1283.39, 1500 walks, N=1048573):
- hash r=4: 1.330 (SE 0.018) -> holds
- hash r=64: 1.0135 (SE 0.0136) -> in [0.95,1.07]
- hash r=2: 9.42; r=8: 1.069; r=16: 1.041; r=32: 0.998 (SE 0.013)
- mod arms: r=2 287.9, r=4 139.9, r=8 71.2, r=16 37.2, r=32 19.2, r=64 10.1
- minimum arm ratio 0.998 (hash r=32), above 0.95.
Controls: null random mapping 0.991 (SE 0.014), within 3 SE: pass. Positive (r=2 > 1.25): pass (9.42). A/A r=64: 1.0139 vs 1.0096, z=0.22: pass.
Verdict (run.py): consistent_with_prediction. Controls all passed.

## Reading
The toy tests the claimed obstruction directly in an idealised group: hashed r-adding walks approach the random-mapping first-repeat cost as r grows (r>=32 indistinguishable from 1 within about 1 SE) and none beats it. The numbers match the verdict field; no discrepancy. Notable: the mod (linear) partition is catastrophically bad (10x to 288x), because x mod r partitions interact with additive structure on Z_N. Hash r=32 at 0.998 vs r=64 at 1.0135 is noise (SE ~0.013). Differences under about 5% are unresolved, per protocol. Z_N additive is an idealisation; no EC, negation, DP or parallel effects.

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction: no tested walk beat the random mapping, so walk choice yields at best no gain over sqrt(pi N/2) steps in this model.

## Standing
[pcc:unverified], single local toy run, no independent replay.
