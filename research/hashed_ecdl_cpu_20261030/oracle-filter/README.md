# Exact suffix rejection filter

## What was done

Round22 recorded 0.360492/0.483973 ms oracle medians in profiled fixtures 1/2,
the largest individual stages, with 5,567/9,001 probed rows and one preparation
per fixture in the ordinary candidate. This change adds a packed occupancy
bitmap before the existing compressed-point hash lookup. It uses the low key
bits with the same power-of-two mask as the table capacity. A clear bit rejects
the query before full hash mixing and collision probing. A set bit always
continues to the exact table, retaining the first lexicographic suffix ID.

Insertion sets the bit for every key, including duplicate sums and identity.
Therefore an inserted key cannot be rejected by the bitmap. Collisions only
cause extra exact lookups; they cannot accept an absent point. No uniformity
assumption is needed for correctness. Low-bit distribution affects performance.
Returned witnesses still undergo checked summation and complete certificate
replay. Query order and row/work caps are unchanged.

## Decisions made

For S hash slots, bitmap storage is 8*ceil(S/64) bytes (before allocator
capacity), versus S*(8+sizeof(usize)) bytes for slots. Memory accounting includes
the actual bitmap capacity. Allocation uses checked reservation and fails with
a resource-limit error. Empty indexes allocate no bitmap. Hash misses save two
wrapping multiplications plus probes only when the filter bit is clear; all
queries pay the bit test and preparation pays initialization. This is a
performance candidate, not a measured improvement or a 10x result.

Prior work: round22/EXECUTION.md and results/stage-summary.csv identify the
target; src/lib.rs preserves the existing exact index. Bloom, *Space/time
trade-offs in hash coding with allowable errors* (CACM 13(7), 1970), pages
422–423, describes bit-based rejection with a secondary exact test:
https://courses.cs.washington.edu/courses/csep521/21wi/readings/bloom_cacm.pdf
The present one-bit low-key projection does not import the paper's random-hash
error estimates. No novelty claim. The Princeton mirror returned HTTP403;
the Washington primary-paper copy was read successfully.

## Open threads

Measure complete-cost effects with the original baseline and all fixtures in
a separately frozen, authorized panel. Round22 is immutable and its single
execution authorization is consumed. No new cloud experiment was launched.
Tests include all original exact witness/certificate fixtures plus adversarial
bitmap saturation, duplicate IDs, identity, and word-boundary sizes, compared
against independent linear search over the full 199-point fixture group.

## Errors

All 47 standalone release library tests pass, including full archived certificate
parity and the adversarial filter test. Strict all-target release Clippy and
formatting pass. Example test targets compile and pass (zero tests registered).
Logs and the edited-source hash accompany this note. The scratch source copied
the pinned round22 materialization with only src/lib.rs replaced; no frozen
bundle or historical source snapshot was changed. Standalone CPU build only;
root/CUDA compilation remains excluded. No implementation check failed.
