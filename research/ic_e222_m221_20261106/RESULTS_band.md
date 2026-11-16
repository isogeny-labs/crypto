# Section 5a band for the IC arm and the matched rho on the same instance (local, unvalidated)

**Status:** unvalidated (pre-gate). Local unisolated host; the replay and the recompute are producer-run (`independent: false`). Not a leaderboard row. The unit is GAE = group additions + doublings, for both arms.

**What was built.** `ic solve-prime-file` now reports, for the default path (pair-sum table, joint relation point), the group operations it charges (`group_additions`, `group_doublings`, derived from the scalars each pass consumes: bits - 1 doublings and popcount - 1 additions for a ladder, popcount(a | b) additions for the joint pass) and the native operations the unit does not charge, named `*_uncharged` as AGENTS.md section 5a expects: `table_probes_uncharged`, `table_insert_attempts_uncharged`, `point_negations_uncharged`, `sqrt_mod_p_calls_uncharged`, `random_scalars_drawn_uncharged`, and the elimination's modular operations (`elimination_modular_multiplications_uncharged`, `..._additions_...`, `..._inversions_...`, `elimination_word_operations_uncharged`, read from the `bitop/v1` tally across the elimination call). `counters_unemitted` is empty for this path; a path that cannot count (the sweep collector, or `IC_SHAMIR` set to an older relation point) flags itself with a `*_not_counted` counter instead. With `--matched-rho-runs K` the same invocation runs the repository's counted negation-map walk (A = 2, the matched reference) K times on the very instance, with distinct walk seeds, and reports its mean GAE and its own counters (`canonicalisations_uncharged`).

**Pricing** (`evidence/band.py`): cost(w) = charged GAE + w x (sum of the `*_uncharged` operation counters), w = 0, 0.1, 1, applied the same way to both arms; byte counters would be listed, not priced (none occur).

**Panel.** 72 fresh instances (generator seed `e222m221-h1default-v1`: Edwards analogs with cofactor 4, Montgomery analogs with cofactor 8, cofactor-1 control; generator rungs 12, 16, 20, 8 each; realised subgroup orders 12 to 21 bits), seed 1, 16 matched-rho runs each, default configuration; the 48 cofactor curves again with `--project-cofactor` (H1). Raw per-instance outputs: `evidence/band/` (hash manifest `band/MANIFEST.json`, binary `band/binary.json`); regenerate with `evidence/run_band_panel.py`.

**Result.** In all 72 default-configuration instances and in all 48 H1 instances the matched rho is cheaper than the index-calculus arm at every weight of the band, so the order never changes inside the band. Pooled medians over the 72 default instances (all sizes together): IC 111,534 / 113,549 / 131,677, rho 453.1 / 482.9 / 751.5, IC / rho 254.3 / 243.7 / 179.2 at w = 0 / 0.1 / 1. With H1 on the 48 cofactor instances: IC / rho 68.0 / 66.4 / 58.0. The distance remaining grows with size; the pooled figures mix sizes, so the per-rung tables are the reading.

### Default configuration, per family and size (medians over the instances in the cell)

