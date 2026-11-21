# Tower up at larger p: a genus-2 and genus-3 Jacobian beyond p = 251

**Status:** construction + measurement, one characteristic, one instrument. A
boundary result, not an improvement. Every measured sentence carries
`[pcc:unverified]`: no independent validator has replayed it. Extends
`../tower/TOWER.md` and `../tower_ladder/LADDER.md`, whose named next step was a
genus-3 Jacobian that handles larger primes.

## What was built

- `src/cryptanalysis/chained_transfer/hyper64.rs`: Mumford/Cantor arithmetic for
  genus <= 3 over `F_p`, `p < 2^31`, `u32` slots and 16-coefficient `u64`
  polynomials, on the **same model as `HCurve`**: the tower curve
  `C : y^2 = F(x^2)` has the rational Weierstrass point `(a, 0)`, sent to
  infinity by `t = 1/(x - a)`, `Yc = y t^(g+1)`, giving the imaginary model
  `Yc^2 = g(t)` with `deg g = 2g+1`. `H64Curve::new` takes that `g` exactly as
  `HCurve::new` does, so a pulled-up instance transfers unchanged. It carries
  point and divisor construction, add, double, negate, identity, reduction, a
  counted scalar multiplication (`CountedGroup::mul`), an independent
  right-to-left multiplication (`mul_rtl`, used for certificates), point counts
  over `F_p` and `F_(p^2)` (the latter `O(p^2)`, threaded, using `x <-> conj x`) and
  `jacobian_order` (the `HCurve` method: `L(T)` from `N_1, N_2`, then the
  multiples of `r` in the Hasse-Weil window that kill 32 random divisors; exactly
  one must survive).
- `Cover64` / `build64`: `Cover` / `tower::build` with `H64Curve`, plus a floor
  `min_r` on the prime `r` and a cap on cover attempts at large `p`
  (11 and `p - 1` reproduce `tower::build`).
- `hyper64::ic::solve`: `ecbench::hyper_ic::solve` ported line for line (same
  random stream, walk, relation loop, sparse elimination, counters).
- `src/bin/tower_large.rs`: the gate, the run, tables, slopes, JSON and SVG.
- Bench record: `benchmarks/records/20261112T0421Z-chained-tower-large-p.json`
  (left open).

## Cross-check against `HCurve` (the gate)

For the eight primes of `tower_bench` (53, 79, 101, 131, 151, 181, 211, 251) the
same tower seeds (`11 + i`) were built with `tower::build` (`HCurve`) and with
`build64` (`H64Curve`). The binary requires, per prime: identical `E`, `#E`,
`r`, planted `x`, `G`, `T`; identical odd-model polynomials, `sigma`, `a`,
`#J`, homomorphism-check counts and pulled-up `D`, `T` on both rungs; `#J`
recomputed from a different seed equal on both sides; point counts over `F_p`
(and `F_(p^2)` at genus 3) equal; 600 random operations per rung (add, double,
negate, subtract, scalar multiples; divisors drawn on each side and imported to
the other) giving identical divisors, identical validity and identical keys after
every operation; and, on 8 algorithm seeds, `hyper64::ic::solve` equal to
`hyper_ic::solve` in recovered log, every phase ledger and **every** counter,
rho on each rung equal in group operations, steps and walks, and rho on `E`
equal. Outcome: **PASS on all eight primes** (per prime 3772 field-level
comparisons, 1200 divisor comparisons, 16 IC solves, 24 rho runs)
[pcc:unverified]. Detail is in `tower_large.json` under `crosscheck` and in `run.log`.
`cargo test --release --lib chained_transfer::hyper64` runs the same gate on
five primes plus the group-law tests (associativity, commutativity, identity,
inverse, `[#J]D = 0`, both multiplication paths) on `p` in 53, 101, 151, 199 and a
fresh `p = 1009` tower.

## Setup of the large-p run

- Primes: 251, 503, 1009, 2003, 4001, 8009, 16001, 32003, 64007 (the next primes
  above 250, 500, ..., 64000). Three instances per prime
  (`build64(p, 1000 + 17 i + p mod 997, p/4, 64)`).
- Each instance: `E/F_p` with a rational 2-torsion point (needed by the genus-2
  rung), prime `r` with `r || #E` and `r >= p/4` (I chose this floor so that `r`
  stays within a small factor of `#E`, as for a cryptographic instance; it
  is not in `tower::build`), a genus-2 and a genus-3 cover, one planted log,
  and both rungs carrying the **same** `G`, `T = [x]G`.
