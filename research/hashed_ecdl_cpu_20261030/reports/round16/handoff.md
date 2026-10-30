# Round16 handoff

## What was done
Frozen protocol 33e294d07d0ffc6f66fb696eac19d01a00c14add29e4532e48ea5e78c3d580c9
was committed and pushed before the one EKS submission. Job UID
7f2930c4-c23f-4718-b814-075d119af23c completed at 2026-10-30T15:50:21Z.
150/150 complete executions, full certificate equality, all binary hash checks,
scalar/replay and corrupted-scalar controls passed. Export verification has
zero findings. Downloaded archive SHA256:
906d0d0f55792906b4eb9fcfce028603046283e8b4b483b669c6677789f4bec8.
Raw records were committed and pushed as d65dee695a45 before this report.

## Decisions
Keep original baseline and all three workloads; do not promote L0 timings.
The observed smallest-fixture median changed from 0.142364 to 0.132843 ms,
while both larger fixture ranges overlap. No matched external comparison,
strict-isolation speedup or full further10x result is established.
Per-Curve inverse cache memory is reported separately from retained sum tables.

## Open threads
Solve remainder still includes target generation, elimination and internal
replay; investigate these costs next, without dropping either verifier.
Original-baseline median ratios are 2.938,7.226,6.084; these diagnostic ratios
do not satisfy the further10x objective. Larger CPU scale remains unqualified.
Current PR and exact-head CI must be checked after report delivery.

## Errors
Initial pod scheduling waited for eligible Spot capacity; the same job
subsequently ran successfully without retries. Container startup printed
`id: cannot find name for user ID 1000`; exit code was zero and all outputs
were uploaded. No failed or excluded execution is hidden. No active run remains.

Report PDF rendered and its sole page was inspected. Poppler pdfinfo printed
`Syntax Error: Suspects object is wrong type (boolean)` for Typst metadata;
the PDF opens and the graph/text are legible. This is not a PDF/A claim.
