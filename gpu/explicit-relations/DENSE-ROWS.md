# Exact twelve-byte rows (device validation pending)

`make cuda-dense ARCH=sm_120` selects the experimental `ER_DENSE_ROWS`
variant with the existing wide prefix index. A row stores the exact64-bit
point key as two32-bit halves and a32-bit original tuple ID. `sizeof` is12
and alignment4, statically asserted. There is no hash or probabilistic match.
The key comparator still sorts by full key then original ID, so lower-bound
lookup returns the minimum duplicate tuple. UINT32_MAX is a valid ID; missing
results still use widened UINT64_MAX. Preparation rejects left_rows>2^32.

The existing16-byte row remains the ordinary wide-index build. Three12-byte
buffers charge36bytes per left tuple, versus48bytes for three16-byte buffers;
this is an exact allocation-formula difference, not measured peak VRAM or a
speed prediction. Prefix offsets, query storage, other allocations and the
25%free-memory reserve remain charged. Full construction/sorting/setup stays
inside cold solve time. Multiple32-bit loads could cost throughput; the
separately frozen v10 panel compares complete52-bit solves with identical
CPU driver, index width, targets and caps, then probes both61/62-bit targets.

Tests exhaust the valid points of a small curve against full-coordinate
lookup, including both signs, sparse misses, duplicate IDs and sole
UINT32_MAX witnesses. Dense mode also runs the2^24-offset boundary test,
repeated prepare/query checks, and a >2^32 rejection before any large
allocation. Memcheck, racecheck and synccheck run before the new data.

Literature: CUDA13.0 programming guide, global-memory size/alignment rules:
https://docs.nvidia.com/cuda/archive/13.0.0/cuda-c-programming-guide/index.html#device-memory-accesses
Unaligned8-byte loads can be incorrect; this representation uses naturally
aligned32-bit members, not a packed unaligned64-bit field. SEC1 section2.3.3
supports the existing exact x/parity point key. No algorithmic novelty,
certified speedup, hardware maximum, or tensor contraction is claimed.


## Optional queried sort allocation

`make cuda-dense-cub ARCH=sm_120` additionally selects `ER_CUB_SORT`. It uses
CUB DeviceMergeSort with an explicit int64 count and the same full-key/ID
comparator. Before allocating a table it queries the library for all required
scratch bytes, including partition and virtual shared-memory storage, then
charges retained rows plus that scratch and all other allocations against
the existing75%free-memory admission threshold. Scratch is allocated with
cudaMalloc, synchronized and released before prefix construction/queries.
The ordinary Thrust variants retain their three-buffer admission formula.

The API contract and allocation implementation were checked in NVIDIA CCCL3.2:
https://github.com/NVIDIA/cccl/blob/v3.2.0/cub/cub/device/device_merge_sort.cuh
and the adjacent dispatch/dispatch_merge_sort.cuh. Large-count sizing tests
cross INT_MAX and2^32 without allocating those tables. Existing small actual
sort/query differentials, duplicate-ID checks and sanitizers exercise the
backend. GPU validation remains pending until the separately frozenv11 run.

V10 was never executed. V11 retains separate52-bit row-format and sort-backend
controls in a four-arm panel, then probes both61/62-bit targets once. This
preserves each comparison while avoiding a duplicate larger-field panel.
