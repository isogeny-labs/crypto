# Tower ladder: one elliptic log solved at genus 1, 2, 3 and 4

**Status:** construction + measurement, one characteristic, one instrument. A
boundary result, not an improvement. Every measured sentence below carries
`[pcc:unverified]`: no independent validator has replayed it. Extends
`../tower/TOWER.md` by a fourth rung and adds the genus-1 rung measured with
the same instrument.

## Construction (`src/bin/tower_ladder.rs`)

One instance per prime `p` (`tower::build(p, 11 + i)`): an elliptic curve
`E : y² = x³ + a2 x² + a4 x + a6` over `F_p`, a prime `r || #E`, a generator
`G` of order `r`, a planted log `x`, and `T = [x]G`.

- **Genus 1.** `E` itself as `HCurve::new(p, 1, [a6, a4, a2, 1])`,
  `D = point(G)`, `T = point(target)`, `T == [x]D` checked, `#J = #E`.
- **Genus 2 and 3.** The two degree-2 covers of `tower::build`
  (`y² = F(x²)`, see `../tower/TOWER.md`), reused unchanged.
- **Genus 4 (new, degree 3).** Pick `h(x) = x³ + h1 x + h0` and put
  `f(x) = c(h(x))` with `c(s) = s³ + a2 s² + a4 s + a6`. Then `deg f = 9 = 2·4+1`
  and `C₄ : y² = f(x)` has genus 4 (`HCurve::new` checks squarefree). The map
  `(x, y) ↦ (h(x), y)` sends `C₄ → E`. The pull-back of an affine
  `Q = (s, Y)` is the Mumford pair `u = h(x) − s`, `v = Y` (`deg 3`); the
  pull-back of `O` is the identity (`3∞`). `h` is searched (SplitMix64 stream)
  until `jacobian_order(r)` succeeds with `r | #J4` and `r² ∤ #J4`.
  Checks, per prime: every pull-back is a valid reduced divisor;
  `ord φ*(G) = r`; **`φ*([k]G) = [k]φ*(G)` for every `k` in `1..r`**; and
  `φ*(T) = [x]φ*(G)`. All passed (last table: `hom checks`; the genus-1 and
  genus-4 columns equal `r−1`).
- **Instrument.** `ecbench::hyper_ic::solve` with the `tower_bench` parameters
  (`fb_size = usize::MAX`, no large primes, 8 extra relations, 64 branches,
  `d_step_in = 4`, `max_trials = 5_000_000`) at every rung. "IC" below is
  `adds + doubles` summed over `outcome.phases`; "LA" is the sum of the counters
  whose key starts with `linalg_mulmods` (unpriced linear algebra). Rho is
  `ic_boundary::rho_reference` on `E` (`EcGroup`). 8 algorithm seeds per cell,
  means reported; the instance is fixed per `p`. Every one of the
  8 primes × 8 seeds × 5 solves recovered the planted `x` [pcc:unverified].

Primes: all of 53, 79, 101, 131, 151, 181, 211, 251 built at genus 4; none
dropped [pcc:unverified]. `jacobian_order` at genus 4 (point counts over
`F_p`, `F_{p²}`, `F_{p³}`) was fast enough at `p = 251`; the whole run takes
about 70 s on 4 cores.

## Results

Counted group operations (mean of 8 seeds; IC excludes linear algebra):

| p | r | rho_E | IC1 | IC2 | IC3 | IC4 | ok |
|--:|--:|--:|--:|--:|--:|--:|:-:|
| 53 | 11 | 131.4 | 43.0 | 70.1 | 169.6 | 487.0 | true |
| 79 | 11 | 136.8 | 46.0 | 105.6 | 298.0 | 1888.4 | true |
| 101 | 19 | 185.2 | 71.0 | 125.8 | 292.6 | 762.8 | true |
| 131 | 67 | 337.0 | 83.0 | 147.6 | 502.5 | 1423.2 | true |
| 151 | 19 | 181.2 | 87.0 | 181.1 | 580.0 | 2111.9 | true |
| 181 | 41 | 249.2 | 96.0 | 210.4 | 683.5 | 2900.6 | true |
| 211 | 31 | 224.2 | 107.0 | 234.0 | 789.1 | 4383.1 | true |
| 251 | 37 | 248.4 | 125.0 | 267.2 | 829.0 | 2997.9 | true |

Unpriced linear-algebra multiplications `linalg_mulmods*` (mean of 8 seeds):

