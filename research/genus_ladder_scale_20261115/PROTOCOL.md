# Genus ladder, follow-up 2: genus 2 to p ≈ 2^24 and genus 3 to p ≈ 2^15 (frozen protocol, 2026-11-15)

Goal context: the standing objective is a significant, verified improvement for the
hyperelliptic DLP at genus 2–4. This run extends the measured ladder to sizes where the
fixed set-up costs that distorted the first run's genus-2 exponents are negligible, and
asks the sharpest question the instrument can answer cheaply: does double-large-prime
index calculus cross rho at genus 2 within p ≤ 2^24, and how fast does the genus-3 gap
open. Every figure `[pcc:unverified]`; no leaderboard row (shared host).

## Instrument
`genus_ladder` binary sha `61ccabd2…` (the first follow-up's; `binary.sha256`), unchanged
code (`genus_ladder.rs` of main `959ea31976`). Instances, arms, counts, certificates as in
`research/genus_ladder_20261114/PROTOCOL.md` sections 2–5.

## Run
```sh
genus_ladder run --primes 8209,16411,32771,65537,131071,262147,524309,1048583,2097169,4194319,8388617,16777259 \
    --genera 2 --curves 3 --seeds 2 --arms acR --cc 2.0 --max-h 8 --threads 4 --rho-cap 3e7 \
    --max-trials 2e8 --seed-search 600 --out scale_g2.jsonl
genus_ladder run --primes 8209,16411,32771 --genera 3 --curves 3 --seeds 2 --arms acR --cc 2.0 \
    --max-h 8 --threads 4 --rho-cap 3e7 --max-trials 2e8 --seed-search 600 --out scale_g3.jsonl
```
Primes: the first prime at or above 2^13 … 2^24 (genus 2) and 2^13 … 2^15 (genus 3).
Arms: a (full base + prefilter), c (double large prime + prefilter, base `ceil(2 p^{1−1/g})`),
R (negation rho). Launched under `tools/benchrec.py run`. If a genus-2 cell at the largest
primes exceeds memory or `max_trials`, its runs are recorded as exhausted/skipped and the
cell is reported as such; nothing is rerun with other parameters under this protocol.

## Analysis
`genus_ladder_analyze` (bootstrap, FPM and GAE bands, exponent fits over the primes
present), `render.py` of the first study.

## Predictions (fixed before data)
| id | prediction | falsified if |
|---|---|---|
| X1 | every run that is not exhausted certifies; no genus-2 cell below 2^22 is exhausted | otherwise |
| X2 | genus-2 rho exponent (FPM primary, over the primes run) in [0.90, 1.10]; genus-2 arm c exponent in [0.85, 1.20] | 95% interval excludes the range |
| X3 | genus-2 `FPM_mid(c)/FPM_mid(rho)` at the largest prime run is at least 2× smaller than at 8209 | otherwise |
| X4 | genus 2: at least one prime at which arm c is `below` rho across the whole FPM band (a measured crossing); if none, the ratio at 2^24 is reported as the distance remaining | no such prime (reported as failed, with the distance) |
| X5 | genus 3: arm c exponent in [1.20, 1.45], rho in [1.35, 1.65]; `c/rho` below rho across the FPM band at 32771 with ratio ≤ 0.6 | otherwise |
| X6 | genus 2 arm c: the linear-algebra share of FPM (g²·la_mul / FPM) grows with p and exceeds 0.5 at 2^20 (the large-prime graph at genus 2 has about p/2 vertices and long cycles) | share at 2^20 below 0.5 |

## Limits
Shared 4-vCPU host (counted units only); 3 curves × 2 seeds per cell; `h ≤ 8`; genus 2
only above 2^15. Not an isolated-host measurement.

## Amendment 1 (2026-11-15 05:10Z, before any datum of the cells it changes)

The full-base arm a at genus 2 spends its time in a dense linear algebra of about
`p/2` columns (`la_mul` 5.8e10 at p = 262147, 54 minutes for one run) and would take
days at p ≥ 2^20; it answers none of X1–X6, which are about arm c and rho. The run was
stopped after the p = 262147 cell (108 runs, all certified; no run of a larger cell had
started) and relaunched for `524309,1048583,2097169,4194319,8388617,16777259` with
`--arms cR` only, everything else unchanged; then the genus-3 command as written. The
arm-a exponent (X2's second clause) is therefore fitted on 8209…262147 only, and
that restriction is reported with it.
