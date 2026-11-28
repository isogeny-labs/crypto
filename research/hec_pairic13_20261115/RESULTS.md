# Pair-line large-prime engine reproduces the source-verified solve

Single frozen release run exited0, stderr empty. Source7cb3afb0f885 and
checkpoint20261115T171249Z preceded execution. No altered base size, retry,
cap, fallback or scientific rerun. Both public inputs were mapped anew.
Recovered k183049 verifies on source and destination, matching mappedic13.

| Quantity | Exhaustive control | Pair-line candidate |
|---|---:|---:|
| Rational point list |129|129, exact ordered match|
| Affine root-generation mechanism |127²grid points|127polynomial linear parts|
| Line candidates |16257projective lines|1128FBpairs,878unique lines|
| Distinct split relations |664|314|
| Final anchored solve columns |128|47|
| Final rank/nullity |127/1|46/1|

These are algorithmic counts and exact checks, not timings or a total speed
ratio. Point-generation mechanisms perform different work per counted unit.
Smallbase48fixed before execution. Pair candidates:250duplicates,564unique
lines rejected,314split lines accepted. Of accepted lines,14have0LP,86have1LP,
214have2LP. Eliminating81LPcolumns leaves232rows/47columns. All129point logs
are recovered by reverse pivot tracing, with no unresolved coordinates here.
All314original-row residuals pass and every known coordinate equals the full
reference kernel times166553. Full129known-log vector is retained in RUN.txt.

The traced eliminator produces identical rows and diagnostics to the unmodified
pinned routine. Sparse_ops14455 and entries9480 retain its existing accounting
conventions, including a fixed128charge per inversion; these are comparison
diagnostics, not calibrated field-operation or hardware measurements. Trace
storage/allocation is not included in those diagnostics. The exhaustive
reference and two eliminator copies are correctness controls, not candidate
cost work to include in a later optimized-only timing arm.

Candidate descent: D1at0translates with log34511; B at13translates with
log829912. Both yield183049 by ratio. Same source point pair, kernel, Q1 and
lambda10; no supplied scalar or prior mapped encoding used. Exact full-matrix
controls retained their664residuals, corrupted-row rejection and8principal-class
checks. Arithmetic is reused and is not an independent implementation.

This removes the exhaustive line sweep and point grid from the candidate path
and reduces the final solve dimension. The remaining FBsolve is dense; this
single fixed-field gate does not prove the complete staged Diem complexity or
an end-to-end10xgain. Next measure complete costs with frozen phases and replace
remaining sparse/collection bottlenecks where evidence warrants it. Kernel and
quotient construction plus calibration, map scans, LP reconstruction, descent
and verification all need charging. Genus2/4 work remains open. No historical
closed record, default or leaderboard changed. New COUNTS.svg/REPORT.pdf;
previous reference graphs remain unchanged.
