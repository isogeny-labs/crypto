# Four bounded leads, independent parallel workers

The user requested all four leads in parallel. Each lead has a separate Rust
module, fixed inputs and exclusive output. The combined source is pinned before
four actual native processes are dispatched concurrently. This is finite
correctness and structural work, not a time or scaling comparison.

## Fixed interventions and references

1. **Exact Split4 auxiliary support.** For every existing quartic cell, build
   the left/right pair-sum abscissa supports and their intersection. Add its
   vanishing equation and exact auxiliary domain to native Split4. Compare
   unrestricted Split4, constrained Split4, direct pair joins and the ordinary
   affine oracle. Empty intersections skip a solver call explicitly. Record
   pair-construction and join work; this construction is not a free oracle.
2. **Label-law design.** Compare weights1/2/4/8 and1/3/9/27 with two fresh valid
   points per weight. Use the same deterministic raw u stream for each law,
   build uG+vQ and canonicalize both labels correctly. Different laws may reject
   different draws and yield different coordinate sets. Keep all draws, all70
   unordered quadruples, all16 B-sign sums, uncertain tuples and group incidents.
   Compare the exact sign criterion with the conservative valuation predicate.
3. **Finite-domain quotient reduction.** Reduce native Expanded4's summation
   equation by the four monic coordinate vanishing polynomials, preserving its
   actual values and retaining the original domain equations. Compare original
   and reduced F4 with direct Cartesian evaluation and the affine oracle. Record
   terms/degrees and reduction work; enumerate the same complete tuple products.
4. **Per-cell pairing.** Compare all three Split4 pairings with cold/shared
   pools. The fixed selector minimizes |Fi|*|Fj|+|Fk|*|Fl|, ties by pairing index.
   It uses domain sizes, never the enumerated root set. Keep all pairing controls and
   selection work; retain full witnesses before coordinate projection.

All leads use the same public F101 curve, order107, target37G and seeds20261115/
20261116. Label-law candidates regenerate points; they are not a fixed-coordinate
comparison. The other leads preserve the nineteen-cell, seventy-tuple cover,
including every uncertain cell. Fixed source input reports preserve the
prior validated sampler stream; no favorable seed search or retry is allowed.

## Predictions, controls and limits

Hypotheses: certified auxiliary support and quotient reduction preserve complete
root sets; label-law changes can change cancellation patterns; input-only pairing
selection preserves roots. Structural reductions or different native counters
do not establish a runtime or total-cost improvement [pcc:unverified].

Per F4 call: cooperative2000ms limit, degree24, cache8entries/100000words where
used. Each lead process has420seconds and one Rayon thread; four processes run
concurrently. All outputs use native create_new; stdout/stderr are separate.
Workers reserve their outputs before a common start barrier is released;
the barrier wait is bounded at30seconds inside the declared process limit.
Run the actual wrapper's transport control before dispatch; retain deliberate
mock errors and ensure every planned lead remains present. Capture all process
exits, failures/timeouts/empty files and later workers even if one fails. Do not
replace an incomplete result with an empty-root success or retry to get green.

The scientific launcher records planned/attempted/completed/failed leads and
each lead retains its internal seed/cell/route failures. Source/build/pre/post
manifests and readonly qualifier hashes are distinct. No profiler or elapsed
time ranking is produced. Cold arithmetic totals, inversions, memory and full
accounting remain unpriced; actual named construction/solver diagnostics stay
in the literal output. Register native statements before public result citation.

## Prior art and remaining dependencies

Sources checked: APS author manuscript sections3.1/3.4 through
[Unitn](https://iris.unitn.it/handle/11572/202653), native APS/F4 builder and
visitor, and prior frozen triple/quartic/encoding reports. Bounded label design
is related to [distinct subset sums](https://www.combinatorics.org/volume_5/pdf/v5i1r3.pdf),
but repeated labels require a stronger bounded-multiplicity condition. Monic
coordinate reduction and pair-sum joins are standard algebra/group operations;
no novelty claim. KB MCP/catalog remains unavailable in this runtime.

Conductor is disconnected; the user's explicit native/bounded-correctness
exception applies. It does not authorize performance/EKS, denied publication
or a substitute control plane. The canonical protocol/export files previously
resolved to unreadable VLT1 blobs. Preserve native full data/flow for a later
schema-validated export; do not invent a conforming experiment.export/v1.

Root code/lock, previous frozen evidence and existing workflow assertions remain
unchanged. The full native ledger, shared logs/index, raw host/build/dependency
logs and rejected histories remain private. Keep auto-merge off and do not merge
while ledger publication/reconciliation is held. Fresh prospective checkpoint,
complete accounting/replay/export and frozen admitted protocol remain required
before performance work. No broader research completion is implied.
