# Spot losses in the capped runs (PKM fall-degree study)

Every number is `[pcc:unverified]`. Class: **measured** (EKS API-server audit log `/aws/eks/isolab/cluster`, the status route, S3 listings).
Stage B's two losses are in `STAGE_B_LOST.md`. This file lists losses of the Amendment 7 B capped jobs (`pfd-x-*`).
Rule: owner decision 3 (`research/open_leads_execution_20261108/DECISIONS.md`): a lost Spot cell is reported as lost, with at most one
relaunch under a new name. Nothing from a lost job is admitted or analysed.

| # | job (card) | node | pod started | interruption warning | pod killed | ran | relaunch |
|--:|:--|:--|:--|:--|:--|:--|:--|
| 1 | `pfd-x-t13-k0-c6-20261108` (Y3-1) | `r7a.4xlarge`, us-east-2a, spot | 2026-11-10 03:27:50Z | 03:35:08Z (Karpenter `SpotInterrupted`, NodeClaim `cpu-spot-kdj9r`, node `ip-10-84-23-34`) | 03:38:11Z, exit 137, pod reason `Terminated`, "Pod was terminated in response to imminent node shutdown." | about 10 min | `pfd-x-t13-k0-c6-20261110-r1` (`requests_r2g/pfd-x-t13-k0-c6-r1.json`, equal to the original except job name and provenance note) |

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
