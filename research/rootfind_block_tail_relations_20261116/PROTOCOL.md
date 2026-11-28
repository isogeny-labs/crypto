# Harvesting the first hit block's remaining relations

Frozen before data, 2026-11-16. Source parent:
`7db5987e6d7f7d4300485c005cdef624ed5cc64a`. This is an exploratory
follow-up to `../rootfind_multihit_relations_20261116/`: that screen
showed fewer distinct queries but more pair probes when every base point
was scanned. Its stream and result motivated this candidate. The present
K19/K23 streams are disjoint from that screen's sampled residues; K13
reorders the entire subgroup. No random-population or larger-curve
inference follows. The shared KB search for a matching block-tail method
was non-exhaustive; no novelty claim is made.

## Mechanism, controls, and prediction

The production m=3 batch path computes one special first-point block,
then blocks of at most eight factor-base points. A hit in a block does
not undo the additions already performed for the remainder of that
block. The current oracle probes in order and returns the first valid
triple. The candidate probes the rest of that *same computed block*,
returns every valid triple in it, and stops before another block. The
matrix keeps only independent rows. This may reduce distinct target
queries without extra point-subtraction blocks, but incurs additional
lookups, pair re-additions and row work.

Controls: (A) first-hit, charged for its complete hit block;
(B) same-block tail harvest; (C) all-hit, charged for all blocks, as a
secondary ceiling. Every arm uses the same first-witness torsion pair
table, factor-base order, verified collision rows, torsion seed, and
residual stream. Every retained triple is independently re-added and
every row enters the same modular elimination routine. A wrong scalar,
inconsistent row, failed group identity, base mismatch or duplicate
target is invalid, not evidence against the mechanism.
[pcc:unverified]

Prediction for the fixed streams: B reaches full rank in fewer target
queries than A while using no more charged batch subtractions. If it
reaches rank at the same or later target, or its total modeled group
additions rise despite fewer targets, that specific operation-count
prediction is unsupported. Lookups and matrix work are separately
reported and must be priced before any complete-cost conclusion.

## Fixed inputs and stopping

Use registered Koblitz E0 n=13/19/23, standard-subspace dimensions
5/7/8, signed Frobenius-orbit columns and the production torsion batch
pair-table builder. Pin exact ICV1 IDs, base SHA-256 digests, point and
key counts, collision counts and subgroup orders to the committed
multihit test. Public target `Q=[641]G`; cofactor `h=4`; residuals
`R_t=Q+[a_t]G`, `a_t=(50000+53t) mod r`, `b=1`. Step 53 is coprime
to each subgroup order. K13 takes all `r` residues, skipping the one
identity; K19/K23 take the first 1,024 distinct residues. For K19/K23,
these `a_t` lie in 50,000..104,219 and do not overlap the previous
117..37,968 sample. Sample sizes and order do not change after data.

For each target the source-bound harness enumerates hits to check all
three policies. A and B charge all subtractions in the block containing
their first hit, even when the hit occurs early. A charges lookups and
pair re-additions through that hit; B charges them through the end of
the same block. C charges the complete scan. A miss charges all blocks.
The first singleton block is followed by consecutive eight-point blocks,
including any final short block. Record the first full-rank target and
cumulative subtractions, lookups, pair re-additions and row attempts
through that target for each arm. Verify recovered `d=641` whenever
full rank occurs. Retain a censored arm if it does not reach full rank.

The operation proxy is batch subtractions plus pair re-additions, each
charged as one group addition. It omits native field work, vector-lane
differences, target construction, row reduction, setup, memory and
verification. No process-wall or F4/F5/F6 ordering is inferred from
this screen. A same-target complete-call timing panel with matched
F4/F5/F6, rank, verified answer, isolation and independent replay is
required before promotion. [pcc:unverified]

One locked library-only release harness run is planned. Commit the
protocol and source before execution; retain exact command, toolchain,
binary and source hashes, raw stdout/stderr, UTC/resource data, a
machine-readable result, editable diagram/chart and PDF. A failed
build is preserved and corrected by a new commit, not overwritten.
`dominated_by`: current first-hit batch oracle. `sota_delta`: no
asymptotic change predicted; complete-call performance unknown.
