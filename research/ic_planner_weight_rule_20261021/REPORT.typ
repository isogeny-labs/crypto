#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set par(justify: true)
#show table: set text(size: 8.5pt)

= A weight-aware factor-base size for the `ic` planner (`ic-planner/2`)

2026-10-21 · study `research/ic_planner_weight_rule_20261021` · protocol frozen in `7cf06ed6e`, amendment 1
in `66f1642d1` (after Phase A, before Phase B) · seven ecbench sessions, 1,152 records, all verified, audits OK
(12 exact replays each) · reference `rho.negation`.

== Question

The planner's prime factor base, `N₀ = round(#E^(1/3)/2)` abscissae, is the cheapest size in counted `S`,
but an earlier study found a doubled base 2–9% cheaper once the unpriced native work (lookups, guard probes,
Legendre symbols, square roots, row operations) is charged at 1 GAE per operation (AGENTS.md §5a). Can the
planner take a declared weight `w` and size the base for it, on curves the rule was not fitted on?

== Phase A: derivation

Base multipliers m ∈ {0.5, 0.71, 1, 1.41, 2, 2.83, 4} on the registered curves `icv1-fp24-tm5099-a6f137c6`,
`icv1-fp26-tm1775-7e8fb6df` and `icv1-fp28-t28201-65099091`. `G(m, w)` is the geometric mean over the three
curves of `S_w(m·N₀)/S_w(N₀)`, from `ecbench band`; the frozen rule takes `m*(w) = argmin G`.

#table(columns: 9, align: (left, right, right, right, right, right, right, right, right),
  [*w*], [*0.5*], [*0.71*], [*1*], [*1.41*], [*2*], [*2.83*], [*4*], [*m\**],
  [0], [1.513], [1.126], [1], [1.119], [1.601], [2.760], [5.208], [*1*],
  [0.03], [1.536], [1.138], [1], [1.103], [1.558], [2.668], [5.019], [*1*],
  [0.1], [1.584], [1.161], [1], [1.072], [1.471], [2.478], [4.629], [*1*],
  [0.3], [1.686], [1.210], [1], [1.005], [1.287], [2.076], [3.801], [*1*],
  [1], [1.858], [1.295], [1], [*0.890*], [0.972], [1.392], [2.394], [*1.41*],
)

#figure(image("figures/derivation_grid.svg", width: 80%), caption: [Phase A: G(m, w). The minimum moves off m = 1 only at w = 1.])

== Phase B: held out

Four registered prime-order curves in none of the planner's evidence; `v1` = `N₀` against `v2-m141` =
`round(1.41·N₀)`. Paired `S(v2-m141)/S(v1)`, ecbench `compare --weight`, cluster 95% intervals, 24 pairs each:

#table(columns: 5, align: (left, right, left, left, left),
  [*curve*], [*log#sub[2] r*], [*N₀ → v2*], [*w = 0*], [*w = 1*],
  [`icv1-fp21-t377-d7bba3fb`], [20.59], [58 → 82], [1.040 [0.911, 1.176]], [0.850 [0.713, 1.002]],
  [`icv1-fp22-tm2947-e15fed69`], [21.19], [67 → 94], [1.058 [0.956, 1.167]], [0.855 [0.743, 0.972]],
  [`icv1-fp24-t2705-27cbb01b`], [23.38], [111 → 157], [0.996 [0.884, 1.113]], [0.759 [0.642, 0.883]],
  [`icv1-fp29-t47-14392892`], [28.25], [342 → 482], [1.062 [0.993, 1.144]], [0.819 [0.749, 0.900]],
)

#figure(image("figures/holdout_ratio.svg", width: 80%), caption: [Held out: the w = 1 base against N₀ at w = 0 and w = 1.])

*Predictions.* P1 (m\* non-decreasing in w) held. P2 (m\*(0) = 1, m\*(1) ∈ {1.41, 2, 2.83}) held. P3 (held out,
w = 1, interval below 1 on at least 3 of 4) held, 3 of 4; h21's reaches 1.002. P4 (held out, w = 0, v2 ≥ v1) not
strictly held: h24 reads 0.996 and every w = 0 interval contains 1. The larger base's counted price is within
noise there, and 8.5–14.4% (every interval above 1) on the derivation curves.

== Distance remaining to rho

`S(IC)/S(rho.negation)`, paired; rho pays its native work at w = 1 too. Rho is cheaper everywhere. On the held-out
curves the plan sits 9.0–18.3× rho at w = 0. At w = 1 the `N₀` base reads 10.6–21.9× and the weighted base
9.1–17.9×: the weighted base closes part of the gap that pricing the native work opens. These are distances
remaining, not a pass/fail gate.

#figure(image("figures/rho_gap_w1.svg", width: 72%), caption: [S_1(IC)/S_1(rho.negation) for both base sizes.])

== Decision and class

Adopted as `ic-planner/2` under the decision rule (P3): `ic plan --native-weight w` and
`ic bench --auto --native-weight w` use `round(m*(w)·N₀)`, `m*` read at the largest measured weight not above
w. The default (w = 0) is unchanged, as are every other stage and every non-prime class. Engineering
(AGENTS.md §3): the weight is a declared pricing assumption, and a w = 1 plan is cheaper only at w = 1, never a
ranking across the §5a band. No counted S improves on a measured baseline, so no row of tables A–C moves. The
validation gate needs an independent replay certificate and validator report, so no leaderboard or
`benchmarks/README.md` row is added here.

Deviations: cards 2 and 3 stopped before writing any session file (ecbench refused a busy host, PSI 6.02 and
6.37 > 5); card 4 waits for 30 s of quiet; specs, seeds and binary unchanged. `1.41 × 128 = 180.48` was run as 180.
