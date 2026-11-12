# Cover-image factor bases, round 3: the m = 3 lead (2026-11-12)

Frozen before any measured cell of this round. Round 2
([`../cover_image_fb_r2_20261112/`](../cover_image_fb_r2_20261112/README.md)) found every cover
map at m = 2 indistinguishable from its random-coefficient null and wider than PKM's tower. One
ungated m = 3 cell went the other way: at t = 3 (|F| = 8) the `cubic` and `quart` systems had
W = 1,866 and 1,876 against PKM's 3,285. PKM's system climbed from input degree 4 to D = 6
there; the cover systems stayed at their input degree. This round tests whether that lead holds
at larger t, and whether it comes from the cover's structure or from the system's shape.

Why it matters either way: at equal `|F|` the decomposition probability per target is the same,
so a cheaper solve at equal `|F|` is an improvement of the decomposition oracle whatever its
cause. The null decides only the explanation.

## Arms

`examples/pkm_tower_pilot.rs --engine f4 --kinds kummer --m 3 --controls tower,null --null-chain`,
`p = 2013265921`, pilot default seed, 3 planted + 3 random targets per cell, `t = 2…5`, budget
1,800 s per system. Maps: (none) = PKM tower, `quad`, `cubic`, `quart`, `generic3`.
`--null-chain` gives every `S₃` link of the m = 3 chain random coefficients on its own support.

## Measures

- **Primary: W** = widest F4 matrix (`max_cols`), max over the cell's targets.
- Secondary: F4 wall time per system (median over the cell's targets; shared 4-core container,
  descriptive only), `max_rows`, and E = D − input degree.
- A cell with any timed-out system is censored and enters no gate.

## Gates and hypotheses (per arm X ∈ {quad, cubic, quart, generic3})

- **R0 (replication).** The `tower` W at t = 2, 3 equals round 2's `cells.tsv` for none, cubic,
  quart, generic3. Failure stops the reading.
- **H1 (the cover makes the m = 3 solve cheaper).** `W_X / W_none ≤ 0.8` at both t = 3 and t = 4.
- **H0.** `W_X / W_none ≥ 1` at t = 4.
- Otherwise inconclusive. t = 5 is reported if uncensored and does not enter the gate.
- **S (structure), read only for an arm with H1.** `W_X / W_null,X ≤ 0.8` at t = 4 means the
  cover's structure matters; otherwise the gain is the system's shape.
- Prior (before data): H1 for at least one arm, 0.4; S given H1, 0.15. The likeliest reading is
  that the degree-12/16 cover equations already contain what PKM's degree-4 system must climb to
  D = 6 to build, and that a null on the same support does the same.

## Not claimed

No `S`, no relation-phase cost, no rho comparison, no leaderboard or `benchmarks/` row. A stage
diagnostic of the decomposition solve. Matrix width is a proxy for F4 cost, not the cost.
