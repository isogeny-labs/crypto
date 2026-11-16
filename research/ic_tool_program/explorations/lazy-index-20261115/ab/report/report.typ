#set page(paper: "a4", margin: 18mm)
#set text(size: 9.5pt)
= Lazy point-index map in the collector and the descent solver: same-session A/B with its own A/A

One `ic` binary (`fde5bce6…`, built at `58295b01c8` with `--features scan-probes`) and three labels a round:
- *a* and *q*: `KIC_LAZY_INDEX=0`, the map built at construction;
- *l*: the default, built on first use, which the aimed pair-table collection and descent never reach.

Rows are suite `M1-T01` at `k0n53` and `k0n61`. There are 24 rounds, cycled over the six orders. Each process ran under `isolated_bench --cpus 2` with `RAYON_NUM_THREADS=1` on this session's container. The busy gate refused 78 launches, which were retried. `PROTOCOL.md` was committed before any row.

== Identity
Zero mismatches. One k0n61 round was dropped as contended, leaving 24 and 23 rounds.

== Result (median of per-round ratios a/l, 95% bootstrap over rounds)
#table(columns: 5, align: left,
[row], [metric], [A/B (eager / lazy)], [A/A (eager / eager)], [verdict; prediction],
[k0n53], [*collect-setup*], [*2.795* [2.697, 2.896]], [0.991 [0.937, 1.012]], [*established*; [2.0, 3.5]: held],
[k0n53], [*descent-setup*], [*1.486* [1.445, 1.546]], [[0.959, 1.057]], [*established*; [1.1, 1.8]: held],
[k0n53], [set-up], [1.034 [0.981, 1.070]], [0.995 [0.949, 1.045]], [not established; [1.00, 1.05]: held],
[k0n61], [*collect-setup*], [*3.031* [2.831, 3.088]], [1.004 [0.976, 1.018]], [*established*; [2.5, 4.0]: held],
[k0n61], [*descent-setup*], [*1.595* [1.554, 1.618]], [[0.985, 1.011]], [*established*; [1.2, 1.9]: held],
[k0n61], [set-up], [1.014 [0.994, 1.038]], [1.012 [0.992, 1.031]], [not established; [1.00, 1.03]: held])

Collect-setup, median over rounds: 10.36 → 3.68 ms (k0n53) and 15.65 → 5.22 ms (k0n61).

#image("ab.svg", width: 100%)

== Decision (frozen rule)
The collect-setup gain is established at both sizes, and no set-up regression is established, so lazy stays the default. The whole set-up gains about 7–10 ms, 1–3%, under this host's resolution, as predicted.

== Scope
A component result for two set-up phases on this host class. It makes no claim against rho and adds no board row (validation gate). Answers and counted work are unchanged: building the map is not counted work, and its contents are the same whenever it is read.
