# Tower up: one elliptic discrete log solved at genus 1, 2 and 3

**Status:** construction + measurement, same characteristic. A boundary, not
an improvement. Every number is an unverified assertion `[pcc:unverified]`
(no independent validator has replayed it). Follow-up to PR #3898
(`../ATLAS.md`, `../RESULTS.md`).

## Question

The transfer atlas showed that a degree-2 cover never moves the log to its
complementary factor. The other direction remains: go **up** the tower. Carry
the log into higher-genus Jacobians and ask whether the destination is easier.

## Construction (`src/cryptanalysis/chained_transfer/tower.rs`)

For a cubic model `E : Y² = c(s)` and `a ∈ F_p^*`:

```
F(X) = c3·Y_ + c2·Y_² + c1·Y_³ + c0·Y_⁴,   Y_ = X − a²,    C : y² = F(x²)
φ : C → E,   (x, y) ↦ (s, Y) = (1/(x² − a²), y·s²)       (F(x²) = c(s)/s⁴)
```

- `C` has genus 3 when `c0 ≠ 0`, and genus 2 when `c0 = 0 ≠ c1`, that is, when
  E has a 2-torsion point at `s = 0`. So the **same** curve E covers both a
  genus-2 and a genus-3 curve: the genus-2 cover uses the model translated
  to a rational 2-torsion point, the genus-3 cover a model with `c(0) ≠ 0`.
- The Jacobian is the repository's word-size Cantor group `HCurve`, on the odd
  model with `(a, 0)` sent to infinity.
- The pull-back is explicit: `φ^*([Q]−[O]) = [P₊−∞] + [P₋−∞] + [W−∞]`, with
  `W = (−a, 0)`. This holds because `φ^*(O) = ∞ + W` and `[W − ∞]` is
  2-torsion.
- The one carried log is checked as follows. `ord φ^*(P) = r` is checked.
  `φ^*([k]P) = [k]φ^*(P)` is checked on every multiple `k < 80` whose
  preimages are rational: 3–35 checks per rung. Every solve recovers the
  planted log.
- Unit test: `cargo test --release --lib chained_transfer::tower` passes.

## Measurement (`tower_bench`)

Eight primes `p ∈ {53 … 251}` (the `HCurve` range). One instance per prime, [pcc:unverified]
8 seeds per cell, counted group operations, all solves verified.

| p | r | rho on E | rho on J(C₂) | rho on J(C₃) | IC on J(C₂) | IC on J(C₃) | IC₃ / IC₂ |
|--:|--:|--:|--:|--:|--:|--:|--:|
| 53 | 11 | 131 | 132 | 134 | 70 | 170 | 2.42 |
| 79 | 11 | 137 | 132 | 129 | 106 | 298 | 2.82 |
| 101 | 19 | 185 | 177 | 180 | 126 | 293 | 2.33 |
| 131 | 67 | 337 | 314 | 305 | 148 | 503 | 3.40 |
| 151 | 19 | 181 | 181 | 168 | 181 | 580 | 3.20 |
| 181 | 41 | 249 | 276 | 255 | 210 | 684 | 3.25 |
| 211 | 31 | 224 | 220 | 229 | 234 | 789 | 3.37 |
| 251 | 37 | 248 | 270 | 251 | 267 | 829 | 3.10 |

The IC columns count Jacobian group operations only. `hyper_ic` leaves its
sparse linear algebra over `Z/r` and its smoothness oracle unpriced. Priced
per AGENTS.md §5a, at 0, 0.1 and 1 GAE per linear-algebra multiplication, IC
divided by rho on E is:

| p | IC₂ @0 | @0.1 | @1 | IC₃ @0 | @0.1 | @1 |
|--:|--:|--:|--:|--:|--:|--:|
| 53 | 0.53 | 2.09 | 16.1 | 1.29 | 4.73 | 35.7 |
| 101 | 0.68 | 4.25 | 36.4 | 1.58 | 12.35 | 109.3 |
| 151 | 1.00 | 7.19 | 62.9 | 3.20 | 24.21 | 213.3 |
| 251 | 1.08 | 10.97 | 100.0 | 3.34 | 35.82 | 328.1 |

The smoothness-oracle field multiplications are left out of this table too.
Including them could only raise the IC figures.

The figure `tower.svg` plots the group-operation columns.

## What the run shows

1. **Going up the tower makes the solve harder.** IC on genus 3 costs
   **2.3–3.4×** IC on genus 2 at every p. This is about the 3× that the
   smoothness probability `1/g!` predicts (`3!/2! = 3`).
2. **Genus 3 is above rho at every price.** It costs at least 1.29× rho on E
   even when the linear algebra is free.
3. **Genus 2 cannot be ranked against rho at this size.** At 0 GAE its
   group operations alone are 0.44–1.08× rho, so the order does not hold
   across the §5a band. At 0.1 and 1 GAE it is above rho (2.1–100×). Two
   things depress the 0-GAE figure: the linear algebra is unpriced, and the
   rho reference is dominated by fixed set-up at these tiny `r` (11–67).
4. **The gap widens with p.** The least-squares log-log slope in p is 0.85
   for IC on genus 2 and 1.06 for IC on genus 3, against 0.44 for rho on E
   (derived; 8 points, no interval). This is the `Ω(p)` relation count
   against rho's `√r`.

*Band caveat (replay F12, 2026-11-12):* the §5a band here is one-sided. The
rho reference has no unpriced counters, so only the IC arm moves across 0,
0.1 and 1 GAE. Rankings that hold across the band remain valid. Where the
order flips, the flip comes from the price of the IC linear algebra, not from
any uncertainty on the rho side.
5. **Rho is unchanged by the transfer.** On the order-`r` subgroup of either
   Jacobian, rho matches rho on E: the subgroup is the same size.

## Limits

- `p < 256` (`HCurve`) and `r ≤ 67`. These are toy sizes: rho is dominated
  by set-up and the exponents are fitted over a short range. Genus-2 IC at
  larger p is in `../RESULTS.md` (p up to 1619, 9.8–42.9× rho).
- One instance per prime. Spread is over seeds only.
- The cross map to the genus-3 curve's complementary factor (a genus-2
  Jacobian, `w² = u·F(u)`) is zero by the same deck-involution argument as in
  `../ATLAS.md`. That argument is not measured here.
- No independent validation. The result is not admitted to the benchmarks
  table (validation gate).

## Reproduce

```
cargo test  --release --lib chained_transfer::tower
cargo run   --release --bin tower_bench -- run --json tower.json --svg tower.svg
```
