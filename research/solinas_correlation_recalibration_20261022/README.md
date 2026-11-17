# P-256 Solinas bit-pair correlations: z-score recalibration (2026-10-22)

## Defect

`flag_outliers` and the test `solinas_correlation_z_distribution` in
`src/cryptanalysis/solinas_correlations.rs` computed
`z = (p_joint - E) / sqrt(E(1-E)/n)` with `E = p_r * p_c`. Under independence
with estimated marginals the null standard error of `p_joint - p_r p_c` is
`sqrt(p_r(1-p_r) p_c(1-p_c) / n)`. At `p_r = p_c = 1/2` the old scale is
`sqrt(0.1875/n)` against the correct `sqrt(0.0625/n)`, so every z was too small
by `sqrt(3)`. The fix is `CorrelationTable::z_score`, used by both callers.

## Calibration test

`z_score_is_calibrated_on_independent_bits` (seed 20261022): 64 x 128 = 8,192
pairs of independent uniform bits, n = 4,000. Result
(`calibration_test_output.txt`): fraction |z| > 3 is 0.00183 (nominal 0.0027),
max |z| 3.45 (sqrt(2 ln m) = 4.25), E[z^2] within 0.9 to 1.1 (asserted). The
test asserts fraction in [0.0008, 0.0060], max within 0.8 of sqrt(2 ln m), and
E[z^2] in [0.9, 1.1]. The old scale gives fraction about 0 and max near 2.0 on
this population. The pairs share marginals, so they are not independent and the
max sits below the independent-pairs value; the test bounds are set for that.

## Re-run

```
SOLINAS_CORR_N=1000000 cargo test --release --lib \
  cryptanalysis::solinas_correlations::tests::solinas_micro_bit_correlations \
  -- --nocapture --exact
```

Seed 42 (fixed in the test), N = 10^6, release build, shared target dir. Raw
output: `raw_output_n1e6.txt`. The test's own runtime is 75.2 s (the `real` time of
27 min 58 s in the file includes waiting for the shared cargo lock and the
build).

| | old scale (recorded) | corrected |
| --- | --- | --- |
| max \|z\| | 2.46 | 4.26 |
| top 20 range | 2.17 to 2.46 | 3.76 to 4.26 |
| Bonferroni threshold (alpha 0.001/131,072, as the test prints) | 6.11 | 6.11 |

Ratio 4.26 / 2.46 = 1.73, the predicted sqrt(3). Top outliers (|z|): (149, 54)
4.26, (197, 164) 4.12, (117, 119) 4.02, (103, 101) 4.01, (122, 185) 3.97.
Pairs are (output bit i, input bit j).

The histogram of all 131,072 |z| (`z_distribution.svg`, counts in
`expected.tsv`) matches the standard normal: bins of width 0.5 observed versus
expected 49,972/50,191; 39,615/39,291; 23,998/24,077; 11,484/11,549;
4,392/4,336; 1,263/1,274; 276/293; 68/53; 4/7.4; 0/0.8.

## Verdict

The null verdict survives. The corrected max |z| = 4.26 is the size a calibrated
null gives for 131,072 pairs (sqrt(2 ln m) = 4.84 for m = 131,072), and it
is far below the 6.11 threshold. The old recorded value 2.46 was an artefact of
the deflated scale, not evidence for an unusually clean null. This is a
single-seed run; it is not an independent replication.

## Files

- `raw_output_n1e6.txt`, `calibration_test_output.txt`: raw command output.
- `z_distribution.svg`: |z| histogram versus N(0,1); `expected.tsv`: bin counts.
- `report.pdf`: this README rendered (pandoc, then headless Chromium).
