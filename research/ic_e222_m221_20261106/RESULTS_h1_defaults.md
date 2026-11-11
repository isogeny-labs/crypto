# H1 (cofactor-projected factor base) with the new defaults (local, unvalidated)

**Status:** unvalidated (pre-gate). Local unisolated host, serial seeded interleaved order; no replay certificate, validator report or matched-rho band. Not a leaderboard row. Metrics: `trials_total` (operation count) and solver `total_ms` (wall, diagnostic, milliseconds per solve).

**Question.** H1 (`--project-cofactor`, entries [h]F) cut the baseline's trials by about 1/h when the collector was the per-trial sweep with fb 8 (`RESULTS_h1.md`, `RESULTS_h1_holdout.md`). Does it still do so with the current defaults (pair-sum table, fb 32, extra 2, joint relation point)?

**Prediction registered before the run.** On/off trials about 1/h: Edwards (h = 4) 0.25, Montgomery (h = 8) 0.125, cofactor-1 control exactly 1.000. Mechanism: without the projection only about 1 in h factor-base pairs lies in <G> and can decompose a target; with it every pair does.

**Design.** 72 fresh instances (generator seed `e222m221-h1default-v1`, families E, M, W, generator rungs 12, 16, 20, 8 per cell), solver seeds 1, 2, arms off/on with the same binary and no environment flags: 288 runs, all recovered the planted logarithm. Data: `results/h1_defaults/rows.jsonl`. Runner: `run_h1default.py`.

**Result (paired by instance and seed, on / off; bootstrap 95% CI of the median, n = 48 per family):**

| family | h | trials on/off | prediction | total time on/off | median ms off to on |
|---|---|---|---|---|---|
| E | 4 | 0.277 (0.258 to 0.300) | 0.250 | 0.357 (0.296 to 0.508) | 8.0 to 2.5 |
| M | 8 | 0.153 (0.143 to 0.164) | 0.125 | 0.196 (0.164 to 0.300) | 14.5 to 2.6 |
| W | 1 | 1.000 (exact in all 48 pairs) | 1.000 | 1.065 (1.040 to 1.146) | 2.2 to 2.5 |

**Reading.**
- H1 still helps with the new defaults: the Edwards analogs need 0.28 and the Montgomery analogs 0.15 of the trials, and with the projection on their time (2.5 and 2.6 ms) matches the cofactor-1 control's (2.5 ms at on, 2.2 ms off).
- The prediction is met in direction and approximate size but not exactly: both intervals exclude the predicted 1/h (E 0.258 to 0.300 against 0.250, M 0.143 to 0.164 against 0.125), so the projection recovers a little less than the full factor of h. Not explained here. One candidate (not tested): the torsion classes of the small-x factor-base points are not uniform over E/<G>, so the unprojected fraction of usable pairs is not exactly 1/h, which would shift the ratio. Rung-by-rung ratios in the report scatter between 0.20 and 0.34 (E) and 0.13 and 0.17 (M) with no trend.
- The time ratio is larger than the trial ratio (E 0.357 against 0.277): at these sizes fixed costs (factor-base construction, table build, one [h]F per entry) are a visible part of a solve of a few milliseconds.
- On the cofactor-1 control the flag changes nothing in trials and costs about 6.5% in time (an extra [1]F per entry); it should not be used there.

**Limits.** One producer, toy sizes (generator rungs up to 20 bits, so p up to about 2^24), local host, wall times of a few milliseconds. Not run at native E-222 / M-221 sizes.
