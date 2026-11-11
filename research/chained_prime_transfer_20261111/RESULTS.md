# Results: chained prime-field transfer, measured boundary

Run: `chained_transfer_bench run` (bench record
`benchmarks/records/20261111T0540Z-chained-prime-transfer.json`), deterministic.
A second run under the record reproduced every count exactly. Local
development check (exact operation counts, not timings); one seed per cell.
All values are `[pcc:unverified]`.

| p | r | rho on E1 (reference) | rho on E1′ (isogeny, deg) | rho on J(C) after φ* | IC on J(C) | IC trials | IC / rho |
|--:|--:|--:|--:|--:|--:|--:|--:|
| 211 | 109 | 428 | 530 (2) | 396 | 4,184 | 244 | 9.8× |
| 419 | 113 | 357 | 562 (2) | 497 | 9,366 | 532 | 26.2× |
| 821 | 71 | 370 | 274 (3) | 389 | 13,313 | 869 | 36.0× |
| 1619 | 401 | 847 | 649 (2) | 560 | 36,363 | 1,588 | 42.9× |

Units: counted group operations (elliptic additions and doublings for rho on
`E1`/`E1′`; Cantor additions and doublings for `J(C)`). The IC column counts
relation collection only; linear algebra over `Z/r` is extra and is not
included, so the IC figure is a lower bound on its total cost.

![cost vs p](cost-vs-p.svg)

## What the run establishes

1. **The chain is a valid DLP transfer at every size.** Each hop passed the [pcc:unverified]
   in-code admissibility checks (`h(P) ≠ 0`, `ord h(P) = r`,
   `h([x]P) = [x]h(P)` on sampled `x`), the composed chain was verified end to
   end, and the index calculus recovered the known log in every cell.
2. **F1 (rho advantage) — not met.** Rho on the isogenous curve and on the
   order-`r` subgroup of `J(C)` lands within the run-to-run scatter of rho on
   the source (single seed per cell, so no tighter statement is made). This is
   expected: isogenous curves have the same order, and the `J(C)` subgroup is
   still cyclic of order `r`. Per group operation, a Cantor step costs more
   field work than an elliptic addition, so the field-operation comparison only
   favours the source.
3. **F2 (index calculus beats rho) — not met.** Genus-2 IC on `J(C)` costs
   9.8× to 42.9× the matched rho, and the ratio grows with `p`.
4. **F3 (IC exponent).** A least-squares fit of ln(IC group operations)
   against ln p over the four sizes gives slope **1.01** (derived; four points,
   no interval). This is the `Ω(p)` relation count of genus-2 index calculus
   (about `p/2` degree-1 places, smoothness about 1/2, so about `p` trials).
   Rho scales as `√r ≤ √p`. The gap widens as `p` grows: the boundary is
   structural, not a tuning constant.

## What it does not establish

- No timing claim; no isolation tier; no validation-gate admission.
- One bielliptic curve per size, one seed per cell; no spread for the rho
  columns.
- Nothing about other cover families, genera above 2, or cross-characteristic
  maps (none is constructed).
