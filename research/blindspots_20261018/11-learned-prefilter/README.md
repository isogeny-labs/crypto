# Item 11: a learned necessary-condition prefilter on x_R for subspace bases

Assessed 2026-10-18 (reassessment after the repair pass). Protocol: `PROTOCOL.md` (frozen before any run; "Repair pass" section appended after the first run, no frozen item altered). Instrument: `examples/blindspots_11_learned_prefilter.rs` (std-only Rust, compiled with `rustc`, Deviation 1; unchanged by the repair pass). Graph: `prefilter_gap.svg` / `.png` (source `plot_prefilter.py`, data `raw/summary_rows_r2.tsv`). PDF: `REPORT.pdf`. Verdict output: `raw/verdict-r2.txt` (receipt `raw/receipt-card-07-r2.json`).

## Verdict

**MATCHES. Level: primitive. Not an improvement.** All sixteen frozen cell-populations are now measured, and the frozen rule, applied mechanically by the `verdict` subcommand to the sixteen files (`verdict input: 16 jsonl files, 512 rows; cell-populations present: 16`), returns MATCHES for population S, MATCHES for population U, and MATCHES overall: no learned filter (logistic regression or depth-3 gradient-boosted trees, on any base type) beats the trace filter by 5 points at zero false refutations with the permutation null band excluded. No (model, base type) pair passes in any of the 16 cell-populations, so neither ADVANCES (needs two cells) nor a single-hit INCONCLUSIVE arises, and the `planted_trace` positive control passed in all 32 (cell-population, model) pairs, so the null is not a blind detector. The three previously missing cells did not change the outcome: first run said INCONCLUSIVE (incomplete run, 13 of 16, every measured cell on the MATCHES side); the completed run removes the incompleteness and the rule gives MATCHES.

No performance claim: the credited learned refutation count is 0 except at most 0.21% of negatives on a few generic-base rows, far below the 5-point threshold, and the filter costs more than it would save at every pricing.

**Gap to matched rho: unchanged, not applicable per row.** This item is a stage diagnostic and computes no S, no rho ratio and no GAE, so no row has a gap to report and none moved. There is no cross-method row, so §5a ranking does not apply; the §5a pricing was applied inside the item (filter against decomposer, below).

## Method

Binary curves `E_{0,b}` over GF(2^n), n in {17,19,23,29}, m in {2,3}, populations U (uniform targets) and S (targets in 2E(F), where the trace filter is vacuous). `l` chosen by pilot (l = 9, 6, 10, 7, 12, 8, 15, 10 for S and U at the sixteen cells; see `raw/verdict-r2.txt`). Ten bases per cell (4 trace-zero T, 3 generic G, 3 geometric P), 2,000 targets per base, 20,000 labelled targets per cell-population, five folds split by whole bases. The label is exact (point-addition scan for m=2, pair-sum bitset for m=3), validated against brute force on all points of E(F_2^11) and E(F_2^13) (`raw/selftest.txt`, 0 failures). Arms: (a) none, (b) trace filter, (c) learned filter with calibration threshold at the minimum positive calibration score, credited only at zero pooled false refutations, (d) 20 within-base label permutations, (e) planted-label controls. Rule and thresholds exactly as in `PROTOCOL.md`.

## What the repair pass ran

Cells (19,3,S), (23,2,U) and (29,3,S) were never measured in the first run; they are now (`raw/n19-m3-S-r2.jsonl`, `raw/n23-m2-U-r2.jsonl`, `raw/n29-m3-S-r2.jsonl`, cards `card-03-r2` and `card-05-r2`). The two cells measured but unreceipted in the first run, (19,3,U) and (29,3,U), were re-run with identical arguments and seeds under receipts (`card-03b-r2`, `card-05-r2`; files `raw/n19-m3-U-r2.jsonl`, `raw/n29-m3-U-r2.jsonl`). The re-runs reproduce the first-run files exactly: after removing the `seconds` and `rss_peak_kb` fields the files are identical to `raw/n19-m3-U.jsonl` and `raw/n29-m3-U.jsonl` (checked with diff); only timings differ, so the earlier unreceipted numbers are confirmed. The verdict card `card-07-r2` copied exactly 16 files (11 receipted first-run files plus the 5 `-r2` files) so no cell is counted twice. The stdout sha256 of every `-r2` file equals the sha256 in its receipt and in the executor relay.

