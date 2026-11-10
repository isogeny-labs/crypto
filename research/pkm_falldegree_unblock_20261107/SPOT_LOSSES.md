# Spot losses in the capped runs (PKM fall-degree study)

Every number is `[pcc:unverified]`. Class: **measured** (EKS API-server audit log `/aws/eks/isolab/cluster`, the status route, S3 listings).
Stage B's two losses are in `STAGE_B_LOST.md`. This file lists losses of the Amendment 7 B capped jobs (`pfd-x-*`).
Rule: owner decision 3 (`research/open_leads_execution_20261108/DECISIONS.md`): a lost Spot cell is reported as lost, with at most one
relaunch under a new name. Nothing from a lost job is admitted or analysed.

| # | job (card) | node | pod started | interruption warning | pod killed | ran | relaunch |
|--:|:--|:--|:--|:--|:--|:--|:--|
| 1 | `pfd-x-t13-k0-c6-20261108` (Y3-1) | `r7a.4xlarge`, us-east-2a, spot | 2026-11-10 03:27:50Z | 03:35:08Z (Karpenter `SpotInterrupted`, NodeClaim `cpu-spot-kdj9r`, node `ip-10-84-23-34`) | 03:38:11Z, exit 137, pod reason `Terminated`, "Pod was terminated in response to imminent node shutdown." | about 10 min | `pfd-x-t13-k0-c6-20261110-r1` (`requests_r2g/pfd-x-t13-k0-c6-r1.json`, equal to the original except job name and provenance note) |
| 2 | `pfd-x-t13-k0-c6-20261110-r1` (Y5-1; the one relaunch of loss 1) | `r6a.4xlarge`, us-east-2a, spot | 2026-11-10 03:45:40Z | 04:02:18Z (Karpenter `SpotInterrupted`, NodeClaim `cpu-spot-twfhv`, node `ip-10-84-24-207`) | 04:05:21Z, exit 137, pod reason `Terminated`, same message | about 20 min | none: owner decision 3 allows one relaunch |


## Cell `pfd-x-t13-k0-c6` (t = 13, draw 0, cap 6): reported lost

- Both attempts were killed by EC2 Spot interruptions before the run wrote a result row (both `runs/X-m2-t13-k0-c6.jsonl` are 0 bytes in S3). Nothing from either is admitted or analysed.
- Under owner decision 3 the cell is reported lost. A third attempt needs the owner.
- **Owner decision 7 (2026-11-10, about 05:25Z):** one third attempt is allowed, at 56Gi under Amendment 14
  (`research/open_leads_execution_20261108/DECISIONS.md`). Job `pfd-x-t13-k0-c6-20261110-r2`, request
  `requests_r2h/pfd-x-t13-k0-c6-r2.json` (equal to `requests_r2h/pfd-x-t13-k0-c6.json` except job name and provenance note; VALID).
  It is submitted after `pfd-x-t13-k1-c6-20261108` has ended, because jobs run one at a time. If it is lost again the cell goes
  back to the owner.
- Money spent on the two attempts: about 30 node-minutes of a 16-vCPU Spot node, well under US$1 (derived from the times above; no price was read).

## Which nodes were interrupted (measured, EKS API-server audit log, node labels)

