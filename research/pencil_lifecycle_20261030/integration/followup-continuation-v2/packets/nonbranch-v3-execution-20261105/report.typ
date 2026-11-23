#set page(paper: "a4", margin: 16mm)
#set text(size: 10pt)
#set par(justify: false)
= Nonbranch replay V3 — native receipts
[pcc:unverified] Nonbranch.lean compiled (exit 0). IndependentControls.lean failed (exit 1). Readback and ExpectedFailure were not invoked. Complete pre/post custody dictionaries and bytes match: 16,736 files and 3,111,892,318 bytes each. Shared-cache transient changes are not excluded.
#image("phase-times.svg", width: 100%)
Outer worker-wall time: 582.323951853 seconds. Nonbranch command: 245.954116135 seconds; IndependentControls: 56.934397451 seconds (derived end minus start). CPU time is not inferred. Children were reaped; the supervisor confirmed terminal phase owners.
== Recorded failure
[pcc:unverified] IndependentControls.stdout: line 19 resolves an unqualified residual as a topological filter; lines 21, 26 and 34 leave polynomial equalities unsolved; line 74 reports unknown tactic; lines 120–121 try generic multiplicative map_mul for an additive valuation. Source correction hypotheses are described in REPORT.md; none were compiled as part of this archive.
== Controls and accounting
Attempt 3 of 3, no retry. Pre/compiler/post ceilings: 900/600/900 seconds (2,400 total). Resources: one CPU, 8 GiB, zero swap, 128 tasks. Prior conservative debits: 300 + 900 seconds, preserved. Campaign: 46,800 worker-seconds, deadline 2026-11-06 04:00 UTC. Actual timing and ceiling reservations are recorded separately. Axiom readback and negative control remain unexecuted.
== Replayable evidence
46 raw files, including custody dictionaries and Nonbranch.olean, are preserved in the content-addressed archive in artifact-store.json (2,286,453 bytes). Downloaded readback SHA256 matched:
#text(size: 8pt)[fef9f053bf8b823dc314f3671252b1e123e35f2ebe51c97d44b47d83b6809a8d]
Verify archive and per-member hashes before extracting into a fresh directory. Sources: native phase receipts; outer/completion.json; native/IndependentControls.stdout; raw-artifacts.json. Structured data: result.json and measurements.csv. Graph: editable phase-times.svg. This preservation step did not invoke the compiler.
