# Launch configuration: rule v9 on Iso Lab EKS (2026-11-06)

Route selected by the owner on 2026-11-06 after the local first attempt was
lost with its container (`../PROTOCOL.md`, amendment 1). The submitting
identity is `coreuser` in account 104186811093 (STS, 16:5x UTC). The EKS API
admits two fixed CIDRs only and this container has no Docker daemon, so the
layout follows `research/arity_completion_20261104/launch/`:

- **Image:** `ubuntu:24.04` plus one layer, published to `isolab/tasks` by
  digest with `taskq/deploy/eks-karpenter/oci-layer-publish.py`
  ([`build-layer.sh`](build-layer.sh); receipt [`image.json`](image.json)).
  The layer holds the five binaries `../binaries.sha256` names,
  `tree.tar` (`git archive` of `research/ic_tool_program`,
  `research/ic_single_target_20260930` and `research/ic_descent_20260930` at
  the launch commit, which is what `icprog rule` reads), AWS CLI v2, and
  Ubuntu 24.04's `procps` and `jq` packages, all by SHA-256 in `image.json`.
- **Job:** [`job.sh`](job.sh), submitted as `isolab.eks.command-request/v1`
  through `remote/launch-remote-command` ([`make-request.sh`](make-request.sh)
  writes `request-a<N>.json`).

| job | pool | instance | cpu / memory | deadline | retries |
|---|---|---|---|---|---|
| `rule-v9-20261106-a<N>` | `cpu-l2-spot`, profile `l2` | c8a.2xlarge | 4 / 12Gi | 43200 s | 0 (Kubernetes); at most 2 resumptions by new requests |

Rows sync to `s3://isolab-results-104186811093-us-east-2/runs/<job>/<pod-uid>/rule/{v9,v9c}/runs/`
every 60 s; the Job's log, pins, host and binary hashes go to `…/job/`, with
`SHA256SUMS` and `job-exit.json` at exit.

## Identity checks

- `job.sh` stops unless the image's binaries are exactly those
  `binaries.sha256` names, then records them per pod (`binary.sha256`).
- `build-layer.sh` refuses to archive trees that differ from the commit, and
  refuses binaries other than those `binaries.sha256` names.
- Launch inputs are checked with `check-experiment-git` before submission
  (receipt in `receipts/`).
- After the Job: download, check every row against `SHA256SUMS`, then run
  `claims`, `analyse` and `matched` on the checkout.

## Budget

Ceiling **USD 12** for this study: c8a.2xlarge Spot at about USD 0.16/h
over at most 12 h, three attempts at most (USD 5.8), plus CodeBuild launcher
minutes, S3 and ECR storage. Charged against `isolab-daily-cost-100`.

At 17:16 UTC on 2026-11-06 `DescribeBudgets` read that budget at USD 202.84
of 200 for the day (and `isolab-monthly-account-cost` at 759.92 of 1000), so
no Job is submitted on 2026-11-06 (COST_CONTROLS.md: no new launch while the
remainder is exhausted). Submission waits for the daily reset and a fresh
reading with room for the ceiling.

### Owner authorization, 2026-11-10 22:0x UTC

`DescribeBudgets` at 22:02 UTC (data from about 17:50 UTC): `isolab-daily-cost-100`
USD 102.81 of 200; `isolab-monthly-account-cost` USD 1360.11 of 1300 (the
limit had been raised from 1000); `spot-experiments-daily-25` 46.33 of 200.
The monthly budget is exhausted, so COST_CONTROLS.md would hold the launch.
The owner authorized this study's USD 12 ceiling despite the monthly overrun
("Submit anyway (owner OK)", in the session), and request a1 was submitted
on that authorization. No other launch is covered by it.
