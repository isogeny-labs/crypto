# K13 local collector diagnostic, before data

Status: frozen exploratory protocol. This panel uses fresh public targets and
does not alter or replace `../rootfind_f456_batch_combined_20261115/`.

## Question and hypothesis

On the registered Koblitz E0 curve over `F_(2^13)`
(`icv1-f2m13-t181-515ee569`, subgroup order 2003), which complete cold
phase currently limits `rootfind-negfold-batch:m=3` against F6-IC on the same
target? The proposed next mechanism is a direct abscissa-indexed pair table
for this small binary field. It may reduce hash probes, but its setup and
memory could erase any gain. The present panel measures the unmodified
candidate first; it does not test the proposed dense table.

## Frozen inputs and controls

`SPEC-K13-LOCAL.json` has SHA-256
`29073e20aebe7854b0e81e4a7f96ea261d111ff907494a1d7abba4ad9fc58ffc`
and canonical `ecbench` spec ID `ECS1hb376719703c9`. It fixes eight public
targets from seed `202611160401`, four measured rounds, one warmup, alternating
order, and algorithm seed `202611160402`: 200 executions in total. These are
disjoint from the confirmatory panel's target seed `202611151301`. All IC
arms use the identical dimension-five standard-subspace factor base and
`max_trials=200000`. The five arms are strong rho, F6-IC m=3, scalar
rootfinding m=3, batched rootfinding m=3, and an independently executed
batched A/A control. Parent Rust source is commit
`6ebf026274395b5de50ddbc8d0523a72ba016a7b`; record the built binary
hash and the actual run commit before execution.

Every answer must pass `[d]G=Q` under `ecbench verify --replay-all`. Record
all execution failures, timeouts, missing phases, workload IDs, source and
binary identities, memory, host and isolation. The A/A arm measures local
noise; its source and method parameters match the candidate. No previous
target or host's timings are pooled into these pairs.

## Measures and decision boundary

Primary diagnostic is per-execution complete cold `time.process_wall_ns`,
including setup, relation search, rank, descent, recovery and verification.
Secondary diagnostics are phase wall totals, table construction and probe
counters, `ecbench work` completeness, and peak RSS. Keep `bitop/v1` incomplete
where native work is unpriced. Report each arm's 32 measured executions,
paired batch/F6 and batch/scalar ratios, target range and A/A spread, with
failed pairs retained. Derive ratios only from exact matching target-round
pairs with valid answers and complete walls. These local L0 wall observations
are an engineering screen, not an isolated speed claim or the fixed-panel
verdict. A faster component cannot be promoted to a full-cost advantage.

If batch is consistently slower than F6 while table lookup or setup dominates,
implement a separate dense-table method and measure it on a new held-out target
seed. If batch is faster, retain the frozen EKS panel as the confirmatory test.
If output is incomplete, preserve it and repair the specific producer defect
before another run. Regardless of outcome, the existing EKS panel remains
necessary for a same-host L2/A/A F4/F5/F6 decision and independent replay.

## Execution and custody

Use one release `ecbench` binary from the frozen source, `--cpus none
--allow-busy --wait` on the shared benchmark lock, and a new output directory.
Each execution has a 120-second cap; stop after the single 200-execution plan.
Do not retry a measurement epoch automatically. Before launch, push a complete
resume checkpoint to S3 and create an open `bench.record/v1` record. Afterward
retain raw records in S3, close the record including failed/censored outcomes,
write a source-linked report, SVG and inspected PDF, and publish a ready PR.
No EKS image or job is part of this local diagnostic.
