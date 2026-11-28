# Wall-clock ratio of the 2LP+prefilter+SGE(8,3) arm to negation rho (frozen protocol, 2026-11-22)

Question: in end-to-end wall time on this host, same binary, same instances, same seeds, one thread, what is the
IC arm's time over negation rho's time? Measurement only; existing binary (hash in `binary.sha256`, HEAD f6acc4c2e4,
faster Wiedemann). Every figure `[pcc:unverified]`.

## Design
Genus 3, p in {65537, 131101, 262147}; R2's (`genus_ladder_g3_highp_20261121`) first six admitted curves per prime,
curve seeds read from R2's instance lines: 65537: 64/86/157/158/217/250; 131101: 2/14/21/41/58/61;
262147: 13/19/23/69/113/206. Command per prime (see `run_all.sh`, run strictly one after another, `uptime` into
`raw/load.log` before and after each):
`genus_ladder run --primes P --genera 3 --curves 6 --seeds 2 --arms cR --sge 8,3 --cc 2.0 --max-h 8 --threads 1
--max-trials 4e8 --rho-cap 3e9 --order-method bsgs1 --curve-seeds P=s1/../s6 --out raw/run_P.jsonl`.
Arms in the output: `C_double_lp+pre` (no SGE, not the subject), `C_double_lp+pre+sge(8,3)` (the IC arm), `R_rho_neg`.
`wall_ns` is the per-run wall time from the library; the instance search and order computation are timed separately
(`order_search_wall_s` on instance lines) and are not inside any run's `wall_ns`, so they are excluded for both arms.

## Estimand
Per prime, ratio = mean over curves of per-curve mean-over-seeds `wall_ns` of the IC arm, over the same for rho
(paired per (instance, seed)). Interval: 95 % percentile of 4000 hierarchical bootstrap draws (curves with
replacement, then seeds within drawn curve; same draw for both arms). Old-binary cross-check: the same estimator on
R2's rows for the same 6 curves (R2 ran `--threads 4`; the rows' `wall_ns` is per-run, so per-run wall is
single-threaded within the run, but four runs shared the host, so it is a loaded-host figure).
Exponents: least squares of log(mean wall) on log p over the three primes, per arm, descriptive.

## Predictions (fixed before data)
* V1: every run certifies; for each (instance, seed, arm) all count fields (everything except `wall_ns`) equal R2's row.
  Falsified by any certification failure or any differing count field.
* V2: the new-binary wall ratio IC/rho is, at each prime, within +-30 % of the count-based re-pricing of R8 (0.176,
  0.228, 0.198 at 65537, 131101, 262147), i.e. in [0.123, 0.229], [0.160, 0.296], [0.139, 0.257].
  Falsified if any point estimate is outside its band.
* V3: ratio is flat or falling from 65537 to 262147 (predicted: flat within +-15 %, since the count-based line is
  non-monotone and the rho and IC both scale with the same field arithmetic). Falsified by a rise of the point
  estimate from 65537 to 262147 by more than 15 %.
* V4: at 262147, old-binary wall ratio >= 2 x new-binary wall ratio. Falsified otherwise.

## Limits (declared)
One shared VM, one thread, no PMU, walls not isolated; 6 curves per prime; rho implementation not tuned to the
degree of the IC linear algebra (reported as ns per counted field operation for both).
