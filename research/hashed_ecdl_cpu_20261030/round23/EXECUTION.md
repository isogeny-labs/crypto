# Round23 execution

## What was done
T-1328 ran the owner-authorized panel exactly once after confirming no existing job/results and no active ordinary-pool workload. All 180 executions completed (162 measured, 18 warmups); all 18 cells and byte-identical full certificates passed. All 379 worker-manifest entries verified before and after native export generation. Native export created successfully. Job/container identity and exact times are in execution-outcome.json.

## Decisions made
The single authorization is consumed. 2 vCPUs, 4 GiB, 300 seconds, zero retries, normal Spot autoscaling and the unchanged one-node cap were preserved. No frozen inputs changed. Raw evidence is published before reports/round23. Historical preparation/qualification records remain intact and are superseded operationally by this outcome.

## Open threads
Filter-on complete-time medians are lower in all three fixtures at both batch sizes. Descriptive reductions are 4.19/2.65/2.99 percent at batch64 and 1.77/2.64/2.53 percent at batch1, computed as 100*(1-on/off). Fixture0 ranges overlap; fixtures1/2 ranges do not overlap in either matched filter comparison. These are L0 diagnostics, not isolated tenfold acceptance. Continue CPU work using complete cost and exact witnesses; no additional paid attempt is authorized.

## Errors
Initial pending placement resolved through normal Spot provisioning. The missing UID1000 username warning was nonfatal. No retry, deadline extension, missing cell or failed validation. Scoped execution log preserves the lane handoff without editing shared session logs.

The generic staged whitespace check flags original kernel cpuinfo/process-status whitespace. Those worker artifacts are preserved byte-for-byte to retain manifest validity; whitespace checking is scoped to authored files. The first report render failed on an unescaped multiplication symbol; formatting was corrected before the successful two-page render and inspection.
