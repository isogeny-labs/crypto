# Matched affine-query recovery comparison

Completed V1: all 12 recoveries verified on one RTX 4090. See [results](RESULTS.md)
for the per-input timings, matched controls and complete evidence. The large-target
effect was mixed, so affine subtraction remains opt-in.

Specialize the full-cache point subtraction while preserving all inputs,
ordinary affine storage, hashing, table layouts, normalization, search minima,
resource limits and certificate semantics. See
[`AFFINE-QUERY.md`](../../gpu/explicit-relations/AFFINE-QUERY.md) for the
primary EFD source and exceptional cases. V21 completed the same two63-bit
public targets; the new comparison reruns baseline and candidate on one host.

The frozen plan has12 executions:52/63 bits, seeds101/202, arms b (mixed),
v (affine) and x (baseline repeat). Order b/v/x then x/v/b at each size.
All use12 slots, tau=ceil(7*ceil(r^(1/12))/5)+1, query multiplier128,
max16 trials, rows2^32, work2^34, chunk2^20, columns16384 and25%reserve.
Every successful execution must have full cache, a fresh CPU certificate
replay and scalar+1 rejection. Complete certificate bytes must agree across
both formulas and the repeat; relation/query counts must agree in analysis.

Run `bash research/explicit_relations_affine_20261030/run.sh` on the pinned
CUDA13.2 RTX4090 image with EPHEMERAL_OUT set. Full GPU preflight precedes data:
29 Rust tests; original differential/query/sanitizer checks;37,584 independent
arithmetic cases on CPU and CUDA; affine packed/unpacked/persistent variants
under memcheck/racecheck/synccheck; full service verification and cap headers.
Host arithmetic fixture and native runner/analyzer compile checks can run
locally; the root crypto plug-in is not built locally under the disk constraint.

Each successful cell charges cold startup, bases, relation queries, elimination,
extraction and internal replay. The outer audit adds fresh replay and negative
control. All clocks remain L0 and wall_valid=false. `ratios.csv` describes
candidate/mean(b,x) and abs(b-x)/mean(b,x) separately for each fixed input;
no population inference or cross-panel timing comparison. Only verified
complete cells enter ratios; all limits/timeouts stay in rows.csv.

Per-solve180s at52bits and7200s at63bits; worker18000/controller18300s.
At the previous observed baseline, all12 cells plus preflight are expected
within five hours, but the worker deadline still applies. Finish the current
rung and stop larger rungs after an incomplete cell. No adaptive caps or
inputs. Periodic60s artifact copies preserve evidence, not solver restart.
One task GPU only, maximumUSD0.90/hour, delete after archive collection.

## Handoff — 2026-10-30

What was done: implemented opt-in formula and fixtures;37,584 host cases pass.
Native panel and analyzer compile; shell syntax and protocol schema checked.
Decisions: formula is the sole varied factor; baseline repeat at both sizes.
Open threads: freeze source, run CUDA preflight and panel, publish all raw
outcomes and inspected report, update the existing PR if still open.
Errors: draft protocol initially lacked required freeze metadata; corrected
before freeze/data. No CUDA result exists at source publication time.
