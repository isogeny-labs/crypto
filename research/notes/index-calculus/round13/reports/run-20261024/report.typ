#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
= Run report: hyperelliptic IC round thirteen, negation fold on the full-width walk

*Card* `CARD-HEIC-ROUND13-1` (sha256 `e536ff90…2558e1`), commit `41718e59`; binary `hyperelliptic_rho_negation_full_width` sha256 `597159e4…886705`, built from `41718e59` (`examples/hyperelliptic_rho_negation_full_width.rs`). Receipt `../../run1/receipt.json`: status completed, all nine commands exit 0.

*Host and isolation.* Intel Xeon @ 2.80 GHz, 4 logical CPUs, kernel 6.18.44, x86-64. `tools/isolated_bench.py`, reserved CPU 3, preflight `--max-other-cpu 1.0`; record `contended: false`, wall 771.6 s. Wall time is not a reported value; all values are counted operations, deterministic in the seed.

*Arms and workload.* Distinguished-point rho with full-width branch coefficients (the reference since round twelve), plain and folded on `±` classes with round nine's look-ahead. Same seeds `0xC0FFEE + t`, t < 40, on the thread's 18 instances. 18 × 40 × 2 runs; every recovered `k` verified.

#figure(image("swalk.svg", width: 100%), caption: [Recorded `S_walk` per row (mean over 40 seeds), plain and folded, log scale, against the ideal constants. Source: `../../run1/R13.txt`.])

*Recorded verdict lines* (printed by the binary):
- N1, median folded/plain walk operations over the 7 rows with N ≥ 10^5: 1.580 (bound 0.80): *falsified*.
- N2, largest fruitless share as counted: 5.7% (bound 10%): holds as counted.
- N3, median folded `S_walk` over those rows: 2.033 (bound 1.05): *falsified*.
- N4, every run verified: holds.

*Observations.* Below N ≈ 3·10^4 the fold gives 0.57–0.90 of the plain walk (ideal 0.707). Above, the ratio grows with N to 3.98 and 4.15 on the two largest rows. The fruitless counter records look-ahead trials and one step per gap escape; steps spent inside a fruitless cycle before the escape fires are not counted, so N2 does not bound them. Round fourteen tests whether those trapped steps (fruitless cycles longer than two, escaped only at a gap limit of about √N) are the cause.

*Missing.* No cross-method comparison; no unpriced counters, so §5a pricing cannot be applied. One seed set per row; no interval on `S_walk`. No validator report, so no `benchmarks/README.md` row (validation gate).
