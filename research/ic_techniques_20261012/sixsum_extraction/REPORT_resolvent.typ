#set page(margin: 1.8cm, numbering: "1")
#set text(size: 10pt)
= Six-sum kernel B/B2: direct quartic root solve (resolvent cubic)

*Class:* engineering; stage diagnostic of relation extraction. No end-to-end `S`, no rho comparison, no key-recovery or deployed-curve claim.
*Date:* 2026-10-18. *Branch:* wt/t04. *Host:* shared 4-vCPU container, load about 2, no isolation tier earned; wall times are secondary, operation counts are exact.

== What ran
`koblitz_sixsum_extract_probe` (report version 4), Koblitz $K_0$ $n=41$ ($r approx 2^39$), seed 20261012, $K in {10, 20}$, 32 verified relations per cell, kernels 4sum and 6sum_b2_s4_strided, `KIC_S4_ROOTS=split` (control, unchanged) against `resolvent` (new), `--solver-bench=3000` (3000 complete quartics, best of 7 interleaved repeats). Every relation is re-added in the group: 0 rejects, 0 lift failures, 0 `s4_fallbacks`; both solvers agree on all benchmarked quartics. Raw records: `raw_resolvent_20261018/n41_k10_k20_{split,resolvent}.json` and `.stderr.txt`; frozen summary `resolvent_summary.json`.

== Measured
#figure(image("sixsum_s4_resolvent_n41.svg", width: 100%),
 caption: [Left: per-solve wall at $n=41$ (resolvent-run values; the split run measured S3 354 ns, roots split 9.54 us, resolvent 0.611 us). Middle: wall per verified relation. Right: S3-equivalents per relation, S4 solves priced by the measured S4-pair wall cost in S3 units. Sources: `raw_resolvent_20261018/n41_k10_k20_*.json`.])

#table(columns: 5, align: (left, right, right, right, right),
 [*measured*], [*4-sum*], [*B2 split*], [*B2 resolvent*], [*unit*],
 [K=10 wall], [2.37], [3.89], [0.350], [s / relation],
 [K=20 wall], [0.475], [1.34], [0.128], [s / relation],
 [K=10 S3-equiv.], [5.22e6], [1.00e7], [6.79e5], [S3 / relation],
 [K=20 S3-equiv.], [1.01e6], [3.58e6], [2.42e5], [S3 / relation],
 [roots per solve], [-], [9.54 us], [0.611 us], [n=41, split run],
 [field ops per quartic], [-], [1710 mul, 395 sqr, 7 inv], [70 mul, 49 sqr, 2 inv, 8 table maps], [counted],
)
S4 solves per relation are identical across split and resolvent (3.71e5 at K=10, 1.32e5 at K=20); only the root finder changes.

*Derived* (from the table): resolvent against split, 11.1x (K=10) and 10.5x (K=20) less wall per relation; against the 4-sum, 6.8x and 3.7x. The resolvent root solve alone costs 1.73 S3 by wall and 2.19 by priced field operations (split run); an S4 pair with its coefficients costs 1.83 S3 by wall and 2.51 priced (resolvent run), against the break-even of about 12 S3 named on 2026-10-12.

*Estimate, not measured end to end:* the 4-sum above uses the crate's loop half-trace S3 (about 373 ns). A 4-sum using the byte-table half-trace S3 (about 187 ns in the same run) would be roughly 2x cheaper, which would reduce the B2 resolvent margin to about 3.4x (K=10) and 1.9x (K=20).

== Scope and limits
Before (2026-10-12): one S4 root solve cost 20-60 S3 and B2 was 1-8x more S3-work than the 4-sum. After, on this ladder only: B2 with the resolvent solver is below the 4-sum at both $K$. No factor-base, linear-algebra or descent cost is included; $n=53$ and an end-to-end cold `S` against the frozen baseline are not measured. $n=23$ and $31$ cells are saturated (a few S3 solves per relation) and give solver costs only (`raw_resolvent_20261018/n23_31_*`). Distance to matched rho: not measured here.

== Reproduce
`KIC_S4_ROOTS=resolvent koblitz_sixsum_extract_probe --n=41 --a=0 --columns=10,20 --kernels=4sum,b2 --relations=32 --seed=20261012 --max-targets=4096 --max-s3=400000000 --max-s4=50000000 --cell-wall-s=1200 --solver-bench=3000`; graph: `plot_resolvent.py`.
