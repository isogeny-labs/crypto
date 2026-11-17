#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set par(justify: true)
#show table: set text(size: 8pt)

= Does the weight-aware base size carry to binary and Koblitz curves? (`ic-planner/3`)

2026-10-26 · `research/ic_planner_weight_rule_binkob_20261026` · protocol `1fffc5c45`, amendment 1 `2f87b17ad` · nine
ecbench sessions, 1,568 records, all verified, audits OK.

On prime curves the cheapest base grows 1.41× at w = 1 GAE per unpriced native operation. Here: three binary and three
Koblitz curves in Phase A, base multipliers ¼–4, with `G(m, w)` the geometric mean of `S_w(m)/S_w(plan)`. Phase B tests the
Koblitz rule on three held-out curves.

*Binary: no weight rule.* `m* = 1` at every weight; at w = 1 the nearest alternative is `d₀ + 1` with G = 1.061.

*Koblitz: the base shrinks.* `m* = 0.71` at w ≥ 0.3 (G = 0.928 at w = 0.3, 0.847 at w = 1).

#figure(image("figures/derivation_grid.svg", width: 98%), caption: [Phase A: G(m, w) per family.])

#table(columns: 5, align: (left, left, left, left, left),
  [*held-out curve*], [*plan → alt*], [*w = 0*], [*w = 0.3*], [*w = 1*],
  [`icv1-f2m19-tm797-9c54981b`], [2 → 1], [1.587 [1.298, 1.939]], [0.923 [0.790, 1.077]], [0.802 [0.700, 0.919]],
  [`icv1-f2m23-tm5197-1f85e9e1`], [4 → 3], [1.121 [1.022, 1.230]], [0.928 [0.876, 0.983]], [0.880 [0.840, 0.923]],
  [`icv1-f2m39-t1481485-829ef1d1`], [6 → 4], [1.166 [1.043, 1.303]], [0.895 [0.841, 0.952]], [0.838 [0.797, 0.880]],
)

#figure(image("figures/koblitz_holdout.svg", width: 72%), caption: [Koblitz held out, paired.])

P3 held, with 2 of 3 intervals below 1 at w = 0.3 and 3 of 3 at w = 1. `ic-planner/3` gives `max(1, round(0.71·c₀))` Koblitz columns for
w ≥ 0.3, and keeps the binary dimension with this study cited. The default (w = 0) plan is unchanged. P1 (non-decreasing `m*`)
failed for Koblitz: a Koblitz trial's native work grows with the base, while a prime trial's does not.

The plan sits 2.6–4.8× rho in counted S on the held-out curves. Engineering class; no leaderboard row moves. Deviations: no resume
checkpoint (no token; the owner said proceed), and the binary was built at `b561ebca8`, after the freeze (that change does not touch these arms' counts).