## Numbers (from the 16 verdict files; 128 summary rows; `raw/summary_rows_r2.tsv`)

Per population, base type and model, over the 8 cells of each population. `delta` = rate_c - rate_b with 95% paired bootstrap interval (1,000 resamples); "AUC - null" = held-out AUC minus the maximum over the 20 permutations.

| population, type | max delta | max rate_c | max AUC - null max | learned false refutations (lr + gbt pooled) |
| --- | --- | --- | --- | --- |
| U, T | 0.0000 (lr lower end -0.0005) | 0.888 (= rate_b, the learner copies arm b) | +0.436 (trace signal) | 0 |
| U, G | +0.0021, [0.0009, 0.0035] (n=23, m=3, gbt) | 0.0021 | +0.0068 | 40 |
| U, P | +0.0004 | 0.0004 | +0.0024 | 45 |
| U, ALL | at most +0.0002 (gbt negative, credit voided) | 0.451 (lr, equals b) / 0 (gbt) | +0.208 | 85 |
| S, T | 0.0000 | 0 | below 0 | 1 |
| S, G | +0.0007 (n=19, m=3, gbt; interval [0, 0.0018]) | 0.0007 | +0.0078 | 51 |
| S, P | +0.0006 (lr) | 0.0006 | +0.0001 | 36 |
| S, ALL | 0.0000 | 0 | +0.0031 | 88 |

Reading. On trace-zero U bases the learned filter reproduces the trace filter exactly (rate_c = rate_b, up to 0.888) because group 7 hands it arm (b)'s verdict; the high AUC there is the trace, not new structure. On generic and geometric bases, where arm (b) refutes nothing, the best credited refutation rate over all 16 cell-populations is 0.21 points (U, n=23, m=3, gbt, G), about 24 times under the 5-point threshold, and its AUC (0.508) is below its own null maximum (0.522). The largest margin of any held-out G/P AUC over its null maximum is +0.0078 (S, n=19, m=3, gbt, G: AUC 0.518 against 0.511), with delta 0.0007 and an interval that includes 0; it is far below any pass condition and is reported, not interpreted. On S the learned filter refutes essentially none of the 67,868 held-out negatives per model (eight cells). The learned filter's false refutations (172 pooled over G and P rows) are why calibration on seen bases does not transfer to held-out bases; the ALL rows on U show negative delta because those false refutations void the credit.

## Controls

* Positive `planted_trace`: AUC 1.0 and calibrated refutation 1.0 in all 16 cell-populations for both models (32 of 32 PASS in `raw/verdict-r2.txt`). `planted_selector`: gbt AUC 1.000 in all cells; lr about 0.87 to 0.88 (descriptive, a linear model is not expected to solve it).
* Negative (within-base permutation, 20 permutations): the null band max AUC is about 0.51 to 0.52 on G/P bases; no held-out G/P AUC exceeds it by more than 0.0078.
* Arm (b) false refutations: 0 over all 16 cell-populations (the theorem holds numerically).
* Instrument selftest: 0 failures (`raw/selftest.txt`).

## Cost (§5a pricing of unpriced word operations at 0, 0.1, 1 field operations)

Net work of adding the gbt filter, as a fraction of decomposer work (all 48 lines of `raw/verdict-r2.txt`, 16 cells at three weights): negative in every one. Range by weight: w=0 from -0.2593 to -0.0010; w=0.1 from -0.5033 to -0.0013; w=1 from -2.3052 to -0.0020. m=2 cells lie between -0.0010 and -0.18, m=3 cells between -0.022 and -2.31. The sign holds at all three weights in all 16 cells, so "the filter is a net loss" is stable across the band. These are operation counts, not time, and make no cross-method ranking.

