# K13 x86 duration admission probe

Status: frozen before any probe execution. This is an admission check for the
separately frozen [ten-arm K13 panel](../rootfind_f456_batch_combined_20261115/PROTOCOL.md),
not an amendment to its targets, comparisons, or decision rule. The probe
does not decide whether rootfinding outruns F4, F5, or F6-IC.

## Question and exact workload

Can the existing `rootfind-negfold-batch` m=3 complete solve earn ecbench L2
wall admission on the intended x86 host? `grade_v3` requires
`time.solve_wall_ns >= 1,000,000` in addition to quiet-host conditions.
The local M4 Pro K13 repeat has solve durations below that floor; its L0
measurements do not predict x86 duration.

`SPEC-K13-ADMISSION.json` copies the registered Koblitz E0 n13 curve,
public target seed `202611151301`, all eight target indices, standard-subspace
dimension five, `max_trials=200000`, algorithm seed `202611151302`, and
batch m=3 method from the frozen panel. Two identically configured arms,
`batch-m3` and `batch-m3-aa`, run in alternating order. Each has one warmup
and two measured rounds: 8 targets × 2 arms × 3 rounds = 48 fresh processes,
32 measured. Do not change the binary, source commit, image, placement,
thread count, or ecbench grading version between this probe and the full
panel. If any changes, repeat the probe before drawing an admission inference.

## Controls and records

Use one source-bound release `ecbench` binary on the native Iso Lab EKS
`cpu-l2-spot` runner, `c8a.2xlarge`, four reserved CPUs and four threads.
The two arms are an A/A control. Retain every warmup and measured process,
including errors, timeouts, and L0/L1 grades. Each answer must verify
`[d]G=Q`; replay every recorded execution on an independent host class before
using the probe as a source-binding or correctness check. Require exact
registered curve ID, factor-base digest, target point, workload hash, source
and binary hash within each target-round pair. The measure is the child's
`time.solve_wall_ns` and the recorded `isolation.level` and blockers;
complete cold `time.process_wall_ns`, phases, peak RSS and A/A ratios are
diagnostics. A null population is irrelevant to this fixed-instance gate.

The probe passes its narrow admission screen only if all 32 measured rows
verify, have complete phase and process clocks, are L2 or L3, and each solve
clock is at least 1 ms. An A/A anomaly is reported, not silently discarded;
the full panel's A/A interval gate remains independent. A solve under 1 ms
or a sub-L2 grade means the frozen panel cannot satisfy its all-pairs rule
on that source, host and workload observation. A timeout, wrong answer, or
missing identity invalidates this screen and calls for investigation.
Passing 32 rows makes the full panel eligible for submission, not assured
of admission or of a performance result. No cross-host timing ratio is used.

## Execution and stopping

One fixed cell only: 48 executions, each capped at 30 seconds, with a
600-second job watchdog; any watchdog stop retains partial records and is
inconclusive. No automatic retry. A fresh infrastructure-only retry requires
its own immutable record. A USD 5 new-job ceiling includes image build,
queue, failed execution, storage and transfer. Before any new image or job,
re-read both daily and monthly headroom, including reserved spend, on account
`104186811093`; require at least USD 5 available in each. Reconcile any
existing request or Job for this spec and source. The currently exhausted
budget blocks launch. The branch-specific `.resume/RESUME.md` and
`scripts/checkpoint.sh gate` must pass before any trial; Conductor T-6814
currently holds that file. Neither gate can be bypassed.

Publish committed source, spec and protocol with an exact
`check-experiment-git` receipt before launch. Bind OCI image, release binary
SHA-256, ecbench grading version, CPU features, run CPU and telemetry. Archive
exact command, UTC, source and dirty state, parameters, seeds, stdout/stderr,
raw records, `verify --replay-all`, grade and identity audit, A/A and censored
rows, resource use, graph, report and inspected PDF. A claim-changing review
needs a separate independent reviewer and machine-readable claim record.

The four archived L0 source-bound K13 batch durations and the separate
32-row local repeat are diagnostic inputs only. The frozen K13 ten-arm panel
remains the comparison test. K19 would require its own same-source,
same-target panel; archived K19 matrix timeouts are bounds, not ratios.
