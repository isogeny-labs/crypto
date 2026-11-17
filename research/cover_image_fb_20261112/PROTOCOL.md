# Cover-image factor bases on E(F_p): preregistration (2026-11-12)

Frozen before any measured cell. Instrument: `examples/pkm_tower_pilot.rs`,
`--engine f4` (`f4_fp`, the engine of every committed pilot and matched-prime
row), with the new opt-in flag `--cover-map` and control `randbase`. Without
the flag every committed command line builds and measures what it did before
(the cover map is drawn after the tower, and `target_curve`, `plant`,
`summation` take their old path when `tower.cover` is `None`).

## Question

Idea 2 of the 2026-11-12 discussion: use a cover `φ: C → E` only to choose
the factor base. Take `V` a structured set of `u`-coordinates on `C` (here a
PKM tower set), and use `F = x∘φ(V) ⊂ x(E(F_p))`. Relations are still found
and solved on `E`. Does the summation system restricted to `φ(V)` have a
lower F4 solving degree `D` than (a) its own random-coefficient null,
(b) PKM's tower base of the same size, (c) a random base of the same size?

## Construction

A degree-2 map `X = N(u)/D(u)` pulls `E: Y² = f(X)` back to the double cover
`C: v² = h(u)`, `h = D⁴·f(N/D)`, `φ(u, v) = (N/D, v/D²)`. The genus is set by
the map's two critical values: both on branch points of `E` → genus 1
(a 2-isogeny, i.e. a PKM Vélu step); one at ∞ → genus 2; neither → genus 3.
Prime-order curves have no rational root of `f`, so genus 2 is the smallest
non-isogeny case of this kind. Arms, `--cover-map`:

| map | `x∘φ(u)` | genus of `C` | role |
|---|---|---|---|
| `square` | `u²` | 2 | positive control: on a Kummer tower this is the tower's own first step, so `φ(V)` is the depth-`t−1` Kummer tower (`|F| = 2^{t−1}`) |
| `quad` | `λ(u−α)²+β`, random | 2 | main arm |
| `generic` | `N/D`, random coprime quadratics | 3 | main arm |

Every row records the map, `genus` (from a squarefree check of `h`),
`deg h`, `|F|` and how many base abscissae carry points.

## Systems (m = 2; m = 3 as a secondary cell)

- `tower` (+ cover): `S₃(x∘φ(g·y_{1,0}), x∘φ(g·y_{2,0}), x_R)` cleared of
  denominators (`s3_dbl_args`), plus the tower equations on the `u`-blocks.
  `N = m·t` tower unknowns, `|F| ≈ 2^t`.
- `null` (+ cover): same monomial support, random coefficients, planted zero.
- `randbase`: unknowns `x_1, x_2`, `S₃(x_1, x_2, x_R)` and
  `∏_{f∈F}(x_i − f)` with `F` random on-curve abscissae, `|F|` equal to the
  cell's base.
- Reference: the same cells without `--cover-map` (PKM tower, its null,
  randbase at `|F| = 2^t`).

Planted targets: 3 per cell (each system is checked to vanish at the planted
solution before F4 runs, and `planted_ok` checks the basis). Random targets:
3 per cell. Prime `p = 2013265921` (the committed pilot prime; `2^27 | p−1`),
tower kinds `kummer` and `dickson`, `t = 2…8` at m = 2 (stop at the first
`t` where every run times out, 600 s budget), `t = 2…4` at m = 3. Seed: the
pilot default.

## Metric and predictions

Primary: `solving_degree_max` (D) per cell, max over its targets, planted
and random reported separately. Counts only; wall time is logged, not
compared (local 4-core container, not an isolated host).

- **P0 (positive control).** `square` on Kummer: `D_square(t) ≤ D_tower(t)`
  at every `t`, and `D_square(t) = D_tower(t−1)` at ≥ 4 of the measured
  `t`. If P0 fails the instrument does not see the collapse and the run is
  not read.
- **H1 (idea survives).** For `quad` or `generic`: over the measured `t`,
  `D_cover(t) < D_null,cover(t)` at ≥ 2 values of `t`, **and** the
  least-squares slope of `D_cover` against `t` is ≤ that of the PKM tower.
- **H0 (falsifier).** `D_cover(t) ≥ D_null,cover(t)` at every `t` and
  `D_cover(t) ≥ D_tower(t)` at every `t`: the cover image is no better than
  an unstructured system of its shape, as the PKM tower was
  (`research/pkm_matched_prime_20261106/`).
- Prior expectation (stated before data): H0, probability about 0.8. The
  substitution `x = φ(u)` doubles the degree of `S₃` in each tower variable,
  and nothing in the image of a fibre gives F4 a lower-degree syzygy.

Anything between H1 and H0 is reported as inconclusive, cell by cell.

## Not claimed

No relation-phase cost, no `S`, no comparison against rho, no leaderboard or
`benchmarks/` row. A stage diagnostic of the solver axis. Small prime, small
`t`; the PKM deciding cell (m ≥ 4, N ≈ 20) is out of scope.
