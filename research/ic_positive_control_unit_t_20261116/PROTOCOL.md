# Positive control: unit-T walk and deterministic fallback roots

Frozen 2026-11-16 before any measured run (`frozen.sha256`, `binary.sha256`).
Every number this study produces is `[pcc:unverified]` until a validator
replays it.

## 1. Question

After the coordinate Wiedemann (`research/ic_positive_control_la_20261116`,
arm S) two costs remain that a diagnostic on training curves (seed 5001, not
part of any verdict) singled out:

- every relation carries a random `T`-coefficient, so the matrix has one dense
  column costing about N multiplications per Krylov step;
- where `3 | n/3` (p = 521, 2053, 8209, 65537) the Cardano cube root does
  not exist and root finding falls back to randomized recursive
  Cantor–Zassenhaus.

This study measures one fix for each, against the matched negation rho on
the §5a band, and adds one larger prime (131101, the first prime ≥ 2^17) to
read the trend in p.

## 2. Arms (same binary, same instances, same seeds)

| arm | splitting / roots | walk | linear algebra |
|---|---|---|---|
| S | Cardano–Lucas + Cardano roots, CZ fallback (`LucasRoots`) | mixed `aG + bT`, jumps `αG + βT` | coordinate Wiedemann, square subset |
| U | as S | unit-T: start `aG + T`, jumps `αG`; unknown `h·log T` (column −1) | as S |
| V | as U, fallback = one deterministic CZ split + quadratic formula (`LucasRootsFast`) | unit-T | as S |
| R | negation-map rho on the order-`r` subgroup | – | – |

`cc = 2` (the training result of the previous study). U and V change the
walk, so their relations differ from S's; comparisons are paired on
(p, curve, seed), not on identical relations.

## 3. Instances and seeds

The earlier studies' primes (first prime ≥ `2^7 … 2^16`) plus 131101;
`curve_seed0 = 1`, `max_h = 64`; curves per prime 5 (≤ 2053), 3 (4099,
8209), 2 (16411, 32771), 1 (65537, 131101); 4 seeds per curve and arm.

```
$BIN run --primes 128,256,512,1024,2048,4096,8192,16384,32768,65536,131072 \
  --curves 5 --curves-map 4099=3,8209=3,16411=2,32771=2,65537=1,131101=1 \
  --seeds 4 --arms SUVR --threads 3 --json $d/run.json
```

## 4. Gates

- G1: S and R rows equal the previous study's S and R rows on every shared
  (p, curve, seed) (group ops, relation and LA multiplications, trials,
  relations).
- G2: every log certified in every arm.
- G3: unit tests (`unit_t_walk_certifies_and_removes_the_dense_column`,
  the every-cubic oracle test including `LucasRootsFast`) pass at the frozen
  commit.

## 5. Metrics

- U/S LA multiplications; V/U relation multiplications per trial; per p,
  two-stage bootstrap (curves, then seeds), 95%.
- Arm total ÷ R total at w = 0, 0.1, 1; `native_all` beside multiplications.

## 6. Predictions (before data)

- P1: U/S LA multiplications in [0.60, 0.85] at every p ≥ 4099.
- P2: V/U relation multiplications per trial ≤ 0.90 at each of 521, 2053,
  8209, 65537, and in [0.95, 1.05] at every other p ≥ 1031.
- P3: V/R at w = 0.1, p = 131101, in [0.8, 1.8].
- P4: no arm below R at all three prices at any p (verdict unresolved): at
  w = 1 every arm stays above R.
- P5: the w = 0.1 ratio of V falls with p: least-squares slope of
  ln(V/R at w = 0.1) on ln p over p ≥ 4099 is negative.

## 7. Decision rule

- Unit-T advances the component if G1–G3 hold and U/S LA ≤ 0.85 at every
  p ≥ 4099.
- The fallback advances the component if V/U relation per trial ≤ 0.9 at
  every fallback prime ≥ 2053.
- Any p where V's w = 0.1 interval lies entirely below 1 is reported as such,
  with the §5a verdict (which needs all three prices) stated beside it, and
  goes to validator review before any wording beyond `[pcc:unverified]`.

## 8. Deviations (as the earlier studies)

Local run (no AWS identity here, so not isolab-eks); seed-deterministic
counts; no S3 checkpoint or raw upload; no Conductor client.
