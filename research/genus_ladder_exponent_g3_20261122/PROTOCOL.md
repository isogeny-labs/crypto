# Does the genus-3 IC arm's exponent separate from rho's at p = 2^19 to 2^20? (frozen protocol, 2026-11-22)

Measurement only, existing binary (`binary.sha256`; HEAD f6acc4c2e4, faster Wiedemann; copies in scratchpad `r10/`,
hashes equal to the verified `r9/` copies and to R9's `binary.sha256`). Every figure `[pcc:unverified]`.

## Design
Genus 3, new primes p in {524309, 1048583}, fresh curves (no curve seeds given: first 3 admitted of seeds 1..600).
Per prime, strictly one after the other, nothing else running, smaller first (`run_all.sh`, frozen):
`genus_ladder run --primes P --genera 3 --curves 3 --seeds 2 --arms cR --sge 8,3 --cc 2.0 --max-h 8 --threads 1
--max-trials 1e9 --rho-cap 3e10 --seed-search 600 --order-method bsgs1 --out raw/run_P.jsonl`.
`uptime` into `raw/load.log` before and after each invocation (append-only). Arms: `C_double_lp+pre` (no SGE, not
the subject), `C_double_lp+pre+sge(8,3)` (IC arm), `R_rho_neg`. `wall_ns` is per run; instance search and order
computation are outside it for both arms (as in R9).
Joined with R9's rows (`../genus_ladder_wallclock_vs_rho_20261122/raw/run_{65537,131101,262147}.jsonl`, same binary,
6 curves x 2 seeds each; R9's curves = R2's curves 1-6) for a five-prime fit. New primes have 3 curves x 2 seeds.

## Budget and the --seeds decision (before data)
Rho mean wall 118.8 s at 262147 (R9). With p^1.5: about 336 s at 524309, about 950 s at 1048583. Six rho runs
(3 curves x 2 seeds): about 34 min and about 95 min. IC+SGE and no-SGE arms add about 0.5 h at 1048583. Rho alone at
1048583 is under the 2.5 h threshold, so `--seeds 2` is kept at both primes (no reduction). Rho is heavy-tailed; if a
prime's rho block turns out much longer, that is reported, not hidden.

## Estimands
* E2 per prime (524309, 1048583): ratio = mean over curves of per-curve mean-over-seeds `wall_ns` of the IC+SGE arm
  over the same for rho, paired per (curve, seed). 95 % percentile interval of 4000 hierarchical bootstrap draws
  (curves with replacement, then seeds within drawn curve; same draw for both arms).
* E3 wall exponents: for each arm, log-log least squares of per-prime mean wall on p over the five primes. Interval:
  4000 draws; in each draw, within each prime resample curves with replacement then seeds within curve (paired draw
  for both arms), recompute the five per-prime means and the slope; 95 % percentile. Separation test: the IC arm's
  upper end < rho's lower end. Also reported (secondary, not the test): interval of the slope difference IC minus rho
  from the same draws (the paired view, tighter because the draws are shared).
* E4 count exponents, same fit and bootstrap, on per-prime means of: relation phase of the IC+SGE arm
  `cantor_mul + 10 cantor_inv + rel_mul`; linear algebra `la_mul`; rho `cantor_mul + 10 cantor_inv`.
  Compared with 4/3 (relation phase) and 3/2 (LA, rho).

## Predictions (fixed before data)
* E1: every run (IC, no-SGE, rho; 3 curves x 2 seeds x 3 arms = 18 per new prime) certifies, no skips, no
  max-trials stop. Falsified by any uncertified, skipped or capped run.
* E2: IC/rho at 524309 = 0.18 with acceptance band [0.11, 0.28]; at 1048583 = 0.17, band [0.10, 0.27]. (From R9's
  wall exponents 1.47 vs 1.53 the ratio drifts down by about 4 % per octave; the bands are wide because 6 paired runs
  with heavy-tailed rho.) Falsified if a point estimate lies outside its band.
* E3: separation does NOT hold. Predicted IC wall exponent about 1.47 (interval roughly 1.35 to 1.6) and rho about
  1.50 (1.40 to 1.65); intervals overlap. Falsified (prediction wrong) only if the IC upper end is below rho's lower
  end. A separation would be a finding; no exponent claim is made unless it holds.
* E4: relation-phase count exponent within [1.25, 1.45] (4/3 inside); LA count exponent within [1.40, 1.60] (1.5
  inside); rho count exponent within [1.4, 1.6]. Falsified if a point estimate is outside its range.

## Reporting rules
Failures reported as failures. Per-prime table: n runs, walls of both arms, ratio with interval, count exponents are
fitted over primes (not per prime). Limits: one VM, one thread, no PMU, three curves per new prime, five points,
rho runtime random, instance sample differs between old (R2 curves) and new (fresh search) primes.
Analysis code (`analyze.py`) is written after the run script is frozen and before data are read; any later change to
it is declared in RESULTS.md under Deviations.
