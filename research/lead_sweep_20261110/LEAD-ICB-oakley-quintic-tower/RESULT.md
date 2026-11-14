# RESULT: LEAD-ICB-oakley-quintic-tower

## Outcome
- status: ran; wall 3 s (run.status: exit=0 wall=3); single run, no retries. run.stderr empty.

## Statistic vs preregistered prediction
- Prediction (PROTOCOL.md s3): 0.7 <= R <= 1.4 on both curves (a=0,b=1 and a=1,b=1); positive control >= 3x null mean; A/A ratio in [0.6, 1.67].
- Measured (k=4, field 2^20, |FB|=15, 3003 5-subsets):
  - a=0: coll_fb=2987, null_mean=1393.7, R=2.143; pos ratio 2.086; A/A 0.9975.
  - a=1: coll_fb=2987, null_mean=1392.0, R=2.146; pos ratio 2.088; A/A 1.000.
- Verdict (run.py): inconclusive on both curves. Controls: A/A passed; positive control FAILED (2.09 < 3); null ran. So controls are not all valid and R outside the band cannot be read as falsification.

## Reading
- Even the null has about 1390 collisions among 3003 subsets, almost half. The "tens" the protocol expected (s6) did not occur. The cause is largely trivial: negation closure makes any subset containing P and -P reduce to a 3-subset sum, and with |FB|=15 (not the ~17 expected) the sums land in a small set.
- FB = points with x in F_16 is E(F_16), a subgroup of order <= ~25 inside the full group. All FB sums therefore lie in a tiny subgroup, so FB collisions (2987 of 3003, near saturation) are structural. The null draws random points from the whole group, so it is not the same kind of null. R about 2.14 reflects that subgroup structure, but saturation also caps the ratio, and the weak positive control (2.09, also saturated) shows the detector has little dynamic range here.
- The toy is badly sized: with the sum space far smaller than the number of subsets, collision counting cannot separate uniform behaviour from structure. It does not test the n=5 uniformity heuristic at meaningful scale. run.py's verdict field matches the numbers.

## Bearing on the sub-sqrt(p) question
none: inconclusive, saturated toy; the subfield-FB-lies-in-E(F_16) observation is a known fact for subfield factor bases, not a new signal, and says nothing about F_{2^155}/F_{2^185}.

## Standing
[pcc:unverified], single local toy run, no independent replay.
