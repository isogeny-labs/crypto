# Lightest-row floor (`--excess E,0.01`) on fresh genus-3 instances, p = 65537, 131101, 262147 (frozen protocol, 2026-11-22)

Lead from `genus_ladder_light_rows_20261121` (post-hoc, same 3 curves per prime that found it): collect (1+E)x the
relations the stopping rule needs, peel, keep only the lightest `cols + margin` rows (`keep` at its floor, K = 0.01),
then SGE(8,3) and Wiedemann. There FPM_mid was 0.877 / 0.866 / 0.843 of the plain SGE(8,3) arm at 32771 / 65537 / 131101
(E = 0.2) and `la_mul` 0.63-0.65. Question: does it hold on instances not used to find it, with E and the thresholds fixed
before data? Measurement with an existing binary, no source change. Every figure `[pcc:unverified]`.

## Instances (fresh)
Curves 4-9 of `genus_ladder_g3_highp_20261121` at each prime (curves 1-3 are the ones the lead was found on and are
excluded): curve seeds
65537: 158, 217, 250, 254, 255, 283; 131101: 41, 58, 61, 79, 84, 101; 262147: 69, 113, 206, 207, 241, 261
(from the `instance` lines of `run_P.jsonl` of that study). 6 curves x 2 seeds = 12 runs per arm per prime.
Their instance lines must equal R2's field for field (`f`, `order`, `h`, `r`, `curve_seed`, `p`, `g`, `family`).
Rho rows: R2's `R_rho_neg` runs of the same curve seeds (12 per prime), not re-run.

## Run (one invocation per prime, 262147 last, nohup)
```sh
genus_ladder run --primes P --genera 3 --curve-seeds P=s4/s5/s6/s7/s8/s9 --seeds 2 --arms c --sge 8,3 \
  --excess 0.1,0.01 --excess 0.2,0.01 --excess 0.3,0.01 --cc 2.0 --max-h 8 --threads 4 --max-trials 4e8 \
  --order-method bsgs1 --out run_P.jsonl
```
Arms per (curve, seed): `C_double_lp+pre` (unmerged; kept as the identity check), `+sge(8,3)` (plain), and
`+sge(8,3)+ex(0.1,0.01)`, `+ex(0.2,0.01)`, `+ex(0.3,0.01)`. Binary: a copy of the clean committed tree's
`target/release/genus_ladder` (HEAD d046bb88ac; hash in `binary.sha256`), used only from the copy.

## Estimands (fixed before data)
* FPM_mid = cantor_mul + 10 cantor_inv + rel_mul + 9 la_mul (IC arms; rho: cantor_mul + 10 cantor_inv). Relation phase =
  cantor_mul + 10 cantor_inv + rel_mul.
* Paired ratio of an excess arm to the plain arm: mean over the 12 (curve, seed) pairs of the per-pair ratio; 95%
  percentile interval of 4000 hierarchical bootstrap draws (curves with replacement, then seeds within the drawn curve),
  seed 20261122. Same for `la_mul` and the relation phase. A point estimate decides a threshold; the interval is reported.
* Ratio to rho: mean FPM of the arm over the 12 runs / mean FPM of the 12 R2 rho runs on the same curves; interval by the
  same bootstrap (rho resampled with its own curves).
* Best E: the E with the lowest paired FPM ratio at 262147 (point estimate). It is chosen after data, from three
  preregistered settings, and used for L5 and the exponent below; all three are reported.
* Mean weight after selection: `mean_weight_after` of the excess arm over `mean_weight_before` of the plain arm
  (mean over the 12 runs; the plain arm's weight does not change).
* Exponent (outside the scorecard): least squares of log(mean `la_mul`) on log p over the three primes, for the plain arm and
  the best setting (6 fresh curves each, so not comparable in level to R2's 9-curve 1.52 over six primes; a three-prime fit is
  noisy and is descriptive only).

## Predictions (fixed before data)
| id | prediction | falsified if |
|---|---|---|
| L1 | the plain `+sge(8,3)` rows (and the unmerged arm) equal R2's rows for the same (instance, seed) field for every field R2's rows have except `wall_ns`; the 6 instance lines per prime equal R2's; every run (all 5 arms) certifies with the planted t | any field differs, any instance differs, any run does not certify |
| L2 | FPM_mid ratio of ex(0.2,0.01) to plain <= 0.90 at 65537, <= 0.87 at 131101, <= 0.85 at 262147; `la_mul` ratio <= 0.70 at all three | any point estimate above its bound |
| L3 | relation-phase cost ranks E = 0.1 < 0.2 < 0.3 and `la_mul` ranks 0.3 <= 0.2 <= 0.1 (means over 12 runs, at each of the three primes); the lowest-FPM E at 262147 is 0.2 or 0.3 (my pick: 0.3, by a thin margin: from R3's (0.2,min) -> (0.5,min) `la_mul` ratio 0.64 -> 0.48, E = 0.2 -> 0.3 saves about 0.05 of plain `la_mul` = 0.04 of FPM at LA share 0.67 and costs 0.1 x 0.33 = 0.03 in relations, net about -0.01, inside noise) | an ordering violated at any prime, or the 262147 optimum is E = 0.1 |
| L4 | mean weight after selection <= 0.90 of the plain arm's mean weight, for ex(0.2,0.01) at each prime | any prime above 0.90 |
| L5 | with the best E, FPM_mid of the excess arm over rho at 262147 is <= 0.33 (R2's plain arm, nine curves: 0.377 [0.263, 0.565]) | point estimate above 0.33 |
Failures are reported as failures. The plain arm's ratio to rho on these six curves is also reported, since the six are a subset of R2's nine.

## Limits (declared)
Shared 4-vCPU host (no walls used), 6 curves x 2 seeds per prime, rho rows reused, one SGE setting, genus 3 only, no
independent replay, no validation gate; the 262147 optimum E is selected on the same data it is then scored on (L5 is
therefore optimistic by the selection among three settings); `analyze.py` is written before data and not changed after
the first prime's rows except where declared in RESULTS.md.
