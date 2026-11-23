#set page(paper: "a4", margin: 18mm)
#set text(size: 9.5pt)
= Split min chains in the least-rotation kernel: same-session A/B with its own A/A

One `ic` binary (`1d02c642…`, built at `939ededac9` with `--features scan-probes`) and three labels a round:
- *a* and *q*: `KIC_KEY_CHAINS=1`, one chain of minima a vector, today's kernel;
- *k*: the default at that commit, odd and even rotations in two chains.

Rows are suite `M1-T01` at `k0n53` and `k0n61`. There are 42 rounds, cycled over the six orders. Each process ran under `isolated_bench --cpus 2` with `RAYON_NUM_THREADS=1` on this session's container (Intel Xeon @ 2.1 GHz). The busy gate refused 133 launches, which were retried. `PROTOCOL.md` was committed before any row. Keys are identical, by test.

== Identity
Zero mismatches; no round dropped.

== Result (median of per-round ratios a/k, 95% bootstrap over rounds)
#table(columns: 5, align: left,
[row], [metric], [A/B (one / split)], [A/A (one / one)], [verdict; prediction],
[k0n53], [*collect*], [1.020 [1.006, 1.034]], [0.992 [0.967, 1.030]], [not established; [1.00, 1.05]: held],
[k0n53], [build], [1.030 [0.984, 1.041]], [1.004 [0.984, 1.030]], [no regression; [1.00, 1.03]: held],
[k0n53], [set-up], [1.029 [1.009, 1.048]], [1.000 [0.966, 1.040]], [not established; [1.00, 1.04]: held],
[k0n61], [*collect*], [1.019 [1.004, 1.035]], [0.993 [0.983, 1.009]], [not established; [1.00, 1.05]: held],
[k0n61], [build], [1.001 [0.993, 1.022]], [0.997 [0.978, 1.014]], [no regression; [1.00, 1.03]: held],
[k0n61], [set-up], [1.016 [1.005, 1.028]], [0.996 [0.981, 1.005]], [not established (equal ends); [1.00, 1.05]: held])

#image("ab.svg", width: 100%)

== Reading
- *Direction:* the medians move as predicted, about 2% in collection at both sizes. The build's key step falls from 4.35 to 4.23 and from 4.87 to 4.63 ns per pair.
- *Not established:* no collect interval clears its A/A. At k0n61 the lower end (1.004) is under the A/A's upper end (1.009). At k0n53 the A/A is wide, with an upper end of 1.030.
- *Decision (frozen rule):* the primary metric is not established, so the default reverts to one chain. The split kernel stays as an option, `KIC_KEY_CHAINS=2`.

== Also explored and dropped (micro-benchmark only, `microbench/two_level_filter.rs`)
- *The idea:* a two-level presence filter, an L2-sized first level (1–3 bits a key) in front of today's 8 MB filter.
- *Result:* with branch-free compaction and prefetching, two levels cost 5.2–5.4 ns a query against 4.9–5.0 ns for one.
- *Why:* the probe's floor is about 2.3 ns even when the filter fits L1, so at most about 3 ns a probe is memory, and the extra pass costs about that much.

== Scope
A component measurement on this host class, with no board row.
