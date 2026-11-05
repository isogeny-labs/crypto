# Protocol: Semaev S_3 in the T_0-invariant coordinate w = x + 1/x (written before data)

Date 2026-11-02. Branch `leadE-t0-algebraic` (from main 514a86e7c6). Frozen before any
cell below was run. Class: a component measurement of the `pkm-semaev` oracle (F4 over the
bit tower) at m = 2. It makes no ranking claim against the generic baseline or rho and
reports no distance to rho.

## Question

Does writing the summands of the m = 2 algebraic oracle in the T_0-invariant
`w = z + d/z` (Montgomery `w = x_M + 1/x_M`) lower the degree and the F4 cost, against the
plain Weierstrass bits form, at equal base size? STRUCTURE.md section 2(b) states a factor
2^(m-1) in the degree of S_(m+1) after T_0-symmetrisation; this run checks it.

## Construction (what the new form `t0-bits` is)

- Curve `y^2 = x^3 + ax + b` with the rational 2-torsion point T_0 = (s, 0) (here
  `s = A/3`, the Montgomery T_0). With `z = x - s`: `y^2 = z(z^2 + 3s z + d)`,
  `d = 3s^2 + a` (`d = 1` for the Montgomery miniature). Translation by T_0 is `z -> d/z`.
  The invariant is `w = z + d/z`; each `w` covers the points `{+-P, +-(P + T_0)}`.
- The 2-isogeny `phi : E -> E/<T_0>` has x-map `y^2/z^2 = w + 3s`. In short Weierstrass form
  the quotient is `y^2 = x^3 + a2 x + b2` with `x = w + s`, `a2 = -3s^2 - 4d`,
  `b2 = 2s^3 - 8sd`.
- **Symmetrised polynomial.** `N(w1, w2, w3) = S_3^{E/<T_0>}(w1 + s, w2 + s, w3 + s)`,
  the ordinary Semaev polynomial of the quotient. For `d = 1` it equals
  `S_3(z1,z2,z3) S_3(1/z1,z2,z3) / (z2 z3)^2`, the product of S_3 and its T_0-image divided by
  the monomial factor of STRUCTURE.md (unit test
  `t0_symmetrised_polynomial_is_s3_times_its_t0_image`). It vanishes exactly on valid
  w-triples (unit test `..._vanishes_exactly_on_valid_w_triples`, exhaustive over a 229-element
  field: `N = 0` iff there are points over the `w_i` with `P3 = +-P1 +-P2`).
- **Degree, derived before data.** `N` has degree 2 in each summand and total degree 4,
  exactly like S_3 (unit test `t0_symmetrised_degree_equals_plain_degree`). The factor
  `2^(m-1) = 2` of STRUCTURE.md appears only for the non-polynomial presentation
  `S_3 / (z1 z2 z3) = (w1 w2 w3 - D)/2 - 2(w1 + w2 + w3) - 4A` with `D = prod (z_i - 1/z_i)` and
  `D^2 = prod (w_i^2 - 4)` (bilinear in each `(w_i, D_i)`), which needs the extra variable
  `D_i` with `D_i^2 = w_i^2 - 4d`. That is not a bit-tower variable and is **not built here**.
  What is measured is the polynomial-in-w form, in which the per-summand degree is not lower.
- Membership: `w = lo + sum_j 2^j b_j`, `b_j^2 = b_j`, over `[lo, lo + 2^t)` (form name
  `t0-bits`, offset by `t0-bits@LO`). `x_R` is replaced by the target's `w_R`
  (one inversion per target). Hybrid guessing G fixes the top G bits of `w_1`, as in `bits`.
- **Base** (harness side, `t0_factor_base`): every curve point whose `w` is in the interval:
  closed under T_0 and negation, up to four points per `w`. `columns` counts abscissae (two
  per `w`), so it is even.
