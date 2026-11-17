#set page(paper: "a4", margin: 22mm)
#set text(font: "Liberation Sans", size: 10pt)
= Tian quartic: published-class line-sieve DLP

*Command:* `tian-quartic-km --line-sieve-dlp` and `tian-line-sieve-analyze` from `native/Cargo.toml`. *Host:* shared Intel Xeon 6975P-C; L0 operation counts only.

The solver received only the published destination classes S,T, without 1987. It enumerated all 522 rational affine points of the quartic and 11,116 fully split lines, obtaining relation rank 521 modulo the prime subgroup order. A rational degree-three divisor split for S and another for 6T after five failed target attempts. The divisor relations were independently checked by constructing the rational divisor classes and comparing them in the Jacobian. The resulting scalar 1987 was verified on the destination and by recomputing 1987P=Q on the source.

At M+0.5S+10I+0.1A, the full cold cost is 606292001.4 multiplication equivalents versus 1079600.82 for the mean of five optimized source-rho solves, a 561.59× loss. Across 36 prices this solver loses in every cell (40.85–684.92×). Quartic enumeration, line relations, modular elimination, failed descents and both verifications are included. Allocation, comparisons and control flow are unpriced, so wall speed is unknown.

#figure(image("cost-ratio.svg", width: 100%), caption: [Complete counted cold operation cost for one published-class destination DLP versus mean source rho.])

*Raw receipt:* `correctness/tian-quartic-line-sieve-dlp-v1.json`. *Full 36-price grid:* `results/tian-quartic-line-sieve-price-grid.csv`. The source-to-Jacobian map was not executed: S,T were direct published inputs, so this is not a complete transferred source attack or a general-size result.