| p | LA1 | LA2 | LA3 | LA4 |
|--:|--:|--:|--:|--:|
| 53 | 880 | 2041 | 4517 | 9058 |
| 79 | 845 | 1834 | 6269 | 7265 |
| 101 | 2140 | 6620 | 19958 | 43253 |
| 131 | 2704 | 9681 | 29254 | 72202 |
| 151 | 3548 | 11223 | 38079 | 72407 |
| 181 | 4015 | 17210 | 50933 | 90468 |
| 211 | 5196 | 19194 | 56991 | 93709 |
| 251 | 6736 | 24577 | 80667 | 113057 |

Per-rung ratios of group operations (smoothness-probability prediction about g):

| p | IC2/IC1 | IC3/IC2 | IC4/IC3 | IC4/IC1 |
|--:|--:|--:|--:|--:|
| 53 | 1.63 | 2.42 | 2.87 | 11.33 |
| 79 | 2.30 | 2.82 | 6.34 | 41.05 |
| 101 | 1.77 | 2.33 | 2.61 | 10.74 |
| 131 | 1.78 | 3.40 | 2.83 | 17.15 |
| 151 | 2.08 | 3.20 | 3.64 | 24.27 |
| 181 | 2.19 | 3.25 | 4.24 | 30.21 |
| 211 | 2.19 | 3.37 | 5.55 | 40.96 |
| 251 | 2.14 | 3.10 | 3.62 | 23.98 |
| mean | 2.01 | 2.99 | 3.96 | 24.96 |

Least-squares log-log slope against p (8 points, no interval):

| series | slope |
|--|--:|
| rho on E | 0.44 |
| IC1 group ops | 0.71 |
| IC2 group ops | 0.85 |
| IC3 group ops | 1.06 |
| IC4 group ops | 1.21 |
| IC1 linalg mulmods | 1.43 |
| IC2 linalg mulmods | 1.81 |
| IC3 linalg mulmods | 1.96 |
| IC4 linalg mulmods | 1.89 |
| IC1 ops + 1.0 x linalg | 1.40 |
| IC2 ops + 1.0 x linalg | 1.79 |
| IC3 ops + 1.0 x linalg | 1.94 |
| IC4 ops + 1.0 x linalg | 1.83 |

IC_g / rho_E with each linear-algebra multiplication priced at w GAE (w = 0, 0.1, 1):

| p | g=1 @0 | @0.1 | @1 | g=2 @0 | @0.1 | @1 | g=3 @0 | @0.1 | @1 | g=4 @0 | @0.1 | @1 |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 53 | 0.33 | 1.00 | 7.03 | 0.53 | 2.09 | 16.07 | 1.29 | 4.73 | 35.67 | 3.71 | 10.60 | 72.65 |
| 79 | 0.34 | 0.95 | 6.52 | 0.77 | 2.11 | 14.18 | 2.18 | 6.76 | 48.02 | 13.81 | 19.12 | 66.94 |
| 101 | 0.38 | 1.54 | 11.94 | 0.68 | 4.25 | 36.41 | 1.58 | 12.35 | 109.31 | 4.12 | 27.47 | 237.60 |
| 131 | 0.25 | 1.05 | 8.27 | 0.44 | 3.31 | 29.16 | 1.49 | 10.17 | 88.30 | 4.22 | 25.65 | 218.47 |
| 151 | 0.48 | 2.44 | 20.06 | 1.00 | 7.19 | 62.92 | 3.20 | 24.21 | 213.29 | 11.65 | 51.60 | 411.14 |
| 181 | 0.39 | 2.00 | 16.49 | 0.84 | 7.75 | 69.89 | 2.74 | 23.18 | 207.09 | 11.64 | 47.93 | 374.60 |
| 211 | 0.48 | 2.79 | 23.65 | 1.04 | 9.60 | 86.63 | 3.52 | 28.93 | 257.66 | 19.55 | 61.33 | 437.42 |
| 251 | 0.50 | 3.22 | 27.62 | 1.08 | 10.97 | 100.03 | 3.34 | 35.82 | 328.12 | 12.07 | 57.59 | 467.26 |

Genus-4 cover data:

