# Genus 4: the positive-control speed-ups on the genus ladder

Frozen 2026-11-16 before any measured run (`frozen.sha256`, `binary.sha256`).
Every number this study produces is `[pcc:unverified]` until a validator
replays it.

## 1. Question

The genus-ladder r-bound study (`research/genus_ladder_bound_r_20261114`)
measured double-large-prime index calculus with the parity prefilter (arm c)
on genus-4 hyperelliptic Jacobians with `h ≤ 2` against negation-map rho. At
p = 8209 arm c was below rho at w = 0 and w = 0.1 and unresolved at w = 1
(0.84, interval [0.48, 1.64]); its linear algebra was 3.43e7 of its 7.77e7
native multiplications.

The genus-3 positive control cut the linear algebra to about 0.25× with two
changes, now ported to the genus ladder as `genus_ladder::IcExt`:
coordinate Wiedemann on a square row subset (arm v) and, in addition, the
unit-T walk with capped restarts (arm w). This study measures them at genus 4
on the same instances, against the same rho, on the §5a band.

A smoke run (p = 2053, the three r-bound instances, seeds 1–2; not part of any
verdict) had arm c reproduce the r-bound rows exactly, w's LA at 0.22× of c
and w/R at w = 1 between 0.75 and 1.02.

## 2. Arms (one binary, same instances and seeds)

| arm | what |
|---|---|
| c | double large prime + parity prefilter (the r-bound arm) |
| v | c + coordinate Wiedemann on a square row subset |
| w | v + unit-T walk with capped restarts |
| R | negation-map rho (`gl::run_rho`) |

`cc = 2.0`, restart 4096 (as the r-bound run), `max_trials` default.

## 3. Instances and seeds

The r-bound study's admitted genus-4 curves, re-built by their seeds (same
admission path, `--max-h 2`): p = 2053 seeds 40, 60, 109; p = 4099 seeds 72,
247, 418; p = 8209 seeds 363, 393, 407. Solver seeds 1–4 (the r-bound run used
1–2).

```
$BIN run --primes 2053,4099,8209 --genera 4 --curves 3 --seeds 4 --arms cvwR \
  --cc 2.0 --max-h 2 --threads 3 --rho-cap 1.2e8 \
  --curve-seeds 2053=40/60/109,4099=72/247/418,8209=363/393/407 --out $d/run.jsonl
```

## 4. Gates

- G1: every instance rebuilt has the r-bound study's order, h and r; every c
  and R row with seed 1 or 2 equals the r-bound row (group ops, relation and
  LA multiplications, trials, relations; rho group ops and native work).
- G2: every log certified.
- G3: unit tests `ext_default_is_run_ic_count_for_count`,
  `ext_arms_certify_at_genus_3_and_4` pass at the frozen commit.

## 5. Metrics

Per p, paired on (curve, seed): v/c and w/c LA multiplications; w/c relation
multiplications; arm total ÷ R total at w = 0, 0.1, 1 (`group_ops + w ·
(rel_mul + la_mul)`; rho `group_ops + w · native`), two-stage bootstrap
(curves, then seeds), 2000 resamples, 95%. Also the FPM reading of the
genus ladder (`cantor_mul + 10·cantor_inv + rel_mul + g²·la_mul`).

## 6. Predictions (before data)

- P1: w/c LA multiplications in [0.15, 0.35] at every p; v/c in [0.25, 0.45].
- P2: w/R at w = 1, p = 8209, point estimate in [0.40, 0.75].
- P3: w below R at w = 0 and w = 0.1 (upper bound < 1) at every p.
- P4: at p = 8209 the w = 1 interval of w/R has its upper end below 1
  (a cell below rho at all three prices). Uncertain: the rho variance across
  three curves is large; written down so that a miss is recorded.
- P5: relation multiplications are at least 75% of w's native
  multiplications at every p.

## 7. Decision rule

- v and w advance on c at a prime if their totals are below c's at w = 0.1
  and w = 1 (point estimates, paired).
- A prime where w's upper 95% bound is below 1 at w = 0, 0.1 and 1 is
  reported as below rho across the §5a band, for these instances, with the
  instances' scope (synthetic genus-4 curves, `h ≤ 2`, p ≤ 8209). It goes to
  an independent validator replay before any wording beyond
  `[pcc:unverified]`, and to red-team review before it is described as more
  than a positive control.

## 8. Deviations (as the earlier studies)

Local run (no AWS identity, so not isolab-eks); seed-deterministic counts; no
S3 checkpoint or raw upload; no Conductor client.
