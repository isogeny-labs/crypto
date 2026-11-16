#set page(margin: 1.8cm, numbering: "1")
#set text(size: 10pt)
= Item 11: learned necessary-condition prefilter on x_R for subspace bases

_Assessed 2026-10-18. Measured (toy scale, host L0). Protocol PROTOCOL.md frozen before runs. Cards CARD-BLINDSPOT-11-01, -02, -04, -06, -07 receipted; -03 and -05 incomplete._

== Verdict
*INCONCLUSIVE (incomplete run: 13 of 16 cell-populations), every measured cell on the MATCHES side. Level: primitive. Not an improvement.* The frozen rule applied to the 13 cells on disk gives MATCHES in both populations: no learned filter (logistic regression, depth-3 boosted trees) beats the trace filter by 5 points at zero false refutations with the permutation null excluded. Missing: (n=19, m=3, S), (n=23, m=2, U), (n=29, m=3, S), because cards 03 and 05 returned before finishing. U cannot reach ADVANCES with one missing cell; S could only if both missing S cells passed. Gap to matched rho: unchanged (this stage diagnostic computes no S, ratio or GAE).

#figure(image("prefilter_gap.svg", width: 100%),
  caption: [Left: credited refutation rate of the learned filter against the trace filter (U, gbt, 7 cells, held-out bases); trace-zero bases lie on the diagonal (the learner copies arm b), generic and geometric bases sit at 0. Right: held-out AUC against the maximum of 20 within-base permutation AUCs on G and P bases (all points near or below the diagonal). Source plot_prefilter.py, data raw/summary_rows.tsv.])

== Numbers (measured; 104 summary rows)
#table(columns: 5, align: (left, right, right, right, right),
  [*pop, type*], [*max delta*], [*max rate_c*], [*max AUC - null max*], [*false refutations*],
  [U, T], [0.0000], [0.886], [+0.433], [0],
  [U, G], [+0.0021 \[0.0009, 0.0035\]], [0.0021], [+0.0068], [34],
  [U, P], [+0.0004 \[0, 0.0013\]], [0.0004], [+0.0024], [41],
  [U, ALL], [-0.265 to -0.465], [0], [+0.208], [75],
  [S, T/G/P/ALL], [0.0000], [0], [<= +0.0044], [1/43/30/74])
Threshold for a pass: delta >= 0.05 with lower interval end > 0, roc0 and AUC above the null band, planted_trace control passed. The best row (delta 0.0021) is about 24 times below the threshold.

== Controls
planted_trace: AUC 1.0, calibrated refutation 1.0, 13 of 13 cells, both models. planted_selector: gbt AUC 1.0 everywhere. Permutation null: G/P AUC within 0.007 of null max. Arm (b) false refutations: 0. Selftest: 0 failures, decomposer equals brute force on every point of E(F_2^11), E(F_2^13).

== Cost (section 5a pricing at 0, 0.1, 1 per unpriced word operation)
Net work of adding the gbt filter is negative at all three weights in every cell: -0.0010 (n=29, m=2, U, w=0) to -2.31 of decomposer work (n=17, m=3, S, w=1).

== Deviations
Cards 03 and 05: no receipt, binaries left in target/blindspots-11, cards now fail lint, check-relay could not run. Raw files n19-m3-U.jsonl and n29-m3-U.jsonl are complete (done lines) but unreceipted. Card 07 ran before 03 and 05 finished (13 input files); receipts are write-once. Relays for 01, 02, 04, 06, 07 verified by sha256 against receipts and raw files, not re-saved as full text.

== Caveats
n = 17 to 29 only; two model classes and the listed feature families only; no wall-time claim; U alone re-derives the trace, S is the deployable population and shows no learnable signal. See README.md.
