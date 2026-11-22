#set page(paper: "a4", margin: 18mm)
#set text(size: 9.5pt)
= Transparent huge pages for the pair table's scanned arrays: same-session A/B with its own A/A

One `ic` binary (built at `35eb722035` with `--features scan-probes`) and three labels a round:
- *a* and *q*: `KIC_HUGE_PAGES=0`, the table's arrays on 4 KiB pages;
- *h*: the default, `madvise(MADV_HUGEPAGE)` on the 2 MiB-aligned spans of `bucket_start`, `rests` and the presence filter, right after allocation.

The arrays, answers and counts are the same in both arms. Rows are suite `M1-T01` at `k0n53` and `k0n61`. There are 24 rounds, cycled over the six orders. Each process ran under `isolated_bench --cpus 2` with `RAYON_NUM_THREADS=1` on this session's container, with THP set to `madvise`. The busy gate refused 80 launches, which were retried. `PROTOCOL.md` was committed before any row.

== Identity
Zero mismatches. One k0n53 round was dropped as contended, leaving 23 and 24 rounds.

== Result (median of per-round ratios a/h, 95% bootstrap over rounds)
#table(columns: 5, align: left,
[row], [metric], [A/B (off / on)], [A/A (off / off)], [verdict; prediction],
[k0n53], [*filter stage*], [1.067 [0.989, 1.086]], [[0.957, 1.050]], [not established; [0.97, 1.08]: held],
[k0n53], [admitted stage], [1.076 [1.010, 1.113]], [[0.954, 1.070]], [not established; [0.97, 1.08]: held],
[k0n53], [collect], [1.029 [1.001, 1.066]], [[0.988, 1.030]], [not established; [0.98, 1.04]: held],
[k0n53], [build], [1.075 [1.031, 1.118]], [[0.977, 1.109]], [not established, no regression; [0.90, 1.05]: missed (above)],
[k0n53], [set-up], [1.055 [1.017, 1.074]], [[0.996, 1.111]], [not established; [0.97, 1.03]: missed (above)],
[k0n61], [*filter stage*], [*1.116* [1.098, 1.174]], [[0.949, 1.016]], [*established*; [1.03, 1.15]: held],
[k0n61], [*admitted stage*], [*1.084* [1.073, 1.108]], [[0.974, 1.011]], [*established*; [1.02, 1.12]: held],
[k0n61], [*collect*], [*1.047* [1.025, 1.071]], [[0.962, 1.014]], [*established*; [1.00, 1.06]: held],
[k0n61], [build], [1.036 [0.991, 1.072]], [[0.961, 1.028]], [not established, no regression; [0.90, 1.05]: held],
[k0n61], [*set-up*], [*1.040* [1.027, 1.070]], [[0.964, 1.019]], [*established*; [0.99, 1.05]: held])

Filter stage, cycles a summand, median over rounds: 8.77 → 8.29 (k0n53) and 9.38 → 8.30 (k0n61).

#image("ab.svg", width: 100%)

== Decision (frozen rule)
Filter, admitted, collect and whole set-up gains are established at k0n61, and no build or set-up regression is established at either size, so the advice stays on by default. At k0n53 every median points the same way, but the A/A intervals are wider than the effect.

The feared cost, direct compaction and huge-page zeroing in the build, did not show: the build medians are 1.075 and 1.036, both in huge pages' favour, though not established.

== Scope
This is a component result for this host class, with THP in `madvise` mode; with THP off the advice does nothing. It makes no claim against rho and adds no board row (validation gate). Answers and counted work are unchanged.
