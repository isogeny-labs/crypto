# Branch counter work, 2026-11-01 23:55 UTC

## What was done

Instrumented dispatcher projection, delta bounds and exact word increments compiled: 11 declarations, 8 controls, 19 readbacks. PDF SVG and graph-data JSON rendered; two failures and negative control retained.

## Decisions made

Use exact finite control values for graph, with no performance inference. No scientific execution. Shared log reserved by T-1351, so retain this scoped record without editing its reservation.

## Open threads

Publish packet in PR #3237; mirror log after reservation release. Independent fidelity, component cardinality and native storage composition remain.

## Errors

Helper bound inference required explicit branch arguments. The anchored union control needed tied-size simplification before decide. Both failures are retained.
