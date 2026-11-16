# Genus ladder, follow-up 1: `r` bound to `p`, genus 4 and plane quartics to p = 8209 (frozen protocol, 2026-11-14)

Follow-up to `research/genus_ladder_20261114/` (next step 3 of its RESULTS.md). Every
figure `[pcc:unverified]` unless the validator report named in RESULTS.md says otherwise.
Synthetic random curves only; no deployed-curve statement.

## 1. Question

With the cofactor bound tightened to `h <= 2` (so `ln r` is within `ln 2` of `ln #J`
and rho's cost tracks `p^{g/2}` without the cofactor scatter of the first run), what
are the measured costs of double-large-prime index calculus (with the parity
prefilter) and of negation-map rho on genus-4 hyperelliptic Jacobians and on plane
quartics over `F_p`, `p = 2053, 4099, 8209`, and what are the exponents in `p`?

## 2. Instrument

The binary `genus_ladder` from the merged branch (source commit `14ab74ec8b`, the same
code as `main` at `959ea31976` up to a `mod` ordering), SHA-256
`61ccabd2e84b33ef...` (full hash in `binary.sha256`). Instances, arms, counts and
certificates are those of the first protocol, sections 2 to 5, with these settings:

```sh
genus_ladder run --primes 2053,4099,8209 --genera 4 --curves 3 --seeds 2 \
    --arms acR --cc 2.0 --qc 3.0 --max-h 2 --threads 4 --rho-cap 1.2e8 \
    --seed-search 4000 --quartic --out bound_r.jsonl
```

Arms: `a` (full base + prefilter), `c` (double large prime + prefilter), `R` (rho),
and the three quartic arms. Rho is skipped and recorded as skipped above
`1.2e8` expected steps (hyperelliptic) and `1.2e6` (quartic).

## 3. Analysis (fixed before data)

`genus_ladder_analyze` as in the first protocol (hierarchical bootstrap, 2000
resamples, seed 20261114; FPM primary reading; band verdicts; exponent fits).

Predictions:

| id | prediction | falsified if |
|---|---|---|
| B1 | every run certifies | an uncertified run |
| B2 | genus-4 rho exponent (FPM primary, 3 primes) within `2.0 +- 0.25` | the 95% interval excludes it |
| B3 | genus-4 arm c below rho across the FPM band at all three primes, and below rho at `w = 0` and `w = 0.1` of the GAE band at 8209 | the opposite verdict |
| B4 | quartic 2LP below quartic rho across the FPM band at all three primes | the opposite verdict |
| B5 | admitted genus-4 curves have `f` with at most 2 irreducible factors (the rational 2-rank `k - 1` must divide into `h <= 2`) | any admitted curve with 3 or more factors |

## 4. Validation

After the run, an independent validator (a separate agent with the sources, the
binary hash and the raw data, not this session's reasoning) replays a sample of the
rows with a binary it builds itself from the stated commit and compares every counted
field, and checks the certificates; its report is `VALIDATION_REPORT.md`. The host is
the same shared 4-vCPU container, not an isolated host: the rows therefore remain
pre-gate under AGENTS.md (no isolated-host measurement), whatever the replay says.

## 5. Limits

Three primes give a weak exponent fit. `h <= 2` at genus 4 is rare (about 1 in 50
seeds), so the admitted population is small (3 curves per cell). The first run's
genus-5 and genus-2/3 cells are not repeated.
