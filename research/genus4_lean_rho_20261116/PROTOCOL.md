# Genus 4: arm w against lean rho references, p = 8209, 16411, 32771

Frozen 2026-11-16 before any measured run (`frozen.sha256`, `binary.sha256`).
Every number is `[pcc:unverified]` until a validator replays it.

## 1. Question

Validator finding B13 (`research/genus4_rho_refs_20261116/ERRATA.md`) and the
owner's independent audit (`research/genus4_independent_audit_20261116`)
showed the w = 1 verdict at p = 8209 rested on the reference walk's per-step
16-key window: against plain rho without it, arm w / rho = 1.049 (C floored),
2.098 (C = 0) in Q3. This study measures two lean rho references on every
target and asks (a) where arm w stands against the cheapest of them, per prime,
and (b) how that ratio scales with p, the index-calculus question that matters.

## 2. Instances

- p = 8209: curve seeds 363, 393, 407 (rho-refs study) and 457, 474, 519 (scale study).
- p = 16411: curve seeds 172, 180, 2640 (scale study).
- p = 32771: the first 3 admitted curves (`h ≤ 2`, the binary's r-bound
  admission) in seed order from 1. p = 32771 is the largest prime of the ladder
  whose genus-4 order fits in u64.
- Targets: 3 per curve at 8209 and 16411 (target seeds `cs + k·1000003`, k = 0..2),
  1 per curve at 32771 (k = 0). Solver seeds 1–2.

## 3. Arms (binary `binary.sha256`, commit 3384161241 source)

- w: double large prime + prefilter + coordinate Wiedemann + capped unit-T walk (unchanged).
- B: negation rho, `cycle_window = 0`, Brent cycle detection (`WalkPolicy::v2`), Wiener–Zuccherato look-ahead.
- Q: plain rho, `cycle_window = 0` (the walk on points; same walk as P, no compares).
- R (negation rho with the window) is not re-run: its rows exist at 8209/16411
  (earlier studies, same instances) and enter the reference set there; at 32771
  it is omitted for cost (~5 CPU-hours), and B's smoke compares (3.0 vs 17.85 per
  step) make it the dearer negation walk in Q3/Q4 at w > 0.

## 4. Gates

- G1: every log certified and equal to the planted value.
- G2: arm w rows at 8209/16411 equal the earlier studies' rows for the same
  (curve, target, seed) in every count (same IC code; instrument check).
- G3: Q's steps and group operations at p = 8209 seeds 1–2 target 1 equal the
  earlier P rows for the same keys (Q is P without the window).

## 5. Readings

Q1–Q4 of `research/genus4_rho_refs_20261116/PROTOCOL.md` §3 (G priced at 1 GAE,
every native operation at w GAE; w = 0, 0.1, 1), with one change of the rho
canonicalisation count: **C is the walk's actual count** (0 for Q, whose walk
performs none); the floored count (C = steps) is a sensitivity.
Primary: Q3 (owner rule); Q4 (adds rho's closed-form hash and two coefficient
additions per step) is co-primary for the w = 1 verdict.
Reference per prime and reading: the cheapest mean cost among the measured rho
arms (B, Q; and R at 8209/16411 from the earlier studies' rows). Ratio of means, bootstrap over targets then
seeds (2000 resamples, 95%, seed 20261121).

## 6. Predictions (before data)

- P1: B's counted compares per step in [2.5, 3.5]; Q's are 0.
- P2: at w = 1 the cheapest rho is Q in Q3 and Q4 at every prime.
- P3: Q3, w = 1, arm w / cheapest rho: p = 8209 in [1.6, 2.6] (above rho);
  p = 16411 in [1.0, 2.0]; p = 32771 in [0.6, 1.4].
- P4: the least-squares slope of log(ratio) on log p (Q3, w = 1, three primes)
  is in [−0.8, −0.2] (double-large-prime theory: about −0.5 at genus 4).
- P5: at w = 0 and w = 0.1, arm w is below the cheapest rho at every prime in Q1–Q4.

## 7. Decision rule

A prime is "below rho at w = 1" only when the upper bound is < 1 in both Q3
and Q4 against the cheapest measured rho. The slope (P4) is reported with a
bootstrap interval over targets. Positive control of a known asymptotic effect;
not an ECDLP result.

## 8. Deviations known at freezing

Local shared host (4 cores; the scale study's resume and plain-rho stage run
alongside); no AWS identity, so not isolab-eks; no S3 checkpoint or raw upload;
no Conductor client. Counts are deterministic; timings informational.
