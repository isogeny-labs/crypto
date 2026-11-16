#set page(paper:"a4",margin:17mm)
#set text(size:10pt)
= Reuse powers in split mode evaluation
T-1872 · 2026-11-07 · Prime-field compressed recovery

The evaluator now returns block values without recomputing a power at every node. Memoized binary exponentiation supplies the right-block power. One cache belongs to a fixed support/scale and can serve both C2 modes; a changed scale clears it.

#image("flow.svg",width:100%)

*Correctness.* Induction on exponent proves cached powers; induction on block length proves the same streamed polynomial value. The predecessor remains a separate reference adapter.

#table(columns:(3fr,1fr),inset:4pt,
 [Native control],[Observed],
 [Mode / independent power equalities],[240 / 1,320],
 [Reference / reused component products],[16,960 / 9,136],
 [Cold / warm products removed],[2,416 / 5,408],
 [Maximum mode cache entries / coefficient slots],[13 / 134],
 [Canonical state equalities],[2,516],
 [Source calls:281targets at cap64 / cap16],[2,881 / 3,152])

Four support shapes include unresolved-only, mixed, selected-only and empty;80scale resets are checked. Public recovery routes remain identical with zero unresolved targets. All inherited controls pass.

*Costs.* Value composition uses L−1 products plus cold power construction, conservatively O(log² L), per present component. Cache lookups, cloned values, capacities and map overhead remain charged. The134slots are live coefficient lengths, not peak allocated bytes. Product totals are structural kernel calls, not field-operation or timing ratios. Source grids, initial C0 and coefficient generation remain charged.

*Failure retained.* validation01 failed in the checker because an empty root list was passed to a nonempty product-tree API. validation02 uses the unit polynomial for empty support and passes; production evaluator unchanged. Both receipts and the failed checker snapshot are retained.

*Evidence.*72input pins per attempt; inspected PDF/SVG and publication manifest. Producer assertion; independent signed verdict pending. No cloud/timing run. Original4,563cohort unchanged. Full derivation/replay: DERIVATION.md, RESULTS.md, README.md.

*Sources and next.* Exact split predecessor; official FLINT quotient/powering conventions at flintlib.org/doc/nmod_poly/index.html. KB unavailable. Next share coefficient traversal across phase branches, accounting for simultaneous live state.
