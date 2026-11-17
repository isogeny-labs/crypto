# RESULT: LEAD-ICB-composite-degree-c2pnb

## Outcome
- Status: ran to completion, single run. Exit code 0, wall 0 s by the shell timer (run.py elapsed_s = 0.3). No time-guard hit.
- run.py verdict field: consistent_with_prediction. smoke=false, seed 20261110.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md section 3): median generic ratio in [0.6, 1.3]; every subfield-lifted ratio <= 0.1; A/A median |ratio(A1)-ratio(A2)| <= 0.3. Falsified if any lifted ratio > 0.5.

Measured:
- median generic ratio = 0.6014 (band lower edge 0.6; passes by 0.0014)
- max subfield-lifted ratio = 0.0863 (limit 0.1; falsify at 0.5)
- median A/A ratio difference = 0.1347 (limit 0.3)

Verdict: consistent_with_prediction. Controls: positive (generic curves) pass only marginally; null/subfield arm passes; A/A passes. controls_ok = true, with the caveat below.

Per-config generic coverage/heuristic (A1, A2) vs lifted (1, 2), from the rows:
- (2,3) m=6: 0.27, 0.84 | 0.036, 0.036
- (2,5) m=10: 0.51, 0.09 | 0.008, 0.050
- (2,7) m=14: 0.046, 0.046 | 0.003, 0.003
- (3,3) m=9: 1.43, 1.47 | 0.086, 0.042
- (3,4) m=12: 0.69, 0.69 | 0.077, 0.086
- (3,5) m=15: 0.048, 0.183 | 0.003, 0.003
- (4,3) m=12: 0.93, 1.57 | 0.023, 0.014

## Reading
- Lifted arm is clearly far below the heuristic in every config (all <= 0.086), and base sizes of lifted curves are often tiny (1 to 5 points), matching the claim that the factor base sits in a small subgroup.
- The generic arm is NOT near 1 uniformly. The median passes at 0.6014, essentially on the boundary. Ratios fall with size: m=14 and m=15 configs give ~0.05-0.18, while m=9..12 with s=3,4 give 0.7-1.6. The heuristic uses nominal base size q+1, but actual generic base sizes were 3 to 21 (e.g. 3 of nominal 5 at s=2), so much of the ratio spread is base-size variance, and the (2,7),(3,5) rows suggest the heuristic overshoots when the actual base is small. The positive control is therefore noisy and only weakly confirms the detector.
- The verdict field matches the numbers. Passing the generic band is fragile (0.0014 margin); a different seed could flip it to inconclusive. The lifted-arm conclusion is robust (margin 0.09 vs 0.1 is tight only for the (3,3) row at 0.086, still within).
- The toy tests only coverage of k-fold sums at m <= 15, not the real ladder (F_{2^4} lift, k in {5,7,9,...}), relation yield, or c2pnb sizes.

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction: lifted-from-subfield toys show near-zero sum-set coverage, so the literal C1 ladder is uninformative about decomposition gain; this confirms a structural obstruction at toy scale only and says nothing about generic-coefficient c2pnb curves.

## Standing
[pcc:unverified], single local toy run, no independent replay.
