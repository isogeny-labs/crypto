# Retained primary N83 K2048 natural five-summand search

This is a frozen one-query relation-yield gate on the study's `a=0` curve,
subgroup order `2417851639230796216685689`, public fixture zero, and the
retained hash-policy K2048 base at seed 2026100801. The exact compressed S3
object BLAKE3 is
`82990cda5d57b1c4c4b635673a881ad233c24f707e7ac644a11478aeddddf851`;
its SHA-256 is
`aacb62784dee0c9f5e06328dbb85a4b6a1c7243e778328da82900e7284f3a74a`.
The preparer verifies the earlier manifest, generic replay, upload receipt,
and public corpus, then makes a fresh encrypted S3 GET. The run uses only
that fetched object. The object's 2,048 signed-Frobenius columns and 339,968
point records stay distinct from the diagnostic `a=1` N83 curve.

The existing `chain_search_supervisor.py` freezes a clean source commit and
compiled Linux worker, checks all selected inputs, and launches the exact
phase-aware `primary-orbit-chain-cold` worker with five summands, one trial,
no direct relation, at most 150,000 SAT variables, 1,000,000 domain clauses,
32 models, and 100,000 conflicts. Docker uses one CPU, a 4-GiB hard memory
limit, zero swap, no network, read-only inputs, and a 5,400-second outer wall
cap. The first all-affine public identity pattern is used unless the worker
itself records a later pattern. Record live container limits and retain every
raw worker, event, stdout/stderr, source, config, cap and cleanup receipt.

If a model appears, the worker must lift and check original field equations
and the full curve-group sum. The independent auditor must replay the public
query and any relation against the S3 object and check rank counters. If the
conflict or wall cap fires, preserve `UNKNOWN_solver_cap` or
`UNKNOWN_wall_cap`; neither is UNSAT or zero natural yield. A one-query
relation, even if valid, cannot estimate repeated yield or complete rank.
No base is selected until natural relation rate, useful rank, LA, target
extraction, verification and same-point rho are fully charged on a frozen
multi-point panel. The conservative search envelope is 5,400 seconds in
addition to build/preparation and all prior study work; observed wall time
is reported separately.
