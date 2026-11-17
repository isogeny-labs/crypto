#set page(paper: "a4", margin: 22mm)
#set text(font: "Liberation Sans", size: 10pt)
= Tian cubic: mapped source-to-answer line-sieve DLP

*Commands:* `tian-cubic-map-gate`, `tian-quartic-km --trace-map-pq`, `tian-quartic-km --line-sieve-mapped-dlp`, `tian-line-sieve-mapped-analyze`. *Host:* shared Intel Xeon 6975P-C; L0 operation counts only.

The published source P and Q coordinates were mapped with Tian's six rational functions; Q was supplied directly, not generated from the planted scalar. Their degree-three cuts were checked on the smooth quartic and their Frobenius traces represented as Jacobian classes. A separate cohort gate matched 100 successive source multiples and Q to destination classes. The line-sieve DLP consumed the mapped P and Q classes, recovered 1987 without it as a solver input, and verified the answer on source and destination.

At M+0.5S+10I+0.1A, the complete cold pipeline costs 697039875.2 multiplication equivalents versus 1079600.82 for mean source rho, a 645.65× loss. All 36 price cells lose (47.14–787.55×). The 561.59× published-class-only control omits source mapping.

#figure(image("cost-ratio.svg", width: 100%), caption: [Mapped cubic source-to-answer index calculus versus optimized source rho; the published-class-only result is shown for comparison.])

*Evidence:* `correctness/tian-cubic-map-gate-v2.jsonl`, `correctness/tian-cubic-trace-map-pq-v1.json`, `correctness/tian-quartic-line-sieve-mapped-dlp-v1.json`, and `results/tian-quartic-line-sieve-mapped-price-grid.csv`. Memory and control flow are unpriced. This is one published fixture; an independent quotient-from-kernel proof and growing-size comparison remain open.