| family (cofactor) | subgroup bits | instances | IC cost w=0 / 0.1 / 1 | rho cost w=0 / 0.1 / 1 | IC / rho w=0 / 0.1 / 1 | rho cheaper at every w |
|---|---|---|---|---|---|---|
| E (4) | 12 | 4 | 5,988 / 6,808 / 14,190 | 167.6 / 177.7 / 268.8 | 36.6 / 39.3 / 54.6 | 4 / 4 |
| E (4) | 13 | 4 | 6,417 / 7,259 / 14,823 | 168.3 / 178.5 / 269.8 | 36.3 / 38.8 / 52.1 | 4 / 4 |
| E (4) | 16 | 4 | 113,402 / 115,467 / 134,052 | 467.1 / 498.7 / 783.1 | 249.8 / 238.6 / 177.8 | 4 / 4 |
| E (4) | 17 | 4 | 108,681 / 110,667 / 128,542 | 453.6 / 483.5 / 753.2 | 263.0 / 251.3 / 187.4 | 4 / 4 |
| E (4) | 20 | 3 | 2,853,359 / 2,879,515 / 3,114,920 | 1,426.8 / 1,552.8 / 2,687.5 | 2,051.8 / 1,901.5 / 1,185.3 | 3 / 3 |
| E (4) | 21 | 5 | 1,675,272 / 1,690,987 / 1,832,418 | 1,332.1 / 1,448.5 / 2,495.6 | 1,687.8 / 1,574.0 / 1,012.7 | 5 / 5 |
| M (8) | 12 | 6 | 12,476 / 13,408 / 21,800 | 167.2 / 177.2 / 267.3 | 76.1 / 77.1 / 83.0 | 6 / 6 |
| M (8) | 13 | 2 | 14,042 / 14,973 / 23,349 | 184.1 / 195.8 / 301.5 | 77.4 / 77.6 / 79.4 | 2 / 2 |
| M (8) | 16 | 3 | 175,806 / 178,551 / 203,258 | 491.0 / 524.9 / 830.4 | 379.4 / 361.5 / 264.8 | 3 / 3 |
| M (8) | 17 | 5 | 213,446 / 216,549 / 244,476 | 432.6 / 460.3 / 708.9 | 489.2 / 466.9 / 344.3 | 5 / 5 |
| M (8) | 20 | 4 | 4,157,788 / 4,195,611 / 4,536,011 | 1,176.1 / 1,276.9 / 2,184.5 | 3,251.6 / 3,019.4 / 1,900.2 | 4 / 4 |
| M (8) | 21 | 4 | 4,352,630 / 4,392,138 / 4,747,715 | 1,273.1 / 1,383.6 / 2,377.8 | 3,593.4 / 3,338.2 / 2,104.9 | 4 / 4 |
| W (1) | 12 | 4 | 2,641 / 3,552 / 11,547 | 159.8 / 169.1 / 253.6 | 15.9 / 20.2 / 44.4 | 4 / 4 |
| W (1) | 13 | 4 | 2,662 / 3,531 / 11,566 | 163.2 / 172.7 / 258.3 | 16.1 / 20.4 / 44.8 | 4 / 4 |
| W (1) | 16 | 5 | 28,800 / 29,842 / 39,218 | 417.2 / 443.6 / 681.2 | 70.5 / 68.7 / 58.7 | 5 / 5 |
| W (1) | 17 | 3 | 24,364 / 25,505 / 35,770 | 490.8 / 524.1 / 824.4 | 51.1 / 50.1 / 44.1 | 3 / 3 |
| W (1) | 20 | 3 | 529,221 / 534,702 / 584,036 | 1,360.6 / 1,479.6 / 2,551.1 | 341.9 / 317.0 / 200.9 | 3 / 3 |
| W (1) | 21 | 5 | 596,930 / 603,117 / 658,803 | 1,363.9 / 1,483.5 / 2,560.5 | 419.5 / 389.0 / 243.9 | 5 / 5 |

### With the H1 cofactor projection (cofactor curves only)

