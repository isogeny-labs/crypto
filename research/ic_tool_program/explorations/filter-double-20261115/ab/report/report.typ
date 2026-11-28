#set page(paper: "a4", margin: 18mm)
#set text(size: 9.5pt)
= A doubled presence filter for CPU (Top-layout) pair tables: same-session A/B with its own A/A

One `ic` binary (built at `2826b353f9` with `--features scan-probes`) and three labels a round:
- *a* and *q*: `KIC_FILTER_DOUBLE=0`, today's filter width (8 bits a stored key, rounded up to a power of two);
- *d*: the default, one bit wider for a `Top`-layout table, which doubles the filter.

`Low` tables (the table files, the GPU builder and `ecbench`'s methods) keep today's width. The stored pairs, answers and reported counts are the same in both arms. Rows are suite `M1-T01` at `k0n53` and `k0n61`. There are 24 rounds, cycled over the six orders. Each process ran under `isolated_bench --cpus 2` with `RAYON_NUM_THREADS=1` on this session's container. The busy gate refused 75 launches, which were retried. `PROTOCOL.md` was committed before any row.

== Identity
Zero mismatches, and no round dropped: 24 rounds at each size.

== Result (median of per-round ratios a/d, 95% bootstrap over rounds)
#table(columns: 5, align: left,
[row], [metric], [A/B (single / doubled)], [A/A (single / single)], [verdict; prediction],
[k0n53], [*collect*], [*1.100* [1.063, 1.125]], [0.994 [0.976, 1.018]], [*established*; [1.04, 1.12]: held],
[k0n53], [filter + admitted], [*1.294* [1.226, 1.342]], [[0.951, 1.054]], [established; [1.15, 1.30]: held],
[k0n53], [admitted], [*2.607* [2.483, 2.717]], [[0.962, 1.053]], [established; [2.0, 2.8]: held],
[k0n53], [build], [1.012 [0.982, 1.043]], [[0.915, 1.022]], [no regression; [0.95, 1.03]: held],
[k0n53], [*set-up*], [*1.062* [1.046, 1.081]], [[0.971, 1.017]], [*established*; [1.02, 1.08]: held],
[k0n61], [*collect*], [*1.079* [1.063, 1.115]], [0.981 [0.964, 1.016]], [*established*; [1.04, 1.12]: held],
[k0n61], [filter + admitted], [*1.210* [1.176, 1.236]], [[0.969, 1.024]], [established; [1.12, 1.30]: held],
[k0n61], [admitted], [*2.574* [2.500, 2.654]], [[0.963, 1.015]], [established; [2.2, 3.0]: held],
[k0n61], [build], [0.987 [0.936, 1.054]], [[0.980, 1.054]], [no regression; [0.95, 1.03]: held],
[k0n61], [*set-up*], [*1.066* [1.045, 1.099]], [[0.968, 1.036]], [*established*; [1.03, 1.10]: held])

Filter + admitted stage, cycles a summand, median over rounds: 12.45 → 9.48 (k0n53) and 11.34 → 9.43 (k0n61).

#image("ab.svg", width: 100%)

== Decision (frozen rule)
The collect gain is established at both sizes, and no build or set-up regression is established, so the doubled filter is the default for `Top` tables. The whole set-up gain, 6.2% and 6.6%, is established too. Every prediction held.

The wider filter admits about a fifth as many absent keys: an admitted key was 2.8% and 2.0% of summands, and is now about 0.6% and 0.4%. That more than pays for the larger filter's slightly costlier probe. With huge pages (\#4316), the wider filter did not slow the build.

== Scope
This is a component result on this host class (Intel Xeon container, THP `madvise`). It makes no claim against rho and adds no board row (validation gate). Answers and the counts `ic price` reports are unchanged. A `Top` table's filter memory doubles (for example, 8 MB to 16 MB at 2^47.2). `Low` tables are unchanged, and the replay sentinels reproduce 51 of 51 sessions.