- **Lift-back, charged.** A solution `(w_1, w_2)` of the grid or of F4's consistent system is
  mapped to the base abscissae over each `w_i` (a map built once in `prepare`, one inversion
  per base abscissa on the set-up native ledger), then two abscissa choices are tried (all first
  roots; last summand on its other root, i.e. one T_0 shift) with signs, by `lift_abscissae`,
  which re-adds the points on the counted group ledger. Counters: `pkm_t0_target_inversions`,
  `pkm_t0_target_fp_mults_uncharged`, `pkm_t0_lift_choices_tried`,
  `pkm_t0_lift_hits_with_t0_shift`, `pkm_t0_lift_unrealised_w`, set-up
  `pkm_t0_base_inversions`, `pkm_t0_base_map_fp_mults_uncharged`. Every returned relation
  is re-summed exactly to the target (`R = +-Q1 +-Q2` over base points, T_0 shifts included
  because the base is closed). The grid recovery keeps its existing counter
  `pkm_grid_fp_mults_uncharged`. F4 multiply-adds are the only solver cost in the primary
  metric, as in leads 4 and 5; the native counters are reported beside it.
- `R = T_0` has no invariant (`w = infinity`); the oracle returns no relation for it. It is not
  in the order-`l` subgroup that targets are drawn from, so it does not occur in the cells.

## Arms and the pairing rule (fixed here)

The previous comparison (lead 5) was confounded because the arms had unequal base sizes at
equal t. **Pairing rule R1 (primary):** the plain arm is `bits` with `lo = 0` and
`t in {4, 5, 6}` as in lead 5. For each t the `t0-bits` arm uses the same t (same number of
bit variables, 2t) and the offset `lo in [0, 400)` that minimises `|columns(t0) - columns(plain)|`
(ties: smallest `lo`), found by `curve25519_family t0size 20 t 0 399`. The base then has the
same number of columns and points, up to the parity effect: the closed base has an even number
of columns and the plain bases have 7, 11 and 25, so a mismatch of exactly one column is
unavoidable at these t. It is reported in the table, and the per-relation cost uses each arm's own
`coverage_exact`, so the base-size difference enters the primary metric. Chosen pairs:

| t | plain `bits` (lo = 0) columns / points | `t0-bits` lo | columns / points |
|--:|:--|--:|:--|
| 4 | 7 / 14 | 0 | 6 / 12 |
| 5 | 11 / 22 | 1 | 12 / 24 |
| 6 | 25 / 50 | 34 | 26 / 52 |

**Pairing rule R2 (secondary, "X" arm):** the w-form with one column per class, i.e. a base
whose number of `w` classes equals the plain arm's number of columns (the accounting of a pair
table that keys each `{P, P + T_0}` once). This needs a window with 2^(t+1) values. Offset: the
`lo in [0, 400)` that minimises `|w_classes - plain columns|` (ties: smallest `lo`). Chosen:

| matched plain t | plain columns | `t0-bits` t | lo | w classes | points of the closed base |
|--:|--:|--:|--:|--:|--:|
| 4 | 7 | 5 | 0 | 7 | 28 |
| 5 | 11 | 6 | 68 | 11 | 44 |
| 6 | 25 | 7 | 68 | 25 | 100 |

The X arm has `2(t+1)` bit variables against `2t`; it asks whether halving columns by T_0
identification pays for the extra bit. Its relations are exact over the closed base, which
implies the relation over class representatives plus a T_0 term (equal after the cofactor
projection `[8]`).

## Cells

k = 20 miniature (`build_miniature(20)`), `m = 2`, mode `full`, G in {0, 3}, 100 random
subgroup targets, seed `2026110200 + t_plain` (the same target stream for every arm of a
pairing: the targets are the same group elements), plus 20 planted targets per cell
(`F4COST_PLANTED=20`, sums of two random base points; every one is decomposable, so a miss is a
completeness failure; their F4 work is reported separately and is not in the per-target means).

| arm | cells (t, G) |
|:--|:--|
| plain `bits` | (4,0) (4,3) (5,0) (5,3) (6,0) (6,3) |
| `t0-bits` R1 | (4,0) (4,3) (5,0) (5,3) (6,0) (6,3) at the lo above |
| `t0-bits` X (R2) | (5,0) (5,3) (6,0) (6,3) (7,3) at the lo above; the (7,0) cell is not run (about 8e10 mul-adds per target in lead 5, over any wall cap here) |

