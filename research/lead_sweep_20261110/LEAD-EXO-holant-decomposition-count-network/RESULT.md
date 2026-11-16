# RESULT: LEAD-EXO-holant-decomposition-count-network

## Outcome
- status: ran; exit code 0; wall 1 s (run.status: exit=0 wall=1; run.py elapsed_s=1.22). run.stderr empty.
- Single run of run.py, unmodified. verdict field: "inconclusive".

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): for p in {251,509,1021}: s_x >= 0.85; |s_x - mean(s_rand)| <= 0.03; DL s <= 0.35 * s_x at p=1021.
Falsified if s_x <= 0.8 * mean(s_rand) at any p. Consistent only if all three clauses hold and controls pass.

Measured (s = max over 3 middle bonds of rank/ceiling):
- p=251: s_x=1.0, s_rand=1.0 (all 4), DL s=1.0. Bond ranks x: 64,252,64.
- p=509: s_x=1.0, s_rand=1.0, DL s=1.0. x ranks 64,256,256.
- p=1021: s_x=1.0, s_rand=1.0, DL s=1.0. x ranks 256,1021/1024,256; random 1022-1023 at the middle bond; DL 64,256/1024,256.
- p=127 (sanity, excluded): s_x=1.0, DL 0.875.
- Clauses: x_high true, x_match true (deviation 0.0 everywhere), aa_ok true (spread 0.0), dl_low FALSE, pos_ctrl true (structured rank 2 vs relabelled 15/30/15).
- Controls: null passed, A/A passed, positive control passed. The in-lead DL positive control did not read as compressed under the preregistered statistic.
- Verdict: inconclusive (falsification not triggered; the DL clause failed).

## Reading
- The x-label clauses are met: x-labelling is at ceiling, indistinguishable from random relabelling, so the recorded digit-QTT obstruction re-derives on independent curves and FB.
- The inconclusive verdict comes from the DL clause, and it looks like a statistic artefact rather than a real failure. s takes the max over bonds, and the off-middle bonds (4 and 6 at p=1021) saturate at their small ceiling (4^k = 256) for every labelling, DL included. So DL s=1.0 regardless of structure. At the middle bond (5) DL rank is 256/1024 = 0.25 of ceiling and 256/1021 = 0.25 of the x rank, which would satisfy the <=0.35 clause. I report this as a post-hoc observation, not as a changed verdict.
- Rank at bonds 4/6 is capped by the small side, so the max-over-bonds statistic is dominated by the cap. The preregistered DL clause was badly specified.
- GF(2) rank only bounds real rank from below (PROTOCOL s6), so low DL rank is weak evidence anyway. Single curve per p.

## Bearing on the sub-sqrt(p) question
closes or confirms the obstruction: exact digit-TT rank of the x-labelled Semaev-2 matrix sits at the ceiling like random labels at p up to 1021, matching the record. DL-label structure is a statistic, not a cost, and truncated/other contraction models are untested.

## Standing
[pcc:unverified], single local toy run, no independent replay.
