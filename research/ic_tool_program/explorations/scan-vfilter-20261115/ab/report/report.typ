#set page(paper: "a4", margin: 18mm)
#set text(size: 9.5pt)
= Block-filled filter tests in the aimed scan: same-session A/B with its own A/A

One `ic` binary (`57441859…`, built at `d9d14dbbd7` with `--features scan-probes`) and three labels a round:
- *a* and *q*: `KIC_SCAN_VFILTER=0`, the per-key loop, which hashes and slots each key between its loads;
- *v*: the default, each block's hashes, filter words and bits filled eight lanes at a time (AVX-512F/DQ) before the test loop.

Rows are suite `M1-T01` at `k0n53` (2^44.3) and `k0n61` (2^47.2). There are 42 rounds, cycled over the six orders. Each process ran under `isolated_bench --cpus 2` with `RAYON_NUM_THREADS=1` on this session's container (Intel Xeon @ 2.1 GHz). The busy gate refused 136 launches, which were retried. `PROTOCOL.md` was committed before any row. Admitted keys, answers and probe counts are identical, by test.

== Identity
Zero mismatches; no round dropped.

== Result (median of per-round ratios a/v, 95% bootstrap over rounds)
#table(columns: 5, align: left,
[row], [metric], [A/B (per-key / block)], [A/A (per-key / per-key)], [verdict; prediction],
[k0n53], [*collect*], [*1.086* [1.071, 1.121]], [1.002 [0.980, 1.041]], [*established*; [1.01, 1.07]: missed high],
[k0n53], [set-up], [1.049 [1.027, 1.074]], [1.012 [0.991, 1.043]], [not established (1.027 < 1.043); [1.00, 1.05]: held],
[k0n53], [build], [0.997 [0.963, 1.023]], [1.007 [0.990, 1.033]], [no regression; [0.98, 1.02]: held],
[k0n61], [*collect*], [*1.097* [1.076, 1.115]], [1.007 [0.985, 1.022]], [*established*; [1.01, 1.07]: missed high],
[k0n61], [*set-up*], [*1.086* [1.056, 1.102]], [1.007 [0.992, 1.020]], [*established*; [1.01, 1.06]: missed high],
[k0n61], [build], [0.996 [0.982, 1.014]], [0.999 [0.979, 1.018]], [no regression; [0.98, 1.02]: held])

#image("ab.svg", width: 100%)

== Mechanism: scan stages, ns per summand (median over processes)
#table(columns: 5, align: (left, right, right, right, right),
[stage], [k0n53 per-key], [k0n53 block], [k0n61 per-key], [k0n61 block],
[subtract], [2.21], [2.09], [2.02], [1.98],
[key], [4.48], [4.48], [5.02], [4.88],
[*filter*], [5.19], [*4.08*], [5.55], [*4.37*],
[admitted], [2.07], [2.07], [1.50], [1.48])
Only the filter stage moves, by 1.1–1.2 ns a summand. That is more than the micro-benchmark's 0.5 ns, because the real loop also sheds a per-key probe counter and the hash vector's bookkeeping.

== Decision (frozen rule)
The collect gain is established at both sizes, and no build regression is established. So the block fill stays the default. The whole set-up is established faster at k0n61 (8.6%), where collection is 84% of set-up. At k0n53 its lower end (1.027) misses the A/A's upper end (1.043).

== Scope
A component result on this host class. It is not a cold-time claim against rho, and no board row follows from it (validation gate). Answers and counted work are unchanged; the `ecbench` replay sentinels are run before the PR.
