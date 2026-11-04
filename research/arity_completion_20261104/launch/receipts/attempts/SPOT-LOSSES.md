# Spot losses, 2026-11-04 (from `aws ec2 describe-spot-instance-requests`)

| instance | type | launched | status code | status time | pod lost |
|---|---|---|---|---|---|
| i-064df182dcab5b874 | c8a.2xlarge | 16:01:12Z | instance-terminated-no-capacity | 16:05:39Z | Koblitz a1 (a6009182) |
| i-0f4850452e4af740d | c8a.2xlarge | 16:30:53Z | instance-terminated-no-capacity | 16:41:38Z | Koblitz a2 (03123eb7) |
| i-0adb36aac31128862 | c8a.2xlarge | 16:36:19Z | instance-terminated-no-capacity | 16:57:40Z | prime main a2 (eedeb161) |

Both a2 pods ran their exit handler (SHA256SUMS and job-exit.json uploaded;
37 and 21 files verify), i.e. they received SIGTERM, not an OOM SIGKILL;
Kubernetes recorded only BackoffLimitExceeded and the pods were gone by
17:50Z. `get-spot-placement-scores` (us-east-2, 1 instance) at 17:55Z:
c8a.2xlarge 1, m8a.2xlarge 2, m7a.2xlarge 2, m7i.2xlarge 2, c7a/c7i/r8a 1.

Not a Spot loss: prime a2's kummer-m5-t3 ended at 20:53.93 wall with
"memory allocation of 162324 bytes failed", peak RSS 8292560 kB, signal 6,
under `prlimit --as=8589934592`: the protocol's 8 GiB memory cap
(inconclusive_cap), reached during degree-7 steps (step 61: nnz 410038654).
