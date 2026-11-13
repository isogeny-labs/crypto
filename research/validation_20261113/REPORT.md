# Independent validation: negation re-measurement (2026-11-13)

Validator role, for the target `research/negation_remeasure_20261113/`. The
harness did not let the validator write this file, so its returned report is
transcribed here. Raw outputs are in `out/`, the scripts in `scripts/`, and
the resume checklist in `PROGRESS.md`. Every figure is `[pcc:unverified]`.

## Disposition at hand-back: insufficient evidence (commit binding only)

Every measurement check passed. The single failure was that the artifacts were
uncommitted: the record bound a dirty `47dc95a` that lacked the code. Those
exact bytes are now committed in `8bf77ba333`. All 25 target lines of
`out/snapshot_start.sha256` match that commit, and the record is closed in the
following commit. Admission (writing `VALIDATION.json`) is left to a validator
pass that confirms the committed hashes.

## Checks

| # | Requirement | Result |
|--|--|--|
| 1 | `frozen.sha256` | pass. Protocol ctime 03:00:01Z, before the first run at 04:20:02Z; this is filesystem evidence, not git |
| 2 | Record raw sha256 | pass, 9 of 9 |
| 3 | Certificates (independent `vlib` and validator code) | pass, 6,020 of 6,020 logs. A tamper control was rejected (28 errors) |
| 4 | Replay | pass. Every cell of v3, cover, tower p ≤ 16001 and tower p = 32003; 0 differences outside timing |
| 5 | Tables recomputed from raw JSON | pass. 0 mismatches across 162 arm-rows, 214 metrics, 478 recorded values, 1,497 RESULTS cells and 8,272 merged values |
| 6 | Strongest rho | pass. Tuned negation walk. The automorphism fold is vacuous: none of the 38 curves has `j = 0` or `1728` (checked independently) |
| 7 | Same process, native counters emitted on both sides | pass, with the lower-bound gaps below |
| 8 | §5a band | pass. 156 of 162 rows above at 0, 0.1 and 1; 6 unresolved lower bounds; 0 below |
| 9 | Calibrated reading | pass. It agrees with the band except on the same 6 rows (best-of-N timing, labelled as such) |
| 10 | Same unit | pass as a counting convention. A Jacobian step costs 4.3–22.5 elliptic additions in time, so "IC above rho" is conservative |
| 11 | Bound to a commit | was the failure; now addressed by `8bf77ba333` |

## Discrepancies (none changes a class)

1. **Binding.** The measured parts came from an earlier build whose source was
   not kept. The source files were last modified during the p = 32003 run.
   Replaying from the committed source closes this gap except at tower
   p = 64007.
2. **"Old" at p = 64007.** The "old" column at p = 64007 is a 4-seed rerun,
   not the recorded original. For example, the g3 old ratios are 141.78 /
   159,687 / 1,595,592 against the recorded 141.97 / 160,179 / 1,600,508.
3. **Arms A/B/C are lower bounds.** They are labelled complete, but they omit
   the walk's partition hash, the large-prime HashMap and the DSU
   (`ic_control.rs:1129–1155`). Reversing any order would need at least 165
   (v3), 4,585 (tower g3) or 2,587 (cover) extra native operations per arm-C
   group operation.
4. **Other lower bounds.** The v3 and cover recorded collectors emit no
   relation-phase work, and the cover first-collision arms emit no counters.
   These account for the 6 unresolved rows.
5. **`metrics[].n`.** It holds the rho run count, not the arm sample size.
6. **Disclosed deviations, confirmed in the data.** Cover rho runs on seed 0
   only. Tower p = 64007 used 4 and 2 seeds instead of 8 and 4. Rho word ops
   are priced, which is conservative.

## Not verified

- The p = 64007 runs were not replayed; only their instances were rebuilt.
- Solver operation counts were reproduced only by re-running producer code.
- S3 copies of the raw files; none were uploaded because there are no
  credentials.
- Pre-data freezing, beyond filesystem ctime.
