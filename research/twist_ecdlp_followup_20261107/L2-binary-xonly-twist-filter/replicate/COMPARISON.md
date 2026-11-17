# L2-binary-xonly-twist-filter: mechanical replication comparison

Originals: `research/twist_ecdlp_followup_20261107/L2-binary-xonly-twist-filter` (results.json, results/, RESULTS.md, VERIFICATION.json, untouched).

Replicate runs (all under `replicate/`, each a copy of the directory, outputs not written to originals):

| Frozen command | Run directory | Finished | Status |
|---|---|---|---|
| `./run.sh full` | `rerun_full/` | 21:59 (log ends "target/ deleted") | completed |
| `./run.sh quick` | `rerun_quick/` | 21:42 | completed |
| `./run.sh analyze` | `rerun_analyze/` (copy of rerun_full) | 21:59 | completed |
| `L2SEED_XOR=777 ./run.sh quick` | `rerun_seed777/` | 21:47 | completed |
| `./verify_replay.sh <quick> <seed777>` | run inside `rerun_full/` | 21:59 | completed |

Also compared: the earlier attempt's `./run.sh full` output (`replicate/full/`, deleted after comparison; its run log is in `logs/earlier_attempt_full_run.log`). That process (PID 18889) was still running when this task began; it was not started or killed here. It exited on its own and its log ends "target/ deleted" (21:37).

## Comparison (non-timing numbers only; `wall_s` is the only timing column, dropped; results.json keys containing wall/time/elapsed: none present)

| Replicate | Compared | Result |
|---|---|---|
| fresh full (`rerun_full`) vs original | 32 TSVs, 72,394 non-timing cells | 0 mismatches |
| fresh full, samples | 18 files | byte-identical |
| fresh full, pca | 10 files | byte-identical |
| fresh full, gp_verify.txt | 28 lines | identical |
| fresh full, results.json | 1,290 non-timing leaves | 0 mismatches |
| analyze (`rerun_analyze`) vs original | same as above: 72,394 cells, 1,290 leaves, 18 samples, 10 pca, 28 gp lines | 0 mismatches |
| fresh quick (`rerun_quick`) vs original | 19 TSVs, 39,795 cells; 9 samples; 10 pca | 0 mismatches |
| fresh quick, gp_verify.txt | 19 lines | all 19 lines appear verbatim among the original 28 (quick covers a subset of samples) |
| earlier-attempt full (`full/`) vs original | 32 TSVs, 72,394 cells; 18 samples; 10 pca; 28 gp lines; 1,290 json leaves | 0 mismatches |
| same-seed replay check, `verify_replay.sh` (quick vs fresh full) | 19 TSVs; 9 samples | 19/19 TSVs identical excluding wall_s; 9/9 samples identical |
| verdict rows F1 to F9 (auto_tables.md), fresh full vs original | 9 rows | identical text |

Seed-offset replay (`rerun_seed777`, L2SEED_XOR=777): this is an independent replay with a different seed, so its numbers are not expected to equal the originals and are not counted in the match criterion. Its verdict rows are: F1, F2, F3, F4, F5, F7, F9 yes; F6 NO (1 prime call-ratio cell outside the declared band); F8 NO (sizes 17 and 19 outside [0.8, 1.6]). This is the pattern the original VERIFICATION.json reports for the seed-offset replay. Numeric differences are in the TSV cells (e.g. arms_B_m2_k17 row 1, col 6: 131314 vs 131426).

## Scope of `reproduced`

Compared for the match criterion: fresh full, analyze, fresh quick (as a subset), the same-seed replay, and the earlier-attempt full output. Every such non-timing number matches the originals.

## Notes

- The original `VERIFICATION.json` has a stale `replication` block (`reproduced: false`, "Not yet run"). It was written before these runs finished. It was not edited, since originals must not be overwritten. The fresh replay result is in `logs/replay_VERIFICATION_fresh.json`.
- Full-run stdout is in `logs/rerun_full_run.log`. Quick, seed-offset and analyze logs are in `logs/`. The comparison script is `logs/compare_L2.py`.
- Bulky raw outputs (TSVs, samples, source copies, figures, RESULTS.md) were deleted after comparison to save disk (1.4 GB free at start).
