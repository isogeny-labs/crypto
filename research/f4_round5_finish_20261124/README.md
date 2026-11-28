# F4 leads finished: branch traces, deterministic traces (2026-11-24)

**Standing.** `[pcc:unverified]`. One shared 4-vCPU VM, isolation L0, no PMU. PR #5144; follow-up to #4859 (`research/f4_round4_leads_20261121/`).

## Result

Pre-registered (`CONFIRM_PROTOCOL.md`, frozen at `ea2c575bdd`). `jv_cover --exp dlp --trace`, p = 53, default Hilbert rule, same-binary A/B against `F4_FP_BRANCH=0`:

- **Deterministic.** Counted oracle multiplications were identical in every run of each arm: 7,003,932,907 with branches (20 runs), 7,302,797,284 without (10 runs). Before this change they varied by up to 0.9 % with thread scheduling.
- **Branch traces:** counted S 298.08 → 285.95 (1.042×). 2,531 diverged replays continued on one of 11 branches.
- **Wall:** 1.018 median (8/10), CPU 1.040. That is below the A/A deviation, so not significant.
- **Correctness:** 31 relations and a correct logarithm in all 30 runs.
- **Tests:** `cargo test --release --lib f4_fp`: 142 passed.

Report: `reports/run-20261124T1700Z/`. Record: `benchmarks/records/20261124T1642Z-jv-cover-f4-branch-traces-confirm.json`.

## Changes

- `src/cryptanalysis/f4_fp.rs`:
  - The replay cursor is a (node, base) pair.
  - `TraceStep::branches` holds the recorded branches.
  - `TraceMode::RecordBranch` records a branch; a branch is committed only from a run that ends cleanly.
  - `F4_FP_BRANCH=0` turns branches off; `set_trace_growth` and `branch_totals` are new.
  - Test `diverged_replays_record_and_follow_branches`.
- `src/cryptanalysis/jv_cover.rs`:
  - A sequential warm-up (`JV_TRACE_WARMUP`, default 256 residuals) grows the trace tree, which is read-only after it.
  - Recording and eviction happen only while growth is on.
  - The DLP report gains `branch_follows` and `branch_records`.

## Closed without a change, with measurements

- **Round 4's estimate was wrong.** It put the gain from branches near 30 %; the measured gain is 4.1–4.3 %, even with the whole run as warm-up (`data/pilot_runs.jsonl`). A diverged replay continues from where it diverged; it does not restart.
- **Lazy rebuild positions:** +0.7 % F4 instructions. Reverted in `2b6bc8da76`.
- **AVX2 divisor scan and packed pair data:** under 1 % of F4 each, below the wall noise floor. Not built.

## Open

`--trace` is now deterministic and cuts counted S under Hilbert from 392 to 286. Making it the default for `jv_cover` is an owner decision.
