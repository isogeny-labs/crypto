# P3 results: plane models with residual e = 3 (L08 v2), genus 3/4/6

Run 2026-11-20 under `PROTOCOL.md` (frozen, `frozen.sha256`). Instrument: `code/l08v2.py`. Data:
`raw/*.jsonl`, 72 files, 360 rows. Analysis: `code/analyze_p3.py` gives `raw/analysis.md`.
Figure: `p3_cost_vs_rho.svg` / `.png`. Every number is `[pcc:unverified]`. This is one producer
with no independent replay yet.

**This is not a DLP solve.** It measures the counted cost of reaching a relation matrix of full
size by lines on a plane model, with linear algebra (LA) as a bracket. Descent, Picard-group
arithmetic and the certificate are not included. A solve costs at least this much.

## Gates

| Gate | Result |
|---|---|
| `selftest` | ALL PASS (`raw/selftest.log`) |
| G1 + G2 relation verification failures | 0 in 360 rows |
| Rows where `useful` exceeds distinct lines tried (fix (a)) | 0 |
| A/A, 2LP cF=4 rep 2 / rep 1 `total_low` | 24 pairs per model. d=4: 0.96-1.05 (geometric mean 1.005); d=5 m=0: 0.95-1.12 (0.997); d=5 m=2: 0.91-1.05 (0.996) |

## Cost against matched rho

Values are the best arm per curve, geometric mean over 4 curves. Rho is the expected
negation-map rho with the measured hyperelliptic Cantor step of the same genus. That step is a
lower bound for a non-hyperelliptic Picard step, so rho is cheap here and the comparison is
conservative for IC. Each ratio gives the bracket low to high.

| Model (genus) | p=257 | p=1031 | p=4099 | p=8209 |
|---|---|---|---|---|
| Smooth quartic, control (g=3) | 0.24 - 0.87 | 0.11 - 1.11 | 0.052 - 1.28 | 0.038 - 1.46 |
| Two-node quintic (g=4) | 0.067 - 0.10 | 0.017 - 0.051 | 0.0055 - 0.037 | **0.0033 - 0.032** |
| Smooth quintic (g=6) | 1.1e-4 - 1.5e-4 | 6.7e-6 - 2.0e-5 | 5.6e-7 - 3.6e-6 | **1.6e-7 - 1.6e-6** |

Against plain rho (no negation credit), the low end is lower by a factor of sqrt(2).

## Slopes (log cost against log p)

| Model | Total low, all p | Total low, 4 largest p | Total high | Relation phase | Prediction |
|---|---|---|---|---|---|
| d=4 m=0 (g=3) | 0.968 | **0.997** | 1.647 | 1.010 | e=2: 1 |
| d=5 m=2 (g=4) | 1.139 | **1.211** | 1.690 | 1.075 | e=3: 4/3 |
| d=5 m=0 (g=6) | 1.143 | **1.211** | 1.699 | 1.093 | e=3: 4/3 |

## Predictions

- **P3-a (control slope in [0.85, 1.3]): holds**, 0.997.
- **P3-b (d=5 slopes in [1.15, 1.5]): holds** for both models, 1.211 each. The slope rises with p
  (1.14 over all primes, 1.21 over the 4 largest), consistent with approaching 4/3 from below.
  The high end of the bracket (LA before large-prime elimination) has slope about 1.69 and is
  outside the band. The truth lies between, so the exponent is not established by this
  measurement alone: executed LA is needed.
- **P3-c (genus 4 at p=8209 below rho): holds**, at 0.0033 - 0.032.
- **P3-d (genus 6 at p=8209 below 0.01 of rho): holds**, at 1.6e-7 - 1.6e-6.

## Reading

- **The two-node quintic model of a genus-4 curve sits 30-300x below matched rho at p=8209**,
  across the whole LA bracket. The gap grows with p.
  - Every non-hyperelliptic genus-4 curve has such a model (Brill-Noether: g^2_5 exists,
    rho(4,2,5) = 1), so this is not a special family.
  - Compare the lab's validated hyperelliptic genus-4 arm, which sits at 0.88 / 0.64 of ideal
    negation rho at p = 16411 / 32771 (`docs/ACTIVE.md`, PR #4693; a different instrument).
  - Relations from lines need no group arithmetic: one line costs about 100-300 FPM against a
    480 FPM Cantor step, and the residual is degree 3 rather than 4.
- **The two quintic models cost the same.** They share d and e, so their relation mechanics are
  identical. Genus enters only through rho. That is why the genus-6 smooth quintic is
  astronomically below its rho, p^1.2 against p^3. It is a special family (codimension 3 in
  M_6), not a statement about generic genus 6.
- **The control reproduces Diem's quartic slope (1.0).** This validates the instrument's slope
  measurement, and the L08 v1 defects are gone.

## What this does not show, and what would close it

- **No solve.** A complete solve needs:
  - descent of a target into the base;
  - executed LA on the merged matrix, to settle the bracket;
  - Picard-group arithmetic to plant and certify a log.

  The lab has these for plane quartics (`plane_quartic_pic.rs`, `diem_quartic.rs`), but not for
  nodal quintics.
- **No transfer.** A hyperelliptic genus-4 Jacobian has no plane quintic model, so this does not
  speed up hyperelliptic targets. It says that genus-4 non-hyperelliptic DLP is much easier than
  hyperelliptic genus-4 DLP, in this cost unit.
- **Rho is under-priced** by the hyperelliptic step. A matched non-hyperelliptic Picard step would
  only widen the gap.
- **No independent replay yet.**
