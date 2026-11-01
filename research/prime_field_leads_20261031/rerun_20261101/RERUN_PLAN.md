# Prime-field leads 20261031: rerun plan (frozen 2026-11-01, before any rerun data)

Finishes the three cells that `../SYNTHESIS.md` and §5 of
`research/notes/ecdlp-general/RESEARCH_PRIME_FIELD_LEADS_20261031.md` left open:
L2a at s = 12, the L2b ladder as frozen in `../L2b_dreg_trajectory/run_ladder.sh`,
and the C2 msolve leg on the M instance. The leads, metrics and decision rules are
those of `../PLAN.md`, `../L2b_dreg_trajectory/PREREG.md` and the frozen section of
`../C2_volcano/REPORT.md`; nothing here moves a threshold. Driver: `run_stage.sh`.

## Host and route

- One cloud container: 4 vCPU x86-64, 15 GiB RAM, Linux, no other workload. msolve
  0.6.5 (Ubuntu package; `msolve_version.txt`), Rust stable; binary hashes in
  `binaries.sha256`.
- Not Iso Lab on EKS (AGENTS.md §9a): the AWS keys in this container are rejected by
  STS (InvalidClientTokenId), and the GitHub-dispatch route accepts only ecbench specs,
  while these workloads are a standalone Rust harness and msolve. Every decision metric
  is counted (field multiplications, F4 degrees and matrix sizes), so the host does
  not enter a decision; the host enters only through the kill caps.
- Resume checkpoint: the S3 push fails (no upload token; the AWS key is unknown to
  S3). This file and `RESUME.md` beside it are committed and pushed to the branch
  before each stage, in place of the S3 checkpoint.

## Stage 0: instrument parity (run first; nothing else is read before it)

The original host ran Homebrew msolve of unrecorded version on macOS arm64.

- **P1 (C2 S instance).** The 32 committed `../C2_volcano/S/msolve/*.ms` files,
  timeout 300 s, 4 threads. Pass: per cell, rounds, max_deg, max_rows, max_cols and
  sum_cells equal to `../C2_volcano/S/msolve_summary.csv`.
- **P2 (L2b probes).** The three probe cells that completed originally (p = 4093,
  16381, 65521; curve and target seed 9; real arm; cap 600 s). Pass: every column of
  `../L2b_dreg_trajectory/results/probe_cells.csv` equal.
- **P3 (L2a).** Stage 3 recomputes s = 6..11 (the harness is seeded). Pass: every
  `mean_work` in the rerun `per_s_means.csv` equal to the committed one for s = 6..11.

If P1 or P2 fails, the msolve versions differ in their F4 schedule. Then every
msolve comparison stays inside the rerun (crater against floor, real against control,
same instrument), no rerun msolve number is put beside an original one, and the
report says so. If P3 fails, the rerun s = 6..12 rows are the ones fitted and the
mismatch is reported.

## Amendments (each with its reason, fixed before data)

- **A1, order.** Stages run one at a time with the whole machine: P1, P2, Stage 1
  (L2b), Stage 2 (C2 M), Stage 3 (L2a). The original run's timeouts came from host
  load (load average 137 to 430); no two stages share the machine.
