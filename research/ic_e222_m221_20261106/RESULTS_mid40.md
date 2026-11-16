# Mid-size panel at 38 to 40 bit subgroups (H1 on E/M, default on the cofactor-1 control)

Extends `RESULTS_mid34.md`. One instance per family at 38 and 40 bits (the E curve's prime subgroup is one bit longer than labelled: 39 bits for E38), F = 512 and 1024, two matched-rho runs each, `--extra-relations 2`, table forced. The default arm on E and M was skipped (`--skip-default-cofactor`, added to `evidence/run_mid_panel.py`): at these sizes it needs hours per run, and its trial law was checked at 34 to 35 bits. The cofactor-1 curve W runs its default configuration. Data: `evidence/mid40/` (11 outputs, `MANIFEST.sha256`). Everything is [pcc:unverified], one run per cell, producer-run.

| instance | n bits | F | relations | trials observed | trials predicted | obs / pred | IC (GAE, w=0) | matched rho (GAE, median of 2) | IC / rho |
|---|---|---|---|---|---|---|---|---|---|
| E38 H1 | 39 | 512 | 514 | 238.3M | 269.5M | 0.88 | 1.55e10 | 5.3e5 | 2.9e4 |
| E38 H1 | 39 | 1024 | 1026 | 131.6M | 134.5M | 0.98 | 8.6e9 | 5.3e5 | 1.6e4 |
| E40 H1 | 40 | 512 | 514 | 979.7M | 1077.9M | 0.91 | 6.7e10 | 1.35e6 | 5.0e4 |
| E40 H1 | 40 | 1024 | 1026 | 497.2M | 537.9M | 0.92 | 3.4e10 | 1.35e6 | 2.5e4 |
| M38 H1 | 38 | 512 | 514 | 260.0M | 269.5M | 0.97 | 1.7e10 | 8.4e5 | 2.0e4 |
| M38 H1 | 38 | 1024 | 1026 | 136.5M | 134.5M | 1.02 | 8.9e9 | 8.4e5 | 1.1e4 |
| M40 H1 | 40 | 512 | 514 | 1090.0M | 1077.9M | 1.01 | 7.5e10 | 6.8e5 | 1.1e5 |
| M40 H1 | 40 | 1024 | 1026 | 518.7M | 537.9M | 0.96 | 3.6e10 | 6.8e5 | 5.2e4 |
| W38 default | 38 | 512 | 514 | 276.8M | 269.5M | 1.03 | 1.8e10 | 5.6e5 | 3.2e4 |
| W38 default | 38 | 1024 | 1026 | 139.2M | 134.5M | 1.03 | 9.1e9 | 5.6e5 | 1.6e4 |
| W40 default | 40 | 512 | 514 | 1086.9M | 1077.9M | 1.01 | 7.5e10 | 1.06e6 | 7.1e4 |

- All 11 runs recovered the planted logarithm and every matched-rho walk ended at it.
- Observed over predicted trials: mean 0.98, range 0.88 to 1.03. The sampling spread of 514 and 1026 relations is about 4.4% and 3.1%. The E-curve H1 cells sit low in all four (0.88, 0.98, 0.91, 0.92, mean 0.92; the 34-bit E H1 cell was 1.06), so the model may overpredict by a few percent on E; M and W show no such offset. One instance each, so the offset is a pattern to watch, not an established bias.
- Doubling F from 512 to 1024 halves the trials in every pair (ratios 0.54 to 0.57 of the 512 value including the 2F^2 table term), as the model's n / F says; the distance to the matched rho halves with it. IC cost still rises 5 to 6% at w = 1.
- The matched rho is cheaper than index calculus in every cell: the distance runs from 1.1e4 to 1.1e5 times. The rho median comes from two runs and varies by about 2x between instances of the same size (e.g. M40 6.8e5 against E40 1.35e6), which sets the spread in the last column.
- The 40-bit cells cost 35 to 76 minutes each; the largest measured subgroup is now 40 bits.

Limits: one instance per family and size; two rho runs per instance, unweighted GAE (rho is not repriced across the section 5a band); W40 at F = 1024 was lost to a container restart and not rerun; the E/M default arm was not run at these sizes; panel `binary.json` was not written (the run was interrupted), `evidence/mid40/binary.json` records the sha256 of the `ic` binary on disk after the run (the panel's blake3 was not captured). Native size is 218 to 220 bits.
