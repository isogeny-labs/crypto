# Disjoint quartic label-profile cells

This bounded correctness extension uses the existing native `Expanded4`
resultant and exact-domain F4 visitor. It does not change production code,
assertions, label laws or solver identity. The original `structural` binary
and earlier frozen reports keep their original scopes.

## Plan fixed before the dedicated call

Use the same public F101 curve, order107, known-answer target37G and sampler
seeds20261115/20261116. Each actual even-arity sampler produces two points of
each absolute B label1/2/4/8. Enumerate label multiplicities0/1/2 summing to4.
A label occurring once supplies a two-coordinate slot; a label occurring
twice supplies its two coordinates as ordered singleton slots. This removes
permutations while every cell still has four disjoint coordinate domains.

Prediction:19 cells form a disjoint cover of all70 unordered quadruples.
Nine valuation-safe cells cover48 tuples; ten uncertain cells retain22.
An independent combination traversal must reject a missing or doubled cell.
Neither uncertainty nor a possible signed B=0 discards a cell.

Run every cell through both cold and one shared native trace pool, fresh
per seed/route. Degree bound24, cooperative per-cell limit2000ms, pool8entries
and100000words, process limit420seconds, one Rayon thread. Keep every
timeout/error and fail the panel if any call is incomplete. No favorable retry.

For each cell, enumerate coordinate products and compare the complete native
root set with a separate ordinary-affine signed-sum oracle. Separately compare
the union against all70 unordered tuples, classify signed B incidents and use
actual native recovery. Check any recovered scalar by ordinary affine scalar
multiplication and the native curve. Two sampler streams use one target, not
two independent DLP targets. No time, scaling, ranking or complete-cost claim.

The `quartic` binary takes the same source-pin/output arguments as the earlier
binary. Output creation is exclusive, before solver calls. Exit0 requires
complete equality, exit2 retains failures/incompleteness, exit1 means CLI/I/O
failure. The report stores actual sampled points/labels, cell domains, tuple
membership, expected/observed roots, trace diagnostics and recovery outcomes.

## Prior work and boundaries

Sources checked (KB): KB MCP unavailable and wrapper absent in this sparse
runtime; no novelty/absence claim. Prior source and label observations are
in `research/aps_production_bridge_20261116`. Native source references are
`aps_single_relation::build_system` (`Expanded4`) and `f4_fp::solve_visit`.

The [APS author manuscript](https://iris.unitn.it/handle/11572/202653),
section3.1 (equations10/11), uses distinct finite coordinate domains; its
section3.4 instead uses a split S3/S3 encoding for arity4. Here Expanded4 is
the existing native resultant baseline; split4 and larger-scale comparisons
are separate follow-ups. The new part being checked is complete disjoint
label-profile coverage without dropping uncertain tuples, not a new
summation-polynomial construction. The ePrint Semaev source returned403;
the accessible APS manuscript and native resultant implementation were read.

Neither native certification counters nor unsigned evidence establish a
speedup [pcc:unverified]. Performance still needs its prospective checkpoint, complete
accounting/replay/export and frozen admitted protocol. No ledger publication
or reconciliation is authorized by this correctness extension.
