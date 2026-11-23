# Positive control: capped unit-T walk + deterministic fallback roots (arm W)

Frozen 2026-11-16 before any measured run (`frozen.sha256`, `binary.sha256`).
Every number this study produces is `[pcc:unverified]` until a validator
replays it.

## 1. Question

The unit-T study (`research/ic_positive_control_unit_t_20261116`) found the
unit-T walk cuts LA multiplications to 0.76–0.79× and the deterministic
fallback roots cut relation work to 0.77–0.82× at the fallback primes, but
(F1) the uncapped unit-T walk loses rank at small r, and (D1) the arm meant
to combine the two ran with the mixed walk. This study measures the intended
combination, with the restart fix:

- **W**: Cardano–Lucas test, Cardano roots, deterministic fallback roots
  (`LucasRootsFast`); unit-T walk with segments capped at
  `max(64, ⌊√r/16⌋)` steps (`UnitTCapped`); coordinate Wiedemann on a square
  row subset; `cc = 2`.

Smoke on p = 131 and 2053 (curves 0–2, seeds 1000–2002; not part of any
verdict): W certifies in one attempt where uncapped U needed 34–76.

## 2. Arms and reused rows

| arm | source of rows |
|---|---|
| W | this run, every (p, curve, seed) |
| S, R | the unit-T study's `run.json` (same binary family, same seeds; counts are deterministic) |
| S, R reproduction | this run at p ≤ 8209: rows must equal the unit-T study's (gate G1) |

## 3. Instances and seeds

As the unit-T study: primes 131 … 65537 and 131101; `curve_seed0 = 1`,
`max_h = 64`; curves 5 (≤ 2053), 3 (4099, 8209), 2 (16411, 32771),
1 (65537, 131101); 4 seeds per curve and arm.

```
# W everywhere
$BIN run --primes 128,256,512,1024,2048,4096,8192,16384,32768,65536,131072 \
  --curves 5 --curves-map 4099=3,8209=3,16411=2,32771=2,65537=1,131101=1 \
  --seeds 4 --arms W --threads 3 --json $d/run_w.json
# S, R reproduction at p <= 8209
$BIN run --primes 128,256,512,1024,2048,4096,8192 \
  --curves 5 --curves-map 4099=3,8209=3 --seeds 4 --arms SR --threads 3 --json $d/run_sr.json
```

## 4. Gates

- G1: every S and R row of `run_sr.json` equals the unit-T study's row for the
  same (p, curve, seed) in group ops, relation and LA multiplications, trials,
  relations, certification.
- G2: every W log certified.
- G3: unit tests at the frozen commit, including
  `capped_unit_t_walk_solves_small_r_without_fallback`.

## 5. Metrics

W/S LA multiplications, W/S relation multiplications per trial, W/S and W/R
totals at w = 0, 0.1, 1 (two-stage bootstrap, 95%), `native_all` beside, LA
fallbacks per arm.

## 6. Predictions (before data)

- P1: W has at most 5 LA fallbacks in total over all runs.
- P2: W/S LA multiplications in [0.60, 0.90] at every p ≥ 4099.
- P3: W/S relation multiplications per trial in [0.70, 0.88] at 2053, 8209,
  65537 and in [0.95, 1.05] at 4099, 16411, 32771, 131101.
- P4: W/R at w = 0.1 is lower than S/R at every p ≥ 4099 (point estimates);
  at p = 32771 and 131101 in [1.4, 1.9].
- P5: §5a verdict unresolved for W (above rho at w = 1 everywhere).

## 7. Decision rule

W advances on S at a prime if its w = 0.1 and w = 1 totals are both below S's
(point estimates, paired on the same instances). It is the new operating point
if that holds at every p ≥ 4099. Distance to rho is reported as remaining
distance per price. Any prime where W's w = 0.1 interval lies entirely below 1
is reported as such with the §5a verdict beside it and goes to validator
review before any further wording.

## 8. Deviations (as the earlier studies)

Local run (no AWS identity, so not isolab-eks); seed-deterministic counts; no
S3 checkpoint or raw upload; no Conductor client. S and R rows are reused from
the unit-T study by determinism, checked by G1 at p ≤ 8209 only (the larger
primes are not re-run, to save about an hour of compute).
