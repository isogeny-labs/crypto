# RESULT: LEAD-ICB-generic-table-oracle

## Outcome
- status: ran. exit=0, wall=2 s (run.status). Single run, no retries. run.stderr empty.
- run.py verdict field: "inconclusive".

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): ratio_c0(n) >= 1 for all n in {11,13,15,17} (expected well above 1), non-decreasing trend in n.
Falsified if any ratio < 1 with controls passing; consistent if all >= 1 with controls passing; inconclusive if a control fails.

Measured (best oracle group adds / mean rho evaluations):
| n | r | rho mean | best k, nF | oracle adds | ratio_c0 | ratio c=0.1 | ratio c=1 | null ratio_c0 | A/A rel diff |
|---|---|---|---|---|---|---|---|---|---|
| 11 | 991 | 75.7 | 5, 14 | 201 | 2.65 | 2.79 | 4.00 | 5.60 | 0.316 |
| 13 | 4187 | 241.4 | 6, 26 | 1038 | 4.30 | 4.59 | 7.24 | 3.74 | 0.750 |
| 15 | 16247 | 791.2 | 6, 36 | 1829 | 2.31 | 2.46 | 3.81 | 2.35 | 0.160 |
| 17 | 65587 | 3081.1 | 7, 72 | 5839 | 1.90 | 2.00 | 2.96 | 1.83 | 0.107 |

Controls:
- Positive (every relation re-summed and verified): passed at all n.
- Null (random FB of equal size): reported; ratios are similar to the subspace ratios (e.g. 1.83 vs 1.90 at n=17), i.e. no visible subspace advantage.
- A/A (<35% relative difference): FAILED at n=13 (0.75; seed 202 chose a degenerate k=4, nF=2 base with 86 tries). Passed at 11 (0.316), 15, 17.
Verdict: inconclusive by the preregistered rule (A/A control failed). All four ratios are >= 1 (min 1.90), so no falsification was observed.
The "non-decreasing in n" part of the prediction did not hold: 2.65, 4.30, 2.31, 1.90 (decreasing from n=13).

## Reading
- The toy tests a generic pair-table 4-sum oracle against a cheap rho at toy sizes. It does not test the atlas lead's actual oracles (compact-orbit, cap-locus, known-log pools); the atlas note was not read.
- run.py's verdict matches the numbers: the A/A failure at n=13 forces "inconclusive" even though every ratio exceeds 1.
- The A/A failure comes from the best-k selection at tiny nF (a handful of relations) being high-variance, not from verification errors.
- Surprising: the rho reference averages 3081 evaluations at n=17 against a birthday theory value of 321 (about 10x). The "first repeat of a 16-way additive walk" is much weaker than ideal rho at every n (ratio 1.9 to 3.2 at n=11..17), so the oracle-vs-rho ratios are optimistic for rho being beaten. Against the theory value the ratios would be about 5 to 18 and rising with n. The preregistered ratios are therefore flattered downward.
- The ratio falls with n against this inflated rho (4.30 to 1.90 from n=13 to 17), because the 4-sum oracle cost grows more slowly than the weak walk's cost over this narrow range. Four points do not support an extrapolation.
- Group ops are simulated through a log table, so native Koblitz costs are not priced.

## Bearing on the sub-sqrt(p) question
none: inconclusive on the preregistered rule, all ratios >= 1 (no oracle win over even a weak rho), and four toy sizes cannot speak to exponents.

## Standing
[pcc:unverified], single local toy run, no independent replay.
