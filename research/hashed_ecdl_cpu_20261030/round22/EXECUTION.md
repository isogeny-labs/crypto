# Round22 execution

## What was done
T-1320 executed the owner-authorized frozen panel once after the previous capacity blocker became terminal. All 180 executions completed (162 measured, 18 warmups), all 60 stage rows validated, and full certificates matched byte for byte. All 500 worker-manifest entries verified before and after native export generation. Exact job/pod identity and times are in execution-outcome.json; submission-request.json preserves the command and resource limits.

## Decisions made
The single authorization is consumed. Normal Spot autoscaling was permitted; 2 vCPUs, 4 GiB, 300 seconds, zero retries and the unchanged one-node cap were preserved. Historical preparation records remain unchanged. Raw evidence is published before reports/round22. The pinned candidate and original baseline were not rebuilt or modified.

## Open threads
Profiled batch64 oracle medians are 0.360492 and 0.483973 ms in fixtures 1/2; linear medians are 0.106291 and 0.144570 ms. Oracle work is the largest recorded individual stage in these fixtures. Next inspect oracle preparation and residual probing against the exact counters before selecting a further optimization. Profile clocks are descriptive and not additive; ordinary controls retain full cost. The 10x further-improvement objective remains unverified.

## Errors
One status observation timed out; the same Job UID later established completion. No resubmission occurred. The UID1000 username warning was nonfatal. A post-validation filename listing for '*host*' found no matching file; export and all hash checks had already succeeded. No study data are missing. Shared session log is sparse and held by other lanes; this scoped execution log preserves the handoff.

