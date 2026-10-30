# Query transfer optimization

This is a new fixed panel, designed after the normalization panel in
`../explicit_relations_perf_20261030`. No previous measurements are pooled.
The previous query medians motivated reducing repeated query transfers.

All arms retain product-tree normalization, the same four-query workloads,
three sizes, cap 1048576 and explicit chunk size 4096. No user cap is enlarged.
`u` retains the separate match array and host reset upload; `v` uses a radix
winner and device-side reset; `w` isolates packing with the old host reset;
`x` invokes the exact `u` executable again. The native host implementation
checks every answer. This is generic decomposition, not algebraic progress.

The radix is the left table size. A key is `local_right * left_size + left`.
The greatest possible key is `count * left_size - 1`. Packing is admitted only
when `count <= UINT64_MAX / left_size`, reserving UINT64_MAX as the sentinel.
This preserves right-then-left order without assuming 32-bit indices. The old
array path remains available when packing cannot fit. Both variants preserve
early stopping, visited counts and max-work admission. Tests exercise all
variants, repeated queries/prepares, duplicate sums, partial chunks, empty
plans, integer boundaries and CPU witness equality.

The exact metric is query API payload bytes, derived from observed launched
batch counts and hits: `u/x = 16*batches + 8*hits`, `w = 16*batches`,
`v = 8*batches` for these encodable fixtures. It excludes preparation, bus
framing and device memory writes. The packed path also removes `8*chunk`
bytes of match-buffer allocation; no timing conclusion follows from either
count alone. Query, prepare, cold and unchanged normalization timings are
descriptive L0 diagnostics. There is no certified latency, fastest-tool or
full-paper ECDLP performance claim. No equivalent external relation-query
service is included; CUB normalization is unchanged and is not a query
transfer comparator. The prior CUB comparison remains in the prior study.

The frozen panel is 120 rows: four arms, three cases, one warmup and nine
measured repetitions, interleaved with Latin rotation and reversal. The
native analyzer checks unique cells, exact witnesses including visited
counts, and payload identities. It sees neutral labels; this is automated
label masking, not a claim of human blinding. No extra repeats after viewing
results. Failure invalidates the run and is retained. The 1800-second estimate
is an allocation estimate; round zero supplies the declared preflight.

Sources read before implementation: NVIDIA's [API synchronization behavior](https://docs.nvidia.com/cuda/cuda-runtime-api/api-sync-behavior.html)
and [memory API](https://docs.nvidia.com/cuda/cuda-runtime-api/cuda_runtime_api/group__CUDART__MEMORY.html).
They specify pageable upload staging/synchronization, blocking readback,
asynchronous device memset and default-stream ordering. Reset, search and
readback use that same stream. This reuses established CUDA mechanisms and
claims no novel algorithm.

Run `EPHEMERAL_OUT=/results bash research/explicit_relations_query_20261030/run.sh`
in the pinned CUDA development image on one RunPod RTX PRO 6000. Retain full
raw outputs, source/binary hashes and sanitizers. Worker deadline 2400 seconds,
controller deadline 2700 seconds, maximum observed rate $2.50/hour; delete the
pod on completion or failure. Do not compile the root crypto plug-in locally.
