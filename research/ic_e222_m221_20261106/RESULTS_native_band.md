# Native-size band for E-222 and M-221: a model projection (local, unvalidated)

**Status:** unvalidated (pre-gate). **This is a projection from a model, not a measurement.** A real solve at these sizes needs more than 2^100 operations and was not run. Not a leaderboard row. Producer-run model and check (`independent: false` in the evidence bundle).

**What it is.** A counter model of the default index-calculus path of `ic solve-prime-file` (pair-sum table, joint relation point), written down in `native_band/native_band_model.py`, checked against the measured counters of the 72 + 48 band runs (`evidence/band/`), then evaluated at the real E-222 (n = 220 bits, cofactor 4) and M-221 (n = 219 bits, cofactor 8). The model: a table of F factor-base points holds 2F^2 entries; a random R = aG + bQ hits a usable entry with probability about 2F^2 / n (divided by the cofactor unless the H1 projection puts every entry in the prime-order subgroup), so collecting F + 2 relations takes (F + 2) n / (2F^2) trials, each costing about 1.75 x bits(n) group operations (bits - 1 doublings, 0.75 bits + 1 additions) and 3 unpriced operations (two random scalars, one table probe); the table build, the verification, the factor-base square roots and the elimination (measured at about 4.7 F^2 operations for F = 32 to 256) are added. Rho: GAE about S sqrt(r) with S between 0.886 (the negation-walk floor) and 1.8 (the measured spread), unpriced about 0.43 GAE (the `canonicalisations_uncharged` ratio measured on the rho runs).

## Check of the model against the measured panel

| counter | default configuration (72), median observed / predicted [q1, q3] | with H1 (48), median observed / predicted [q1, q3] |
|---|---|---|
| `trials_total` | 0.931 [0.825, 1.076] | 1.032 [0.904, 1.101] |
| `group_additions` | 0.949 [0.859, 1.057] | 1.018 [0.928, 1.074] |
| `group_doublings` | 0.925 [0.817, 1.075] | 1.030 [0.904, 1.099] |
| `table_probes_uncharged` | 0.931 [0.825, 1.076] | 1.032 [0.904, 1.101] |
| `random_scalars_drawn_uncharged` | 0.931 [0.825, 1.076] | 1.032 [0.904, 1.101] |
| `table_insert_attempts_uncharged` | 1.000 [1.000, 1.000] | 1.000 [1.000, 1.000] |
| `point_negations_uncharged` | 1.000 [1.000, 1.000] | 1.000 [1.000, 1.000] |
| `sqrt_mod_p_calls_uncharged` | 0.929 [0.857, 0.986] | 0.914 [0.843, 0.986] |
| `elimination_ops_uncharged` | 0.867 [0.810, 0.953] | 0.996 [0.933, 1.057] |

The deterministic counters (table inserts and negations) are reproduced exactly. The stochastic ones (trials, group operations, probes, scalars) agree within a 7% median bias and an interquartile range of about +-10%, which is the sampling noise of 34 relations per run (about 17%); the factor-base square roots are over-predicted by about 8%, and the elimination by up to 13% in the default configuration. Trials per relation fall as F^-2 (measured 530, 136, 29, 8 for F = 8 to 64 on 16 to 17-bit cofactor-1 instances, saturating once 2F^2 exceeds n).

## Mid-size check: subgroup orders of about 2^27 and 2^31, in the regime n >> 2F^2

`ic solve-prime-file` was limited to fields below 4 x 10^7 by a brute-force point count; it now takes fields up to 2^50 when the group order is supplied (the generator computes it by baby-step giant-step; the count is still done and cross-checked below 4 x 10^7; above, only the Hasse bound and divisibility are checked). With that, 40 runs on 12 fresh instances (Edwards, Montgomery and cofactor-1; generator rungs 26 and 30, realised subgroup orders of 26 to 31 bits; F = 128 and 512; 8 matched-rho runs on each instance; `evidence/run_mid_panel.py`, outputs in `evidence/mid/`). Every run recovered the planted logarithm and every matched-rho run verified. This is the regime the native sizes live in (the table holds a vanishing fraction of the group), unlike the toy panel where 2F^2 was a visible fraction of n.

