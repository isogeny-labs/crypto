# RESULT: LEAD-ICE-semaev-chain-cost-model

## Outcome
- status: ran; exit code 0; wall 0 s (run.status: exit=0 wall=0). run.stderr empty. Single run, no retries.

## Statistic vs preregistered prediction
- Prediction (PROTOCOL s3): S1 = 1 for all m in 3..8, S2 = 1, S3 >= 2 for all m >= 4 (min = 2^ceil(log2 m)). Falsified if S1 > 1 or S2 > 1.
- Measured: S1 = 1 at every m (degrees 4,8,16,32,64,128). S2: relation_count_invariant = true, BUT counts_sample = [0,0,0,0,0], i.e. the random target R had zero decompositions through all 5 trees.
- S3 (distinct topology-aware max-degree values): m=4: {4,8}; m=5: {8,16}; m=6: {8,16,32}; m=7: {8,16,32,64}; m=8: {8,16,32,64,128}. All >= 2. Minimum 2^ceil(log2 m) holds at m=5..8 (8,8,8,8); at m=4 min is 4 = 2^2, consistent.
- Controls: positive (naive 2^leaves topology-dependent) passed (distinct costs 2,3,6,11,23 for m=4..8; 1 at m=3); A/A shuffled-order equal: passed; null (Semaev degree uniform): passed.
- Verdict: consistent_with_prediction (run.py field).

## Reading
- S1 and S3 are consequences of the model definitions (fixed degree 2^(m-1) vs 2^depth), so they are near-tautological; they confirm the code implements the stated models, not anything about a solver.
- Caveat on S2: all relation counts are 0 for the seeded target, so "invariant" is trivially satisfied and the set-identity premise was not exercised with nonzero counts. The run.py verdict field is therefore stronger than the data on S2. The A/A arm on all-zero counts is likewise weak.
- No Groebner or transfer-matrix solver was run; the 2^depth model is an assumption. EV-SEMBIN-1d36a3 and t*=m* were not reproduced.
- Nothing surprising otherwise; runtime trivial.

## Bearing on the sub-sqrt(p) question
none: the toy only shows that topology-dependence appears when a topology-aware degree model is assumed; it measures no solver, no cost, no speedup.

## Standing
[pcc:unverified], single local toy run, no independent replay.
