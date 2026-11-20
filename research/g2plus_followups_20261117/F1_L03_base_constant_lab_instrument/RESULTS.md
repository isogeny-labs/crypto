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
The secondary sweep over cc = 0.5, 1.5 and 3.0 is still running, and this file will be updated
when it lands.

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
