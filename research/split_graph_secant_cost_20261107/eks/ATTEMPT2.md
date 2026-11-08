# Split-graph secant panel: attempt 1 lost unscheduled; attempt 2 prepared (2026-11-08)

## Attempt 1: `sgsecant-panel-20261107`, card R2-1, measured

| event | time / value |
|---|---|
| Job created | 2026-11-08T04:59:57Z |
| Pod scheduling | `sgsecant-panel-20261107-9lfxh` stayed `Pending` (`node: null`) in every one of 36 status polls |
| Deadline | `activeDeadlineSeconds` 3600 |
| End | `DeadlineExceeded`, pod deleted at 05:59:57Z |
| Output | the job prefix in S3 holds only `launcher/`; no panel ran |

Sources: `receipts/R2/status/`, the launcher receipt at
`s3://isolab-results-104186811093-us-east-2/runs/sgsecant-panel-20261107/launcher/14b54813-7a86-45b1-99f8-a1da049791aa/`
(`events-job.json`, `job.json`), and the card receipt `cards/CARD-split-graph-secant-cost-20261107-R2-1.receipt.json`
(stopped at `verify`).

## Cause (inferred, not proved)

The status route returns no scheduling reason. The `cpu-spot` pool was contended over the same window: the mllc
job (8 vCPU) also stayed `Pending` from about 05:03Z until 05:47Z. Karpenter counts whole node capacity, not pod
requests, against the 16 vCPU / 128 GiB nodepool limit, and other sessions share the cluster. Kubernetes counts queue
time against `activeDeadlineSeconds`.

## Attempt 2: `sgsecant-panel-20261108-a2`

- **Request:** `eks/r2img_request_attempt2.json`. It is the attempt-1 request with three changes: the job name, `seconds`
  raised from 3600 to 10800 so queueing cannot exhaust it, and a provenance note. The image, command, protocol hash, cpu and
  memory are identical, checked with `jq -S` against everything except those three fields.
- **Validation:** VALID under `eks/r2img_validate.sh`.
- **Why this is not a rerun after seeing data:** attempt 1 produced no data. PROTOCOL.md's interruption rule is "a full
  rerun under a new job name with both attempts retained", and attempt 1's receipts are kept in full.
- **Launch:** through a director card once `cpu-spot` capacity is free, never alongside a 14-vCPU job.

## Attempt 2: local receipt loss (2026-11-08, about 07:18Z)

The untracked directory `receipts/A2/` was removed from the working tree while card A2-1's wait was polling. The wait
log shows "No such file or directory" from poll 55 at 07:18:41Z onward, in `cards/CARD-...-A2-1.wait-short.stdout`.
The cause is under investigation and was not a git operation, since the directory was never tracked.

- **Launch receipt:** `receipts/A2/launch/result.json` was restored byte for byte from the JSON tail of
  `cards/CARD-...-A2-1.launch.stdout`. Its sha256 is 446f8a9cdad73530…, equal to the digest that card A2-1's receipt
  recorded for that output.
- **Lost:** the per-poll status files for polls 1–54. The wait step's stdout and the card receipt keep the poll log.
- **Not affected:** the job (sgsecant-panel-20261108-a2) and its S3 outputs.
