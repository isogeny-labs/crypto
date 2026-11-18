# First-independent rootfinding relation screen

Frozen before data on 2026-11-16. Parent source and result:
`../rootfind_block_tail_relations_20261116/`. This screen asks whether
continuing an m=3 batch scan **only when the first valid triple is dependent**
can save target queries and charged relation work. It is an exploratory
collector policy; no process-wall or F4/F5/F6 ordering follows from it.

## Mechanism and prior work

The current batch oracle stops at its first verified triple, even if that row
does not advance the current modular relation basis. The candidate passes
each verified triple to the same basis and stops at the first row that raises
rank. If none does, it scans the whole factor base. It never uses the planted
scalar to choose a row. The all-hit policy is a ceiling/control; it scans all
points and offers every verified triple. The first-hit production policy is
the primary control.
[pcc:unverified]

The supplied `rootfinding_decomposition.pdf`, §§3 and 6, states the exact
group-law/pair-table decomposition and separates oracle timing from complete
relation collection. `research/notes/ecc2k130/RESEARCH_KOBLITZ_INDEX_CALCULUS.md`,
"Collection was waiting for rank, not coverage", identifies marginal rank
as the operative stopping quantity. The shared KB at revision
`9141e386254340a9bdba3036f2bb706a` points to that note as
`source:883ab40565c1f261353efc03ba04473e`; its 2026-11-14 refresh
predates both November 16 multihit screens. We claim no novelty.
[pcc:unverified]

## Fixed instances and controls

Use registered Koblitz E0 n=13/19/23; standard-subspace dimensions 5/7/8;
signed Frobenius-orbit columns; and the production negation-folded binary
batch torsion pair-table builder. Pin each curve's ICV1 ID, subgroup order,
factor-base digest and point/key/collision counts to the committed block-tail
harness. Seed every arm with exactly the same independently checked collision
rows and torsion row. Use the public target Q=[641]G, cofactor h=4,
and residual R_t=Q+[a_t]G with b=1. All rows must re-add to R_t; all seed
equations must re-add to the identity; modular elimination must remain
consistent; every full-rank answer must equal 641 and satisfy [d]G=Q.

K13: a_t=(1000+3t) mod r for t=0,...,r-1, skipping the unique identity
residual. This enumerates the subgroup in a different order from the prior
full-subgroup screens. K19 and K23: a_t=105000+3t for t=0,...,1023.
Those 1,024 distinct coefficients lie in 105000..108069, disjoint from
both preceding K19/K23 screens' coefficient intervals. No data-dependent
sample extension or early censoring is permitted. [pcc:unverified]

For each target, enumerate the source-bound first-witness triple at every
factor-base position, recording cumulative lookups, pair re-additions and
the exact hit position. Replay three policies in arrival order against
separate but identically seeded modular bases:

1. **First:** add the first row, whether dependent or independent; stop.
2. **First-independent:** add each candidate row until one advances rank;
   stop there, or at the end when all candidates are dependent.
3. **All:** add every row and scan the whole factor base.

The first and candidate policies charge the entire eight-point block
containing their stopping hit, including the singleton first-point block.
Their lookup/re-addition/row attempts stop at that hit. A complete miss or
all-dependent candidate charges the whole scan. The all-hit policy always
charges the whole scan. Snapshot each arm at its first full-rank query:
query count, batch subtractions, pair lookups, pair re-additions, row
attempts, independent rows, dependent hits skipped, rank and recovered
scalar. Count one modeled group addition per subtraction and pair re-addition.
The counts are proxies: no native field pricing, block-lane timing, target
construction, basis operation cost, setup, memory or verification timing.

## Prediction and decisions

The candidate passes this fixed-stream operation screen only if it reaches
full rank with fewer modeled group additions than first-hit, with no worse
answer validity. Fewer queries alone is an informative component result but
does not pass. All-hit gives the upper relation-yield ceiling and is not a
winner by definition. Report each curve independently, including any
censored full-rank arm; do not pool cells or extrapolate to larger fields.
The decisive next gate is a same-source, same-target complete-call L2 panel
against F4/F5/F6, including setup, rank, verification and independent replay.
[pcc:unverified]

One locked library-only release harness run is planned. Commit this protocol,
then source, before executing. Keep exact command, source/toolchain/binary
hashes, stdout/stderr, UTC and resource data, machine-readable derived
results, editable diagram and PDF. A failed build or run is retained, with
any correction made by a new commit. `dominated_by`: first-hit if the
candidate does not lower modeled additions on the fixed stream.
`sota_delta`: no asymptotic or complete-call performance claim.

The shared `docs/session-logs/2026-11-16.md` is reserved by Conductor
T-6720. This study records its own chronological `WORKLOG.md` and will
reconcile the session log when that reservation clears.