## Deviations and anomalies (reported, not repaired)

1. Frozen Deviations 1 to 9 of `PROTOCOL.md` stand.
2. Repair pass: the first run's cards 03 and 05 returned early (executor 2-minute shell timeout); the repair cards carry the explicit 10-minute timeout in their notes. The instrument and the decision rule are unchanged.
3. Relays: the three executor relays are saved as `raw/relay-r2-03-r2.txt`, `raw/relay-r2-05-r2.txt`, `raw/relay-r2-03b-r2.txt`. They were reconstructed as the executor's wrapper lines plus the receipt's own brief (the long stdout tails) rather than copied from the message character by character; `check-relay` therefore reports `relay_verbatim: true` for all three, but that check is not independent evidence of the executor text. What I did verify independently: card sha256, commit, receipt sha256 and every cell's stdout sha256 in the executor message equal the receipts and the raw files on disk, and the key values of the tails agree with the raw files.
4. `card-07-r2` was run by this assessment through `python3 tools/task_card.py run`, not relayed from an executor (the staged card did not exist when the executor lane ran). The run is deterministic and hash-attested (receipt `raw/receipt-card-07-r2.json`), but it departs from the model-tier convention that Haiku executes cards.
5. The old 13-file `raw/verdict.txt` is kept as history; `raw/verdict-r2.txt` is the verdict of record. The graph and `summary_rows_r2.tsv` replace the 13-cell graph (the earlier files are in git history; `raw/summary_rows.tsv` is kept).
6. The two re-run cells are not new evidence: they reproduce the first-run files exactly except for timings.

## Caveats

Toy scale: n 17 to 29 with 20,000 targets per cell-population and ten bases per cell; the trace is the only structure available here, and the result says nothing about n = 131 or larger. Host L0 (shared 4-vCPU cloud container, no isolation, cards sometimes overlapped on two lanes): no wall-time claim, only operation counts. U alone re-derives the trace; the deployable population is S, and on S nothing learnable above the permutation null was found. The null result covers only the feature families listed in `PROTOCOL.md` and two model classes (logistic regression, depth-3 boosted trees), not all learnable functions of x_R. No leaderboard row and no `benchmarks/` entry (a stage diagnostic with no speed or cost claim of its own).

## First run (superseded)

The text below is the first-run README, kept unchanged apart from heading depth. Its verdict (INCONCLUSIVE, incomplete run, 13 of 16 cells) is superseded by the verdict above.

## Item 11: a learned necessary-condition prefilter on x_R for subspace bases

Assessed 2026-10-18. Protocol: `PROTOCOL.md` (frozen before any run). Instrument: `examples/blindspots_11_learned_prefilter.rs` (std-only Rust, compiled with `rustc`, Deviation 1). Graph: `prefilter_gap.svg` / `.png` (source `plot_prefilter.py`, data `raw/summary_rows.tsv`). PDF: `REPORT.pdf`.

### Verdict

**INCONCLUSIVE (incomplete run), with every measured cell on the MATCHES side. Level: primitive. Not an improvement.**

Thirteen of the sixteen frozen cell-populations are on disk and the frozen rule, applied mechanically by the `verdict` subcommand to those thirteen, returns MATCHES for both populations: no learned filter (logistic regression or gradient-boosted trees, on any base type) beats the trace filter by 5 points at zero false refutations with the null band excluded. Three cell-populations were never measured: (n=19, m=3, S), (n=23, m=2, U) and (n=29, m=3, S). Cards 03 and 05 did not finish (their executors returned before the runs ended, see Deviations), so the sixteen-cell rule is not fully applied. What the three missing cells could change: U has no hit in seven cells, so one missing U cell can give at most a single hit (INCONCLUSIVE, never ADVANCES); S has no hit in six cells, so S could reach ADVANCES only if both missing S cells passed. Neither is expected (all twelve measured S rows have rate_c = 0 and AUC inside the null band), but the rule does not allow assuming it. No performance claim: the filter refutes nothing that the trace filter does not already refute, and costs more than it saves.