- **A2, L2b parallelism and memory cap.** `run_ladder.sh` defaults to 8 in parallel
  with a 16 GiB cap per run; this host has 4 vCPU and 15 GiB. The same lines run in
  the same order: the six long cells (k = 20, 18) with 2 in parallel, then the 64
  cells with k <= 16 with 4 in parallel, cap 1500 s (the script's default) and 6 GiB
  per run. A cell stopped by the memory cap is recorded as `rss_cap_6GiB` and counts
  as not completed, exactly as a timeout does. `PREREG.md` allows a shorter cap if it
  is stated; this is it.
- **A3, C2 M timeout.** The frozen timeout was 420 s with comparison on the common
  prefix of F4 rounds. The rerun uses 3600 s on the same 16 committed inputs, so a
  completed trajectory contains the 420-s prefix. The frozen decision rule is
  evaluated twice and both are reported: (a) on the common prefix, as frozen; (b) at
  completion, on every cell that completes within 3600 s.
- **A4, L2a scope.** The full ladder s = 6..12 is rerun, not s = 12 alone, because
  the committed `attempts.csv` sits in the artifact store and cannot be fetched
  without AWS access, and a fit needs per-attempt rows. Arms, seeds, 3 curves × 32
  targets, the 20-minute cell guard and the frozen statistic (`src/stats.rs`: OLS of
  log₂ mean work on s, hierarchical bootstrap, 4000 replicates) are unchanged.
  Targets cut off by the guard are recorded as `TIMEOUT_20MIN_NOT_RUN` and excluded
  from the mean, as the harness does; their count is reported per cell.

- **A5, msolve kernel (added 2026-11-01 after the first parity attempt, before any
  stage-1 to stage-3 data).** The first P1 attempt (card R0 at 122535fb3; receipt and
  outputs kept in `attempt1_msolve065_l2/`) failed on all 32 cells: msolve 0.6.5
  segfaults in its first F4 round with its default exact sparse kernel (`-l 2`) and
  with `-l 44`, on characteristic 105037. P2 (characteristics below 2^16) passed on
  every column. With the exact sparse/dense kernel `-l 1`, the cell crater_0_t1 gave
  168 rounds, maximal degree 20, largest matrix 268 × 1142 and summed cells 7767106,
  equal to the original record, and a reduced basis identical to the original
  `.out`. Building a newer msolve from source was refused by this session's
  permission policy. So every msolve call of the rerun passes `-l 1` (`MSOLVE_LA=1`
  for the C2 harness, `MSOLVE_EXTRA="-l 1"` for `run_cell.sh`; unset, both keep the
  original command), and P1 and P2 are repeated in full with it before stage 1. The
  F4 round table (degree, rows, columns per round) is produced by symbolic
  preprocessing and is the same for both exact kernels; P1 and P2 test that claim on
  35 cells.

- **Deviation D1 (recorded 2026-11-01 at 20:07Z, during stage 2, before reading any
  stage-2 result beyond four cell status lines).** `run_stage.sh c2m` copies all 32
  committed M inputs (models 0 to 7 per side), while A3 above and the frozen C2 design
  name 16 (models 0 to 3; the original stage was called with a model limit of 4). The
  card runs as compiled. The frozen rule is evaluated on the 16 cells of models 0 to 3
  only; the cells of models 4 to 7 are reported beside it as additional observations
  and enter no decision.

## Decision rules (unchanged, copied for the reader)

- **L2a** (`../PLAN.md`): bootstrap interval of e entirely below 0.5 reopens; an
  interval containing 2 closes. Null prediction e ≥ 1.
- **L2b** (`../L2b_dreg_trajectory/PREREG.md`): trajectory ratio of an arm = median
  maximal step degree at the largest p where both arms completed every run, divided
  by the same at p = 1021. Reopens: real ratio < 1.5 while control ratio ≥ 2.
  null_confirmed: the two ratios agree within 25 % relative. Otherwise writeup.
- **C2** (`../C2_volcano/REPORT.md`, frozen section): only the msolve leg is rerun;
  the counted-work leg is unchanged (ratio 0.9998 on M). Reopens if the maximal step
  degree of every crater cell differs from every floor cell by at least 1 in the same
  direction. Otherwise the msolve leg stays not separated.

## What would change a reading

Only L2b can. A flat real-arm trajectory against a growing control reopens catalogue
M008. Expected outcome, stated now: null_confirmed, since both arms carry three
membership polynomials of degree N = round(p^{1/3}), so the maximal step degree is
at least N in both. An L2a interval below 0.5 would be a wrong harness long before it
is a result. C2 is expected to stay not separated.

## Outputs

`parity/`, `L2b/`, `C2_M/`, `L2a/` beside this file, each with its raw logs and CSVs;
`REPORT.md`, graphs and a PDF after the last stage; then the lead reports, the note's
§5 and `../SYNTHESIS.md` are updated in the same PR.
