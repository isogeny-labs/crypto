# Optional negative prefilter — GPU validation pending

`ER_NEGATIVE_FILTER` adds a register-blocked Bloom prefilter before the exact
sorted-table lookup. `make cuda-filter` combines it with dense CUB tables,
persistent queries and the existing full right-cache/fallback policy. It is
opt-in. Frozen v13 and v14 remain pinned to their earlier source commits and
contain no filter. No filter performance panel has been frozen or run.

Each exact point key selects one 64-bit word and up to four bit positions.
Construction atomically ORs those bits for every left row. Queries use the
same function and may skip exact lookup only when a required bit is absent.
All possible matches still use the complete point key and minimum original
left tuple ID. Duplicate keys, infinity and full-width keys use the same
encoding as the exact table. No probabilistic match becomes a witness.

The no-false-negative argument does not depend on the hash distribution:
construction starts with zeros, OR never clears a bit, every row inserts its
own pattern, and construction finishes before any query reads it. Reprepare
clears and reallocates the filter. A null pointer disables filtering. A
saturated filter passes everything through exact lookup; correctness remains
the same even though it saves no work. Device tests explicitly exercise this
case, complete valid-point inventories, duplicate minima, infinity and misses.
Existing late-winner, complete-miss, tail, empty-plan and reprepare checks also
run with filtering enabled.

Word count is the next power of two at least `ceil(left_rows/8)`, at least
8 bits per row and at most 4 GiB for the admitted `left_rows <= 2^32`.
The word index uses the upper 32 hash bits; the four positions use the lower
24 bits. Repeated positions only affect filtering efficiency. No particular
false-positive rate, random-input property or speedup is assumed.

Filter allocation and construction occur after sort scratch is released.
Admission charges the greater sort/filter phase peak, then includes retained
filter bytes when deciding whether the full right cache fits the existing
25% free-memory reserve. `TABLE filter_bytes` records actual filter storage.
It can make a previously admitted right cache fall back; a future comparison
must record this rather than attribute a confounded difference to filtering
alone. All construction, zeroing, lookup and memory costs belong in the cold
pipeline. A filter admission failure is an implementation limit, not proof
of a hardware maximum.

The standalone host filter fixture and host service syntax check pass. CUDA
compilation, device differentials, memcheck/racecheck/synccheck, full recovery
and performance remain pending. The GPU implementation continues to be
generic decomposition, bounded by sqrt(r); filtering is ordinary engineering.

Literature reviewed before implementation on 2026-10-30:

- Jünger et al., *Optimizing Bloom Filters on Modern GPUs*, sections II and IV:
  https://arxiv.org/html/2512.15595v2 . Register/block locality reduces memory
  probes, but actual workload, false positives and construction costs matter.
- NVIDIA cuCollections `bloom_filter.cuh` and `bloom_filter_policy.cuh`:
  https://github.com/NVIDIA/cuCollections . Existing GPU filtering supports
  device-side insertion/lookup and sectorized policies; no novelty or fastest
  implementation claim is made for this small integration. A matched external
  comparison has not run.
- Sebastiano Vigna's public-domain SplitMix finalizer:
  https://prng.di.unimi.it/splitmix.c . The local unsigned mixer uses those
  constants/operations; hash quality affects work, not exactness.
