# RESULT: LEAD-ICB-frobenius-search-symmetry

## Outcome
- Status: ran. Exit code 0. Wall 2 s (run.status: exit=0 wall=2). run.stderr empty.
- Single run of run.py, unmodified.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md section 3): stable-FB difference == 0 exactly; tau_over_randompair in [0.9,1.1] for n=11 and n=13; fixed fraction < 0.01. Falsified if positive control passes and T < 0.8 or T > 1.25.

Measured:
| n | order | stable-FB diff | random diff (tau) | random diff (random pair) | T | fixed pts | fixed frac |
|---|-------|----|------|------|------|---|--------|
| 11 | 1982 | 0.0 | 6.137 | 6.189 | 0.9916 | 2 | 0.00101 |
| 13 | 8374 | 0.0 | 4.245 | 4.233 | 1.0030 | 2 | 0.00024 |

Verdict (run.py): consistent_with_prediction. All three conditions hold.
Controls: positive control (tau-stable FB exact invariance) passed. Null (random-permutation partner) is the denominator of T and is near the tau value. A/A: no separate second shuffle appears in the output; the 5-seed spread is not reported, only per-n means. A/A noise floor is therefore not directly shown.

## Reading
- The toy tests exactly what the protocol states: group-level conjugation structure of 2-point decompositions on two toy Koblitz curves. The numbers match run.py's verdict field; no discrepancy.
- The stable-FB result is a theorem check (implementation sanity), not evidence.
- T within about 1% of 1 means tau gives no per-target correlation for random FBs. Fixed points are 2 per curve (presumably identity and one other), far under 1%.
- Caveats: only n=11 and 13, 5 seeds each, 2-point decompositions, no descended polynomial system, SAT/CNF or equivariant elimination. No per-seed spread or A/A band in the output.

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction: at toy scale Frobenius acts only as a conjugation with no single-target stabilizer, consistent with the atlas closed_obstructed classification; it does not address the descended-system next step.

## Standing
[pcc:unverified], single local toy run, no independent replay.
