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
