# Assessment: CARD-fp31-phase3-icyield-20261112-2 (with attempt 1, CARD-...-1)

Written by the experiment director (assess mode). No result is interpreted.

```yaml
card_id: CARD-fp31-phase3-icyield-20261112-2
relay_verbatim: not_checked   # no executor report file was supplied; check-relay not run
outcome: executed_as_specified
stopped_at_step: null
recommendation: proceed_to_analysis
attempt_1:
  card_id: CARD-fp31-phase3-icyield-20261112-1
  outcome: impeded   # lost at command 1 (container_restart); not a completed execution
  recommendation: rerun_after_infrastructure_fix   # done: attempt 2
interpretation: not_assessed
```

## Checks (all against files on disk)

1. Card and receipt. `sha256sum` of the card yaml is aa95102d...9f2 (matches the dispatch). Receipt status `completed`; commands run-phase3, analyze-m3, analyze-m2 all exit 0, `attempts` 1, no timeout. run-phase3 took 3276.62 s. stderr of all three commands is empty (sha e3b0c442...). Receipt stdout digest of run-phase3 (2ff832f5...) and the analysis output digests (analysis_m3.json 776c66e0..., analysis_m2.json 3a3b270d...) equal the sha256 of the files on disk. Receipt `dirty_before` lists only the card yaml; `dirty_after` lists only analysis_m2.json and analysis_m3.json.
2. Bound hashes. `sha256sum -c PROTOCOL.sha256` prints OK for all 8 files (PROTOCOL.md, classes.json, run_phase3.sh, phase3.rs, stats.rs, ecbench, p2tool, fp31-class-census). The same 8 sha256 values appear in the card's `binds`.
3. Attempt 2 stdout: 96 lines, 37 `skip` and 59 `ok`, no `FAILED`, no other line. The 96 (id, m) pairs are distinct and equal the 48 ids of classes.json (c008..c055) x m in {3, 2}. `c045 m=3` is an `ok` line (rerun from scratch).
4. Skips versus attempt 1. The 37 `skip` cells (c008..c044, m=3) are exactly the 37 `ok` lines of cards/attempt1-run-phase3.stdout.snapshot (set equality). The snapshot has 37 lines, all `ok`, and is identical to the attempt 1 card stdout file (diff empty).
5. Cell directories. runs/ holds 96 cell directories plus `_interrupted`. Every one of the 96 has `done`, spec.json, session/, audit.json and analysis.json. No `*.partial` path exists under runs/.
6. Audits. All 96 audit.json: schema ecbench.audit/v1, ok true, session_status complete, problems empty, every `checks` value true, records equal planned_executions, 4 replays each, all `reproduced` true with detail `identical`.
7. Specs. All 96 spec.json: rounds 2, targets_per_curve 2, target_seed 20261112, isolation_required L1, two arms per spec (m*-main, m*-pc), no aa arm, oracle `mitm:m=<m>` equal to the cell's m, and equal to the session/spec.json of the same cell. Factor-base sizes: m=3 main 378 and pc 402 (48 cells); m=2 main 1448 and pc 1586 (48 cells). I regenerated all 96 specs with `p2tool spec` (the frozen arguments of PROTOCOL.md section 4 and run_phase3.sh, output to the scratchpad) and compared them with the stored spec.json, ignoring the `description` and `label` text: 0 of 96 differ. Note: `measurement.seed` is 113 in all 96 specs (set by p2tool spec itself); the `--seed 20261112` argument appears as `target_seed`.
8. Analyses. analysis_m3.json: m 3, classes_analysed 48, 48 distinct class ids, missing []. analysis_m2.json: m 2, classes_analysed 48, 48 distinct ids, missing [].
9. Interrupted cell. runs/_interrupted/c045_m3-attempt1/ holds run.log, session, spec.json, spec.log; it has no `done`, `audit.json` or `analysis.json`. phase3.rs reads `{dir}/analysis.json` and `{dir}/spec.json` per cell, and no analysis class list or path refers to `_interrupted`; the analysis reads runs/c045_m3 (done, from attempt 2).
10. Writes. `git status --porcelain` shows only analysis_m2.json, analysis_m3.json and figures/ (all under phase3/). No file outside phase3/ is newer than the card yaml. The loss record benchmarks/losses/20261112-card-fp31-phase3-icyield-20261112-1-01-run-phase3.json was committed before attempt 2 (cause container_restart, source the attempt 1 receipt). Attempt 1 receipt status is `lost at command 1`.

## Observations and deviations to relay

- phase3/figures/ (class_yield.svg, controls_within.svg, yield_vs_H.svg, plot_phase3.py) is untracked, created at 02:41 UTC, after the run. It is inside write_scope but is produced by no card argv; it came from the main session. It is not evidence of an executor deviation, but it is not part of the card's execution.
- `python3 tools/task_card.py lint` on the attempt 2 card now prints `CARD FAIL` (analyze-m3/analyze-m2 argv name analysis_m3.json and analysis_m2.json, which binds does not pin). These two files did not exist when the card was written (they are listed in `must_not_exist`) and exist now, so this is probably a post-run effect of the lint rule. I did not verify the lint result at compile time.
- `check-relay` was not run: no executor report file was supplied, so `relay_verbatim` is `not_checked`. Run `python3 tools/task_card.py check-relay <card> --report <file>` once the executor's text is saved.
- Receipt `commit` is 01e781cd; HEAD is f0782e9c (the commit that added the card). Not a deviation.

## Classification

- Attempt 1 (CARD-...-1): impeded (lost to a container restart at command 1; infrastructure cause). Not `executed_as_specified`; 37 of 96 cells finished, all `ok`, and are reused by attempt 2 through `skip`.
- Attempt 2 (CARD-...-2): executed_as_specified, recommendation proceed_to_analysis. Every check above passes; relay verbatim is unchecked.
