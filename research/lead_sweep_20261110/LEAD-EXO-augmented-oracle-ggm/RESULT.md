# RESULT: LEAD-EXO-augmented-oracle-ggm

## Outcome
- status: ran; exit code 0; wall 3 s (run.status: exit=0 wall=3); run.stderr empty.
- Single run of run.py, no edits to PROTOCOL.md or run.py.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md sec 3): simulator identical in 100% of trials; 0.7 <= G <= 2.165; S >= 1; falsified if any trial's simulated and real transcripts differ or G > 2.165. Positive control gain >= 10; A/A ratio in [0.7, 1.4].

Measured (4 orders 209497, 178261, 217027, 167899; 40 trials each):
- mean plain queries 610.98; mean augmented (add+E, charged) 4209.23; G = 0.1452 (outside [0.7, 2.165], below the lower edge, not above the upper).
- simulator_identical_all = false (run.py reports only the aggregate flag, not which or how many trials differ).
- simulated add queries mean 33659.6; S = 8.00 (>= 1, as predicted).
- Positive control (bisection): gain 34.65 (>= 10, pass).
- A/A plain vs plain: 0.9929 (in [0.7, 1.4], pass).
- Null: G = 1 by definition.
- Controls passed: positive yes, A/A yes, null trivial.
- Verdict (run.py): falsified. By the falsification criterion this is triggered by the non-identical simulator flag, not by G > 2.165.

## Reading
- The "falsified" label comes from the transcript-identity clause. G = 0.145 is a separate violation (prediction lower bound 0.7) and is not itself a falsification trigger.
- G = 0.145 means the lambda-orbit rho is about 7x more expensive than plain rho in charged queries, instead of roughly sqrt(3)/(cost of E queries). This is far worse than expected and suggests a toy artifact (e.g. orbit-walk cycles/fixed-point handling, restarts, and charging two E queries plus adds per step), not a property of the generic group. I did not diagnose it (no code edits allowed).
- The simulator-identity failure is more likely an implementation defect (identity is supposed to hold by construction, per PROTOCOL sec 6) than evidence against GGM simulability; it cannot be separated from a bug without per-trial output, which run.py does not emit.
- Net: the toy did not cleanly test the lead's claim; the controls pass, so the detector works, but the augmented arm looks mis-implemented or poorly tuned. No evidence of any gain beyond a constant (G is well below 1).

## Bearing on the sub-sqrt(p) question
none: the augmented-oracle arm showed no gain at all (G = 0.145), and the failed identity check is plausibly a toy defect, so it neither confirms nor closes the recorded obstruction.

## Standing
[pcc:unverified], single local toy run, no independent replay.
