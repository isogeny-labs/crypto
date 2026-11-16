# RESULT: LEAD-EXO-coordinate-revealing-oracles

## Outcome
- status: ran; exit code 0; wall 7 s (run.status: exit=0 wall=7). run.stderr empty.
- One run of run.py, unmodified. RESULT.json holds the single JSON object.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md section 3): consistent if 0.58 <= s_coord <= 0.85 AND R > 1 AND negative control ratio in (0.5,2) AND controls valid; falsified if s_coord < 0.55 AND R < 1.

Measured:
- s_coord = 0.669 (randfb 0.681, A/A arm 0.644, BSGS 0.489).
- R = adds_coord/adds_BSGS at 2^20 = 15517 / 1723.3 = 9.00 (> 1).
- Negative control randfb/coord at largest size = 0.998 (in 0.5..2). Per size: 0.97, 1.04, 1.75, 1.38, 1.00 (12..20 bits).
- A/A ratio = 0.712 (band 0.7..1.4; barely inside). A/A per size: 1.04, 0.64, 1.55, 1.24, 0.71.
- Positive control: BSGS slope 0.489, within 0.1 of 0.5: passed.
- Relations valid: true (all checked relations validated, sizes <= 18 bits; no capped runs).
- Verdict: consistent_with_prediction. Controls passed (null, positive, A/A).

## Reading
- The toy tests a narrow class: x-coordinate used only as a membership filter in a pair-table relation search. It does not test Semaev/degree structure, non-simulable tests, or field-valued access.
- run.py's verdict field matches the numbers.
- Caveats:
  - The slope is fit over 5 points with noisy counts. Per-size ratios swing from 0.64 to 1.75, and the A/A ratio sits at the edge of its band. The factor-base multiplier chosen per size varies between 0.5 and 2 (the "optimised" B differs between arms and seeds), which adds noise.
  - The 2^20 row has no relation validation (log table only to 2^18), so the largest-size R rests on unvalidated relations.
  - The relation counts are small (9 to 101), so the slope uncertainty is probably about +-0.1, as the protocol states.
  - The random-point factor base performs the same as the coordinate one, as predicted: x gives no extra filtering power here.
  - Linear algebra is not charged, which favours index calculus. Even so, it costs about 9x BSGS at 2^20.
  - Slope near 2/3 (0.67) matches theory for B ~ p^(1/3).

## Bearing on the sub-sqrt(p) question
closes_or_confirms_obstruction: for this filter-only class the cost exponent is about 2/3 and is worse than BSGS (about 9x at 2^20), so coordinate access used as a membership filter gives no sub-sqrt advantage. It says nothing about algebraic (Semaev) use of coordinates.

## Standing
[pcc:unverified], single local toy run, no independent replay.
