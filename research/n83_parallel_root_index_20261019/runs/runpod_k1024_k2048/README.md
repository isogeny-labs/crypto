# Exact N83 root-index parallelism on one RunPod CPU envelope

The four-thread builder preserved the complete base, state, root-table and
first-witness digests at K1024 and K2048. With four pinned CPUs, the median
index-build time fell from 41,665.930 ms to 39,605.441 ms at K1024 (1.0520x),
and from 182,098.662 ms to 167,246.907 ms at K2048 (1.0888x). Each K used
the fixed serial/parallel/parallel/serial order. These are measurements of
target-independent index construction, not target online wall time.

The RunPod Community Tesla V100 PCIe Pod `gi13uqg43bl0w3` exposed an Intel
Xeon Silver CPU. Its allocation cost was $0.19/hour. The stage used the same
pinned CPUs 0-3 and binary SHA-256
`03d17f6f08a09be1917bbdf6b8d6c3dfd44961bb08564a21e7c3d2fe9b471ecd`.
The K1024 report SHA-256 is
`0101f8761e91612a2ef295b28139d808472b31de758f848507b7cb7a45d14518`;
the K2048 report SHA-256 is
`a5dee41a3fde7e070b80a80e1c15ed85297bcee602b54143f9f0c4b1a35d0c11`.
The Pod recorded zero CPU-throttled periods and zero memory-limit denials
across both ABBA panels. The K1024 stage finished at
`2026-10-19T16:27:34Z`; K2048 finished at `2026-10-19T16:41:39Z`.

`k1024/report.json` and `k2048/report.json` retain each run's command,
flags, index-build time, exact digests, and process wall time. The adjacent
`cpu_before.stat`, `cpu_after.stat`, memory-fail counters, binary digest and
`lscpu.txt` preserve the measured resource envelope. `allocation.json` and
the provider stop/terminate receipts bind the Pod lifecycle. The exploratory local
K512/K1024 report remains in `runs/flat_k512_k1024`.
