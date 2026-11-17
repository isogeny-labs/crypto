#set page(paper: "a4", margin: 18mm)
#set text(size: 9.5pt)
= Partition-local presence filter: same-session A/B with its own A/A

One `ic` binary (`430734a6…`, built at `39affc25bf` with `--features scan-probes`) and three labels a round:
- *a* and *q*: `KIC_FILTER_LAYOUT=low`, today's layout;
- *t*: the default, `FilterLayout::Top`. The filter word comes from the hash's top bits, the bits that also pick the build's partition.

Rows are suite `M1-T01` at `k0n53` (2^44.3) and `k0n61` (2^47.2). There are 42 rounds, cycled over the six orders. Each process ran under `isolated_bench --cpus 2` with `RAYON_NUM_THREADS=1` on this session's container (Intel Xeon, AVX-512). The busy gate refused 141 launches, which were retried and are not measurements. `PROTOCOL.md` was committed before any row. Class: engineering. Outputs are identical by construction and by check.

== Identity
Zero mismatches: every process of a row gave the same certificates and counts. One k0n53 round was dropped as contended, leaving 41 and 42 rounds.

== Result (median of per-round ratios a/t, 95% bootstrap over rounds)
#table(columns: 5, align: left,
[row], [metric], [A/B (low / top)], [A/A (low / low)], [verdict; prediction],
[k0n53], [*build*], [*1.104* [1.069, 1.142]], [1.019 [0.988, 1.045]], [*established*; [1.05, 1.25]: held],
[k0n53], [set-up], [1.033 [1.007, 1.061]], [0.991 [0.957, 1.024]], [not established; [1.01, 1.07]: held],
[k0n53], [collect], [0.996 [0.948, 1.028]], [0.996 [0.965, 1.032]], [no regression; [0.97, 1.03]: held],
[k0n61], [*build*], [*1.142* [1.098, 1.164]], [1.004 [0.964, 1.042]], [*established*; [1.05, 1.25]: held],
[k0n61], [set-up], [1.000 [0.983, 1.034]], [0.997 [0.970, 1.028]], [not established; [1.00, 1.04]: held],
[k0n61], [collect], [0.991 [0.969, 1.004]], [0.997 [0.967, 1.031]], [no regression; [0.97, 1.03]: held])

#image("ab.svg", width: 100%)

== Mechanism: build steps, ns per stored pair (median over rounds)
#table(columns: 5, align: (left, right, right, right, right),
[step], [k0n53 low], [k0n53 top], [k0n61 low], [k0n61 top],
[stream push], [12.79], [12.34], [13.76], [13.86],
[counting sort + filter fill], [10.69], [*7.86*], [12.68], [*8.38*],
[additions + keys], [6.60], [6.56], [7.12], [7.26],
[whole], [31.05], [28.04], [34.61], [30.09])
Only the step the layout touches moves: the filter fill now writes inside each partition's own run of the filter. The arm medians of whole build time agree with the A/B ratios (31.05/28.04 = 1.107; 34.61/30.09 = 1.150).

== Decision (frozen rule)
The build gain is established at both sizes, and no collect regression is established at either. So `Top` stays the default for CPU folded builds. The set-up gain is not established: at k0n53 its median is 3.3%, but its lower end (1.007) does not clear the A/A's upper end (1.024). At k0n61 the build is about 15% of set-up, and 14% of it is below this session's resolution.

== Scope
A component result for the build phase on this host class. It is not a cold-time claim against rho, and no board row follows from it. A Table A effect would need rule v10. GPU-built tables, table files and the GPU-parity tools keep the low layout.
