# Multi-target IC vs rho in wall time on RunPod, std vs fast hasher (2026-11-12)

Study: [`research/mt_timed_runpod_20261112/`](../../research/mt_timed_runpod_20261112/) (protocol, pod job, receipts,
results, independent validation). Figures `ratios.svg`, `ratios.png`; report `REPORT.pdf`. Claim PCC-20261112-dc58b3.

On one pinned CPU of a RunPod cpu5c pod (AMD EPYC 9575F; shared host, diagnostic seconds), with both the std
(SipHash) and a fast hasher (identical counted operations): the fastest multi-target IC takes 0.70-0.89x the solve
time of the fastest multi-target rho at 2^24 and 0.74-0.95x at 2^28 (cell medians; faster in 12 of 12
configurations per size and build), and 1.05-1.18x (std) / 1.01-1.15x (fast) at 2^32 (slower; one fast cell
unranked). The fast hasher helps both sides and does not move the crossover past 2^32. BSGS is 1.41-1.62x faster
than the IC. [pcc:unverified]
