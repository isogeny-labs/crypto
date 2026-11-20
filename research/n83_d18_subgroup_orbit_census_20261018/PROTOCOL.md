# Direct subgroup-x census for an N83 structured orbit base

**Frozen before the degree-18 census.** The public curve is
`icv1-f2m83-tm6151469093347-debefd74` (Koblitz a=0, polynomial low
terms 0, 1, 2, 45, cofactor 4, prime subgroup order
2417851639230796216685689). Enumerate each of the 262,144 field words
`x=0..2^18-1` exactly once in ascending order. Lift on the original curve;
retain an x only when a lift exists and multiplying that lift by the exact
subgroup order yields infinity. Both lifts of nonzero x have the same subgroup
status. This is direct subgroup membership, without cofactor projection.

For every retained x, walk its 83 field-squaring phases, require return to x,
record the least full-field word as its signed-Frobenius orbit key, and count
distinct keys. Distinct keys give disjoint x-orbits; each full-length orbit
would give 166 curve points after both signs. Accept the **size premise** only
if at least 16,627 full-length distinct keys exist. Select the first 16,627
keys in ascending order, choosing the smallest enumerated subspace x in each
class as the representative. Save all counts, lengths, representative list,
hashes, source revision and elapsed timings. Run the native example on one
local host; it is an L0 census, not a speed comparison.

The prior dimension-18 archive projects arbitrary source points by the
cofactor and has 130,722 sign columns. Its projected x values generally lie
outside the subspace, so its column count cannot answer this direct
subgroup-x question. This census does not test a SAT membership encoding,
natural relation yield, useful rank, or total index-calculus cost. If the
size premise passes, the next version will export and replay the exact
2,760,082-point selected closure before any natural-query comparison. If it
fails, preserve the count and test a separately frozen larger or hybrid
source policy. The 16,627 target and decision rule are fixed before the run.
