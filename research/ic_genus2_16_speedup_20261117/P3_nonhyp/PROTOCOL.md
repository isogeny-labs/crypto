# P3 protocol: non-hyperelliptic plane models with residual e = 3 (L08 v2)

Frozen 2026-11-20, before the main run, after a p=257 pilot (`pilot/`). Freezing is
recorded by `frozen.sha256`. Every number is `[pcc:unverified]`.

## Question

On plane-curve models, does relation collection by lines reach the double-large-prime
exponent 2 - 2/e predicted by residual degree e, in a counted-cost instrument?

| Model | Genus | Residual e | Exponent | Hyperelliptic 2LP exponent, same genus |
|---|---|---|---|---|
| Plane quintic with 2 nodes (`d=5 m=2`) | 4 | 3 | 4/3 | 3/2 |
| Smooth plane quintic (`d=5 m=0`), a special family | 6 | 3 | 4/3 | 5/3 |
| Smooth plane quartic (`d=4 m=0`), control: Diem's method | 3 | 2 | 1 | — |

Where does each model's cost sit against matched rho?

## Instrument

`code/l08v2.py`: lead L08's collector (`research/g2plus_leads_20261116/L08_*`,
`code/l08.py`) with exactly the two fixes its assessor required (L08 ASSESSMENT §5).

- **(a) Each line counted once.** A line already seen is skipped, and its trial and
  arithmetic stay charged.
- **(b) Linear algebra reported as a bracket.**
  - `la_high` is the v1 charge on the matrix before large-prime elimination.
  - `la_low` is 3·(nb+1)·useful·(d-1): nb columns after elimination, one row per
    useful combination, no fill.
  - Totals are reported at both ends.

The arm grid is changed from v1:
- FB: every smooth F_p-point in the base;
- 2LP with base constant cF in {2, 4, 8};
- an A/A replicate of 2LP cF=4 with an independent pair order.

Inherited gates:
- G1/G2 relation verification on every relation: Weil reciprocity / function check;
- G3 Weil gate (`gate` subcommand);
- `selftest` passes.

## Grid

- **Primes:** p in {257, 521, 1031, 2053, 4099, 8209}.
- **Curves:** 4 per model (indices 0-3, the first admitted seeds).
- **Models:** `d=4 m=0`, `d=5 m=2` and `d=5 m=0`.

## Metrics (`code/analyze_p3.py`)

- **Cost per (model, p, curve):** the best non-exhausted arm by `fpm_total_low`.
  - FPM = F_p multiplications + 10 x inversions.
  - The total is the relation phase (enumeration + relation work) plus the linear
    algebra (LA) bracket.
- **Rho.** Expected negation-map steps sqrt(pi p^g / 4), times the measured
  hyperelliptic Cantor step cost of the same genus in the lab's `genus_ladder`
  (`fffa1b1243`), in FPM:

  | Genus | Step cost | Source |
  |---|---|---|
  | 3 | 290.3 | 4 R runs at p=1031 |
  | 4 | 479.8 | 4 R runs at p=1031 |
  | 6 | 1164.9 | 20 R runs from P1 |

  A non-hyperelliptic Picard-group step costs at least this much, so rho is
  under-priced. That is conservative for index calculus. Plain rho
  sqrt(pi p^g / 2) is printed too, with no free negation credit (the L10 lesson).
- **Slopes:** OLS of the log of the geometric-mean best `total_low` against log p,
  over all primes and over the 4 largest.

## Predictions, stated before the main run

- **P3-a (control).** For `d=4 m=0`, the slope of `total_low` over the 4 largest
  primes is in [0.85, 1.3]. Diem's e=2 gives 1.
- **P3-b.** For `d=5 m=2` and for `d=5 m=0`, the slope of `total_low` over the 4
  largest primes is in [1.15, 1.5]. The prediction for e=3 is 4/3.
- **P3-c.** At p=8209, `d=5 m=2` (genus 4) has `total_low / rho_neg` < 1.
- **P3-d.** At p=8209, `d=5 m=0` (genus 6) has `total_low / rho_neg` < 0.01.

Each prediction is reported as holding, failing or mixed.

## Gates

1. `selftest` passes.
2. G1 + G2 failures in all rows = 0.
3. No row has `useful` above the distinct lines tried (fix (a) holds).
4. The A/A replicate, 2LP cF=4 rep 2 against rep 1, is reported per cell.

## What this cannot show

**This is not a DLP solve.** There is no descent, no Picard-group arithmetic, no
certificate, and linear algebra is a bracket, not executed. It measures the cost of
reaching a relation matrix of full size, which is a lower bound for a solve. It
places no row on any leaderboard.

It does not compare like for like with the hyperelliptic arms of P1, which come
from a different instrument. That comparison is qualitative, by exponent and by
distance to rho.
