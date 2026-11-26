# Multiplier names and geometric rows

Each actual query records four multiplier lifts, four sigma^q parameter-base
keys, and only one normalized signed geometric row. The known-consistent
case explicitly compares the rows at sigma1,-1,h^-3 and finds equality.
Raw/multiplier/index provenance is retained; no alias is silently discarded.
[pcc:unverified]

The written explanation uses two elementary symmetries. Negating sigma
negates the real coordinate of each shifted parameter, swapping the two raw
x branches; after the original unique-branch admission the same raw curve
points remain. Inverting a norm-one parameter keeps its real coordinate.
For templates h,h², the transformation

    sigma' = h^-3 * sigma^-1

makes h*sigma'=(h²*sigma)^-1 and h²*sigma'=(h*sigma)^-1, swapping the two
raw point positions. Their group sum and normalized unordered signed row
are unchanged. This is a pair-row symmetry, not a claim that the full
K20633 parameter bases are identical under this transformation.
The explanation is written; native qualification checks the stated fixed
cases, not a general formal group-action theorem. [pcc:unverified]

The sigma^q key distinguishes parameter-base classes modulo H_q. It does
not identify all geometric encoding/cofactor aliases, and is not itself
a relation-rank or recovery certificate. Counting four multipliers as four
independent rows fails for these records. The separate common-base/target-tag
rank test remains open; no universal rejection of adaptive bases follows.
Original FAMILY_GATE/ROUTE_CRITERIA bytes remain unchanged. [pcc:unverified]
