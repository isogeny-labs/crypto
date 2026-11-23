# RESULT: LEAD-ICP-shared-coefficient-membership

## Outcome
- Status: ran to completion. Wall 52 s. Exit code 0 (run.status: exit=0 wall=52). run.stderr empty.
- Single run, no retries, PROTOCOL.md and run.py untouched.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): 0 chart violations, 0 Abel violations, |z|<4 vs 1/720, |A/A z|<4, at least 30 split trials; negative control false-sum rate <=2%.

Measured (140000 trials, 3 arms):
- chart violations: 0
- Abel-sum violations: 0 (187 split trials; 71 at p=521 seed 11, 88 at p=521 seed 12, 28 at p=1031 seed 13)
- negative-control false sums: 0 of 187 (limit 2%)
- split-rate z vs 1/720: -0.534
- A/A z (seed 11 vs 12, p=521): -1.348
- split rates: 0.001183, 0.001467, 0.001400 vs expected 0.001389

Verdict: consistent_with_prediction. Controls passed: null (negative control, sum not vacuous), positive (Abel sum = O on all genuine split sets), A/A (|z|<4).

## Reading
- The toy tests exactly what the protocol says: the zero set / norm-root correspondence for A+By in L(6O), Abel sum for fully split cases, and generic degree-6 splitting yield. It confirms a known identity and a generic yield.
- run.py's verdict field matches the numbers. No discrepancy found.
- Caveats: the p=1031 arm has only 28 split trials (below 30 on its own); the 30-trial minimum is met only in total/per p=521 arm. Pooled z is fine. The p=521 arms use different random curves per seed, so A/A compares curve+seed together. The negative control hit 0, so its upper bound on the false-sum rate is only loosely informative (about 1.6% at 95% for n=187).
- Nothing surprising. Degenerate charts (B(x)=0) are excluded by definition and not tested.

## Bearing on the sub-sqrt(p) question
none: this sanity-checks a known correspondence and generic split probability; it measures no cost of a shared circuit, no solver slope, and nothing about the m=3 protocol gates.

## Standing
[pcc:unverified], single local toy run, no independent replay.
