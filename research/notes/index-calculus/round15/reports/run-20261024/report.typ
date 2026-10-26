#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
= Run report: hyperelliptic IC round fifteen, collide-first cycle window

*Card* `CARD-HEIC-ROUND15-1` (sha256 `11af6013…c4f7bf`), commit `16e7f548`; binary `hyperelliptic_rho_collide_first` sha256 `8a732b5d…a1b623`, built from `16e7f548` (`examples/hyperelliptic_rho_collide_first.rs`). Receipt `../../run1/receipt.json`: status completed, all nine commands exit 0.

*Host and isolation.* Intel Xeon @ 2.30 GHz, 4 logical CPUs, kernel 6.18.44-fc-v153, x86-64. `tools/isolated_bench.py`, reserved CPU 3, preflight `--max-other-cpu 1.0`; record `contended: false`, wall 433.9 s. All values are counted operations, deterministic in the seed.

*Arms and workload.* Full-width distinguished-point rho: plain; folded on `±` classes with round fourteen's active window (escape on any repeat within 32 steps); folded with the collide-first window (on a repeat, solve from the earlier visit's coefficients when that collision is non-degenerate, escape only otherwise). Seeds `0xC0FFEE + t`, t < 40, on the thread's 18 instances; 18 × 40 × 3 runs, every recovered `k` verified.

#figure(image("ratio.svg", width: 100%), caption: [Folded/plain walk operations per row for the two windowed folded walks, against 1/√2. Source: `../../run1/R15.txt`.])

*Recorded verdict lines* (printed by the binary):
- R15-1: collide-first folded/plain, median over the 10 rows with N ≤ 6·10^4: 0.698 (bound 0.85): *holds*.
- R15-2: the same over the 7 rows with N ≥ 10^5: 0.716 (bound 0.80): *holds*.
- R15-3: collide-first folded `S_walk`, median over those rows: 1.008 (bound 1.05): *holds*.
- R15-4: fruitless share at most 10% on every row (max 4.4%); every run verified; the active arm reproduces round fourteen: *holds*.

*Observations.* Trying a short cycle as a collision before escaping it removes round fourteen's small-group penalty (at N ≤ 6·10^4 active/plain spans 0.64–1.08, collide-first 0.52–0.86) and keeps the large-group gain. As registered, this folded walk becomes the thread's rho reference. That is an accounting correction against index calculus: the reference gets about 0.70× cheaper in walk operations, the √2 of the negation map that index calculus's factor base already uses. It reproduces a known result (lead atlas: `LEAD-RHO-negation-map-fruitless-cycles`, closed_known).

*Missing.* No cross-method comparison. The window's unpriced native work (up to 32 hash comparisons and one coefficient copy per step) is reported, not priced (§5a). One seed set per row; no intervals. No validator report, so no `benchmarks/README.md` row (validation gate).
