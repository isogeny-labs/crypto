# F1 results: 2LP base constant cc = 1 against cc = 2, genus 3, lab `genus_ladder`

Run 2026-11-20 under the frozen `PROTOCOL.md`, with two recorded deviations:
- **D1 (`raw/DEVIATIONS_RUNSPLIT.txt`):** runs split per prime.
- **D2 (`raw/DEVIATION_D2_BINARY.txt`):** the unpatched `fffa1b1243` build replaces the
  frozen binary, which no longer exists. The substitute reproduces the recorded pilot cell
  field for field.

Data: `raw/cc{1,2}_p{8209,16411,32771}.jsonl`, concatenated into `raw/cc1.jsonl` and
`raw/cc2.jsonl`. Analysis is the frozen `analyze.py` (selftest OK). Its outputs are
`analysis/analysis_primary.json`, `analysis/tables_primary.md` and `raw/analyze_primary.log`.

Every number is `[pcc:unverified]`. The producer applied the frozen rule. No independent
assessment has been made yet.

## Frozen-rule verdict: SUPPORTED

The criterion is G(p) = 1 − mean of paired ratios FPM_mid(cc=1) / FPM_mid(cc=2), over 12 pairs
per prime (3 curves × 4 seeds), with a hierarchical bootstrap.

| p | G_mid | 95% CI | A/A G_mid | LA share cc2 / cc1 | lab arm / rho, cc2 → cc1 |
|---|---|---|---|---|---|
| 8209 | 0.356 | [0.346, 0.367] | 0.011 | 0.74 / 0.34 | 0.68 → 0.44 |
| 16411 | 0.376 | [0.373, 0.380] | −0.005 | 0.76 / 0.34 | 1.31 → 0.81 |
| 32771 | 0.397 | [0.393, 0.401] | 0.001 | 0.77 / 0.35 | 0.63 → 0.38 |

PROTOCOL §7 requires G ≥ 0.15 with the CI above 0 at every prime, and G(32771) above the A/A
interval. Both hold. Within the S1 primary pair, cc=1 is the cheaper setting at every prime.
The secondary sweep (cc = 0.5, 1.5, 3.0) completed with no exhausted cells; see the section below.

The mid band prices an inversion at I=10 and an LA product (mod r) at κ=9 F_p multiplications
(PROTOCOL §4).

## Caveat: the gain depends entirely on the LA price (post-hoc sensitivity, not part of the frozen rule)

The low band (I=1, κ=1) gives **G_low ≈ −0.30**: at that price cc=1 is about 30% worse. The
reason is that cc=1 moves work from linear algebra into relation collection, so the outcome
turns on the price of one mod-r LA product relative to one F_p multiplication. Recomputed from
the same records, with I=10:

| κ | 1 | 2 | 3 | 4 | 6 | 9 | break-even κ |
|---|---|---|---|---|---|---|---|
| G, p=8209 | −0.33 | −0.13 | 0.01 | 0.11 | 0.24 | 0.36 | 2.91 |
| G, p=16411 | −0.33 | −0.11 | 0.03 | 0.13 | 0.26 | 0.38 | 2.77 |
| G, p=32771 | −0.32 | −0.10 | 0.05 | 0.15 | 0.28 | 0.40 | 2.63 |

The lab's genus-4 cost calibration (`research/genus4_cost_calibration_20261119`) measured the
price in wall time:
- about 0.0043 GAE per LA product;
- one F_p multiplication is about 1/480 of a genus-4 group operation, about 0.0021 GAE;
- so κ is about **2**.

The calibration's ERRATA E2 calls its split between relation and LA work fragile, and it was
measured at genus 4, not 3.

At κ ≈ 2, cc = 1 is about 10-13% more expensive than cc = 2.

**Reading.** F1 confirms the L03 effect as specified: re-tuning the base constant from 2 to 1
cuts cost 36-40% when an LA product is priced at 9 F_p multiplications. But under the lab's own
measured price, κ ≈ 2, the re-tuning does not pay and slightly hurts. **Do not adopt cc = 1 on
the strength of this result.**

What would settle it: measure κ at genus 3 directly, with the same calibration method at
p = 8209-32771. If κ < 2.6, keep cc = 2. If κ > 3, adopt cc = 1.

## Distance to rho

At the mid band the lab arm with cc = 2 sits at 0.63-1.31 of matched negation rho. With cc = 1
it sits at 0.38-0.81. Under the measured κ ≈ 2 the two settings are within about 10% of each
other, so the distance to rho is essentially unchanged by this lever.

## Secondary sweep S1: the best base constant depends on the LA price (post hoc)

Data: `raw/cc{0.5,1.5,3.0}_p*.jsonl`, 3 curves x 2 seeds, no exhausted cell. Rerun with the
secondary inputs, the frozen analysis (`analysis/tables.md`, `raw/analyze.log`) gives the same
verdict. Its S1 line compares only cc = 1 and 2.

The table below is post hoc. Mean FPM over seeds 1-2 (I=10), for each base constant cc and LA
price κ:

| p | κ | cc 0.5 | cc 1 | cc 1.5 | cc 2 | cc 3 | best |
|---|---|---|---|---|---|---|---|
| 8209 | 1 | 6.28e7 | 3.74e7 | 3.02e7 | **2.84e7** | 3.11e7 | 2 |
| 8209 | 2 | 6.33e7 | 3.94e7 | **3.44e7** | 3.54e7 | 4.55e7 | 1.5 |
| 8209 | 9 | 6.74e7 | **5.35e7** | 6.35e7 | 8.42e7 | 1.46e8 | 1 |
| 16411 | 1 | 1.58e8 | 9.34e7 | 7.54e7 | **7.02e7** | 7.74e7 | 2 |
| 16411 | 2 | 1.60e8 | 9.85e7 | **8.60e7** | 8.82e7 | 1.15e8 | 1.5 |
| 16411 | 9 | 1.70e8 | **1.34e8** | 1.60e8 | 2.14e8 | 3.77e8 | 1 |
| 32771 | 1 | 3.85e8 | 2.24e8 | 1.81e8 | **1.70e8** | 1.90e8 | 2 |
| 32771 | 2 | 3.88e8 | 2.37e8 | **2.08e8** | 2.17e8 | 2.87e8 | 1.5 |
| 32771 | 9 | 4.13e8 | **3.26e8** | 3.97e8 | 5.42e8 | 9.66e8 | 1 |

At the lab's measured κ ≈ 2, the best constant is **cc = 1.5**. It saves 2.5-4.4% against the
lab's cc = 2: 3.44e7 against 3.54e7, 8.60e7 against 8.82e7, and 2.08e8 against 2.17e8. The
36-40% L03 effect appears only when LA is priced far above its measured cost.

**Recommendation:** keep cc = 2, or move to 1.5 for a few percent, until κ at genus 3 is measured
directly.
