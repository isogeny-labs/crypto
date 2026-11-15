# RESULT: LEAD-ICB-learned-cost-model-factor-base-selection

## Outcome
- Status: ran (single run, no retries). Exit code 0, wall 6 s (well under the 1-4 min estimate; no crash or timeout).
- Output: RESULT.json (one JSON object), run.stderr empty, run.status "exit=0 wall=6".

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): mean within-l Spearman of predicted vs measured log yield on the transfer curve (first draw A1) >= 0.5 and > null p95 (permuted labels); positive control >= 0.5; A/A |A1-A2| <= 0.2 on both curves. Falsified if controls are valid and either condition fails.

Measured:
- Ridge held-out Spearman: transfer curve -0.785, same curve -0.773.
- Size-only baseline (reported, not gated): transfer -0.942, same curve -0.949.
- Null p95 (100 label permutations, seed 7): 0.495. Threshold 0.5.
- Positive control (in-sample ridge on test data): 0.946 (>= 0.5, valid).
- A/A abs diff: same curve 0.060, transfer curve 0.0001 (both <= 0.2, valid).
- Weights: [-0.286, 0.049, 0.102, -0.0041, -0.0020].

Verdict: falsified (run.py "verdict" field). All three controls passed (null well-formed, positive control valid, A/A valid), so the falsification is not an artefact of a failed control.

## Reading
- The numbers match run.py's verdict field; no discrepancy.
- The sign is the notable result: the held-out Spearman is strongly negative, not merely low. Yield (distinct P+Q sums over pairs) falls with factor-base size at l = 6..8, as saturation of the 2^13 field sets in. The ridge, trained at l = 3..5, extrapolates a positive log|FB| slope (0.049) and ranks in the wrong direction.
- Within-l ranking is dominated by FB size (the baseline reaches -0.94 in magnitude), so a sign-corrected size-only count would explain most of the ordering. The learned features add nothing that counting points does not.
- Caveats: this tests a stand-in feature set on a non-Koblitz GF(2^13) toy with a pair-sum yield proxy, not the frozen gated ridge, which was not recovered (PROTOCOL s1, s6). The failure may reflect the training range (l <= 5, far from saturation) rather than a flaw in gated models generally. Two curves only. Run was fast, so a larger design was affordable but is outside this frozen protocol.

## Bearing on the sub-sqrt(p) question
none: a stand-in toy on a 13-bit field with a pair-sum yield proxy, with no matched rho, no section 5a band and no replay certificate, says nothing about beating rho; it only shows that naive extrapolation of a log-linear yield model past l = 5 fails here.

## Standing
[pcc:unverified], single local toy run, no independent replay.
