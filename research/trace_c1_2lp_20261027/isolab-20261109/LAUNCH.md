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

## Failure diagnosis (2026-11-09T15:46Z)

`isolab-failure-diagnosis.yml` was dispatched on main (cae9a2eb82), run 42648006875. All three matrix jobs finished `success`. The entry for 42639382601 is job 129266414007, artifact `isolab-failure-diagnosis-42639382601` (ID 13687878530, zip digest `sha256:d240f39b64ba70d6fcf696cbf7a7d6803aa71fafbf2d8754ae40b59060d39fe4`).

- **S3 read was denied.** Each entry logged `An error occurred (403) when calling the HeadObject operation: Forbidden`, including the two pre-existing entries from 2026-11-01. The GitHub diagnostic role currently cannot read pod payload objects, `SHA256SUMS` included.
- **Fallback classification.** For a 403, `diagnose-failed-run` records `artifact_access = payload_read_unavailable` and classifies from the Kubernetes container exit code. With no exit code it reports `artifact_read_unavailable`. The classification is in the artifact and the run's step summary.
- **Not read from this session.** The artifact download host (`*.blob.core.windows.net`) is refused by this session's egress proxy, so the classification has not been read here. It is still open.
- **Lab-side problem.** Restoring the role's `s3:GetObject` on `runs/*/<pod>/{SHA256SUMS,job-exit.json,l2-gate.json,audit.json,control-comparison.json,...}` is a lab permission change and outside this PR.

## Third route: RunPod CPU pod via REST v2 (2026-11-17, user asked "Use runpod", then "Rest v2")

The Iso Lab route is blocked: CodeBuild queue limit 0, artifact-store keys invalid. This route runs the same four frozen specs, **unchanged**, on one RunPod pod.

| field | value |
|---|---|
| API | `https://api.runpod.io/v2` (REST v2). The key is a proxy-injected network secret; `rest.runpod.io` (v1) answered 502 |
| pod | `cpu5c`, 8 vCPU, image `runpod/base:1.0.2-ubuntu2404`, SECURE cloud, `sleep infinity`; deleted after the run |
| binary | `ecbench` built locally at main `94479f7e3eb9de9ae59b764d2b2b50b3238f0a44`, SHA-256 `5bab9023013f06d18f5e54d390f39731667d36c9a8ec5284035d6dca7df39bc2`; streamed in gzip through `POST /v2/pods/{id}/exec` and SHA-checked on the pod |
| specs | `reference-d2/d3/d4/d5.json` as frozen, run sequentially with `ecbench run --spec … --out …`, then `ecbench verify` per session |
| isolation | A RunPod pod cannot reach L2. The specs still say L2, so the wall-time comparison is **not admitted**. The counted-operation (GAE) comparison, the headline statistic of `ecbench compare`, does not depend on isolation |
| ceiling | USD 5, failures included |
| resume checkpoint | S3 upload is unavailable; this pushed commit is the checkpoint |

### RunPod outcome (2026-11-17T15:31Z)

All four specs ran on pod `Z0jehpw47bv0s5` (d2 9 s, d3 9 s, d4 10 s, d5 15 s). Every session passed `ecbench verify --dir` with 0 problems. The d2 standard arms failed deterministically ("projected small factor base has no columns"). The pod was deleted. Results are in [`../runpod-20261117/README.md`](../runpod-20261117/README.md).
