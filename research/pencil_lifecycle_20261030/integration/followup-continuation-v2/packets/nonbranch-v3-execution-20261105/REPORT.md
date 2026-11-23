# Nonbranch replay V3: native receipts

[pcc:unverified] Nonbranch.lean compiled with exit 0. IndependentControls.lean exited 1; Readback and ExpectedFailure were not invoked. Pre and post custody receipts passed; their complete hash dictionaries and serialized bytes match (16,736 files; 3,111,892,318 bytes each). These checks do not exclude transient shared-cache changes.

The outer run ended failed after 582.323951853 worker-wall seconds. Native pre/compiler/post durations were 152.627714706 / 303.263430189 / 125.003910334 seconds. Command durations derived as end minus start were 245.954116135 seconds for Nonbranch and 56.934397451 seconds for IndependentControls. CPU time is not inferred from wall time. Both compiler children were reaped with empty remaining process groups in their receipts. All phase owners were confirmed terminal by the supervisor.

## Recorded compiler errors

[pcc:unverified] IndependentControls.stdout records: line 19, an invalid simp theorem named residual (resolved as a topological filter) and an unsolved polynomial goal; lines 21, 26 and 34, unsolved polynomial equalities involving opaque polynomial constants; line 74, unknown tactic at a theorem argument continuation; lines 120–121, generic map_mul expecting a multiplicative homomorphism instead of the additive valuation identity. These are native diagnostics. Qualifying ActualNonbranchNorm.residual, normalizing Polynomial.C numerals, indenting the continuation and using AddValuation.map_mul are source-level correction hypotheses, not checked fixes.

## Custody, controls and accounting

[pcc:unverified] Exact launch arguments, resource bindings, unit observations, stdout/stderr and receipts are preserved. Limits: one CPU, 8 GiB, zero swap, 128 tasks; phase ceilings 900/600/900 seconds, total 2,400. This is attempt 3 of 3; no automatic retry. Earlier conservative debits remain 300 + 900 seconds. The campaign limit remains 46,800 worker-seconds with deadline 2026-11-06 04:00 UTC. This report records actual time separately and does not silently refund a ceiling reservation. No full axiom readback or negative-control result is available from this attempt.

## Artifact access

All 46 native and outer files are in the immutable content-addressed archive described by artifact-store.json, including the two full custody dictionaries and Nonbranch.olean. The archive is 2,286,453 bytes, SHA256 fef9f053bf8b823dc314f3671252b1e123e35f2ebe51c97d44b47d83b6809a8d; its downloaded readback matched. raw-artifacts.json records every member's bytes and hash. Git contains compact raw copies, result.json, measurements.csv and editable phase-times.svg. Download the URI in artifact-store.json, verify the archive SHA256, then extract into a fresh directory and verify each member against raw-artifacts.json. Do not extract over the original run. No new compiler invocation was made to create this report.
