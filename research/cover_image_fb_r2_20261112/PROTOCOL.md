# Cover-image factor bases, round 2: matrix width, cover degrees 3 and 4 (2026-11-12)

Frozen before any measured cell of this round. Follows
[`../cover_image_fb_20261112/`](../cover_image_fb_20261112/README.md) (round 1, PR #3985),
whose preregistered metric (F4 `solving_degree_max`, D) was floored by the input degree of
the substituted summation polynomial, so its verdict was not read. Round 1's post-hoc
measure, the widest F4 matrix, is here the **preregistered primary measure**, and the cover
maps go up to degree 4.

Instrument: `examples/pkm_tower_pilot.rs --engine f4` (`f4_fp`), `--cover-map` extended
with `fourth`, `cubic`, `quart`, `generic3`. The round-1 kinds draw from the same RNG stream
as before (only the coefficient arrays are zero-padded), so round-1 cells replay.

## Arms

| `--cover-map` | `x∘φ(u)` | deg | genus of `C` (`deg h`) | role |
|---|---|---|---|---|
| (none) | `u` | 1 | – | PKM tower, reference |
| `square` | `u²` | 2 | 2 (6) | positive control: = depth `t−1` tower on Kummer |
| `fourth` | `u⁴` | 4 | 5 (12) | positive control: = depth `t−2` tower on Kummer |
| `quad` | `λ(u−α)²+β` | 2 | 2 (6) | replication of round 1 |
| `cubic` | random monic cubic | 3 | 4 (9) | main |
| `quart` | random monic quartic | 4 | 5 (12) | main |
| `generic3` | random coprime cubics `N/D` | 3 | 5 (12) | main |

Kummer tower only (round 1 gave identical cells for Dickson at every `t`). Controls `tower`
and `null` (the cover system's own monomial support, random coefficients, planted zero).
`randbase` is dropped: round 1 showed it is the univariate route and not comparable.
`p = 2013265921`, pilot default seed, 3 planted + 3 random targets per cell, budget 600 s per
system. m = 2: `t = 3…7`. m = 3 (secondary, `tower` only): `t = 2…3` for none, cubic, quart,
generic3.

## Measures

- **Primary: W** = `max_cols`, the widest F4 matrix, max over the cell's 6 targets.
- Secondary: **E** = `solving_degree_max − max(input_degrees)`, the degree F4 climbs above
  its input (reported, not gated).
- A cell with any timed-out system is censored and enters no gate.

## Gates and hypotheses (per main arm X ∈ {cubic, quart, generic3}, m = 2)

- **R0 (replication).** `quad` reproduces round 1's `cells.tsv` W exactly at every `t` both
  measured, in both controls. Failure stops the reading.
- **P0 (positive controls).** `W_square(t) = W_none(t−1)` and `W_fourth(t) = W_none(t−2)`
  exactly, `tower` control, at every `t` where both cells are uncensored. Failure stops the
  reading.
- **H1 (the cover helps).** `W_X / W_null,X ≤ 0.8` at ≥ 2 of `t ∈ {5, 6, 7}`, **or**
  `W_X / W_none < 1` at the two largest uncensored `t`.
- **H0 (falsifier).** At every uncensored `t ≥ 5`: `0.8 ≤ W_X / W_null,X ≤ 1.25` **and**
  `W_X / W_none > 1`.
- Otherwise inconclusive for that arm.
- Prior (stated before data): H0 for each main arm, about 0.85. Expected, not gated:
  `W_X / W_none` grows with the map's degree.

## Not claimed

No `S`, no relation-phase cost, no rho comparison, no leaderboard or `benchmarks/` row:
a stage diagnostic of the solver axis, counts only, local 4-core container, small prime,
`t ≤ 7`. Covers here are fibre products with a degree-d map `P¹ → P¹` on `x`; other covers
(non-hyperelliptic `C`, maps not factoring through `x`) are out of scope.
