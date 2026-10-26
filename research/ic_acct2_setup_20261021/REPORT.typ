#set page(paper: "a4", margin: 2cm, numbering: "1")
#set text(size: 10pt)
#set par(justify: true)
#show heading.where(level: 1): set text(size: 12pt)

#align(center)[
  #text(size: 15pt, weight: "bold")[rho-neg's sub-½ exponent on 16–26 bits:\ set-up accounting, not the walk]
  #v(2pt)
  `research/ic_acct2_setup_20261021` · protocol frozen 2026-10-21 (2c50f0614) · analysis only
]

= Question

On the bitop ladder's hold-out panel, `rho-neg` fitted in `bitop/v1` over
16–26 bits has exponent α = 0.413 [0.388, 0.447] in additions: the positive
control "a generic walk scales as r^½" tripped. Is that an artifact of
counting set-up, verification and native work in the total (a larger share at
small r), or of mixing two jump counts (8 jumps to 22 bits, 16 from 23)?

This branch also makes `ecbench`'s reusable/per-target split target-aware
(a walk's `setup` uses Q, so it is not reusable) and adds
`bound fit --cost total|per-target|search`.

= Verdict

*Yes, an accounting artifact* (PROTOCOL.md rule, applied mechanically).
Positive control: 6/6 search-basis intervals on the fixed-jump ranges
contain ½. Over 16–26, rho-neg's total interval excludes ½ and its search
interval contains it.

#table(columns: 3, align: (left, center, center),
  [*rho-neg, 16–26*], [*α (additions)*], [*95% CI*],
  [total], [0.413], [[0.388, 0.447]],
  [per-target (= total by construction)], [0.413], [[0.388, 0.447]],
  [search], [0.477], [[0.455, 0.506]],
)

#figure(image("figures/rho_neg_cost_by_basis.png", width: 100%),
  caption: [Non-search work is 37% of rho-neg's total at 16 bits and 9% at 22; it
  steps back up to 14% at 23 bits where the jump table doubles. That shrinking
  constant flattens the total's slope.])

#figure(image("figures/alpha_forest.png", width: 85%),
  caption: [α in additions with 95% CI, total vs search basis, for the three
  generic arms on each range. Dashed: ½.])

= Amortization under the corrected split

Under the stored split rho-neg was credited 2.6M–42.0M reusable gates per
curve; target-aware, 0. IC's reusable work is unchanged. The cold ratio
(IC total over rho total) is unchanged, 6.2 at 16 bits to 78.3 at 40. The
per-target ratio falls (6.36 → 4.41 at 16 bits; 58.94 → 58.92 at 40).
No curve has a break-even target window under either split.

#figure(image("figures/ic_rho_ratios.png", width: 80%),
  caption: [ic-pair-rule over rho-neg in gates, per curve.])

= Limits

Analysis only: no new runs. The search basis drops verification and native
work along with set-up; it does not separate them. The per-target and search
bases are partial costs, named in the unit (`bitop/v1+per_target`,
`bitop/v1+search`), and never compare against a `bitop/v1` total. This is an
accounting correction, not an index-calculus improvement; the gap to matched
rho in total cold cost is unchanged.
