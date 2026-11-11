# RESULT: LEAD-ICP-prym-witness-relation-diversity

## Outcome
- status: ran; exit code 0; wall 17 s (run.status: exit=0 wall=17). Single run, no retries.
- RESULT.json is one JSON object; run.stderr empty.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): (a) frac distinct/A>=0.9 at A=97 >=0.95; (b) frac rank mod Q == 15 at A=97 >=0.95;
(c) mean gain in distinct from A=97 to 194 >=50; A/A mean-distinct gap <3. Falsified if (a)<0.8 or (b)<0.8.

Measured (100 valid seeds):
- (a) 1.0 (mean distinct at A=97: 95.04) -> met.
- (b) 0.0 (mean rank at A=97: 14.0) -> NOT met; below 0.8, so run.py reports "falsified".
- (c) mean gain 91.83 -> met (>=50).
- A/A gap 0.34 (<3) -> passed.
- Positive control (capped S=13, S=9: distinct == S, rank <= S): passed (max rank 13 and 9).
- Null control (generic harvest) ran as the main arm.
Controls ok: yes. Verdict (run.py): falsified.

## Reading
The "falsified" verdict is driven solely by criterion (b), and (b) is a design flaw in the protocol, not
evidence about the lead. Every relation is a difference of two combinations with equal sum_i c_i f_i mod Q,
so each relation vector d satisfies d.f = 0 mod Q for the single hidden vector f. All relations lie in the
kernel of f, which has dimension K-1 = 14. Rank 15 is impossible for any number of witnesses; rank 14 in
100/100 seeds is full rank. The measured mean rank of exactly 14.0 confirms this. (Rank K would only be
reachable if a target relation was attached, which the protocol says is not modelled.)
Criteria (a) and (c) both support the hypothesis: the generic harvest gives near-full diversity
(95 of 97 distinct), doubling A adds about 92 distinct equations, and capped-support generators saturate
exactly at S. So the toy shows duplication caps are generator-specific, but the preregistered
falsification rule fired on an unreachable target. I did not edit or rescore; by the frozen rule the
outcome is "falsified", and the honest reading is "(a),(c) consistent, (b) mis-specified (should be K-1)".
The toy uses no Prym geometry, no real curves, no owner data, no target Q (PROTOCOL s6).

## Bearing on the sub-sqrt(p) question
none: a synthetic collision harvest in Z_4093 with known dlogs says nothing about Prym witnesses or any
sub-sqrt(p) algorithm; the real records remain unlocated.

## Standing
[pcc:unverified], single local toy run, no independent replay.
