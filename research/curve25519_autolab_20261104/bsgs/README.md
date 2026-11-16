# BSGS-class boundary panel for the 0.75x goal

Claim in `pcc/claims/` (this PR), standing `unverified`. Owner ruling 2026-11-08: BSGS-class results are built and reported as labelled boundary results outside the index-calculus ratchet. Baby-step giant-step is time-memory (about sqrt(r) stored points); the repository excludes it from the IC leaderboard. This panel replaces the derived 0.69 estimate in `ideas/NONCOLLISION-20261108.md` with a measurement.

Design: `bsgs.negation`, `rho.negation` and a rho A/A arm on the five fresh family curves of `c1/confirm-curves.json`, 12 targets x 2 rounds per curve, k = 20 and 24 (`spec-k*.json`). Audit 540/540 records, 8 replays reproduced, at each size.

## Result [pcc:unverified]

Mean per-curve S(bsgs-neg) / S(rho-neg):

| k | w = 0 | w = 0.1 | w = 1 | worst curve (w = 1) |
|--:|--:|--:|--:|--:|
| 20 | 0.611 | 0.625 | 0.697 | 0.743 |
| 24 | 0.889 | 0.897 | 0.933 | 0.999 |

Mean S at k = 20, w = 0: bsgs-neg 0.981, rho-neg 1.620 (A/A 1.620). At k = 24: 1.041 against 1.176.

## Reading [pcc:unverified]

- At k = 20 BSGS is below 0.75x of rho-neg across the whole band. The ratio is mostly rho-neg's overhead at small k (S 1.62 against its floor 0.886), not a BSGS gain: BSGS S is about 1.0.
- At k = 24 it is 0.89 to 0.93, above 0.75. The ratio rises with k, towards the 1.0 of BSGS-neg against the rho floor.
- This is not an index-calculus result and does not count toward the ratchet. The best index-calculus ratio remains 1.40 (C = 1, k = 20, `c1/README.md`).
- BSGS-neg memory is about sqrt(r) entries; bytes are not charged under the band rule.