- IC: `hyper64::ic::solve` with the `tower_bench` parameters (full degree-1
  factor base, no large primes, 8 extra relations, 64 branches, `d_step_in = 4`),
  8 algorithm seeds per instance. IC = Cantor adds + doubles over all phases
  (verification included). LA = `linalg_mulmods*`.
- Rho on `E`: `ic_boundary::rho_reference` (`EcGroup`), 64 seeds per instance.
- Every IC solve and every rho solve was checked against the planted log and
  certified by an independent `[k]G == T` (`mul_rtl` on the rung, `Curve::mul` on
  `E`) [pcc:unverified].
- Transfer (building the tower, pulling the instance up) is in Table 5 only and in
  no total, as the owner rule requires.
- Operation counts only; the wall seconds in the JSON are bookkeeping.

## Results

Counted group operations, mean over 3 instances x 8 seeds (rho: 64 seeds per
instance), `p` and the means as measured [pcc:unverified]:

| p | inst | r (mean) | rho_E | rho walk steps | IC2 | IC3 | IC3/IC2 | LA2 | LA3 | ok |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| 251 | 3 | 107 | 415.7 | 10.0 | 278.6 | 766.7 | 2.75 | 33678 | 87075 | true |
| 503 | 3 | 260 | 522.7 | 26.4 | 531.5 | 1648.1 | 3.10 | 101671 | 167804 | true |
| 1009 | 3 | 409 | 617.5 | 33.2 | 1033.4 | 3137.2 | 3.04 | 242501 | 310794 | true |
| 2003 | 3 | 1010 | 713.8 | 57.0 | 2023.8 | 6025.0 | 2.98 | 520039 | 754158 | true |
| 4001 | 3 | 1018 | 720.0 | 67.4 | 4047.9 | 11815.4 | 2.92 | 1017398 | 2376468 | true |
| 8009 | 3 | 3322 | 958.3 | 105.3 | 8054.2 | 24401.5 | 3.03 | 1952942 | 9268816 | true |
| 16001 | 3 | 6672 | 1066.7 | 125.6 | 16047.9 | 47918.2 | 2.99 | 3838200 | 48760065 | true |
| 32003 | 3 | 13395 | 1219.7 | 236.6 | 32003.8 | 95571.8 | 2.99 | 7686567 | 317350510 | true |
| 64007 | 3 | 26650 | 1359.1 | 282.5 | 64019.5 | 192940.9 | 3.01 | 15301048 | 2174988857 | true |

`rho_E` is rho as the instrument charges it: the 16-jump table set-up and the two
scalar multiplications that start each walk are included. `rho walk steps` is the
walk alone (`sqrt(pi r / 2)` scale). The difference `rho_E - rho walk steps` is
405.7 at `p = 251` and 1076.6 at `p = 64007`, so at these `r` the overhead is most of
`rho_E` [pcc:unverified].

Ratio IC_g / rho_E with each linear-algebra mulmod priced at `w` GAE (AGENTS.md
section 5a; rho as instrumented, no unpriced work on it) [pcc:unverified]:

| p | g=2 @0 | @0.1 | @1 | g=3 @0 | @0.1 | @1 |
|--:|--:|--:|--:|--:|--:|--:|
| 251 | 0.67 | 8.77 | 81.69 | 1.84 | 22.79 | 211.32 |
| 503 | 1.02 | 20.47 | 195.55 | 3.15 | 35.26 | 324.22 |
| 1009 | 1.67 | 40.94 | 394.36 | 5.08 | 55.41 | 508.36 |
| 2003 | 2.84 | 75.69 | 731.41 | 8.44 | 114.10 | 1065.02 |
| 4001 | 5.62 | 146.92 | 1418.59 | 16.41 | 346.46 | 3316.87 |
| 8009 | 8.40 | 212.20 | 2046.36 | 25.46 | 992.69 | 9697.74 |
| 16001 | 15.04 | 374.86 | 3613.16 | 44.92 | 4615.93 | 45755.03 |
| 32003 | 26.24 | 656.44 | 6328.24 | 78.36 | 26097.02 | 260265.04 |
| 64007 | 47.11 | 1172.96 | 11305.68 | 141.97 | 160178.55 | 1600507.80 |

