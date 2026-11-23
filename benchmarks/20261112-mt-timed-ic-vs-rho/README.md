# Multi-target IC vs rho in wall time (2026-11-12)

Study: [`research/mt_timed_20261112/`](../../research/mt_timed_20261112/) (protocol, results, data, independent
validation and rerun). Figures `ratios.svg`, `ratios.png`; report `REPORT.pdf`. Claim PCC-20261112-ec2f3f.

On one pinned CPU of the session's 4-vCPU cloud VM (diagnostic seconds; EKS was unavailable), the fastest
multi-target IC takes 0.78-0.83x the solve time of the fastest multi-target rho at 2^24 (12 of 12 configurations
below 1), is fragile at 2^28 with T = 1024 and unranked at larger T, and takes 1.11-1.29x at 2^32 (12 of 12 above
1); an independent pinned rerun reproduces this. Distinguished-point rho is the fastest rho; BSGS is faster than the
IC everywhere (IC 1.33-1.59x). Measured prices: native operations about 0 GAE, hash lookups 0.04-0.85 GAE by table
size, I/M 23-31. [pcc:unverified]
