# t = 14 (and the lost t = 13 draw 0 cell) on non-Spot capacity: what is needed (2026-11-14)

Owner decision 10 (`research/open_leads_execution_20261108/DECISIONS.md`): PKM t = 14, and any further attempt of the
lost t = 13 draw 0 cell, run on non-Spot capacity that the owner arranges; the main session prepares the requests and
does not launch on Spot. Every number is `[pcc:unverified]`.

## The jobs

| job | request | resources | time limit | expected |
|:--|:--|:--|:--|:--|
| `pfd-x-t14-k0-c6`, `pfd-x-t14-k1-c6` (cap 6, both draws; Amendment 7 B: `c* = 7` at t = 14, so cap 6 runs first) | `requests_r2i/pfd-x-t14-k{0,1}-c6.json` | 12 vCPU, 56Gi, `--max-dense 12800000000` (Amendment 14) | 90,000 s (`--budget 86400`) | longer than t = 13's 3.2 h; may reach the budget |
| cap 7 at t = 14, only if cap 6 does not refute | not generated yet (Amendment 7 B step) | 12 vCPU; memory per Amendment 7 A/14 | 90,000 s | unknown |
| `pfd-x-t13-k0-c6` (fourth attempt; decision 10 reads as allowing it on non-Spot capacity) | the r2h request with a new job name | 12 vCPU, 56Gi | 90,000 s | about 3.2 h on the draw 1 evidence (`RESULTS_T13_BRACKET.md`) |

Measured reference: the t = 13 cap-6 run peaked at 20,618 MB with 4.7e9 dense entries (37% of the 12.8e9 limit).

## Why the lab cluster cannot do it as configured

All experiment pools of the EKS cluster are Spot-only by policy: `taskq/deploy/eks-karpenter/spot-only-policy.yaml`
rejects NodePools that request On-Demand capacity, and `cost-controls.json` gives the on-demand pools zero capacity.
Changing that is an administrative action outside this session (owner decision 5: capacity changes are not forced).

## Routes, for the owner to choose

1. **An On-Demand NodePool on the lab cluster** (one node, 16 vCPU / 64 GiB, for example `m7i.4xlarge`), with the
   Spot-only policy exempting it. The existing requests run unchanged except for a node selector, so the frozen
   image, entry script and S3 layout stay identical. Needs the owner's cluster change; on-demand price is roughly three
   times Spot (estimate).
2. **A RunPod on-demand CPU pod.** The key is configured in this session and `research/runpod_kit_20261112/` runs
   frozen binaries on RunPod CPU pods. This changes the host and the upload path, so it needs a short dated amendment
   (host, route, how `host.txt`, the row file and the log are carried back, same binaries by SHA-256) before the first
   run. No cluster change.

Either way, the requests above are ready, and the first job would be `pfd-x-t14-k0-c6`.