| job | memory request | node type | outcome |
|:--|:--|:--|:--|
| `pfd-b-20261108` (2026-11-09 20:36Z) | 56Gi, pinned | `r6i.4xlarge` | interrupted after 16 min |
| `pfd-b-20261109-r1` (21:03Z) | 56Gi | `m7a.4xlarge` | interrupted after 31 min |
| another lab job (23:36Z) | 64Gi | `r6a.4xlarge` | interrupted after about 50 min |
| `pfd-x-t12-k0-c5`, `-k1-c5` (2026-11-10 00:27Z, 01:25Z) | 56Gi | `m7i.4xlarge` | ran 14 s and 12 s; not interrupted |
| `pfd-x-t12-k0-c6`, `-k1-c6` (01:34Z, 02:25Z) | 56Gi | `m7i.4xlarge` | ran 30 min each; not interrupted |
| `pfd-x-t13-k0-c5` (03:09Z) | 112Gi | `r6i.4xlarge` | ran 17 s; not interrupted |
| `pfd-x-t13-k0-c6-20261108` (03:27Z) | 112Gi | `r7a.4xlarge` | interrupted after 10 min |
| `pfd-x-t13-k0-c6-20261110-r1` (03:45Z) | 112Gi | `r6a.4xlarge` | interrupted after 20 min |
| another lab job (04:19Z; 4 CPU, 4Gi) | 4Gi | not read | its pod left the node before 07:09Z; cause not read |
| `pfd-x-t13-k1-c6-20261108` (07:12Z; Amendment 14) | 56Gi | `m8i.4xlarge` | ran 3.2 h; not interrupted |

- Every node with 128 GiB (r family) that ran longer than a minute was interrupted within about 50 minutes (4 of 4). The two 30-minute runs on `m7i.4xlarge` (64 GiB) were not. One `m7a.4xlarge` was interrupted after 31 minutes.
- Added 2026-11-10 12:00Z: the first job under Amendment 14 (56Gi) ran 3.2 hours on `m8i.4xlarge` without an interruption. One run; it is in line with the pattern above and does not establish it.
- That is 7 nodes. It suggests that a 112Gi request, which only r-family nodes can hold, is more exposed than a 56Gi request; it does not establish it.
- The t = 13 and t = 14 capped requests ask for 112Gi (Amendment 7 A). Amendment 14 (`AMENDMENT_R2_F_mem56.md`) addresses that for the cells not yet launched.

## Loss 1: what exists and what is done with it

- Job state `failed` (status route, `receipts/Y3/status/`, poll 4 at 03:39:03Z). The row file `runs/X-m2-t13-k0-c6.jsonl` in S3 is empty (0 bytes): the run wrote no result row.
- A partial F4 log (`runs/X-m2-t13-k0-c6.log`, through step 26) was synced to S3 before the kill. It is **not admitted and not analysed**.
- Disclosure: I read the last line of that partial log while diagnosing the failure. Its step-26 counters equal step 26 of the t = 12 cap-6
  log. I state here, before the relaunch has data, what that makes me expect; it changes no rule and is not a result:
  - the relaunch may take about 30 minutes, as the t = 12 cap-6 runs did, not hours;
  - my earlier expectation of "hours" compared an uncapped row at t = 11 with capped rows at t = 12, which are different computations;
  - whether cap 6 refutes at t = 13 is unknown: the last four steps of the t = 12 runs are where the refutation appeared, and the lost
    run did not reach them.
- Other interruptions seen on the same pool since 2026-11-09 20:00Z (measured): `pfd-b-20261108` after 16 min, `pfd-b-20261109-r1` after
  31 min, and another lab job's node at 2026-11-10 00:26:44Z. The four t = 12 capped jobs and the two t = 13 cap-5 jobs were not interrupted.

## Correction (2026-11-10 12:00Z): the "about 30 minutes" expectation was wrong

- Under "Loss 1" I wrote that the relaunch "may take about 30 minutes, as the t = 12 cap-6 runs did", because the lost run's step-26
  counters equalled the t = 12 log's.
- The completed run `pfd-x-t13-k1-c6-20261108` (draw 1) took 11,570 s (3.2 h) and 32 steps. Its log equals the t = 12 cap-6 log through
  step 26 and differs from step 27 on (113,847 columns against 103,785). `max_dense_entries` was 4,717,115,340 and peak memory
  20,618 MB, against 1,357,804,954 and 6,506 MB at t = 12 (measured).
- So a match of the first 26 steps said nothing about the rest. The statement that whether cap 6 refutes at t = 13 was unknown stands.
- Consequence for Amendment 14: the 56Gi request was enough for this run with room to spare (about 20 GB peak; 4.7e9 dense entries
  against the 12.8e9 limit). The amendment's rules did not depend on the expectation, and its limit rule was not triggered.