| p | #E | #J4 | h1 | h0 | h attempts | hom checks g1/g2/g3/g4 |
|--:|--:|--:|--:|--:|--:|--|
| 53 | 66 | 10429848 | 40 | 20 | 2 | 10/3/5/10 |
| 79 | 66 | 30909384 | 18 | 53 | 1 | 10/3/5/10 |
| 101 | 114 | 144530568 | 84 | 82 | 1 | 18/13/9/18 |
| 131 | 134 | 322784024 | 11 | 11 | 1 | 66/35/27/66 |
| 151 | 152 | 509693392 | 46 | 147 | 1 | 18/7/7/18 |
| 181 | 164 | 1021595032 | 137 | 19 | 1 | 40/19/23/40 |
| 211 | 186 | 1619534736 | 79 | 137 | 1 | 30/15/19/30 |
| 251 | 222 | 4028340960 | 152 | 90 | 1 | 36/15/25/36 |

## Reading the numbers

1. **Each rung up costs more.** Mean per-rung ratios of IC group operations are
   IC2/IC1 = 2.01, IC3/IC2 = 2.99, IC4/IC3 = 3.96 (per-p range 1.63-2.30,
   2.33-3.40, 2.61-6.34) [pcc:unverified]. The smoothness-probability
   prediction is about `g` (2, 3, 4); the means sit on it, with the genus-4
   per-p spread wide (p = 79 gives 6.34, p = 211 gives 5.55)
   [pcc:unverified]. IC4/IC1 ranges 10.7-41.1, mean 25.0 [pcc:unverified].
2. **The slopes steepen with genus.** Log-log slopes in `p` of IC group
   operations are 0.71 (g=1), 0.85 (g=2), 1.06 (g=3), 1.21 (g=4), against 0.44
   for rho on E [pcc:unverified]. Linear-algebra multiplications scale with
   slope 1.43, 1.81, 1.96, 1.89 [pcc:unverified]. These are 8-point fits at
   `p < 256` with no interval.
3. **Pricing (AGENTS.md §5a).** IC_g / rho_E at 0, 0.1 and 1 GAE per
   linear-algebra multiplication is in the priced table. No rung is below rho
   at all three prices at any p [pcc:unverified]: at 1 GAE every ratio is
   6.5 or more (genus 1: 6.5-27.6; genus 4: 67-467) [pcc:unverified]. Only at 0
   GAE do genus 1 (0.25-0.50) and genus 2 (0.44-1.08) fall under 1 for most p,
   and that ordering does not survive 0.1 GAE (genus 1 at 0.1 GAE: 0.95-3.22)
   [pcc:unverified]. So by the §5a rule no IC rung is ranked below rho, and no
   rung is ranked above rho at 0 GAE for genus 1 or 2 either (the order is not
   stable across the band) [pcc:unverified]. Genus 3 and 4 are above rho at
   all three prices at every p (genus 3 at 0 GAE: 1.29-3.52; genus 4: 3.71-19.55)
   [pcc:unverified].
4. **Distance to rho remains** for the best rung (genus 1, E itself): 0.25-0.50x
   rho at 0 GAE, 6.5-27.6x at 1 GAE [pcc:unverified]. Going up the tower does
   not close it; it widens it [pcc:unverified].

*Band caveat (replay F12, 2026-11-12):* the §5a band here is one-sided. The
rho reference has no unpriced counters, so only the IC arm moves across 0,
0.1 and 1 GAE. Rankings that hold across the band remain valid. Where the
order flips, the flip comes from the price of the IC linear algebra, not from
any uncertainty on the rho side.

## Limits

- `p < 256` (`HCurve`) and `r` between 11 and 67: toy sizes where rho is
  dominated by set-up and fitted exponents come from a short range. Not
  evidence about cryptographic sizes.
- One instance per prime (spread over algorithm seeds only). The genus-4
  per-p ratios are noisy (IC4 at p = 79 is 1888 against 487 at p = 53).
- The smoothness oracle's field multiplications are not counted at any rung;
  including them could only raise IC. Rho's unpriced native work is taken as
  zero beyond its counted group operations (no counters beyond adds/doubles
  were read from the rho reference).
- The genus-4 cover has degree 3, the genus-2/3 covers degree 2, so the rungs
  differ in cover construction as well as genus; the ladder does not isolate
  genus alone.
- No independent validator or replay certificate: this is not admitted to the
  leaderboard or `benchmarks/README.md` (validation gate), and no speedup is
  claimed.
- `E` and the genus-2/3 rungs are the same as in `../tower/TOWER.md`; the IC2
  and IC3 columns reproduce that table exactly [pcc:unverified].

## Reproduce

```
cargo build --release --bin tower_ladder
./target/release/tower_ladder --json ladder.json --svg ladder.svg --md tables.md
```

Files: `ladder.json`, `ladder.svg`, `tables.md`, `run.log`.
