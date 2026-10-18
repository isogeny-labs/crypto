# Item 11: a learned necessary-condition prefilter on x_R for subspace bases

Assessed 2026-10-18. Protocol: `PROTOCOL.md` (frozen before any run). Instrument: `examples/blindspots_11_learned_prefilter.rs` (std-only Rust, compiled with `rustc`, Deviation 1). Graph: `prefilter_gap.svg` / `.png` (source `plot_prefilter.py`, data `raw/summary_rows.tsv`). PDF: `REPORT.pdf`.

## Verdict

**INCONCLUSIVE (incomplete run), with every measured cell on the MATCHES side. Level: primitive. Not an improvement.**

Thirteen of the sixteen frozen cell-populations are on disk and the frozen rule, applied mechanically by the `verdict` subcommand to those thirteen, returns MATCHES for both populations: no learned filter (logistic regression or gradient-boosted trees, on any base type) beats the trace filter by 5 points at zero false refutations with the null band excluded. Three cell-populations were never measured: (n=19, m=3, S), (n=23, m=2, U) and (n=29, m=3, S). Cards 03 and 05 did not finish (their executors returned before the runs ended, see Deviations), so the sixteen-cell rule is not fully applied. What the three missing cells could change: U has no hit in seven cells, so one missing U cell can give at most a single hit (INCONCLUSIVE, never ADVANCES); S has no hit in six cells, so S could reach ADVANCES only if both missing S cells passed. Neither is expected (all twelve measured S rows have rate_c = 0 and AUC inside the null band), but the rule does not allow assuming it. No performance claim: the filter refutes nothing that the trace filter does not already refute, and costs more than it saves.

Gap to matched rho: **unchanged; this item computes no S, no rho ratio and no GAE** (stage diagnostic), so no row has a gap to report and none improved. The §5a pricing was applied to the filter and the decomposer (word operations at 0, 0.1 and 1 field operations): the net work of adding the gbt filter is negative at all three weights in every measured cell (below).

## Method

Binary curves `E_{0,b}` over GF(2^n), n in {17,19,23,29}, m in {2,3}, populations U (uniform targets) and S (targets in 2E(F), where the trace filter is vacuous). `l` chosen by pilot (l = 9, 6, 10, 7, 12, 8, 15, 10 for the cells measured). Ten bases per cell (4 trace-zero T, 3 generic G, 3 geometric P), 2,000 targets per base, 20,000 labelled targets per cell-population, five folds split by whole bases. The label is exact (point-addition scan for m=2, pair-sum bitset for m=3), validated against brute force on all points of E(F_2^11) and E(F_2^13) (selftest, `raw/selftest.txt`, 0 failures). Arms: (a) none, (b) trace filter `delta(R) not in D^(+m)`, (c) learned filter with calibration threshold at the minimum positive calibration score, credited only at zero pooled false refutations, (d) 20 within-base label permutations, (e) planted-label controls. Rule and thresholds exactly as in `PROTOCOL.md`.

## Numbers (from `raw/n*-m*-*.jsonl`, 104 summary rows; `raw/summary_rows.tsv`)

| population, base type | rows | max delta (rate_c - rate_b) | max rate_c | max AUC minus null max | false refutations of the learned filter (pooled, all rows) |
| --- | --- | --- | --- | --- | --- |
| U, T | 14 | 0.0000 (interval [-0.0005, 0]) | 0.886 (= rate_b) | +0.433 | 0 |
| U, G | 14 | +0.0021 (interval [0.0009, 0.0035], n=23 m=3 gbt) | 0.0021 | +0.0068 | 34 |
| U, P | 14 | +0.0004 (interval [0, 0.0013]) | 0.0004 | +0.0024 | 41 |
| U, ALL | 14 | -0.265 to -0.465 (credit voided by false refutations) | 0 | +0.208 | 75 |
| S, T/G/P/ALL | 12 each | 0.0000 everywhere | 0 | at most +0.0044 | 1 / 43 / 30 / 74 |

Reading. On trace-zero bases of U the learned filter reproduces the trace filter exactly (rate_c = rate_b, 0.65 to 0.89), because group 7 hands it arm (b)'s verdict; its AUC far above the null band is the trace, not new structure. On generic and geometric bases (where arm (b) refutes 0), the best credited refutation rate is 0.21 points (U, n=23, m=3, gbt), 24 times under the 5-point threshold, and held-out AUC lies within about 0.007 of the permutation null maximum in every row. On S the credited rate is 0 in all 48 rows. The ALL rows on U show delta negative because pooling bases with false refutations voids the credit. The learned filter's false refutations (148 pooled over G and P rows, plus ALL) are why calibration on seen bases does not transfer to held-out bases. Largest rate_b: 0.886 (n=29, m=2, U, T).

