# Assessment of the rerun task cards R0-R3 (-v2), 2026-11-01

Scope: classification of how the cards ran, from files on disk only. No scientific
result is interpreted; no improvement is claimed or denied. Written by the
experiment-director tier (assess mode). Branch `research/prime-field-leads-reruns-20261101`,
HEAD 6f27556c5. Originals are in `/home/user/rerun-wt/research/prime_field_leads_20261031/rerun_20261101/`
(detached at bc7b2d085, which equals the `commit` of every card and receipt). All paths
below are relative to `research/prime_field_leads_20261031/rerun_20261101/` unless absolute.

## How the cards were run (applies to all four)

- Each card was run by the main session with `python3 tools/task_card.py run <card>`, not
  relayed to `executor-mechanical` (RERUN_PLAN.md "Host and route"). So there is no
  executor report: `check-relay` was not run, and `relay_verbatim` does not apply. This
  is a route deviation from the usual tier flow, recorded in the plan. It is not an
  `executor_deviated` event (no unlisted command ran, see below).
- Each `receipt.json` is `crypto.autoresearch.task-card-receipt.v1`, `status: completed`.
- The receipt does not store a per-bind digest list. It stores `card_sha256`, and the
  tool checks every bind before running. So "the bound digests equal the card's binds" was
  checked here directly: every card bind was re-hashed against the files in
  `/home/user/rerun-wt` (the pin) and on the branch.
- For all four cards: `card_sha256` in the receipt equals sha256 of the card on the
  branch and of the card in `/home/user/rerun-wt`; the receipt's argv list equals the
  card's argv list, in order; no command has `retry`; `attempts` is 1 for every command;
  every `exit_code` is 0 (in `success_exit_codes: [0]`); `timed_out` false everywhere;
  every recorded `seconds` is below the card's `timeout_seconds`.
- Files written outside `write_scope` (`research/prime_field_leads_20261031/rerun_20261101/`):
  none. The `dirty_after` lists of the four receipts contain no entry outside it that was
  not already in `dirty_before`, and `git status --porcelain` in `/home/user/rerun-wt`
  shows only paths under `rerun_20261101/`. `git status --porcelain` on the branch is clean.
- The receipt `dirty_before` list of R0 shows one path with its first letter missing
  (`esearch/...cards/R0-parity.yaml`). This is a path-parsing artefact of the tool's
  `git status` reading; the card file itself exists and hashes correctly.

## R0 `CARD-primeleads-rerun-20261101-R0-v2` (instrument parity, P1 and P2)

- Receipt: 2 commands, both exit 0. `parity-c2` 1.091 s (timeout 2400), `parity-l2b`
  26.143 s (timeout 1500). Neither timed out.
- Digests: stdout and stderr sha256 in the receipt equal the files beside it for both
  commands (`parity-c2.stdout` 4144 bytes, `parity-l2b.stdout` empty, both stderr empty).
  The originals in `/home/user/rerun-wt` have the same digests.
- Declared output `parity/C2_S/msolve_stage.log` exists; its sha256 (f9414270d18b...)
  equals the receipt's record and equals the `parity-c2` stdout digest. `parity-l2b`
  declares no output; its files `parity/L2b_probes/cells.csv` and `instances.jsonl` exist.
- Binds: 40 in the card. All 40 equal the files at the pin in `/home/user/rerun-wt`.
  On the branch 39 equal; `RERUN_PLAN.md` does not (see finding 1).
- Branch copy vs original: `parity/` is byte-identical except that `parity/L2b_probes/work/`
  (12 files: .ms, .out, .row, .log.gz of the three probe cells) was not copied.
- Classification: `executed_as_specified` at the level of the tool (every command ran
  with exit 0, no timeout, digests match).
- Mechanical equality observations (not an interpretation of what they mean): the 5 compared
  columns (rounds, max_deg, max_rows, max_cols, sum_cells) of `parity/C2_S/msolve_summary.csv`
  equal `../C2_volcano/S/msolve_summary.csv` on all 32 cells, all with status `ok`.
  `parity/L2b_probes/cells.csv` shows three probe rows identical to
  `../L2b_dreg_trajectory/results/probe_cells.csv` (diff of the three lines is empty).
  The plan says P1 and P2 are tested with `-l 1` (A5).

