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