Model against these runs (median observed / predicted [q1, q3]): trials 1.001 [0.989, 1.025] for the default configuration (24 runs) and 0.962 [0.924, 1.022] with H1 (16 runs), which is inside the sampling noise of 130 to 514 relations per run; the deterministic counters are exact; factor-base square roots 0.99; the elimination is over-predicted by 16 to 18% (a coefficient of about 3.9 F^2 fits better than 4.7 at F = 128 and 512), which does not matter next to the trial term.

Distance at these sizes (IC cost over the matched rho's, w = 0, medians over the runs in a cell; all 40 runs have the rho cheaper at every weight of the band): cofactor-1 curves 6,500 (30 bits, F = 128) and 1,727 (F = 512); Edwards with H1 7,488 (31 bits, F = 128) and 1,658 (F = 512); Montgomery with H1 6,756 and 1,738; without H1, Edwards 28,817 and 7,141, Montgomery 54,894 and 13,130 (31-bit subgroups). Quadrupling F cuts the distance by about 4, and a 16-fold increase in n raises it by about 4 (cofactor-1: 1,783 at 26 bits to 6,500 at 30 bits for F = 128), as the model's sqrt(n) / F says.

Cross-check of the projection: scaling each measured mid-size ratio (H1, F = 512, w = 0) by sqrt(n_native / n) x (F / 2^20) x (bits_native / bits) gives log2 distance 97.8 (E-222, 4 runs, range 97.4 to 97.9) and 96.6 (M-221, 4 runs, range 96.4 to 96.8), against the model's own range at F = 2^20 with H1 of 96.7 to 97.8 and 95.7 to 96.7. The scaling uses the measured trial counts, per-trial costs and the measured rho GAE of those runs, not the model's formulas, so it is a check of the formulas' structure (sqrt(n) / F and the per-trial cost growth with the bit length), and the two agree to within about a bit.

## Projection

| curve | H1 | factor base F | table entries | trials (log2) | IC cost, w = 0 / 1 (log2 GAE) | IC / rho (log2), best to worst case for IC |
|---|---|---|---|---|---|---|
| E-222 | no | 2^5 | 2^11 | 216.1 | 224.7 / 224.7 | 2^113.8 to 2^114.8 (w = 0), 2^113.3 to 2^114.3 (w = 1) |
| E-222 | no | 2^15 | 2^31 | 206.0 | 214.6 / 214.6 | 2^103.7 to 2^104.8 (w = 0), 2^103.2 to 2^104.3 (w = 1) |
| E-222 | no | 2^20 | 2^41 | 201.0 | 209.6 / 209.6 | 2^98.7 to 2^99.8 (w = 0), 2^98.2 to 2^99.3 (w = 1) |
| E-222 | no | 2^25 | 2^51 | 196.0 | 204.6 / 204.6 | 2^93.7 to 2^94.8 (w = 0), 2^93.2 to 2^94.3 (w = 1) |
| E-222 | yes | 2^5 | 2^11 | 214.1 | 222.7 / 222.7 | 2^111.8 to 2^112.8 (w = 0), 2^111.3 to 2^112.3 (w = 1) |
| E-222 | yes | 2^15 | 2^31 | 204.0 | 212.6 / 212.6 | 2^101.7 to 2^102.8 (w = 0), 2^101.2 to 2^102.3 (w = 1) |
| E-222 | yes | 2^20 | 2^41 | 199.0 | 207.6 / 207.6 | 2^96.7 to 2^97.8 (w = 0), 2^96.2 to 2^97.3 (w = 1) |
| E-222 | yes | 2^25 | 2^51 | 194.0 | 202.6 / 202.6 | 2^91.7 to 2^92.8 (w = 0), 2^91.2 to 2^92.3 (w = 1) |
| M-221 | no | 2^5 | 2^11 | 215.1 | 223.7 / 223.7 | 2^113.8 to 2^114.8 (w = 0), 2^113.3 to 2^114.3 (w = 1) |
| M-221 | no | 2^15 | 2^31 | 205.0 | 213.6 / 213.6 | 2^103.7 to 2^104.7 (w = 0), 2^103.2 to 2^104.2 (w = 1) |
| M-221 | no | 2^20 | 2^41 | 200.0 | 208.6 / 208.6 | 2^98.7 to 2^99.7 (w = 0), 2^98.2 to 2^99.2 (w = 1) |
| M-221 | no | 2^25 | 2^51 | 195.0 | 203.6 / 203.6 | 2^93.7 to 2^94.7 (w = 0), 2^93.2 to 2^94.2 (w = 1) |
| M-221 | yes | 2^5 | 2^11 | 212.1 | 220.7 / 220.7 | 2^110.8 to 2^111.8 (w = 0), 2^110.3 to 2^111.3 (w = 1) |
| M-221 | yes | 2^15 | 2^31 | 202.0 | 210.6 / 210.6 | 2^100.7 to 2^101.7 (w = 0), 2^100.2 to 2^101.2 (w = 1) |
| M-221 | yes | 2^20 | 2^41 | 197.0 | 205.6 / 205.6 | 2^95.7 to 2^96.7 (w = 0), 2^95.2 to 2^96.2 (w = 1) |
| M-221 | yes | 2^25 | 2^51 | 192.0 | 200.6 / 200.6 | 2^90.7 to 2^91.7 (w = 0), 2^90.2 to 2^91.2 (w = 1) |

The full grid (F = 2^5 to 2^30, both curves, with and without H1, w = 0, 0.1, 1) is in `native_band/native_band_results.json`.

## Reading

- The cost of this algorithm falls as 1/F while the table grows as F^2; the unpriced operations and the fixed costs (table build, elimination) are negligible next to the trial term at every F shown, so the elimination's scaling does not change the projection, and the IC cost itself is the same at w = 0 and w = 1 to within 0.1 in log2; the distance moves by about half a bit between w = 0 and w = 1 because the matched rho's own unpriced operations (about 0.43 per GAE) are weighted too.
- At F = 2^20 (a table of 2^41 entries, already beyond ordinary memory) with H1, the projection is 207.6 (E-222) and 205.6 (M-221) in log2 GAE; the matched rho is about 2^110 to 2^111, so the distance is about 2^97 to 2^98 on E-222. H1 buys a factor of the cofactor (2 bits for E-222, 3 bits for M-221). The distance remaining is reported as such; it is the distance of this algorithm with a generic factor base and a two-point decomposition, and says nothing about other decompositions or other algorithms.

## Limits

- It is an extrapolation across about 190 bits of subgroup order (the largest measured subgroup is 31 bits); the model's structure (uniform hits, success probability proportional to table size) is checked in the regime n ~ F^2 (toy panel) and in the regime n >> 2F^2 (mid-size panel, 2^27 to 2^31), not at n ~ 2^220.
- At native size the hit probability is in the linear regime (collisions among table points are negligible), where the model is the simplest; the mid-size panel tests that regime (F = 128 and 512, n up to 2^31), the toy panel the saturated one. The mid-size panel has 1 to 2 runs per cell and two factor-base sizes.
- Elimination cost at F >> 256 and fill-in were not measured; a dense F^3 elimination would add about 2^90 at F = 2^30, still negligible.
- Memory (2F^2 entries) is not constrained in the projection beyond being reported; F >= 2^20 is not physically available.
- The rho constants S and the 0.43 unpriced ratio are measured at 10 to 21 bits; the range S in [0.886, 1.8] covers the spread and does not change the result at this scale.
- Square roots are counted as one operation each, whatever their cost; so are table probes. The unit prices operations, not their size.
- Producer-run; the gate tool does not read this note.
