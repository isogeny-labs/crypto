# Genus 3 at p = 65537, 131101, 262147 with nine curves per prime (frozen protocol, 2026-11-21)

Replicates the extension run of `research/genus_ladder_fast_order_20261121` (3 curves x 2 seeds per prime) with
9 curves x 2 seeds per prime. Question: where do the SGE arm's ratio to rho and the exponents of both arms land?
Every figure `[pcc:unverified]`. Measurement only: an existing binary, no source change.

## Run (one invocation per prime, nohup, smallest first)
```sh
genus_ladder run --primes P --genera 3 --curves 9 --seeds 2 --arms cR --sge 8,3 --cc 2.0 --max-h 8 \
    --threads 4 --max-trials 4e8 --seed-search 600 --order-method bsgs1 --rho-cap 3e9 --out run_P.jsonl
```
`--arms cR` exactly (no arm `a`). `--curves 9` with the same seed search reproduces the earlier three admitted
curves first (the search walks seeds in order), then six more. Binary: copy of the clean committed tree's
`target/release/genus_ladder` (hash in `binary.sha256`), analysis by the copy of `genus_ladder_analyze`.

## Estimands (fixed before data)
* Run cost FPM_mid = cantor_mul + 10 cantor_inv + rel_mul + 9 la_mul (IC arms; rho rows: cantor_mul + 10 cantor_inv).
* Ratio to measured rho (per prime): mean over instances of the per-instance mean (over the 2 seeds) FPM of the arm,
  over the same for rho. Interval: 95% percentile of 4000 hierarchical bootstrap draws (curves with replacement,
  then seeds within the drawn curve), the same draw for both arms. The analyzer's cell means and intervals are
  reported alongside as a cross-check.
* Expected-rho reference per instance: E = 0.886 sqrt(r) (cm + 10 ci), where cm, ci are cantor_mul and cantor_inv per
  walk step, pooled over the rho rows of that prime (sum over runs / sum of `steps`). Ratio to expected rho: pooled SGE
  mean over the mean E of the same instances; same bootstrap (E is fixed per instance).
* Exponents: least squares of log(cell mean FPM) on log p over 8209 ... 262147 (six primes). Rows at 8209, 16411,
  32771: rho from `genus_ladder_scale_20261115/scale_g3.jsonl`, SGE arm from `genus_ladder_sge_scale_20261115/sge_g3.jsonl`
  (3 curves x 2 seeds each; the scratchpad copy of `scale_g3.jsonl` is checked equal to the committed one). Rows at 65537,
  131101, 262147: this run only (18 runs per arm per prime; the earlier extension's rows are the first three curves of
  these and are not added again). Interval: bootstrap over curves within each prime, refit, 95% percentile.
  An exponent over the five primes 8209 ... 131101 is also given for comparison with the earlier study.
* LA share per prime: 9 la_mul / FPM_mid of the SGE arm (mean over runs).

## Predictions (fixed before data)
| id | prediction | falsified if |
|---|---|---|
| P1 | every run certifies; the first three admitted curves per prime reproduce the earlier instance lines, and their run rows equal the earlier rows field for field except `wall_ns` | any run not certified, any instance line or non-wall run field differs |
| P2 | pooled over 9 curves, SGE over measured rho is in [0.25, 0.45] at all three primes | any point estimate outside |
| P3 | rho exponent (6 primes) has an interval containing 1.5 and a point in [1.35, 1.55]; the SGE arm exponent is in [1.38, 1.46] | any of the three conditions fails |
| P4 | ratios SGE / expected rho differ from the measured-rho ratios by less than the measured ratios' interval width (at each prime 65537, 131101, 262147), and fall monotonically from 32771 to 262147 (predicted: monotone fall, not flat; 32771 from the earlier 3 curves) | a difference exceeds the width at any prime, or a point estimate rises between consecutive primes (a rise-free but within-10% line is reported as flat, not as held) |

## Limits (declared)
Shared 4-vCPU host, walls not isolated, no independent replay; 9 curves per prime is still a small sample of the
admitted-curve population (h <= 8 filter, near-prime order); the lower three primes keep 3 curves each.
