# Isolated confirmation of `batch_trials` on the native IC pipeline (PROPOSED)

**Status: PROPOSED. Not frozen, not approved, nothing has been run.** This file
becomes a frozen protocol only when the owner approves it and a commit says so.
Merging this PR does not freeze it and does not authorize a run. Until then no
job is submitted to Iso Lab and no data is taken. Loading the isolab-eks skill
does not authorize a run (AGENTS.md §9a).

## Question

Round 0026 (`research/ic_candidate_tournament_20260915/runs/round-0026/REPORT.md`)
found `batch16` at 0.621x the incumbent `batch64` in complete cold Valgrind
instruction reads, but its wall-time condition is **unqualified**: the hosted
runner recorded no core reservation or contention receipt. This round asks the
open part only: **under a core reservation (ecbench L2, with replay and A/A
admission), does `batch_trials = 16` cost less wall time than `batch_trials = 64`
for the same pair-table pipeline, and where in `batch_trials` is wall time
smallest?**

The counted-unit screen (`research/ecbench_batched_screen_20261101`, merged)
could not answer this. Counted GAE fell monotonically as `batch_trials` shrank
to 1 on all four cells, and counted GAE does not price the fixed cost of a solve
attempt, field arithmetic or wall time. Batch 1 makes 2.1-4.1 solve attempts per
run where batch 64 makes one, so wall time may disagree with the counted unit.
That disagreement is what this round measures.

## Instrument

`ic.koblitz_batched` (PR #3134, merged): the tournament worker's pair-table
pipeline, pinned to the real worker's counts on four round-0026 cells. Only
`batch_trials` differs between arms; `fb_seed` 43, `max_trials` 4096,
`summands` 3 are the worker's defaults. Binary: built from the source commit
frozen at approval (a 40-character revision, to be filled in then).

## Design

| Item | Value |
|:--|:--|
| Cells (Koblitz) | a=0 n=13 (78 pts), a=1 n=17 (102), a=0 n=19 (114), a=0 n=23 (138), **holdout** a=1 n=19 (114) |
| Arms (per cell) | `ic-b1`, `ic-b2`, `ic-b4`, `ic-b8`, `ic-b16`, `ic-b64` (baseline), `ic-b64-aa` (A/A control) |
| Targets | 32 public targets per curve |
| Target seeds | 202611031001-202611031005 (fresh; the screen used 202611011001-1004) |
| Measurement | 4 rounds after 1 warmup, order `alternate_measured`, seed 202611032001 (the screen used 202611012001) |
| Isolation | `isolation_required: L2`; native L2/replay/A/A admission before any timing counts |
| Specs | `specs/<cell>.json`, written by `make_specs.sh`; all five pass `ecbench plan` |

`batch32`, `batch128` and `batch256` are dropped: the screen and round 0026
already place them above `batch16` and the question is the low end. The holdout
`n19a1` is round 0026's holdout cell; its targets here are fresh, so none of
round 0026's points or the screen's points are reused.

## Endpoints

Primary (per cell, from the paired bootstrap of `ecbench compare --a ic-b64
--b ic-b16`): L2 wall-time ratio `ic-b16 / ic-b64` with its 95% interval.
Secondary: the same ratio for `ic-b1`, `ic-b2`, `ic-b4`, `ic-b8`; the counted-GAE
ratio for every arm (a consistency check against the screen); the arm with the
smallest wall time per cell.

## Predictions (recorded before data; any may be wrong)

- **W1.** `ic-b16 / ic-b64` wall time has its upper 95% bound below 1 on all five
  cells (the direction round 0026 saw in instruction reads).
- **W2.** The smallest-wall-time arm lies in {4, 8, 16} on at least four of five
  cells. The counted unit put the minimum at batch 1 on every cell; I expect the
  extra solve attempts of small batches to cost real time the unit does not see,
  but I have no measurement behind this.
- **W3.** Counted-GAE ratios for `ic-b16`, `ic-b1` and `ic-b64-aa` match the
  screen's within the screen's 95% intervals on the four shared cells (fresh
  targets, same distribution; counted costs are deterministic per target).

## Controls and validity

- `ic-b64-aa` against `ic-b64` is the A/A control. The lab's A/A admission
  decides whether a run's timing counts; I do not set a numeric tolerance here
  because the lab defines it. A counted-GAE A/A ratio outside 0.999-1.001 kills
  the round (the screen's gate).
- Every record must be `VERIFIED` and `ecbench verify --replay-all` must report 0
  problems; a session that does not verify is a failure to report, not to retry
  with the same seeds.
- A Spot interruption invalidates the interrupted timing interval; the round
  restarts the benchmark epoch with fresh warmup, L2 checks and A/A (isolab
  recovery policy), and says so in the report. No timing is stitched across epochs.
- Seeds and cells are not changed after data is seen.

## What this can and cannot claim

It can say whether `batch16` is faster than `batch64` in L2 wall time on these
five Koblitz cells for this pipeline, and what the wall-time curve looks like for
`batch_trials` 1-64. It cannot say anything about security, rho, scaling in `n`,
or other curve families, and it is not a leaderboard row: the validation gate
needs a replay certificate, a validator report and a matched rho reference
(`docs/VALIDATION_GATE.md`), none of which a within-method comparison provides.
Counted GAE is reported next to wall time; neither is converted into the other.

## Not yet decided (blocks a run)

1. **Owner approval** of this design, its arms, cells and predictions.
2. **Submission route and launcher argv.** The ecbench route is
   `isolab_k8s_benchmark`; its tools are not connected in the session that wrote
   this, so the job object, launcher argv, non-secret environment, receipt
   directory and timeout are **not frozen** here. The director must freeze them
   from the live tool schema (or the AWS-bridge `isolab.eks.request/v1`) and
   verify source commit, image digest and S3 hashes (`isolab-eks` skill).
3. **Source commit and image.** A verified image built from the frozen commit.
4. **Budget and cap.** Per `taskq/deploy/eks-karpenter/COST_CONTROLS.md`; to be
   stated by the director with the card. The screen's local L0 cost (about two
   minutes of wall time per cell) is not a guide to the L2 cost, which includes
   warmup, admission and A/A epochs.
5. **Resume checkpoint** (`scripts/checkpoint.sh init`) filled before launch.

## After the run

Report, graph and PDF per the research-visuals skill, in this directory; the
`benchmarks/` entry and leaderboard are not touched by this round (no validated
row). Records over 1 MiB go to the artifact store, with the pointers committed.
