#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= T1716 materializer recovery
#text(fill: rgb("555555"))[Run: t1716-materializer-20261105 | recovered 2026-11-07 | pcc:unverified]

The native receipt records a vendor-phase timeout after *880.0473303619947 seconds*, with *13,018 files counted*. Address-space/data limits qualified. Recorded cgroup controls were 2 CPU, 4 GiB RAM, zero swap and 512 tasks. The compiler was not invoked; a complete materialized-inputs.json was not recovered.

#image("pipeline.svg", width: 100%)

== Accounting
One attempt started 2026-11-05T20:12:04.297392+00:00. Native failure: `RuntimeError('880-second work limit')`. Conservative charge: 900 worker-seconds. Recorded cumulative charge: 5,400 of 46,800 worker-seconds. Arithmetic remainder: 41,400. Absolute deadline: 2026-11-06T04:00:00Z, now expired; the remainder does not renew it. CPU consumption, dollars, uncertainty and matched ratios are unavailable. Elapsed time is an observation, not a comparative benchmark.

== Custody and limitations
The launch owner was InvocationID e324c062a3d14d53acb39481c2fabce5. Its activating-state Result=success is not a final outcome. The native receipt records child cleanup but still requires outer cgroup confirmation. Current recovery readback finds the unit and cgroup absent after reboot. Launch boot differs from recovery boot. Historical same-invocation terminal custody is *not verified*. No execution was repeated during recovery.

== Evidence and scope
Pinned materialized revision: 29861c9a55f9ae5a33e1e09b3866aa397fbde605. This input-materialization attempt produced neither full-library validation nor matched-cohort results. The original 1,260-target scope remains unchanged.

Sources: native-receipts.tar.gz (11 unchanged files), raw-manifest.json (member SHA-256 and sizes), recovery-observation.json (current readback), result.json and metrics.csv (derived views). The archive contains exact launch argv, publication, ownership, controls, debit, progress and native failure receipt. Extract into a fresh directory and verify all members against raw-manifest.json. pipeline.svg is the editable figure. A diagram replaces a comparative graph because only one attempt and no matched timing arm are available.

== Next dependency
Diagnose vendor materialization cost and prepare any continuation prospectively under renewed execution limits. T-1731 owns this recovery packet; the global journal is reserved by T-1730 and was not edited. See REPORT.md for the component session note.
