# T1716 materializer recovery report

[pcc:unverified] The native receipt records a vendor-phase timeout after 880.0473303619947 seconds, with 13,018 files counted. Native address-space/data limits qualified and the recorded cgroup limits were 2 CPU, 4 GiB RAM, zero swap and 512 tasks. The compiler was not invoked; a complete materialized-inputs.json was not recovered.

## Observation and accounting

One attempt started at 2026-11-05T20:12:04.297392+00:00. The native failure is `RuntimeError('880-second work limit')`. The raw receipt includes child cleanup records; those do not replace the required outer terminal custody check. The launch-time ownership receipt identifies InvocationID e324c062a3d14d53acb39481c2fabce5. Its initial Result=success is an activating-state observation, not a final outcome.

The conservative 900-second charge is retained. Recorded campaign charges total 5,400 of 46,800 worker-seconds; the arithmetic remainder is 41,400. The absolute authorization deadline was 2026-11-06T04:00:00Z and has passed. Remaining seconds do not renew that deadline. CPU consumption, dollars, uncertainty and matched performance ratios are unavailable; elapsed time is a native observation, not a comparative benchmark.

## Custody limitation

The current recovery readback records an absent unit/cgroup after reboot. Current boot differs from launch boot 7643b049-6f9d-444a-a345-77d07d463277. No historical same-invocation terminal readback is recovered. This packet therefore does not assert verified historical terminal custody. No worker, compiler, trial or retry was launched during recovery.

## Scope and reproducibility

This is input materialization for pinned revision 29861c9a55f9ae5a33e1e09b3866aa397fbde605, not a completed full-library validation or a 1,260-target evaluation. The frozen cohort remains unchanged. Exact launch command, source publication, ownership, controls, debit, progress and native receipt are in native-receipts.tar.gz. raw-manifest.json binds every unmodified member by SHA-256 and byte count. Extract into a fresh scratch directory; verify every member against that manifest before analysis. result.json and metrics.csv are standardized derived views. pipeline.svg is editable vector source and is embedded in report.pdf. A pipeline diagram is used because only one attempt and no matched arm are present.

## Component session note

Recovery task T-1731 reserved only this packet. The global session journal was reserved by T-1730 and was not edited. Root coordination received the result and can append it when that reservation clears. This packet preserves prior failure records and performs no source changes. Next work is diagnosis of vendor materialization cost and a prospectively reviewed continuation within renewed execution limits.