The same against the walk steps alone, the harsher distance for IC [pcc:unverified]:

| p | g=2 @0 | @0.1 | @1 | g=3 @0 | @0.1 | @1 |
|--:|--:|--:|--:|--:|--:|--:|
| 251 | 27.8 | 364.3 | 3392.1 | 76.6 | 946.4 | 8775.0 |
| 503 | 20.2 | 405.8 | 3876.5 | 62.5 | 699.0 | 6427.3 |
| 1009 | 31.2 | 762.7 | 7346.2 | 94.6 | 1032.1 | 9469.7 |
| 2003 | 35.5 | 948.0 | 9160.7 | 105.7 | 1429.0 | 13339.0 |
| 4001 | 60.1 | 1570.1 | 15160.6 | 175.4 | 3702.6 | 35447.6 |
| 8009 | 76.5 | 1931.0 | 18621.7 | 231.7 | 9033.4 | 88248.6 |
| 16001 | 127.8 | 3184.5 | 30694.6 | 381.6 | 39213.3 | 388698.5 |
| 32003 | 135.3 | 3384.4 | 32626.6 | 404.0 | 134548.9 | 1341853.0 |
| 64007 | 226.7 | 5643.9 | 54398.6 | 683.1 | 770718.4 | 7701036.4 |

Every emitted unpriced counter priced at `w` on both arms (IC: LA mulmods, LA
inversions, oracle F_p multiplications, factor-base scan multiplications,
lookups, canonicalisations, step draws; rho: one native operation per step plus
one per distinguished point) [pcc:unverified]:

| p | g=2 @0 | @0.1 | @1 | g=3 @0 | @0.1 | @1 |
|--:|--:|--:|--:|--:|--:|--:|
| 251 | 0.67 | 13.25 | 121.32 | 1.84 | 79.45 | 745.97 |
| 503 | 1.02 | 28.22 | 257.34 | 3.15 | 144.47 | 1334.78 |
| 1009 | 1.67 | 53.06 | 484.43 | 5.08 | 240.35 | 2215.30 |
| 2003 | 2.84 | 100.85 | 907.55 | 8.44 | 452.99 | 4111.86 |
| 4001 | 5.62 | 196.71 | 1750.66 | 16.41 | 1038.09 | 9346.26 |
| 8009 | 8.40 | 292.31 | 2568.66 | 25.46 | 2144.44 | 19134.21 |
| 16001 | 15.04 | 525.39 | 4590.69 | 44.92 | 6702.49 | 59734.80 |
| 32003 | 26.24 | 930.28 | 7684.77 | 78.36 | 29445.34 | 248859.60 |
| 64007 | 47.11 | 1702.49 | 13931.49 | 141.97 | 164407.12 | 1377901.52 |

Trials per relation and the other emitted unpriced counters, mean [pcc:unverified]:

| p | tr/rel g=2 (2!) | tr/rel g=3 (3!) | columns | oracle F_p muls g=2 | oracle g=3 | LA inversions g=2 | LA inv g=3 |
|--:|--:|--:|--:|--:|--:|--:|--:|
| 251 | 1.93 | 5.32 | 131 / 134 | 8866 | 225528 | 122 | 131 |
| 503 | 1.97 | 6.41 | 256 / 247 | 19382 | 551149 | 230 | 237 |
| 1009 | 2.02 | 6.14 | 497 / 501 | 35549 | 1104740 | 433 | 479 |
| 2003 | 1.94 | 5.91 | 1027 / 1009 | 86941 | 2338641 | 877 | 953 |
| 4001 | 1.95 | 5.75 | 2062 / 2043 | 174226 | 4838241 | 1743 | 1928 |
| 8009 | 2.01 | 6.15 | 3994 / 3959 | 375822 | 10815826 | 3358 | 3721 |
| 16001 | 2.00 | 5.97 | 7990 / 8012 | 788387 | 22197467 | 6716 | 7532 |
| 32003 | 1.99 | 5.95 | 16051 / 16058 | 1683379 | 46104700 | 13474 | 15093 |
| 64007 | 2.00 | 6.04 | 31957 / 31908 | 3645504 | 101801676 | 26813 | 29957 |

Least-squares log-log slopes against `p` (9 points; the second column uses the 6
points with `p >= 1000`; point estimates, no interval, no bootstrap) [pcc:unverified]:

