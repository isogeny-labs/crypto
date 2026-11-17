# Exact full right-sum cache — validated through 62 bits

`make cuda-right-cache ARCH=sm_120` adds an optional full affine right-sum cache
to the persistent query with dense CUB left tables. Each cached sum is exactly
16 bytes: two u64 coordinates, with x=UINT64_MAX reserved for infinity and
therefore disjoint from every supported field coordinate. There is no hash,
lossy compression, square-root reconstruction or change to tuple ordering.

The table is built once per prepared set of bases and reused across relation
targets. A query loads the cached affine point and forms target minus that
point, instead of rebuilding the right tuple. Construction, normalization,
allocation and transfers remain inside cold recovery accounting. The new
option does not change the ordinary backend or promote the earlier partial
cache, whose matched52-bit experiment showed a loss.

Admission preserves the existing free-minus-free/4 threshold. The peak bound
is max(sort-phase bytes, retained table/index/points/results plus full-cache
bytes). The sort scratch has been synchronized and freed before the full
cache is allocated. Existing prefix/partial reservations remain charged;
the optional combined variant can enable both cache macros. The full cache
takes precedence; charged partial tables serve as its fallback. An overflow-safe
capacity test selects the full cache only when it fits both the memory
budget and an explicit byte cap. Otherwise the same exact persistent search
runs uncached. Unexpected CUDA allocation errors remain errors, not hidden
retries. TABLE diagnostics record right_cache_rows and the phase peak bound.

Tests cover exact coordinate/infinity round trips and capacity/overflow edges.
The forced two-block persistent query fixture compares a full cache against
a zero-byte-cap fallback and the CPU oracle, including a minimum at tuple1088,
complete misses, partial tails, repeated calls, empty reprepare and rebuilding.
The full service and query sanitizer suites passed before the v13 panel.
Every prescribed cell recovered and passed fresh CPU certificate replay;
matched52-bit certificates were identical. Both61-bit and both62-bit cells
actually used full right caches. The62-bit cache sizes were2,276,312,640 and
2,759,268,600 rows; admission estimates60,880,922,152 and68,978,885,440bytes
include the phase peak and are not measured peak VRAM. See the
[v13 report](../../research/explicit_relations_e2e_20261030/reports/run-20261030T1757Z/report.pdf).

The completed frozen v13 panel used b=chunked dense CUB, u=persistent dense
CUB, v=persistent plus full right cache, x=identical v. At52bits it required
full caching for every successful v/x control and zero cache rows for b/u.
It then probed both fixed61/62-bit targets with v, explicitly retaining any
fallback, refusal or timeout. V12 was never executed and remains preserved.
V16 next tests filtering with this cache policy held fixed at52bits; larger
cache fallback remains explicit in TABLE records.

Literature: NVIDIA CUDA Best Practices Guide, memory optimizations and device
memory accesses, read2026-10-30 before implementation:
https://docs.nvidia.com/cuda/cuda-c-best-practices-guide/index.html#memory-optimizations
See PERSISTENT-QUERY.md for grid/atomic correctness and DENSE-ROWS.md for CUB
scratch sizing. The cache reuses invariant group sums; construction and memory costs remain
charged in every cold solve.
