#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= Adaptive normalized transport reaches recovery
2026-11-05 · T-1722

The quartic recovery program now consumes characters from the normalized transport source. A consuming cursor retains the last two normalized vectors, middle coordinates and projective geometry tail. It continues the existing window without rebuilding it. [pcc:unverified]

#image("pipeline.svg", width: 100%)

== State invariant and lazy skips
The next point uses a public anchor, direct initialization when fewer than two offsets exist, or the established homogeneous recurrence. Anchor precedence handles identity crossings. Each step rotates the saved vectors and coordinates. A skipped character advances geometry and every column but does not evaluate the residual. A length1 window can still need one direct initialization on extension. [pcc:unverified]

The total constructor limit covers the initial window plus allowed extra steps. Zero length, exhausted steps and checked-add overflow are rejected. Ownership and lifetimes bind the cursor to one prepared geometry. Inherited prime-field, prime-subgroup and seed-law assumptions remain explicit. [pcc:unverified]

== Recorded correctness checks
The native suite passes207 tests. New cursor checks cover100 split windows,1375 advances and450 skipped residuals across F5/r7, F7/r13 and F509/r479. They compare represented ratios and actual residual/phase/value call counters with the frozen full-window source. Thirty-six dependency hashes bind the final build. [pcc:unverified]

All20 established synthetic public points recover the same scalars as the frozen batch-reciprocal path and pass forward multiplication. The route uses16 extra signs,4 skipped signs,20 extra geometry steps and480 extra conversions. Actual phase/value initialization calls total33 across these recovery calls; none occur in their extensions. These counts do not include setup. [pcc:unverified]

A separate checker imports no producer code, checks the preserved479-input polynomial identity and20 records, and rejects changed scalar and initialization-count packets. It was authored in the producing session. The initial module-path build failure and exact failing source are preserved. Thirty native compiler warnings remain. [pcc:unverified]

== Next construction and evidence
Instrument the complete path against the frozen batch-reciprocal source: setup/compiler, initial phase/value work, column updates, final powers/resultants, allocations, decoding, skips and verification. Per-step vector allocation/copying remains. No comparative timing or cloud job; original4,563-case cohort unchanged.

Claims PCC-20261105-456c87 and PCC-20261105-70349e remain unsigned with local verdict not_covered. Evidence: receipt.json, test.log, checker-receipt.json and source-lock.json. Sources checked: saved multiseed consumer, adaptive geometry tail, quartic/batch adapters and checker; inherited mathematical references unchanged. KB unavailable (MCP absent, wrapper executable missing).
