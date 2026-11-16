# RESULT: LEAD-STR-glv-gls-isogeny-loop-scalar-mult

## Outcome
- status: ran (single run, no retries). exit code 0, wall 3 s (run.status: exit=0 wall=3).
- Params: p=167899, b=18, N=168253, lambda=32704, beta=58980, 300 scalars, 20 seeds. stderr empty.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md section 3): S1 in [1.45, 2.0] with all GLV products equal to binary products; S2 in 0.408 +- 0.08; wrong-lambda control fails >= 15/20; A/A plain ratio in [0.8, 1.25], else verdict `inconclusive`.

Measured:
- S1 = 1.908 (in band), glv_correct = true.
- S2 = 0.4106 (in 0.328..0.488; 1/sqrt6 = 0.4082).
- Negative control (wrong lambda): 20/20 failures (pass).
- Positive control (negation-only): 0.9225 vs expected 0.707 (reported, not gated).
- A/A plain ratio: 0.7244, OUTSIDE [0.8, 1.25] (fail).

Verdict: inconclusive (run.py field). Per the preregistered rule an A/A failure forces `inconclusive`, even though S1 and S2 are in band. Controls: null/negative passed; A/A failed; positive control not gated, but it was well off its expectation.

## Reading
- S1 and S2 point values match the prediction, but the instrument's noise floor was not shown to be adequate. With 20 seeds, plain-arm means of birthday draws have a relative sd of roughly 1/sqrt(20) * ~0.52 (about 12%), so an A/A ratio of 0.72 is a modest excursion, not extreme. The same noise implies S2 has an uncertainty comparable to its +-0.08 tolerance.
- The positive control (negation-only labelling) gave 0.92, not about 0.707. This is a discrepancy. Orbit label is min over the 6 elements; the negation-only arm should halve the class count and give about 0.707. Possible causes are noise or a label-set difference between arms. I did not investigate (rules forbid editing or new scripts). It weakens the claim that the detector sees the effect at sizes below the full 6-orbit, though S2 matched 1/sqrt6 closely.
- No gain below the unit ceiling was seen (S2 well above 0.33). The toy tests only H1 (GLV op ratio vs plain binary) and the sqrt|Aut| birthday ceiling. It does not test GLS, isogeny loops, neighbour routing or a wNAF baseline (protocol section 6).
- S1 = 1.908 sits near the top of its band (2.0 upper edge) against a plain binary baseline, so it says nothing about the recorded 1.525x over wNAF.

## Bearing on the sub-sqrt(p) question
none: the toy only reproduces the known unit-orbit ceiling and a scalar-multiplication speedup, and the verdict is inconclusive because the A/A control failed.

## Standing
[pcc:unverified], single local toy run, no independent replay.
