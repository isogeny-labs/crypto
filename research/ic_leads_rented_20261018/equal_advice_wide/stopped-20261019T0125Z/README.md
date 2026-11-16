# P4-min (equal-advice, n = 83 only): stopped at 01:22Z, BL precompute could not finish inside the pod's cap

Pod `mhjde0s1ysa5aq`, RunPod `cpu3g-32-128` (32 vCPU, 119 GiB cgroup), $1.28/h,
booted 22:08Z from `claude/ic-leads-rented-launch-ready` (PR #2225), job
`ea-wide` with `RUNGS=n83`, lifetime cap 6 h. Stopped by the launching session
at 01:22Z (3 h 14 min, about $4.15) and terminated. **No result**: the frozen
protocol (`research/equal_advice_wide_20261018/PROTOCOL.md`, `run.sh`) never
left its first phase, so no IC arm, claim or decision exists.

## Recorded

- Preflight and build passed (`STATUS`, `n83-preflight.json`).
- BL arm, table precompute at 32 threads (`n83-bl.stderr`): 240,428,492 of the
  934,680,064 distinguished points after 11,166 s, about 3.74 × 10⁹ walk steps
  per 590 s epoch, constant. The new DPs per epoch fell from 14.47 M (epoch 1)
  to 11.22 M (epoch 19), a factor 0.9862 per epoch.
- The scalar sidecar was deliberately not fetched; only logs, the preflight,
  the point list and the host record were copied.

## Why it was stopped (*derived*)

| assumption | hours to 934.7 M DPs | precompute cost on this host |
|---|---|---|
| gain per epoch stays at 11.2 M | 13.3 | $17 |
| gain keeps falling ×0.9862 per epoch (`bl-progress.png`) | 26.8 | $34 |

Both are far past the 6 h cap: the pod would have been killed at about
04:08Z still precomputing, with nothing usable. The card's estimate (BL walks
1.4–3.5 h) was low by a factor of 4 to 8 on this host.

## What a real attempt needs

A host that can run the BL precompute to completion: about 13–27 h at 32
threads (fewer hours with more cores; the walks are embarrassingly parallel),
then the IC arm. On RunPod `cpu3g-32-128` that is $17–$34 for the
precompute alone; an AWS spot c7i/c7a with 64+ vCPU
(`../../aws_spot/aws_spot.py`) would roughly halve the hours. The card's
cost table should be corrected before any relaunch. The frozen protocol is
unchanged.

Graph: `bl-progress.svg` / `.png` (`plot.py`).
