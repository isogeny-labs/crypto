# Matched Expanded4 / Split4 correctness comparison

This extension keeps the previously registered nineteen disjoint label cells,
two sampler seeds20261115/20261116 and one known-answer F101/order107 target37G.
It compares the existing native `Expanded4` resultant with existing `Split4`
(two S3 equations and a fifth, full-field auxiliary abscissa).

The first four domains remain exactly the cell domains. The fifth Split4
domain is `None`, meaning native full-field enumeration, not a selected list.
Every full witness is retained before projection to the four coordinates.
Multiple auxiliary witnesses are deduplicated only in the projected root set.
Both encodings must equal the separate affine oracle, cover all cells,
retain uncertain cells and pass native plus separate affine scalar recovery.

## Fixed bounded call

Eight arms in fixed order: seed, encoding (Expanded4 then Split4), cache mode
(cold then shared). Each arm visits all19 cells, not just cells with roots.
Per-cell cooperative budget2000ms, degree24, pool8entries/100000words,
fresh per arm, one Rayon thread, outer process420seconds. Errors and incomplete
calls remain in the literal report, exit2 rather than a successful empty set.
No favorable retry or post-hoc seed/profile change.

Prediction: both encodings have identical complete projected root sets on
these distinct-coordinate rational-point fixtures. Raw witness multiplicities
may differ. No prediction of timing, superiority or total operation cost.
Shared trace statistics price only their named certification diagnostics;
cold totals and complete costs remain unavailable.

Before the scientific call, run `encoding-compare transport-control` through
the actual wrapper. It creates its own new output, deliberately fails all
eight arms without calling a solver, and must retain every later arm. Separate
stdout capture prevents stealing ownership of the worker's create_new path.
Output-existence/argument guards test all three binaries.

| Requirement | Acceptance evidence |
| --- | --- |
| Same cells/seeds/target | native sampler and immutable cell plan; source pin |
| Both encodings and cache modes | eight planned and attempted arms, none excluded |
| Complete roots and recovery | every cell plus all70-tuples affine oracle |
| Auxiliary correctness | full Split4 witnesses retained; project only first4 |
| Failure retention | transport control and incomplete/error fields |
| Certification diagnostics | native named counters; no total-cost claim |

## Source review and open dependencies

Primary source: [APS author manuscript](https://iris.unitn.it/handle/11572/202653),
section3.4, uses S3/S3 at arity4. Section3.1's finite coordinate constraints
motivate the preserved cells. Native `aps_single_relation::build_system`
and `f4_fp::solve_visit` define the actual encodings and full-field visitor.
This is a matched finite correctness check, not a new construction or a claim
about general scaling. The earlier QUARTIC.md remains its historical design.

KB tools are unavailable and the wrapper/catalog absent from the sparse runtime;
no absence/novelty claim. Conductor remains disconnected; the user's explicit
native/bounded-correctness exception applies, not performance admission.

Current report-evidence requires a canonical unified export. Its referenced
crypto-autoresearcher skill/schemas are encrypted VLT1 blobs through the available
repository access; connector retrieval also failed. Their contents could not
be read or validated. Keep this gap open; do not invent a conforming export.
The native literal report and flow preserve the data for later export once the
canonical plaintext schema/checker is available. No performance/EKS run,
pricing/accounting completion or frozen four-size admission is implied.

Private ledger/shared-log/raw-build/host publication restrictions persist.
Every producer assertion remains unverified without a signed native verdict.
