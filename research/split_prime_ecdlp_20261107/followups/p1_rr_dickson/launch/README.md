# P1 round 3 launch: T24 m5 t4 on Iso Lab EKS (2026-11-08)

Protocol: `../PROTOCOL-R3.md`. The route follows the command-Job precedent in
`research/arity_completion_20261104/launch/`:

- **Submission:** `taskq/deploy/eks-karpenter/remote/launch-remote-command` through the
  CodeBuild bridge.
- **Image:** `ubuntu:24.04` plus one layer, built by `build_image.sh` and published with
  `oci-layer-publish.py`. The receipts are `image.json` and `build-info.json`. The layer holds:
  - msolve 0.6.5 from the noble `.deb`, with sha256 equal to the R2b bind;
  - its libraries, plus bc, procps and AWS CLI v2;
  - p1tool, built `--locked` from the source commit;
  - the frozen drivers.
- **Requests:** written by `make-requests.sh` and validated with
  `research/pkm_falldegree_unblock_20261107/validate_command_request.sh`.

| job | command | cpu / memory | deadline |
|---|---|---|---|
| `p1r3-smoke-20261108` | the run_cells_r3 driver (RR, TLIMIT 20 s) plus an S3 upload; ran on the first image `sha256:7f2fa5f7…` (succeeded, instance sha matched, `bc` missing libreadline so wall was empty; fixed in the current image) | 2 / 4Gi | 1,800 s |
| `p1r3-rr-a1-20261108` | `job-r3.sh rr` (VLIMIT 60 GiB, TLIMIT 86,400 s) | 2 / 64Gi | 100,800 s |
| `p1r3-t-a1-20261108` | `job-r3.sh t` (VLIMIT 56 GiB, TLIMIT 86,400 s) | 2 / 60Gi | 100,800 s |

## Budget (recorded before submission)

AWS Budgets readback at 2026-11-08 20:5xZ:

| budget | spent | limit |
|---|---|---|
| `isolab-monthly-account-cost` | USD 1,156.59 | USD 1,100 (exhausted) |
| `isolab-daily-cost-100` | USD 150.92 | USD 200 |
| `spot-experiments-daily-25` | USD 48.44 | USD 200 |

COST_CONTROLS.md says no new job may start while the monthly budget is exhausted. The owner
explicitly authorized going over it for this run (2026-11-08, answering the session's
question: "Launch on EKS anyway"). The ceiling is **USD 60 for all attempts**, failures
included. The estimate is USD 10–25: two Spot nodes with about 128 GiB each, for about 28 h.

The limits for this run:

- At most two jobs at a time, each 2 vCPU, 124Gi of memory in total.
- At most two resubmissions per system after a Spot loss, under new names
  (`make-requests.sh a2|a3`).
- No new launch once the charged cost would pass USD 60, or when it is unknown.

Results: `s3://isolab-results-104186811093-us-east-2/runs/<job>/<pod-uid>/`.
Launcher receipts: `receipts/`.

### Owner update (2026-11-08, after launch): "Exceed target budget"

The USD 60 target is lifted. The owner authorized spending past it. Spending stays finite because
the protocol limits the attempts:

- at most three attempts per system (a1 plus two resubmissions);
- each attempt runs at most 100,800 s;
- at most two jobs run at a time.

That is at most six node-runs of 28 h each. At about USD 0.4–0.7 per hour for a 128 GiB Spot node,
the worst case is roughly USD 70–120. The memory sizes are unchanged: the namespace quota is 128 GiB
for both jobs together, and raising it would be a separate cluster change.

## Attempt log

| attempt | job | outcome |
|---|---|---|
| RR a1 | `p1r3-rr-a1-20261108`, pod ce76e299… | **Spot loss.** CloudTrail recorded `BidEvictedEvent` at 21:35:35Z for i-0c270755aeadb845c. The last S3 sync was at 21:36:02Z, and a status request returned the Job as `failed` with no pod. msolve was still in its degree-7 step. The partial records are in `receipts/rr-a1-results-partial/`; they are kept and supply no bound (PROTOCOL-R3). Resubmitted as a2. |
| RR a2 | `p1r3-rr-a2-20261108`, job 4c60cc68…, pod 45447107… | running (submitted 22:30Z) |
| T a1 | `p1r3-t-a1-20261108`, pod 117e17e4… | **Spot loss.** CloudTrail recorded `BidEvictedEvent` at 22:51:20Z for i-038d43f9a77c1938d. The last S3 sync was at 22:51:30Z, and a status request at 23:2xZ returned `failed` with no pod. msolve was in its degree-9 step. Partial records: `receipts/t-a1-results-partial/`, kept and supplying no bound. Resubmitted as a2. |
| RR a2 | `p1r3-rr-a2-20261108`, job 4c60cc68…, pod 45447107… | Pending 22:30Z → 08:36Z (about 10 h), because pool `cpu-spot` was at its node limit (Karpenter event: "node limits have been exhausted for nodepool"). Ran from 08:36Z, instance sha matched. **Spot loss:** `BidEvictedEvent` 11:09:30Z for i-02123e2feff6642a7, last sync 11:09:53Z, still in the degree-7 step. Partial records: `receipts/rr-a2-results-partial/`, kept and supplying no bound. Resubmitted as a3. |
| T a2 | `p1r3-t-a2-20261108`, job 8fce259e…, pod 828468c7… | Pending 23:37Z → 02:38Z. **Completed as a recorded result:** msolve hit the 56 GiB `ulimit -v` cap (exit 139) after 21,267 s in the degree-9 step, giving D ≥ 9 and widest ≥ 1,305,345 (847,295 × 1,305,345). Instance sha matched. `job-exit.json` exit 0 at 08:33:10Z. Records: `receipts/t-a2-results/`. |
| RR a3 (last) | `p1r3-rr-a3-20261108`, pinned `instance_type: r6a.4xlarge` | AWS Spot Advisor for us-east-2, read 2026-11-09, lists r6a.4xlarge at <5 % interruption and r7a.4xlarge, which Karpenter chose before, at >20 %. The pin changes only where the job runs; the workload and every frozen value in PROTOCOL-R3 are the same. See receipts/rr-a3. |

### NodePool limit (2026-11-09)

The owner chose to raise the `cpu-spot` limit, and approved a temporary /24 on the EKS endpoint
allowlist to do it. Before acting I re-checked: T a2 had finished, and no cpu-spot node was
running, so RR a3 fits the existing limit of 16 vCPU / 128Gi on its own. **The raise was not
applied, and the endpoint allowlist was not touched.**