### The first R0 attempt (`attempt1_msolve065_l2/`)

- Card `CARD-primeleads-rerun-20261101-R0` (no `-v2`), pinned to 122535fb3 (the card
  files of that attempt are in git history, commit 4d1b7eebd, not in `cards/` now).
  Its receipt (`attempt1_msolve065_l2/receipts/.../receipt.json`) has `status: completed`,
  both commands exit 0 (0.092 s and 22.131 s), and its stdout/stderr digests equal the
  files beside it. So at tool level it looks successful.
- It failed on the content: in `attempt1_msolve065_l2/parity/msolve_summary.csv` all 32
  cells have status `exitNone`, rounds 1 (max_deg 20, max_rows 100, max_cols 725,
  sum_cells 72500), i.e. none reproduced the committed values. The plan (RERUN_PLAN.md A5
  and RESUME.md) gives the reason: msolve 0.6.5 (Ubuntu `0.6.5-1build2`) segfaults in
  its first F4 round with the default sparse kernel (`-l 2`, and also `-l 44`) on
  characteristic 105037; the L2b probe cells (characteristics below 2^16) passed on every
  column (and `attempt1.../L2b_probes_cells.csv` is identical to the v2 probe file). The fix
  adopted before any stage 1 to 3 data was A5: pass `-l 1` to every msolve call, and
  repeat P1 and P2 in full. The segfault itself is the plan's statement; this assessment
  verified only the `exitNone` / `rounds=1` pattern on disk, not the crash cause.
- Lesson for a reader: exit 0 and `status: completed` on a receipt do not by themselves
  establish a parity pass; the stage log and summary CSV must be read.

## R1 `CARD-primeleads-rerun-20261101-R1-v2` (L2b ladder)

- Receipt: 1 command `l2b`, exit 0, 2827.484 s (timeout 18000), not timed out, 1 attempt.
- Digests: stdout (984 bytes) and stderr (empty) digests equal the files beside the
  receipt, and equal the originals. No outputs are declared in the card.
- Binds: 8 in the card; 7 equal on the branch, `RERUN_PLAN.md` differs on the branch only
  (finding 1); all 8 equal at the pin.
- Branch copy vs original: `L2b/results/` (cells.csv, instances.jsonl, summary.csv) is
  byte-identical to the original; `L2b/work/` (280 files) also shows no difference under `diff -rq`. `L2b/dreg_vs_log2p.svg` exists only on the branch (finding 3).
- Stage content that a reader should know (counts only): `L2b/results/cells.csv` holds 70
  cell rows: 68 with status `ok`, 2 with status `timeout_1500s`; no `rss_cap_6GiB` row.
  The stdout summary lists the two timed-out cells as p = 1048573 (ctrl and real, n_ok 0 of 1).
  The card still exits 0 in that case; per RERUN_PLAN.md A2 a timed-out cell counts as not completed.
- Classification: `executed_as_specified` (tool level). Any ratio or decision is not assessed.

## R2 `CARD-primeleads-rerun-20261101-R2-v2` (C2 on M, msolve leg)

- Receipt: 1 command `c2m`, exit 0, 446.771 s (timeout 16200), not timed out, 1 attempt.
- Digests: stdout (3952 bytes) and empty stderr equal the receipt's, and the originals.
  Declared output `C2_M/msolve_stage.log` exists, sha256 eeb0e78fa09a... equal to the
  receipt's record and to the stdout digest.
- Binds: 37 in the card; 36 equal on the branch, `RERUN_PLAN.md` differs (finding 1);
  all 37 equal at the pin.
- Branch copy vs original: `C2_M/` identical (msolve_stage.log, msolve_summary.csv,
  msolve_trajectories.csv, inputs.sha256, msolve/ with 64 files), except
  `C2_M/graph_msolve_trajectory.svg`, which exists only on the branch (finding 3).
- Stage content (counts only): `C2_M/msolve_summary.csv` has 32 cells, all status `ok`.
  This is 32 cells, not the 16 named by the frozen design and A3; this is recorded as
  Deviation D1 in the branch copy of RERUN_PLAN.md, written at 20:07Z during the stage
  and not part of the bound plan (finding 1).
