# GPU relation-query profile

This study diagnoses the two fixed 63-bit affine inputs from
`research/explicit_relations_affine_20261030`. It captures the first eight actual
oracle queries for each seed through a transparent native proxy, retaining the
original bases, table dimensions, cache, filter, limits and query responses.
Capture ends intentionally; it is not a completed recovery. Every replay must
produce the identical complete protocol response stream.

Five interleaved rounds per seed run stock / staged / stock-repeat in one
profiling binary, reversing the stock labels on odd rounds. No new speedup is
being tested. All clocks are L0 diagnostics. The staged kernel preserves the
persistent minimum-ID traversal but inserts compiler fences, block barriers,
and cycle reads. The original kernel remains the stock control. CUDA events
bracket the search kernel only, excluding table setup and winner resolution.

The five bracketed intervals are cache load plus input Montgomery conversion;
affine subtraction; block product-tree inversion; affine reconstruction plus
output conversion; negative filter plus exact index lookup. An empty barrier
and clock interval is also retained, never subtracted. Thread zero accumulates
block cycles and visited tiles/rows. These cycles include warp scheduling and
barrier waits. Summing concurrent blocks does **not** yield GPU elapsed time,
and stage shares do not partition the previous end-to-end oracle seconds.

The profiling code is compiled only with `ER_QUERY_PROFILE`; set
`ER_PROFILE_MODE=stages` to select it. It requires a full right cache. Unset or
`stock` selects the original persistent kernel with event timing. Production
builds contain neither instrumentation nor the profiling kernel. Existing
normalization and affine arithmetic are factored into shared helpers, with
independent arithmetic and kernel differential checks before data.

Before data, run 37,584 arithmetic fixtures, persistent query differentials,
three CUDA sanitizers on the profiling kernel, native driver tests/Clippy and
service/driver known-answer tests. Profile fixtures include deterministic hits,
exhaustive misses, infinity, duplicate residuals, and partial block tails.
`tools/profile.sh probe`, `nsys`, and `ncu` retain unsupported/error results;
there is no attempt to bypass provider counter permissions.

Sources checked before implementation: the preceding affine study and
`AFFINE-QUERY.md`; repository `.agents/skills/profile/SKILL.md`; NVIDIA
[clock semantics](https://docs.nvidia.com/cuda/cuda-programming-guide/05-appendices/cpp-language-support.html#clock-and-clock64)
and [Nsight profiling/replay guide](https://docs.nvidia.com/nsight-compute/ProfilingGuide/index.html).
Device clock intervals include time slicing; Nsight replay/cache controls
perturb execution. No instruction-counter estimate or production stage-time
claim is substituted if hardware profiling is unavailable.
