# Launch record (2026-11-10)

Frozen commit (source_revision): `a4db61b2e1143348f7c377ce03d58556ace302a9` (`frozen.sha256` in that commit).
Route: `isolab-experiment.yml` dispatched on `main` (workflow head `a9ca7cb792`) through the GitHub API;
instance `c8a.2xlarge`, `seconds` 3600, not resumable.

| k | spec | dispatched (UTC) | workflow run |
|---|---|---|---|
| 28 | `spec-k28.json` | 2026-11-10T22:21:08Z | 42913750091 (run 215) |
| 32 | `spec-k32.json` | 2026-11-10T22:21:10Z | 42913756188 (run 216) |

Mapping of run to spec is by dispatch order (k = 28 first); confirm from each run's "Read the experiment
spec" step before analysis.

Outcome (checked 23:10Z): both runs failed at "Launch, await and verify the EKS experiment": Kubernetes
Job `gh-42913750091-1` (UID `cd697db9-6dd6-46f6-b8e9-1895a8ba2e12`) state `failed`, `pod_uid` empty, launcher
exit 23; image `isolab/tasks@sha256:54dea54bdac79fb9a325affa0194040b7d5b0704a9661fd99a151900a5c094e7`,
ecbench SHA-256 `3e0cde943dc7e044bbe84b9d7a22f160b74506e1e1422561173c84b3bcdf3033`; receipts at
`runs/gh-42913750091-1/launcher/1f9325fa-8c31-4249-b68f-ed5d0e6ebc7c/` in the lab bucket (not readable from
this session). Run 42913756188 failed the same way. Panel then ran locally under Amendment 1.
