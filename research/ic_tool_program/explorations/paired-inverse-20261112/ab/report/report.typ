#set page(paper: "a4", margin: 18mm)
#set text(size: 9.5pt)
= Paired inverse (lever 2): same-session A/B with its own A/A

One `ic` binary (`21b0a4f7…`, built at `064334a304`). Three labels a round: *a* and *q*, unpaired (`KIC_SCAN_PAIRED=0`, today's path); *c*, paired (the default). Rows: suite `M1-T01` at `k0n53` (2^44.3) and `k0n61` (2^47.2). 42 rounds, with the order cycled over the six permutations. Each process ran under `isolated_bench --cpus 2` with `RAYON_NUM_THREADS=1` on this session's container (Intel Xeon @ 2.1 GHz, AVX-512, VPCLMULQDQ, GFNI). `PROTOCOL.md` was committed before any row. Container restarts stopped the session twice; it resumed from its rows with nothing redone. Class: engineering. Every output is identical by construction and by check.

== Identity
Zero mismatches: every process of a row gave the same certificates and counts. Dropped rounds (contended): k0n53 3, k0n61 1.

== Result (median of per-round ratios, 95% bootstrap over rounds, exact LCG, high-bit index)
#table(columns: 6, align: (left, left, left, left, left, left),
[row], [metric], [A/B a/c], [A/A a/q], [verdict], [prediction],
[k0n53], [collect], [*1.040* [1.019, 1.059]], [0.982 [0.936, 1.017]], [*established*], [[1.02, 1.08]: held],
[k0n53], [set-up], [1.040 [0.984, 1.078]], [0.978 [0.959, 1.004]], [not established], [[1.00, 1.05]],
[k0n61], [collect], [1.027 [1.000, 1.056]], [1.003 [0.990, 1.017]], [not established], [[1.05, 1.13]: missed],
[k0n61], [set-up], [1.023 [0.991, 1.049]], [1.003 [0.979, 1.014]], [not established], [[1.03, 1.10]])

#image("ab.svg", width: 100%)

== Reading
- *Collect, k0n53:* the paired kernel is faster in the collect phase by about 4%, and the interval's lower end clears the A/A's upper end by 0.002. That is the only established effect.
- *Collect, k0n61:* the median is 2.7%, but its interval touches 1. The exploration's −9.8% there (three processes an arm) did not reproduce.
- *Whole set-up:* not established at either size. The build and selection phases add noise of the same order as the gain.
- *What it is:* a component result on this host class. It is not a cold-time claim against rho, and no board row follows from it.
