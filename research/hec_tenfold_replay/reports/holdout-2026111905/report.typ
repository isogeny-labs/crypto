#set page(paper: "us-letter", margin: (x: 0.65in, y: 0.6in))
#set text(size: 10.5pt, fill: rgb("#173342"))
#text(size: 22pt, weight: "bold")[Genus-2 index calculus on a frozen holdout]
#v(3pt)
#text(fill: rgb("#42616d"))[Public-pair seed 2026111905 · arm64 macOS · single thread · shared host]
#v(7pt)
*126 of 128 cases cleared 10× in both timing orders.* Each dot shows the median of eight paired ratios of pencil time to candidate time for a complete solve. The candidate alternates doublings of the generator and target, and verifies the scalar in the Jacobian.
#v(3pt)
#image("holdout.svg", width: 100%)
#text(size: 9pt)[Teal shows forward order, purple shows reverse order. The orange line marks 10×. Vertical scales are logarithmic. All 256 case medians are shown, including the largest observation of 219.52×.]
#v(7pt)
*The two misses.* At prime 127, case 26 measured 7.89× forward and 7.95× reverse. Case 27 measured 9.42× and 9.11×. Every other case exceeded 10× in both processes. Source code and thresholds were held fixed after the development panel exposed a degree-one input.
#v(7pt)
*Reproduction.* A separate native forward replay from the archived Rust and C++ sources passed all scalar checks and reached 10× on 126 of 128 holdout cases. Its minimum was 7.89×. A clean Git export also reproduced the certificate counts byte for byte and ran all 4,096 reverse-order holdout timings.
#v(7pt)
#text(size: 9pt)[Source data: results/g2-holdout-1.txt and results/g2-holdout-2.txt. Figure source: plot.rb. Scope: four genus-2 curves, 32 generator–target pairs per curve. The comparison measures the combined solver. Its mathematical components have separate ablations.]
