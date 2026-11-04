# Round 7 (RR): Riemann–Roch norm-divisibility system under F4

## Prediction

Written 2026-11-04T05:55:06Z (`date -u`), before any RR system was generated
or solved. [pcc:unverified]

Predicted F4 solving degree D_RR (msolve 0.6.5, grevlex, 1 thread), against
the cherry tree T on the same planted instances:

| cell | D_T (reference) | predicted D_RR |
|---|---|---|
| m3 t4 | 6 | 6 |
| m3 t5 | 6 | 6–7 |
| m3 t6 | ≥ 7 | 7 |
| m5 t4 | ≥ 9 (cut off) | 8–9 (likely cut off) |
| m5 t5 | — | ≥ 9 (cut off) |

Reasoning: the inputs are quadratic, but N ≈ (2m−1)t + 2m is 2–3× the T
variable count, and the tower levels couple through U at every level, so I
expect no D gain at m = 3 and at most one degree at m = 5, with wider
matrices than T. (m3 t3 has no planted instance in `../instances.txt`; it is
added only if a planted t = 3 instance can be built from the same data.)

Kill criterion (from `../round7_beyond/DIRECTIONS.md`, rank 1):

- **killed** if D_RR ≥ D_T in every comparable cell, **or** if D_RR's m-slope
  (m3 → m5 at t = 4) is ≥ 1.5 per summand;
- **trade, not improvement** if D_RR < D_T somewhere but the widest matrix is
  larger in every such cell;
- **improvement** only if D_RR and the widest matrix are both smaller than T's
  in at least one m ≥ 5 cell.
