# Exact deterministic block dispatch and a changed bounded search

Both candidates implement the same fourteen local splitting predicates: the
old arithmetic circuit has the C100/C101 proof; normalized cubics have C104's
full guard certificate plus C105's affine/isomorphism proof. Each candidate
uses exactly the same76parameters and3global q bits. No selection changes the
domain or the accepted set. A deterministic per-block rule minimizes added
CNF clauses after import into a clone of the current graph, with arithmetic
ties. This is a structural heuristic, not selection by observed solve times.

Import starts from the complete output ancestor closure. Constants and all76
inputs are fixed. Topological induction proves each imported node equals its
source; canonical gate reuse changes neither value nor variable semantics.
Dead gates can be removed because they influence no requested predicate.
Twelve independently generated synthetic circuits check every tested output
on64assignments, and32 actual parameter assignments compare each selected
predicate and infinity guard against the old numeric implementations.

Both candidate construction costs, cloning, merging, oracles, graph output,
native formulation, model checks and final reporting count from worker start.
Only if the actual final clause count is below C101's187211 does the worker
attempt a native solve, with the SAME45-second total cap and seed. Otherwise
it records a successful diagnostic/preflight stop, not root completeness.
The90-second outer deadline,78-second demotion,4GiB and CPU0 still apply.
Compiler/template costs remain separately recorded and are not evidence of
qualified end-to-end gain; target-dependent preprocessing cannot be free in
the release comparison.

Every native model is checked against the old local field predicates and
full prefix support. Unsupported H alone is blocked over all q aliases.
Supported H pauses for original source/U recovery. Engine UNSAT is not an
independent original certificate; UNKNOWN remains inconclusive. No graph
count, worker success or timeout qualifies the frozen performance gate.
