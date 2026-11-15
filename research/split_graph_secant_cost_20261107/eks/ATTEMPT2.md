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

## Attempt 2: local receipt loss (2026-11-08, about 06:45:56Z)

The untracked directory `receipts/A2/` was deleted from the working tree at about 06:45:56Z, while card A2-1's wait was
polling. The cause, reported by the experiment-director: a generator test loop wrote a synthetic launch receipt
to `receipts/A2/launch/result.json`, overwriting the real one, then ran `rm -rf research/split_graph_secant_cost_20261107/receipts/A2`.
The test assumed the directory did not exist.

The wait log in `cards/CARD-...-A2-1.wait-short.stdout` and `.stderr` agrees:
- polls 1–21 are normal;
- poll 22 is `launcher_exit 255`;
- from poll 23 at 06:46:41Z onward, every poll is "No such file or directory".

Recovery and scope:
- **Launch receipt:** `receipts/A2/launch/result.json` was restored byte for byte from the JSON tail of
  `cards/CARD-...-A2-1.launch.stdout`. Its sha256 is 446f8a9cdad73530…, equal to the digest that card A2-1's receipt
  recorded for that output.
- **Lost:** the per-poll status files. The poll log is kept in the card's wait stdout and stderr.
- **Not affected:** the job (sgsecant-panel-20261108-a2) and its S3 outputs.
- **Rule from now on:** card-generator tests use a temporary copy of the tree, never real study paths.

## Attempt 2 outcome: lost unscheduled (recorded 2026-11-08 09:15Z)

Job `sgsecant-panel-20261108-a2` ended `DeadlineExceeded` (EKS audit log) about 09:07Z without ever being scheduled.

- **Every poll showed** the pod `sgsecant-panel-20261108-a2-8q5t6` as `Pending`.
- **Scheduler reason:** "1 node(s) didn't match pod anti-affinity rules".
- **Cause:** the only `cpu-spot` node (r8i.4xlarge, 16 vCPU, which fills the nodepool limit) has held the outside job `pq2-msolve-n37-e63bf8e5-s8kgm` since 05:53Z. That job's anti-affinity keeps other isolab job pods off the node, and the 16 vCPU limit stops Karpenter from adding a second node.
- **Data:** no data exists from attempt 2.

## Attempt 3: `sgsecant-panel-20261108-a3`

- **Request:** `eks/r2img_request_attempt3.json`. It is the attempt-1 request with three changes: the job name, `seconds` 43200 (so a long queue cannot exhaust the deadline), and a provenance note. Everything else is checked identical with `jq -S`.
- **Validation:** VALID.
- **Kept:** attempts 1 and 2 are retained, per PROTOCOL.md's interruption rule.

## Attempt 3 outcome: succeeded, collected from the audit log (recorded 2026-11-08 12:30Z)

Job `sgsecant-panel-20261108-a3` (job uid `517e76f8…`) succeeded.

- **Run:** pod `sgsecant-panel-20261108-a3-l9dtd` (uid `90583573…`) ran 10:46:31–10:51:26Z and exited 0 (reason `Completed`). Source: the EKS API-server audit log; the node's kubelet reported that exit.
- **Lost pod object:** the Pod object was deleted at 11:00:43Z when its node was removed, so every later `command-status` answer has `pods: []`.
- **Collect cards:**
  - **A3-C01** was killed during its first status poll, when the executor's turn ended. It ran only the poll and wrote no receipt; it is kept as-is.
  - **A3-C02** stopped at `verify` with exit 50 ("Job is not succeeded with exactly one pod and pod exit 0"), because `pods` was empty.
  - **A3-V01** completed. It is an audit-collect card. Its `pod-from-audit` step (`cards/status_with_audit_pod.sh`) requires the status `succeeded` with `pods: []`. It then accepts exactly one pod created by the job-controller with an ownerReference to the job uid, and that pod's node-reported exit code. The unchanged `verify_results.sh` then passed: the binary hash line is present and `sha256sum -c` passes on 5 files.
- **Recheck:** `sgsecant_recheck.py` recomputed 600 comparison rows, with 0 mismatches.
- **Raw files:** recorded with `add-raw` and uploaded.
