# Uninstrumented full-cache queries: complete recovery comparison

The previous stage profile identified normalization and lookup as the largest
named intervals. Its staged kernel also specialized the full-cache path, so its
event-time ratio mixed instrumentation with changed code generation. This study
isolates a production kernel without clocks, fences or extra barriers.

Baseline is the unchanged generic persistent affine-query path. Candidate differs
only by `ER_QUERY_FULL_CACHE`: direct full-cache access and conversion reuse.
Both retain identical tables, negative filter, arithmetic, indices and winner
resolution. The same baseline binary also supplies the A/A repeat.

Use the preceding affine study's deterministic 52/63-bit public fixtures and
seeds 101/202, with unchanged limits. Five interleaved rounds at 52 bits and one
round at 63 bits yield 36 planned complete-recovery executions. Orders alternate
b/v/x and x/v/b across round and seed. Every successful cell needs fresh CPU
certificate replay, scalar+1 rejection and byte-identical certificates across
all arms/rounds. Complete the attempted rung before stopping after a failed
rung. Preserve all failed, limited and timed-out attempts.

The existing five-hour worker cap admits the original large workloads plus the
repeated small controls; it does not admit five repetitions of the large panel.
Large-case ratios remain single-round observations. All wall clocks are L0
RunPod diagnostics, not a population timing claim. Cold clocks include all setup,
relation queries, linear algebra and in-process verification; external replay is
recorded separately. No profiler is attached to any measurement.

Sources reviewed before implementation: previous affine study and
`explicit_relations_profile_20261031/RESULTS.md`; `device.cuh`,
`query-profile.cuh`, `curve.cuh`, and their exact differential fixtures;
[NVIDIA CUDA Best Practices](https://docs.nvidia.com/cuda/cuda-c-best-practices-guide/index.html),
assessment, realistic workloads and invariant data reuse. Existing algorithms
are reused; no algorithmic novelty or fastest-external-tool claim is tested.

One full RTX 4090, at most $0.90/hour, worker 18,000 seconds and controller
18,300 seconds. Local work only builds standalone small helpers; CUDA and Rust
driver validation/builds run on the GPU worker. The small and large strata have separate canonical protocols to preserve their
true replicate counts. Both source and frozen protocols
are committed, pushed and checkpointed before launch. The controller deletes
only its owned pod and verifies deletion, including failure/deadline paths.

Version1 stopped before measurement because the standalone driver's lockfile
lacked the newly required sha2 dependency from main. `results-v1/failure.json`
retains the failure, final archive and verified pod deletion. Version2 repairs
the lockfile, preserves the kernel, analyzer and all workloads, and subtracts
180seconds (rounded up from150.841seconds) from the original worker allowance:
17,820seconds worker /18,120seconds controller. Its separate frozen protocols
are `protocol-v2.json` and `protocol-small-v2.json`; no version1 cell was timed.

Version2 passed the full GPU preflight and completed33 verified recoveries:
all30 small cells and the three-arm seed101 large comparison. RunPod recorded
pod deletion at06:38:03UTC during63-202-0-x, before the controller's cleanup.
The latest partial archive is retained; the interrupted attempt has no complete
clock, and seed202 armsv/b were never started. `results-v2/interruption.json`
and both canonical exports preserve that accounting. GPU deletion is verified.

The original frozen whole-panel analyzer remains unchanged and correctly
rejects incomplete coverage. `analyze-retained.rs` is a separately labelled,
unblinded post-interruption descriptive adapter: it preserves the original
formulas and exact completed-cell checks, reports every complete matched set,
and keeps the interrupted/unstarted slots outside timing calculations.
`retained-analysis-v2/analysis-custody.json` identifies its source and outputs.
No new measurement or replacement GPU is authorized by that analysis artifact.

Version3 (`protocol-v3.json`) was frozen after the owner authorized completing
the remaining second input within the original budget. `ladder-v3.rs`,
`analyze-v3.rs` and `run-v3.sh` select only63-bit seed202 in the original x/v/b
order, preserving the GPU/driver source, limits, preflight and exact checks.
All three arms must run on one fresh RTX4090; old-host clocks are never pooled
or substituted. Worker cap9000s, controller9300s, one GPU<=USD0.90/hour.

The first version3 submission was refused with HTTP402 (account balance too
low). No pod was created and no trial or preflight ran; zero GPU time was
allocated. `launch-refused-v3.json` records the refusal and reconciled inventory.
The frozen protocol and verified S3 source archive are ready for a submission
after account funds are restored. No version3 performance result exists yet.