| series | all points | p >= 1000 |
|--|--:|--:|
| rho on E (group ops) | 0.209 | 0.198 |
| rho on E walk steps only | 0.559 | 0.511 |
| mean r | 0.979 | 1.010 |
| IC2 group ops | 0.985 | 0.995 |
| IC3 group ops | 0.991 | 0.995 |
| IC3 / IC2 | 0.006 | 0.000 |
| IC2 linalg mulmods | 1.066 | 0.988 |
| IC3 linalg mulmods | 1.824 | 2.150 |
| IC2 ops + 0.1 x linalg | 1.062 | 0.988 |
| IC3 ops + 0.1 x linalg | 1.804 | 2.126 |
| IC2 ops + 1.0 x linalg | 1.066 | 0.988 |
| IC3 ops + 1.0 x linalg | 1.822 | 2.148 |
| IC2 trials per relation | 0.005 | 0.002 |
| IC3 trials per relation | 0.007 | 0.000 |
| IC2 oracle fp muls | 1.086 | 1.101 |
| IC3 oracle fp muls | 1.090 | 1.087 |
| factor-base columns, genus 2 | 0.995 | 0.998 |

One-time transfer, in no total above [pcc:unverified]:

| p | pull-up Cantor ops g=2 / g=3 | build + pull wall s (mean) | hom checks g=2 / g=3 | phi^*(T) rational and == [x]D |
|--:|--:|--:|--:|--:|
| 251 | 3 / 3 | 0.1 | [(23, 29), (34, 42), (42, 30)] | 2 true, 0 false, 4 irrational |
| 503 | 2 / 2 | 0.1 | [(31, 33), (34, 31), (37, 33)] | 0 true, 0 false, 6 irrational |
| 1009 | 3 / 2 | 0.3 | [(42, 34), (43, 42), (44, 42)] | 2 true, 0 false, 4 irrational |
| 2003 | 3 / 3 | 0.4 | [(42, 38), (45, 39), (45, 40)] | 3 true, 0 false, 3 irrational |
| 4001 | 3 / 3 | 1.4 | [(35, 39), (36, 36), (40, 48)] | 2 true, 0 false, 4 irrational |
| 8009 | 2 / 3 | 4.4 | [(38, 40), (43, 36), (43, 37)] | 1 true, 0 false, 5 irrational |
| 16001 | 4 / 3 | 11.3 | [(32, 32), (37, 46), (39, 31)] | 4 true, 0 false, 2 irrational |
| 32003 | 2 / 3 | 58.1 | [(33, 35), (38, 33), (43, 36)] | 1 true, 0 false, 5 irrational |
| 64007 | 2 / 3 | 266.2 | [(35, 40), (36, 35), (41, 35)] | 2 true, 0 false, 4 irrational |

Order of IC_g against rho across the band `w` in 0, 0.1, 1 (rho as instrumented) [pcc:unverified]:

- p=251: g=2 unresolved (order changes inside the band); g=3 above rho at every weight
- p=503: g=2 above rho at every weight; g=3 above rho at every weight
- p=1009: g=2 above rho at every weight; g=3 above rho at every weight
- p=2003: g=2 above rho at every weight; g=3 above rho at every weight
- p=4001: g=2 above rho at every weight; g=3 above rho at every weight
- p=8009: g=2 above rho at every weight; g=3 above rho at every weight
- p=16001: g=2 above rho at every weight; g=3 above rho at every weight
- p=32003: g=2 above rho at every weight; g=3 above rho at every weight
- p=64007: g=2 above rho at every weight; g=3 above rho at every weight

## Reading

1. **IC3/IC2 stays near 3.** The ratio is 2.75 at `p = 251` and 3.01 at
   `p = 64007`, within 2.92 to 3.10 for `p >= 503`, slope 0.006 [pcc:unverified]. Trials per
   relation are 2.00 (genus 2) and 6.04 (genus 3) at `p = 64007`, against
   `2! = 2` and `3! = 6`; the slope of both in `p` is 0.005 and 0.007 [pcc:unverified]. So the
   `1/g!` smoothness probability persists from 251 to 64007 on this instrument
   [pcc:unverified].
2. **IC group operations are linear in `p`.** Slope 0.985 (genus 2) and 0.991
   (genus 3), about `g! x (p/2)`: columns `~ p/2` (slope 0.995) times `g!` trials
   per relation [pcc:unverified].
