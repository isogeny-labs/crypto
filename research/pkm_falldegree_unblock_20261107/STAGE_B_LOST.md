# PKM stage B (`pfd-b`): lost to two Spot interruptions (recorded 2026-11-09 23:50Z)

Every number is `[pcc:unverified]`; each is measured unless marked.

## What happened

| attempt | job | node | pod started | pod killed | ran |
|:--|:--|:--|:--|:--|--:|
| 1 | `pfd-b-20261108` (card B9-1; Amendment 9 pin) | `r6i.4xlarge`, us-east-2a, spot | 20:36:33Z | 20:52:48Z | 16 min |
| 2 | `pfd-b-20261109-r1` (card BR-1; Amendment 12, no pin) | `m7a.4xlarge`, us-east-2a, spot | 21:03:12Z | 21:34:31Z | 31 min |

Both pods ended with exit 137, pod reason `Terminated`, message "Pod was terminated in response to imminent node shutdown."
(EKS API-server audit log, the nodes' kubelets).

**Both were EC2 Spot interruptions, not Karpenter consolidation.** Karpenter's events on each node (audit log):
`SpotInterrupted` "Spot interruption warning was triggered", `SpotRebalanceRecommendation`, `TerminatingOnInterruption`
"Interruption triggered termination for the Node", then `InstanceTerminating` about two minutes later (20:49:45Z and
20:51:47Z for attempt 1; 21:31:29Z and 21:33:30Z for attempt 2). The pods carried `karpenter.sh/do-not-disrupt`, which
does not stop a Spot interruption.

## What exists, and what is admitted

- Each attempt finished `g1` (exit 0) and started the first stage B closure, `B-m3-tower`. The partial `B-m3-tower.jsonl`
  each uploaded (7,950 bytes and 10,602 bytes) is in S3 under `runs/pfd-b-20261108/` and `runs/pfd-b-20261109-r1/`.
- **Nothing from either attempt is admitted or analysed.** No stage B rule has been read. No one has opened the partial rows.
- Receipts: `receipts/B9/` (attempt 1), `receipts/BR/` (attempt 2, with the price-guard readings).

## Cost guard (Amendment 12), attempt 2

- Start reading, 21:07:54Z: `m7a.4xlarge`, us-east-2a, Spot price 0.2334 $/h; bound `48.25 × (0.2334 + 0.0088)` = US$11.69;
  verdict run (`receipts/BR/price-guard/start.json`).
- Check-in reading, 23:40Z: price 0.2332 $/h; the job had already been lost (`check-20261109T2340Z.json`).
- Estimated spend of the two attempts together: under US$0.5 (derived: about 1.3 instance-hours at 0.24 to 0.33 $/h).

## Where this leaves stage B

Owner decision 3 (`research/open_leads_execution_20261108/DECISIONS.md`): "a lost Spot cell is reported as lost, with at most
one relaunch under a new name". The one relaunch is used, so **stage B is reported as lost**, and the main session does not
relaunch it on its own. The benchmark record `20261109T2031Z-pkm-fd-pfd-b.json` stays open until the owner decides.

What stage B's absence means for the readings (cited from the preregistration and RESULTS files, not new analysis):

- H-LFD on the `(kind, m)` ladders for `m = 3..6` has no reading (RESULTS_STAGE_C.md: "pending `pfd-b`").
- Stage B's own G3 has not run. G3 passed on stage A (144 runs, 0 disagreements) and on stage C's `m <= 6` cells (98 runs, 0
  disagreements).
- The capped runs that decide C-step (`pfd-x-*`) are Kummer `m = 2` cells of stage A. They do not use any stage B row.

## Diagnosis and options for the owner

- The `cpu-spot` pool schedules in us-east-2a only (the scheduler's requirement `topology.kubernetes.io/zone In [us-east-2a]`,
  seen in its messages of 2026-11-08). Two different instance types were interrupted there within 16 and 31 minutes on the
  evening of 2026-11-09. On 2026-11-08 the same pool ran jobs of 3.5 h (`pfd-a`) and 4 h (`pfd-c`, minors) without
  interruption. The cause of tonight's interruption rate is not known (heuristic: tight Spot capacity in that zone).
- `pfd-b` has no checkpoint: `entry.sh` restarts every stage from the beginning, so a job of up to 48 h needs 48 h without
  an interruption.
- Options (the owner's decision; none is taken here):
  1. Allow further relaunches of `pfd-b`, for example one at a time when the zone is calmer, each a cheap loss if interrupted
     early (the two losses so far cost under US$0.5 together).
  2. Allow `pfd-b` to be split into shorter jobs by stage (`hky-B` per `m`, then `d-B`, then `g3`), which needs a dated
     amendment because it changes the job structure, not the cells.
  3. Accept the loss of stage B for this round.
  4. A capacity change (On-Demand or another zone for this pool) is outside the current cost controls and is not proposed here.