Gap to matched rho: **unchanged; this item computes no S, no rho ratio and no GAE** (stage diagnostic), so no row has a gap to report and none improved. The §5a pricing was applied to the filter and the decomposer (word operations at 0, 0.1 and 1 field operations): the net work of adding the gbt filter is negative at all three weights in every measured cell (below).

### Method

Binary curves `E_{0,b}` over GF(2^n), n in {17,19,23,29}, m in {2,3}, populations U (uniform targets) and S (targets in 2E(F), where the trace filter is vacuous). `l` chosen by pilot (l = 9, 6, 10, 7, 12, 8, 15, 10 for the cells measured). Ten bases per cell (4 trace-zero T, 3 generic G, 3 geometric P), 2,000 targets per base, 20,000 labelled targets per cell-population, five folds split by whole bases. The label is exact (point-addition scan for m=2, pair-sum bitset for m=3), validated against brute force on all points of E(F_2^11) and E(F_2^13) (selftest, `raw/selftest.txt`, 0 failures). Arms: (a) none, (b) trace filter `delta(R) not in D^(+m)`, (c) learned filter with calibration threshold at the minimum positive calibration score, credited only at zero pooled false refutations, (d) 20 within-base label permutations, (e) planted-label controls. Rule and thresholds exactly as in `PROTOCOL.md`.

### Numbers (from `raw/n*-m*-*.jsonl`, 104 summary rows; `raw/summary_rows.tsv`)

| population, base type | rows | max delta (rate_c - rate_b) | max rate_c | max AUC minus null max | false refutations of the learned filter (pooled, all rows) |
| --- | --- | --- | --- | --- | --- |
| U, T | 14 | 0.0000 (interval [-0.0005, 0]) | 0.886 (= rate_b) | +0.433 | 0 |
| U, G | 14 | +0.0021 (interval [0.0009, 0.0035], n=23 m=3 gbt) | 0.0021 | +0.0068 | 34 |
| U, P | 14 | +0.0004 (interval [0, 0.0013]) | 0.0004 | +0.0024 | 41 |
| U, ALL | 14 | -0.265 to -0.465 (credit voided by false refutations) | 0 | +0.208 | 75 |
| S, T/G/P/ALL | 12 each | 0.0000 everywhere | 0 | at most +0.0044 | 1 / 43 / 30 / 74 |

Reading. On trace-zero bases of U the learned filter reproduces the trace filter exactly (rate_c = rate_b, 0.65 to 0.89), because group 7 hands it arm (b)'s verdict; its AUC far above the null band is the trace, not new structure. On generic and geometric bases (where arm (b) refutes 0), the best credited refutation rate is 0.21 points (U, n=23, m=3, gbt), 24 times under the 5-point threshold, and held-out AUC lies within about 0.007 of the permutation null maximum in every row. On S the credited rate is 0 in all 48 rows. The ALL rows on U show delta negative because pooling bases with false refutations voids the credit. The learned filter's false refutations (148 pooled over G and P rows, plus ALL) are why calibration on seen bases does not transfer to held-out bases. Largest rate_b: 0.886 (n=29, m=2, U, T).

Gradient-boosted split counts (`gbt_splits` rows) are spread across feature groups, with no group dominating on G or P bases; not interpreted further.

### Controls

