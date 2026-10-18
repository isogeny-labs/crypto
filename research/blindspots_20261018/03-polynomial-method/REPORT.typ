#set page(margin: 1.8cm)
#set text(size: 10pt)
= Item 03: polynomial-method solving on descended Semaev systems
Reassessment after the repair pass. Instrument sha256 `c4a2a1ac...`; receipts `raw/receipt-card-01.json`, `raw/receipt-card-02-r2.json` to `10-r2`; summary `raw/summary-r2.txt`.

== Verdict
*INCONCLUSIVE* by the frozen rule (level: none reached). The frozen sensitivity control needs the Dinur count to move by less than 1% across E = 17, 33, 64; measured 1.36% and 3.99%, so it fails. Descriptive branch with that control waived: *TRADES* (exponent over constant). Both arms cost far more than FES at N = 32 (46.8x and 97.7x); their fitted exponent is lower. No improvement over the index-calculus baseline is measured or claimed. msolve arm BLOCKED.

#figure(image("graph.svg", width: 100%), caption: [Counted cost against N (left, S_3 cells, measured FES and F4, modelled Dinur and LPTWY) and crossover model to N = 400 (right). Source `raw/summary-r2.txt`. Costs are deterministic; no sampling interval. Host shared 4-vCPU, L0.])

== Numbers (log2 bit operations)
#table(columns: 7, align: right,
[cell (N)], [FES real], [dinur], [lptwy], [rho dinur], [rho lptwy], [F4 touches],
[c13-6-2 (12)], [16.807], [24.928], [25.440], [3.59e-3], [2.52e-3], [17.570],
[c17-8-2 (16)], [21.170], [28.473], [29.084], [6.33e-3], [4.15e-3], [21.147],
[c21-10-2 (20)], [25.459], [32.474], [32.764], [7.74e-3], [6.32e-3], [28.847],
[c25-12-2 (24)], [29.700], [36.110], [36.909], [1.18e-2], [6.76e-3], [32.357],
[c29-14-2 (28)], [33.907], [39.818], [40.715], [1.66e-2], [8.93e-3], [cap 80 s],
[c33-16-2 (32)], [38.087], [43.637], [44.698], [2.14e-2], [1.02e-2], [cap 90 s],
)
Fitted exponent c (jackknife = leave-one-cell-out stability range): FES 1.0632 [1.0571, 1.0683]; dinur 0.9372 [0.9329, 0.9418]; lptwy 0.9666 [0.9594, 0.9795]; F4 1.3016 [1.2564, 1.4097] over four cells.

Crossover (counted model, E = N+1): Dinur below FES from N = 80 on the grid, LPTWY from N = 128; Dinur working set 2^71 bits at N = 80; no crossover under a 2^43-bit cap up to N = 400.

== Exponent statement (level: exponent)
Asymptotic c: 0.6946 (d = 2), 0.8115 (d = 3). Decomposition beats rho only if c < 1/2 - 1/m (0 for m = 2, 1/6 for m = 3). Distance remaining 0.6946 and 0.6448; index-calculus exponent relative to rho 2/m + 2c = 2.389 and 2.290 (above 1 loses). Fitted S_3 exponents give 2.87 (dinur), 2.93 (lptwy), 3.13 (FES). No cross-method row exists, so the 0 / 0.1 / 1 GAE pricing band is not applicable.

== Controls
Selftest OK. Positive control 1: 8856 real calls, 710 odd (400 required), 0 wrong; real counter equals the model on all. Positive control 2: published exponents reproduced. Negative control: identical model costs on descended and random systems. Sensitivity: failed (above).

== Caveats
Shared 4-vCPU host, isolation L0; toy scale N <= 32; F4 hit time caps at N = 28 and 32; costs are lower bounds (one core call, no Valiant-Vazirani multiplier). Revision-card relays were not verbatim; disk logs, which hash-match the receipts, were read.
