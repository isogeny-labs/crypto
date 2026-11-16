# Four-rung single-target tournament: queued provisioning report

This is a dated provisioning snapshot, not a completed comparison. Job `ic-boundary-single-20261115-1912`, UID `751ec9e0-1e39-4c6e-a592-cc76c66a59a9`, was created at 2026-11-15 19:24:05 UTC. At the retained snapshot its sole pod is Pending and no evaluation workload has started. Karpenter's EC2 CreateFleet requests fail with `MaxFleetCountExceeded`.

## Request and status

| Rung | Evaluation workloads requested | Tuning workloads per K | Completed | Status |
| --- | ---: | ---: | ---: | --- |
| n=61, a=0 | 64 | 4 | 0 | Queued: worker unavailable |
| n=71, a=0 | 16 | 1 | 0 | Queued: worker unavailable |
| n=73, a=0 | 16 | 1 | 0 | Queued: worker unavailable |
| n=83, a=1 | 16 | 1 | 0 | Queued: worker unavailable |

The original K grids remain unchanged: n=61 uses 300, 400, 500, 600, 700, 800; n=71/73/83 use 200, 600. All are separate one-target workloads, each paired with strong rho on the same public hash-to-curve point. The algorithm and measurement source is `a10cb7b4f8e48f094efe70c31d9a24b6d72aee4a`. Published launch configuration is `ee2052860bcf1b02c08485fa65a84caea75e3be3`.

## Identity, limits and validation

Final ECR image: `sha256:35af97d51319db07a1092ff8b8101b9c57c696720489fd88f623e1c71e111f13`. Image smoke verified Rust 1.99.1, Python 3.11.2, AWS CLI 2.40.3, required utilities, source revision and local dependency manifests. Autolab Python tests passed 52/52; the new shell runner passed ShellCheck. Native producer compilation, online intervals, certificate replay, claim checks, A/A admission and achieved isolation are still untested for this job.

One sequential command Job requests/limits 4 CPUs and 24 GiB on a fixed m8a.2xlarge in cpu-l2-spot. Kubernetes backoffLimit=0, podReplacementPolicy=Failed and FailJob rules prevent experiment replacement after failure/disruption. Ordinary node provisioning reconciliation has made multiple refused CreateFleet calls; no measurement retry or new experiment Job was submitted.

The owner authorized up to USD 20 additional spend despite the monthly ceiling. The absolute deadline is 2026-11-16T08:11:12.640103Z (November 16, 00:11 Pacific). Preparation and queue time consume the original 13-hour window. The pod's script also enforces that absolute deadline. Worker pricing readback is USD 0.48688/hour On-Demand as a conservative reference; execution is Spot-only. Actual billed image/storage charges are not yet reconciled. No measurement worker has been allocated at the snapshot.

## Provisioning diagnosis

The default DescribeFleets listing returned no fleets. This is not an inventory of instant fleets: AWS requires their explicit IDs ([DescribeFleets API](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeFleets.html)). CloudTrail was used to identify older successful instant-fleet IDs. Explicit descriptions of five historical candidate IDs returned InvalidFleetId.NotFound. These observations do not establish the cause or total size of AWS's internal fleet-request count. No fleet, instance, quota, pool, identity or provider was changed during this diagnosis.

The requested comparison remains unresolved until AWS can provision the unchanged job before its original deadline. Node-pool configuration alone does not establish L2 admission; receipts and controls must still qualify any later measurements. There is no algorithm timing, ratio, uncertainty interval, replay result or ledger promotion from this snapshot.

## Evidence and continuation

- [Frozen request](../../request.json), [published Job manifest](../../job.json), [image receipt](../../image.json), [runner](../../run.sh).
- [Job snapshot](job-snapshot.json), [pod snapshot](pods-snapshot.json), [node-claim snapshot](nodeclaims-snapshot.json), [Karpenter error records](fleet-error.jsonl).
- [Queue diagram](queue.svg) and [PDF](report.pdf). The diagram replaces a quantitative chart because no comparable algorithm measurements exist.
- [Private prelaunch archive receipt](../../prelaunch-archive.json): 138,952 bytes; SHA-256 `02d5d3c74881e1155d9eeb4665e66639b058e062a78b1658bc16d507c7b309a3`; upload checksum and independent readback verified.
- Full local build, budget, publication, pricing, CloudTrail and launch receipts: `research/sat_factor_base_review_20260908/autolab/runs/tournament-20261115-single-target/prelaunch/` (unsealed prelaunch evidence).
- A bounded local [watcher](../../watch.sh) observes only the original Job UID, never submits/retries/resumes it, and fetches any terminal S3 outputs and checks their SHA256SUMS. The cluster's deadline applies independently of the local watcher.

Canonical algorithm graphs and scoreboards are unchanged: there is no new measured algorithm result. A later execution or expiry report must preserve this snapshot and describe the terminal state separately.
