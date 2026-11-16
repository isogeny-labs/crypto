# Genus 4, p = 8209: cheapest rho, symmetric pricing, more targets

Frozen 2026-11-16 before any measured run (`frozen.sha256`, `binary.sha256`).
Every number is `[pcc:unverified]` until a validator replays it.

## 1. Question

The genus-4 positive control (`research/genus4_positive_control_20261116`)
found arm w below negation-map rho at w = 0, 0.1 and 1 at p = 8209 when only
multiplications are priced. Its red-team review (`redteam/REDTEAM.md`) and
the genus-3 validator (`research/ic_positive_control_*_20261116/validation/`,
X2) named what that leaves open:

- F1: the lab's rho reference is the **cheapest** registered rho at each
  price; plain rho was not run.
- F2 / X2: the IC's modular additions are unpriced, and so are rho's per-step
  partition hash, its two coefficient additions modulo r and its cycle-window
  compares. Which operations the band prices is an owner decision not yet
  taken.
- F3: no calibrated weight `w_cal`.
- F4: 3 independent targets.

This study measures what a verdict needs under any of those pricing choices.

## 2. Arms and instances

- Instances: the three r-bound genus-4 curves at p = 8209 (seeds 363, 393,
  407, `h ≤ 2`), each with **3 targets** (target seeds `cs`, `cs + 1000003`,
  `cs + 2000006`): 9 independent targets. Solver seeds 1–2.
- Arms: **w** (double large prime + prefilter + coordinate Wiedemann + capped
  unit-T walk; as the genus-4 study), **R** (negation-map rho), **P** (plain
  rho, `RhoWalk::plain()`).
- `--calibrate`: one timing record per curve (ns per Cantor addition, per
  multiplication and addition modulo p, per multiplication modulo r).

```
$BIN run --primes 8209 --genera 4 --curves 3 --targets 3 --seeds 2 --arms wRP \
  --cc 2.0 --max-h 2 --threads 3 --rho-cap 1.2e8 --calibrate \
  --curve-seeds 8209=363/393/407 --out $d/run.jsonl
```

## 3. Readings (all pre-declared; costs in group-addition equivalents)

For IC: `G` = group ops, `M` = `rel_mul + la_mul`, `A` = `rel_add + la_add`.
For rho: `G` = group ops, `C` = canonicalisations as emitted (`native`,
floored at one per step), `H` = partition hashes (one per step), `D` =
coefficient additions (two per step), `K` = window/Brent compares.

| reading | IC cost | rho cost |
|---|---|---|
| Q1 multiplications only (the earlier studies) | `G + w·M` | `G + w·C` |
| Q2 all IC native ops, rho as emitted | `G + w·(M + A)` | `G + w·C` |
| Q3 all emitted native ops on both sides | `G + w·(M + A)` | `G + w·(C + K)` |
| Q4 Q3 plus rho's un-emitted per-step work | `G + w·(M + A)` | `G + w·(C + K + H + D)` |
| Qcal calibrated | `G + w_mul·M + w_add·A` (w_mul, w_add from the timing, mod-p rates for the relation phase and mod-r for the LA) | `G` (rho's native work timed as part of the Cantor step: priced 0) |

`w ∈ {0, 0.1, 1}` for Q1–Q4. The rho reference at each reading and price is
the cheaper of R and P (mean over the same targets).

## 4. Gates

- G1: every w row for target `cs` and seeds 1–2 equals the genus-4 study's w
  row (group ops, relation and LA multiplications, trials, relations); every R
  row there equals that study's R row.
- G2: every log certified and equal to the planted value.

## 5. Statistics

Per reading and price: ratio of mean arm cost to mean cheapest-rho cost over
the 18 runs; bootstrap over targets (9), then seeds, 2000 resamples, 95%.
Also leave-one-curve-out.

## 6. Predictions (before data)

- P1: P/R at w = 0 in [1.25, 1.6] (plain rho needs about √2 more steps). In Q1
  P/R ≥ 1 at every w (the instrument floors plain rho's native work at one
  per step, as negation's). In Q3 and Q4 at w = 1, P is the cheaper rho
  (negation pays its window compares, plain has few).
- P2: w below the cheapest rho at w = 0 and w = 0.1 in every reading (upper
  bound < 1).
- P3: at w = 1, w below the cheapest rho in Q1 (upper bound < 1); unresolved or
  above in Q2; below in Q4.
- P4: in Qcal, w is below rho (upper bound < 1).

## 7. Decision rule

- The statement "below rho across the §5a band at p = 8209 for these
  instances" is made only for the readings in which w's upper bound is below 1
  at w = 0, 0.1 and 1 against the cheapest rho; each reading's verdict is
  reported, none is singled out. Which reading is the lab's is the owner's
  decision (named in RESULTS.md as open).
- Positive control of a known asymptotic effect; not an ECDLP result.

## 8. Deviations (as the earlier studies)

Local run (no AWS identity, so not isolab-eks); seed-deterministic counts; no
S3 checkpoint or raw upload; no Conductor client. `--calibrate` timings are
from a shared host and informational except in Qcal.