* Positive `planted_trace`: AUC 1.0 and calibrated refutation 1.0 in all 13 cells for both models (PASS). `planted_selector`: gbt AUC 1.000 in all 13 cells; lr AUC 0.870 to 0.887 (descriptive, a linear model is not expected to solve it).
* Negative (within-base permutation, 20 permutations): the null band max AUC is 0.51 to 0.52 on G/P bases (0.62 to 0.63 on S ALL where pooled base heterogeneity inflates both observed and null AUC alike); no observed G/P AUC exceeds its band by more than 0.007.
* Arm (b) false refutations: 0 over all cell-populations (the theorem holds numerically).
* Instrument selftest: 0 failures.

### Cost (§5a pricing of unpriced word operations at 0, 0.1, 1)

Because the credited learned refutation count is 0 in every cell except 0.2% on one, "saved by refutation" is 0.0 (printed to one decimal in `raw/verdict.txt`). Net work of the gbt filter as a fraction of decomposer work, all negative: from -0.0010 (n=29, m=2, U, w=0) to -2.31 (n=17, m=3, S, w=1); m=2 cells lie between -0.0010 and -0.18, m=3 cells between -0.022 and -2.31 (every line is in `raw/verdict.txt`). The sign holds at all three weights in all 13 cells, so the conclusion "the filter is a net loss" is band-stable. These are lower-bound counts; no ranking between methods is made.

### Deviations and anomalies (reported, not repaired)

1. Frozen Deviations 1 to 9 of `PROTOCOL.md` stand (rustc not cargo, populations U and S, a = 0, base types, planted label on S, group 7 context feature, pilot-chosen l, replication across two cells, calibrated zero false refutations).
2. **Cards 03 and 05 did not complete.** Their relays say the run was moved to the background and the executor returned before it finished; no receipt exists for either (`raw/receipt-card-03.json`, `-05.json` absent), the build binaries remain in `target/blindspots-11/card-03/` and `card-05/`, and both cards now fail lint ("names repository file ... that binds does not pin"), so `check-relay` could not run (it exits at lint). Of their cells, only the first command of each finished: `raw/n19-m3-U.jsonl` (ends with a `done` line, sha256 prefix addee0bf9c) and `raw/n29-m3-U.jsonl` (done line, fd7cfc648c). Those two files are complete but **have no receipt, so their provenance is not hash-attested** the way the other eleven cells are. They are used in the verdict, flagged here.
3. **Card 07 ran before cards 03 and 05 finished**, so its `verdict` saw 13 files (`verdict input: 13 jsonl files`). The build notes required 07 to run last. Its receipt is write-once; a fresh verdict needs a new card.
4. Relays for cards 01, 02, 04, 06, 07 were not re-saved as full text files (about 60 KB of cell tails). Instead the relayed receipt sha256, card sha256, commit and every cell's stdout sha256 were compared with the receipts and with the sha256 of the raw files on disk: all match (receipts ac5d8bb8, 85718fde, d65fd654, 7be575d4, 4ac4c5af; stdout hashes of all eleven receipted cells and of `verdict`). `raw/relay-3.txt` and `raw/relay-5.txt` hold the two incomplete relays verbatim.
5. Recovery needed (director's step): remove `target/blindspots-11/card-03/` and `card-05/` by hand, move the two unreceipted raw cell files aside or re-run (the cards declare them as outputs), run cards 03 and 05, then a new verdict card. The three missing cells, ~15 CPU-minutes, were not run here; the assess step does not repair relays.

### Caveats

Toy scale: n 17 to 29 with 20,000 targets per cell-population and ten bases per cell; the trace is the only structure available here, and the result says nothing about n = 131 or larger. Host L0 (shared 4-vCPU cloud container, no isolation): no wall-time claim is made, only operation counts. `delta_lo`/`delta_hi` are 95% paired bootstrap intervals over negatives (1,000 resamples). U alone would be a re-derivation of the trace; the deployable population is S, and on S the learned filter refuted none of the 47,218 held-out negatives per model (six cells) with credit. The null result covers only the feature families listed in `PROTOCOL.md` and two model classes (logistic regression, depth-3 boosted trees), not all learnable functions of x_R. No leaderboard row and no `benchmarks/` entry (not a speed result).
