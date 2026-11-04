# Launch configuration: arity completion on Iso Lab EKS (2026-11-04)

Route selected by the main session after the owner's AWS identity became valid
(`coreuser`, account 104186811093, 07:26Z). The EKS API endpoint admits two
fixed CIDRs only and this container has no Docker daemon, so:

- **Submission** goes through the CodeBuild bridge with the new command-Job
  launcher `taskq/deploy/eks-karpenter/remote/launch-remote-command`
  (`run-command` submits one `isolab.eks.command-request/v1` Job through
  `bin/submit-job`; `status-command` reads a Job's state, pods, events, log
  tails and S3 listing). The deployed launcher zip is unchanged: the scripts
  travel by sha256 like `run-request`.
- **Image**: `ubuntu:24.04` plus one layer (the two example binaries built at
  the commit in `image.json`, `bc`, GNU `time`, AWS CLI v2, the frozen
  `koblitz_m5_m6/{run_shard.sh,grid.tsv}` and the `job-*.sh` scripts here),
  published to `isolab/tasks` by digest with
  `taskq/deploy/eks-karpenter/oci-layer-publish.py` (no daemon). Receipt:
  `image.json`.
- **Jobs** (`make-requests.sh` writes `request-*.json`):

| job | command | pool | cpu / memory | deadline | what |
|---|---|---|---|---|---|
| `arity-koblitz-m56-<stamp>` | `job-koblitz.sh` | `cpu-l2-spot`, c8a.2xlarge | 4 / 12Gi | 60900 s | shard m2 → m2 gate → shards m5 ∥ m6 (one single-threaded process per cell, `RAYON_NUM_THREADS=1`) |
| `arity-prime-m5-main-<stamp>` | `job-prime-tower.sh main` | `cpu-spot` | 2 / 10Gi | 42000 s | three controls → controls verdict → t = 2, 3 cells (Kummer, isogeny) |
| `arity-prime-m5-t4-<stamp>` | `job-prime-tower.sh t4 <resume prefix of main>` | `cpu-spot` | 2 / 120Gi | 174600 s | host-memory, controls and t3-cap gates → t = 4 cells; only after `main` |

Records sync to `s3://isolab-results-104186811093-us-east-2/runs/<job>/<pod-uid>/`
every 60 s and at exit (`job-exit.json`, `SHA256SUMS`). A Spot loss is retried
by a new request whose command carries the lost pod's prefix as the resume
argument; `run_shard.sh` and `job-prime-tower.sh` skip records that exist
(the protocols allow infrastructure retries only).

## Deviations recorded before launch (no frozen file changes)

1. **No L2 admission for command Jobs.** The ecbench path's L2 gate, native
   replay and A/A admission do not exist for `isolab_k8s_run`-style Jobs
   (`taskq/deploy/eks-karpenter/mcp`: "These VM pods do not establish legacy
   Iso Lab strict fidelity"). Koblitz invalidation rule 6 therefore applies:
   every second measured here is diagnostic; decided/capped verdicts, D values
   and witnesses are exact and unaffected. The pod still runs on the
   `cpu-l2-spot` pool with the v6 isolation profile.
2. **Koblitz concurrency.** The protocol's placement (three 1-vCPU shards
   concurrently) is realised inside one 4-CPU pod: m2 first (the m5/m6 cards'
   gate), then m5 and m6 concurrently. The L2 pool admits one 8-vCPU node.
3. **Prime-tower cells run sequentially in one pod** with the pod's
   `RAYON_NUM_THREADS` (2), as the development smoke did.
4. **Executor host → pod.** The eight cards of `../cards/` were written for an
   executor host with a checkout and cargo; their frozen cell commands are
   reproduced verbatim in `job-*.sh` (paths rooted at `/work/results`), their
   gates as the same Python. The director's launch cards bind the request
   JSONs, the launcher scripts and the image digest instead.

## Budget

Ceiling USD 40 for this campaign (c8a.2xlarge Spot ≈ 0.16 USD/h × ≤ 17 h;
cpu-spot ≤ 12 h at 10Gi; a 128 GiB host ≤ 48 h at ≈ 0.5 USD/h only if t3
decides; CodeBuild minutes). Budgets at 07:30Z: isolab-daily-cost-100 actual
93.9 of 200; spot-experiments-daily-25 actual 37.5 of 200; monthly 379 of 1000.
