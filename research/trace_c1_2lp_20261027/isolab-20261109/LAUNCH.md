# Iso Lab launch: trace-line exact MITM reference panel (2026-11-09)

Frozen before dispatch. **This is the exact MITM correctness-reference design
only (`reference-d*.json`), not the algebraic C1 experiment of
[PROTOCOL.md](../PROTOCOL.md), which stays blocked on the C1 oracle.** Its
results are a stage diagnostic of relation yield and collector cost, not a
priced end-to-end IC result.

| field | value |
|---|---|
| route | GitHub `isolab-experiment.yml`, `workflow_dispatch` on `main` (AWS bridge unavailable: this session's AWS keys fail STS with `InvalidClientTokenId`) |
| source_revision | `f07b6d3e4eb7501ed51271dcd69e06cb543af96d` (main HEAD; the four specs are unchanged there) |
| specs | `research/trace_c1_2lp_20261027/reference-d2.json`, `-d3`, `-d4`, `-d5` (N = 8, 12, 16, 20) |
| per spec | 2 curves (seeds 20261027, 20261127), 8 public targets, arms standard / selected / standard-aa / rho-neg, 1 warmup + 6 rounds, L2 required, 120 s per execution |
| instance / deadline | `c8a.2xlarge`, `seconds = 1800` (includes provisioning) |
| dispatches | 4, one per spec; no automatic resubmission |
| dollar ceiling | USD 10 for all four, failures included |
| local feasibility | development probe of d5 (1 target, 1 round, L0, not a measurement): 8 executions, all verified, 3.2 s wall total |
| benchmark record | `benchmarks/records/20261109T1453Z-trace-line-reference-isolab.json` |
| resume checkpoint | S3 upload impossible (no token or profile); this pushed commit is the durable checkpoint |

## Dispatch log

Filled in after each dispatch: UTC time, workflow run ID, outcome.
- 2026-11-09T14:55:14Z: `reference-d5.json` dispatched, workflow run 42639382601 (run #214, attempt 1), head f07b6d3e4e. The d2–d4 dispatches are held until #214 shows whether the route completes end to end.
- 2026-11-09T15:21:41Z: run 42639382601 **failed** in "Launch, await and verify the EKS experiment". The build and image publish succeeded:
  - image `isolab/tasks@sha256:fe11b397904fcac9d346737f287a4938476296fe8da9a45b7ccab2a83c6353d6`, ecbench SHA-256 `25a1351c272d3c0d78e775e9296eefb9dc66938745a475f646e76697bfa578af`;
  - launcher `isolab-experiment-launcher:8096a7db-d8a1-4b26-ba03-95e9defdaf3b`, request SHA-256 `31a58ab39a4e99981aafad4cdc861c24d7d469d1717ae91f8b7b2be9e1ea9220`.

  Job `gh-42639382601-1` (UID `eba3a8ca-a1c3-4cd5-a506-b278481a2e92`) reached Kubernetes state `failed` about 3 minutes after submission. The launcher's `result.json` has exit code 23 (Job not succeeded), an empty `pod_uid` and an empty `results_uri`. Launcher receipts are at `s3://isolab-results-104186811093-us-east-2/runs/gh-42639382601-1/launcher/8096a7db-d8a1-4b26-ba03-95e9defdaf3b/`.

  The failing gate is not visible from GitHub. The pod exit code sits only in private S3, and this session has no AWS identity that can read it. Possible classes include native audit, L2 isolation, A/A control and payload execution. **No result was produced, and d2–d4 were not dispatched.** Measured rows: 0.