Gradient-boosted split counts (`gbt_splits` rows) are spread across feature groups, with no group dominating on G or P bases; not interpreted further.

## Controls

* Positive `planted_trace`: AUC 1.0 and calibrated refutation 1.0 in all 13 cells for both models (PASS). `planted_selector`: gbt AUC 1.000 in all 13 cells; lr AUC 0.870 to 0.887 (descriptive, a linear model is not expected to solve it).
* Negative (within-base permutation, 20 permutations): the null band max AUC is 0.51 to 0.52 on G/P bases (0.62 to 0.63 on S ALL where pooled base heterogeneity inflates both observed and null AUC alike); no observed G/P AUC exceeds its band by more than 0.007.
* Arm (b) false refutations: 0 over all cell-populations (the theorem holds numerically).
* Instrument selftest: 0 failures.

## Cost (§5a pricing of unpriced word operations at 0, 0.1, 1)

Because the credited learned refutation count is 0 in every cell except 0.2% on one, "saved by refutation" is 0.0 (printed to one decimal in `raw/verdict.txt`). Net work of the gbt filter as a fraction of decomposer work, all negative: from -0.0010 (n=29, m=2, U, w=0) to -2.31 (n=17, m=3, S, w=1); m=2 cells lie between -0.0010 and -0.18, m=3 cells between -0.022 and -2.31 (every line is in `raw/verdict.txt`). The sign holds at all three weights in all 13 cells, so the conclusion "the filter is a net loss" is band-stable. These are lower-bound counts; no ranking between methods is made.

## Deviations and anomalies (reported, not repaired)

1. Frozen Deviations 1 to 9 of `PROTOCOL.md` stand (rustc not cargo, populations U and S, a = 0, base types, planted label on S, group 7 context feature, pilot-chosen l, replication across two cells, calibrated zero false refutations).
2. **Cards 03 and 05 did not complete.** Their relays say the run was moved to the background and the executor returned before it finished; no receipt exists for either (`raw/receipt-card-03.json`, `-05.json` absent), the build binaries remain in `target/blindspots-11/card-03/` and `card-05/`, and both cards now fail lint ("names repository file ... that binds does not pin"), so `check-relay` could not run (it exits at lint). Of their cells, only the first command of each finished: `raw/n19-m3-U.jsonl` (ends with a `done` line, sha256 prefix addee0bf9c) and `raw/n29-m3-U.jsonl` (done line, fd7cfc648c). Those two files are complete but **have no receipt, so their provenance is not hash-attested** the way the other eleven cells are. They are used in the verdict, flagged here.
3. **Card 07 ran before cards 03 and 05 finished**, so its `verdict` saw 13 files (`verdict input: 13 jsonl files`). The build notes required 07 to run last. Its receipt is write-once; a fresh verdict needs a new card.
4. Relays for cards 01, 02, 04, 06, 07 were not re-saved as full text files (about 60 KB of cell tails). Instead the relayed receipt sha256, card sha256, commit and every cell's stdout sha256 were compared with the receipts and with the sha256 of the raw files on disk: all match (receipts ac5d8bb8, 85718fde, d65fd654, 7be575d4, 4ac4c5af; stdout hashes of all eleven receipted cells and of `verdict`). `raw/relay-3.txt` and `raw/relay-5.txt` hold the two incomplete relays verbatim.
5. Recovery needed (director's step): remove `target/blindspots-11/card-03/` and `card-05/` by hand, move the two unreceipted raw cell files aside or re-run (the cards declare them as outputs), run cards 03 and 05, then a new verdict card. The three missing cells, ~15 CPU-minutes, were not run here; the assess step does not repair relays.

## Caveats

Toy scale: n 17 to 29 with 20,000 targets per cell-population and ten bases per cell; the trace is the only structure available here, and the result says nothing about n = 131 or larger. Host L0 (shared 4-vCPU cloud container, no isolation): no wall-time claim is made, only operation counts. `delta_lo`/`delta_hi` are 95% paired bootstrap intervals over negatives (1,000 resamples). U alone would be a re-derivation of the trace; the deployable population is S, and on S the learned filter refuted none of the 47,218 held-out negatives per model (six cells) with credit. The null result covers only the feature families listed in `PROTOCOL.md` and two model classes (logistic regression, depth-3 boosted trees), not all learnable functions of x_R. No leaderboard row and no `benchmarks/` entry (not a speed result).