17 cells. Command (frozen binary only, strictly sequential, interleaved arm order per (t, G)):
`F4COST_PLANTED=20 curve25519_family_leadE f4cost 20 <form> <t> <G> 100 <seed> 600 2 full 0 30000000`,
under `ulimit -v 3500000` and `timeout 900`. The host is shared with another lead; `uptime` is
recorded at the start and end of every cell, a cell that reaches the 600 s cap is flagged
truncated, and wall times are not compared across cells or arms.

## Metrics

Per cell: solving degree (max), F4 matrix size (`max_cols_max`, `max_rows_max`, `max_nnz_max`
over all calls), polynomial terms per specialisation, F4 multiply-adds per target,
hits and hits verified on the 100 random targets, planted targets found/exact,
`coverage_exact` and per-relation cost `mul-adds per target / coverage_exact`
(`muladds_per_relation_at_exact_coverage`), columns, points, undecided, native counters.
Ratios are `t0-bits / bits` per (t, G).

## Frozen predictions and falsifiers

The expectation stated in the brief (E) and the one derived above (D) are both frozen.

- **P1 (E, lead's expectation):** the solving degree is strictly lower in the w-form at
  G = 0 for every t in {4, 5, 6}. **Falsified** if at any t the w-form's degree is `>=` the
  plain degree.
- **P2 (D):** the solving degree is equal in all six primary pairs (t, G). **Falsified** by any
  difference.
- **P3 (D):** polynomial terms per specialisation are equal in each primary pair (the specialised
  system is dense in the same monomials). **Falsified** by any difference.
- **P4 (D):** F4 mul-adds per target, `t0-bits / bits`, lie in [0.5, 2.0] in all six primary
  pairs (no change beyond a small constant). **Falsified** by any ratio outside.
- **P5 (D):** `coverage_exact` ratio `t0-bits / bits` in [0.3, 0.75] in all three primary base
  pairs: a base closed under T_0 has about half the distinct sums of an unclosed base of the same
  size, because `(Q1 + T_0) + (Q2 + T_0) = Q1 + Q2`. **Falsified** by any ratio outside.
- **P6 (D):** per-relation cost ratio `t0-bits / bits` is above 1 in all six primary pairs (the
  w-form does not lower the per-relation cost at equal base size), and in [1.2, 4.0] at t = 5, 6.
  **Falsified** by any ratio `<= 1`, or (second part) outside the band.
- **P7 (D, X arm):** the per-relation ratio X / plain-with-equal-columns exceeds 3 in all five X
  cells (the extra bit costs more F4 work than the doubled coverage returns). **Falsified** by
  any X cell with ratio `< 1`; cells between 1 and 3 count as "P7 not held, no gain".
- **P8 (completeness and exactness):** in every cell all processed planted targets are found and
  every returned relation (random and planted) re-sums exactly to its target:
  `planted_hits = planted_done`, `planted_exact = planted_hits`, `hits_verified = hits`.
  **Falsified** by any miss or inexact relation.

Unit-test claims (not predictions, stated for the record): the symmetrised polynomial vanishes
exactly on valid w-triples; F4 verdicts equal the grid and the exact decomposable set on
random and planted targets, with and without guessing, for three windows; `m = 3` planted sums
decompose under chain, full and grid.

## Scope of conclusions

Only k = 20, one curve (no spread across curves), `m = 2`, mode full, t in {4,5,6} (X arm to
7), G in {0, 3}, the offsets above, one seed per t. Nothing about `m = 3`, other windows, other
k or curves, the 255-bit setting, or a complete DLP; no comparison with rho. The `(w, D)`
degree-1 presentation is not tested. With `coverage_exact` near 1e-3 the 100 random targets
give about 0.1 hits per cell, so random hits do not discriminate; the planted targets carry the
exactness check. Per-relation figures price only F4 multiply-adds, not the native counters,
which are reported beside them (AGENTS.md section 5a pricing of native work is not applied to a
cost this far from a complete solve).

## Frozen binary

`/home/user/frozen-bin/curve25519_family_leadE`, sha256
`fa565fd4acf1da0695bdb271f12bbe88cd0064c4bb7be9b10898450a2f098846`
(`analysis/binary.sha256`). Verified before data by a 2-target smoke of `t0-bits@1` (a form only
this branch accepts); the smoke is not part of the data.