3. **Rho is `~ sqrt(p)`.** Walk-step slope 0.559 (0.511 for `p >= 1000`), mean `r`
   slope 0.979 [pcc:unverified]. The instrumented `rho_E` has slope 0.209 only because its
   set-up and walk-start overhead is most of it at these `r`; that flatters IC at small `p` and the
   walk-only column is the floor [pcc:unverified].
4. **Distance remaining to rho (not a pass/fail gate).** Group operations only
   (`w = 0`): IC2/rho_E = 0.67 at `p = 251` (the only point under rho, and
   only at `w = 0`, unresolved inside the band), 47.11 at `p = 64007`; IC3/rho_E
   = 1.84 and 141.97. Against the walk steps alone: 226.7 (genus 2) and 683.1
   (genus 3) at `p = 64007`. The gap grows with `p`; no point of this run
   narrows it [pcc:unverified]. At every `w` in the band genus 3 sits above genus 2 and both
   above rho for `p >= 503` [pcc:unverified].
5. **Linear algebra.** Genus 2: LA slope 1.07 (0.988 for `p >= 1000`). Genus 3: LA
   slope 1.82 (2.15 for `p >= 1000`), reaching 2.17e9 mulmods per solve at
   `p = 64007`, 142 times genus 2. With LA priced the genus-3 curve is
   far above genus 2 (Table 2: 160178.55 against 1172.96 at `w = 0.1`) [pcc:unverified].
   This LA is the instrument's sparse Markowitz elimination on rows that also
   carry the walk's accumulated branch offsets (up to 64 extra entries); a
   different relation collector or a structured solver would change it, and I did
   not try one.
6. **Oracle work is not small.** The smoothness oracle spends 101,801,676 `F_p`
   multiplications at genus 3, `p = 64007`, for 192,940.9 group operations (about
   528 per operation) [pcc:unverified]; it enters Table 2c only.

Conclusion: carrying the same elliptic log to genus 2 and genus 3 over primes up
to 64007 leaves index calculus with full degree-1 factor base above rho by a
factor that grows like `sqrt(p)`; the trend seen on `p <= 251` holds, with the
`1/g!` smoothness and the ratio near 3 intact [pcc:unverified]. This is a boundary result for
this method on this instrument, not an improvement and not a relabelling of one.

## Limitations

- One instrument for IC (the `hyper_ic` relation walk, no large primes, no double
  large prime, no Gaudry-Thériault structured factor base for g = 3 beyond the
  full degree-1 base). Another collector could change constants and LA.
- Cross-check covers `p < 256` only (`HCurve`'s range). Above 255 the checks are
  internal: `#J` unique among multiples of `r` in the Hasse-Weil window that kill
  32 random divisors, `[#J]D = 0` and `[r]D = 0` on the rung, associativity and
  inverse tests at `p = 1009`, the homomorphism check on every multiple `< 80`
  whose preimages are rational (23 to 48 per rung), and every recovered log
  certified. No second implementation exists at these sizes.
- `phi^*(T)` is `F_p`-rational for only some instances (Table 5); the rung target
  is `[x]D` by the homomorphism, as in `tower.rs`, and it equals the direct
  pull-back wherever that exists [pcc:unverified].
- Three instances per prime; `r` differs between instances and primes (between 0.250 p and 0.535 p over
  the 27 instances), which moves rho by `sqrt(r)`. The slopes are point estimates
  of nine means.
- Rho is the plain r-adding reference (`A = 1`, no negation), with its set-up
  charged; its unpriced per-step work is not emitted by the instrument, so
  Table 2c prices one native operation per step and per distinguished point as a
  stated assumption.
- The group key of `H64Curve` wraps `u64` past `p = 255`; it is a fingerprint and
  only `rho` on a Jacobian reads it (this run uses rho on `E` only).
- Time was not measured as a cost. What limited `p`: the `O(p^2)` point count
  over `F_(p^2)` that `jacobian_order` needs at genus 3 (the build, 266 s mean at
  `p = 64007` on a shared 4-core host) and then the genus-3 linear algebra; the
  whole run took 911 s under the bench record [pcc:unverified]. `p = 128000` would have
  quadrupled the point count.
- The run was local, not on `isolab-eks`, at the request of the task; the host is
  recorded in the bench record with isolation `unstated`.

## Files

`tower_large.json` (instances, rows, fits, gate), `tower_large.svg`
(IC2, IC3, rho against `p`, log-log), `run.log`, and the bench record above.