| family (cofactor) | subgroup bits | instances | IC cost w=0 / 0.1 / 1 | rho cost w=0 / 0.1 / 1 | IC / rho w=0 / 0.1 / 1 | rho cheaper at every w |
|---|---|---|---|---|---|---|
| E (4) | 12 | 4 | 2,847 / 3,749 / 11,863 | 167.6 / 177.7 / 268.8 | 16.9 / 21.0 / 44.4 | 4 / 4 |
| E (4) | 13 | 4 | 2,660 / 3,603 / 11,392 | 168.3 / 178.5 / 269.8 | 15.6 / 20.1 / 43.7 | 4 / 4 |
| E (4) | 16 | 4 | 30,244 / 31,393 / 41,732 | 467.1 / 498.7 / 783.1 | 64.8 / 63.1 / 53.7 | 4 / 4 |
| E (4) | 17 | 4 | 27,824 / 28,949 / 39,070 | 453.6 / 483.5 / 753.2 | 67.4 / 65.9 / 57.5 | 4 / 4 |
| E (4) | 20 | 3 | 607,188 / 613,405 / 669,357 | 1,426.8 / 1,552.8 / 2,687.5 | 438.9 / 407.3 / 256.7 | 3 / 3 |
| E (4) | 21 | 5 | 623,615 / 630,050 / 687,960 | 1,332.1 / 1,448.5 / 2,495.6 | 469.4 / 436.3 / 276.8 | 5 / 5 |
| M (8) | 12 | 6 | 3,012 / 3,901 / 11,906 | 167.2 / 177.2 / 267.3 | 17.9 / 21.5 / 43.8 | 6 / 6 |
| M (8) | 13 | 2 | 2,642 / 3,493 / 11,146 | 184.1 / 195.8 / 301.5 | 14.6 / 18.2 / 38.1 | 2 / 2 |
| M (8) | 16 | 3 | 31,951 / 33,135 / 43,795 | 491.0 / 524.9 / 830.4 | 64.1 / 62.2 / 52.0 | 3 / 3 |
| M (8) | 17 | 5 | 31,496 / 32,679 / 43,325 | 432.6 / 460.3 / 708.9 | 71.5 / 69.6 / 59.6 | 5 / 5 |
| M (8) | 20 | 4 | 623,361 / 629,792 / 687,676 | 1,176.1 / 1,276.9 / 2,184.5 | 534.7 / 497.6 / 317.8 | 4 / 4 |
| M (8) | 21 | 4 | 570,848 / 576,784 / 630,206 | 1,273.1 / 1,383.6 / 2,377.8 | 471.5 / 438.6 / 279.6 | 4 / 4 |

**Reading.** This is a distance, not a verdict: at these toy sizes the index-calculus arm needs between about 15 and 3,600 times the matched rho's cost, depending on the family, the size and the weight, and the ratio grows with the size. H1 cuts the cofactor curves' ratio by 3.6 to 7.6 times at the 20 and 21 bit sizes (for example Edwards at 21 bits, w = 0: from about 1,700 to about 470; Montgomery at 21 bits, w = 0: from about 3,600 to about 470). The weight w moves the ratio by up to about 2.8 times at the smallest sizes of the cofactor-1 control (15.9 at w = 0 to 44.4 at w = 1 for 12-bit subgroups), where the unpriced operations are a larger share of the IC arm, and by less than 2 times from about 16 bits up, because the charged group operations dominate both arms there.

**Limits.**
- Toy sizes only (subgroup orders up to 2^21); no native E-222 / M-221 solve was run. The ratio at native size is not inferred from this table.
- Cells hold 2 to 6 instances; medians within a cell are not tight, and no interval is given.
- The IC arm's group operations are derived from the scalars, not read from the arithmetic, and assume the pass does not hit a degenerate case (a coincidence between the accumulator and the added point); the field operations behind each group operation are not counted in either arm (the unit is GAE).
- The rho arm lists the counters its walk names (`canonicalisations_uncharged`); any unpriced rho work the walk does not name (for example distinguished-point table handling) is not in its count, so its cost is itself a floor. That could shrink the gap, but by orders of magnitude only if the missing work were comparable to the gap.
- `matched_rho_eligible` records only that a and b are nonzero (the Weierstrass image has generic j); no automorphism beyond negation was searched for.
- The old-defaults arm of the earlier panels is not part of this table.
- Local unisolated host, producer-run replay and recompute. The gate tool still refuses the draft (`evidence/gate-output.txt`): both pinned files say `independent` is not true, and the robustness rows are in the artifact store; the rho reference and band no longer appear among its refusals.