- Classification: `executed_as_specified` for the card as compiled (it ran the stage
  script the card names, with the inputs the script copies). The card/plan mismatch (32
  vs 16 cells) is a specification issue that D1 records, not an execution deviation.

## R3 `CARD-primeleads-rerun-20261101-R3-v2` (L2a ladder s = 6..12 and fit)

- Receipt: 2 commands, in the card's order. `l2a` exit 0, 2316.098 s (timeout 21600);
  `l2a-fit` exit 0, 0.069 s (timeout 600). No timeouts, 1 attempt each.
- Digests: both commands' stdout and stderr are empty (sha256 e3b0c442...), equal to the
  receipt and to the files beside it. Declared outputs `L2a/run.log` (3d501c0ef2f0...) and
  `L2a/fit.log` (empty, e3b0c442...) exist and equal the receipt's records.
  The branch copies equal the originals.
- Binds: 6 in the card; 5 equal on the branch, `RERUN_PLAN.md` differs (finding 1); all 6
  equal at the pin.
- Branch copy vs original: `L2a/` (attempts.csv 1633 lines, results/{fit.csv,
  per_s_means.csv, L2a_work_vs_s.svg}, run.log, fit.log) is byte-identical.
- Stage content (counts only): `attempts.csv` has no `TIMEOUT_20MIN` rows; `run.log` shows
  per-arm lines "done ... (stop guard only)". `fit.log` is empty, yet `results/fit.csv`
  exists with 9 rows; the 0.069-s fit step wrote it.
- Mechanical equality observation (P3 of the plan): for the 45 rows of
  `../L2a_slope_e_m3/results/per_s_means.csv` (s = 6..11), `mean_work` in the rerun
  `L2a/results/per_s_means.csv` (51 rows, s = 6..12) is textually equal in all 45.
- Classification: `executed_as_specified` (tool level).

## Binary hashes now vs `binaries.sha256`

`sha256sum -c binaries.sha256` reports OK for all six, taken now (after the last stage):
l2b-gen 661ec87f..., rsswatch 717a3280..., c2_volcano 3e357b07..., l2a 5a8e442d...,
l2a_stats 5b8733b8..., /usr/bin/msolve c2722288.... They equal the recorded hashes;
this shows no change between recording and now, but cannot show the binaries were
the same at each earlier run moment. `binaries.sha256` (and `msolve_version.txt`) are
bound by R0 and equal their bound digests.

## Findings

1. `RERUN_PLAN.md` on the branch differs from the digest bound by all four cards. The
   copy at the pin (in `/home/user/rerun-wt`) equals the bound digest for every card, so
   every run saw the bound plan. The branch copy adds only Deviation D1 (diff: 8 added
   lines, nothing removed). Anyone running `task_card.py lint` on the branch will see
   those four binds fail; this is by design of the post-hoc addition and should be
   stated in the report.
2. Every card ran with exit 0 and no timeout, so none stopped. Exit 0 does not cover the
   content: L2b has two `timeout_1500s` cells (p = 1048573); attempt 1 of R0 had exit 0
   with all cells failed.
3. Branch-only files with no counterpart in `/home/user/rerun-wt`: `receipts/chain.log`
   (the chain driver's log for R1-R3; its three receipt sha256s and its card sha256s equal
   the files; R0 is not in it), `plot_c2m.py`, `C2_M/graph_msolve_trajectory.svg`,
   `L2b/dreg_vs_log2p.svg`. They were produced after the receipts and are not outputs of
   any card; I did not check their content.
4. `parity/L2b_probes/work/` (12 files) exists only in `/home/user/rerun-wt`. `L2b/work/` (280 files,
   including a `logs/` subdirectory) was outside the stated comparison but `diff -rq` against the
   original reports no difference.
5. R2's 32 cells versus the 16 of the frozen design is a card-vs-spec mismatch recorded
   as D1 before reading stage results; flagged here so the reader does not take it for an
   executor error.
6. Route: all four cards were run by the main session, not by `executor-mechanical`, and
   no executor report exists; the judgment-free separation therefore rests on the
   receipts alone. Recommendation for all four: `proceed_to_analysis`
   (outcome `executed_as_specified` at tool level; `interpretation: not_assessed`).
