# Does the genus-3 arm's exponent separate from rho's at p = 2^19 to 2^20? No. `[pcc:unverified]`

Protocol and run script frozen before data (`PROTOCOL.md`, `run_all.sh`, `frozen.sha256`). Binary: copy in scratchpad
`r10/` of the build of HEAD f6acc4c2e4 (`dirty_src: []`, with the faster Wiedemann); sha256 of `genus_ladder` f38dd2ca...
and `genus_ladder_analyze` 943d7f65... as in `binary.sha256`; equal to the hashes the task gave for the verified `r9/`
copies and to R9's `binary.sha256`. (The marker file `BUILD_DONE4` itself records the commit and a clean tree, not the
hashes.) Genus 3, p = 524309 and 1048583, fresh curves (first three admitted), `--curves 3 --seeds 2 --arms cR --sge 8,3
--cc 2.0 --max-h 8 --threads 1 --max-trials 1e9 --rho-cap 3e10 --seed-search 600 --order-method bsgs1`, one prime at a
time, smaller first, load 1.00 before and after each invocation (`raw/load.log`, append-only, intact). Joined with R9's
rows at 65537, 131101, 262147 (same binary, 6 curves x 2 seeds) for a five-prime fit. Tables: `tables.md`; analysis:
`analysis.json`; figure `fig_exponent.svg`.

## Per-prime table

| p | curves x seeds | mean wall IC+SGE (s) | mean wall rho (s) | **IC / rho** | 95 % interval |
|--:|--:|--:|--:|--:|:--|
| 65537 | 6 x 2 (R9) | 2.89 | 14.28 | 0.202 | [0.159, 0.271] |
| 131101 | 6 x 2 (R9) | 8.48 | 32.51 | 0.261 | [0.220, 0.326] |
| 262147 | 6 x 2 (R9) | 22.19 | 118.76 | 0.187 | [0.122, 0.324] |
| **524309** | 3 x 2 (new) | 56.77 | 480.20 | **0.118** | [0.055, 0.487] |
| **1048583** | 3 x 2 (new) | 150.07 | 817.16 | **0.184** | [0.107, 0.300] |

All 18 runs per new prime certified (IC, no-SGE, rho), no skips. The IC arm is stable across curves (56.3-57.0 s at
524309; 144.7-153.5 s at 1048583). Rho is not: per-curve means 117 / 283 / 1040 s at 524309 and 532 / 568 / 1351 s at
1048583, so the per-curve ratio runs 0.055 to 0.487 and 0.107 to 0.289 (T4 in `tables.md`). Mean rho walk steps:
8.1M, 15.9M, 60.1M, 260M, 438M. Without SGE the arm is 0.141 (524309) and 0.208 (1048583) of rho. (Re-run bootstrap
intervals at the old primes differ from R9's in the third digit: different RNG stream.)

## Exponents over the five primes (log-log least squares on per-prime means, 95 % bootstrap over curves then seeds)

| quantity | exponent | 95 % interval | reference |
|--|--:|:--|--|
| wall, IC + SGE(8,3) | 1.414 | [1.394, 1.433] | |
| wall, negation rho | 1.556 | [1.277, 1.764] | |
| wall, IC without SGE | 1.353 | [1.328, 1.379] | |
| count, IC relation phase (`cantor_mul + 10 cantor_inv + rel_mul`) | 1.308 | [1.307, 1.309] | 4/3 |
| count, IC linear algebra (`la_mul`) | 1.496 | [1.492, 1.501] | 3/2 |
| count, rho (`cantor_mul + 10 cantor_inv`) | 1.555 | [1.271, 1.764] | 3/2 |
| wall exponent difference IC - rho (paired draws, secondary) | -0.142 | [-0.355, 0.141] | 0 |

Three-prime (R9) point exponents were 1.47 (IC) and 1.53 (rho); with the two new primes the IC point exponent falls to
1.41 and rho's rises to 1.56, but the rho interval is 0.49 wide.

**Separation test (E3): IC upper end 1.433 is not below rho lower end 1.277. It does not hold.** The paired difference
interval contains 0. No exponent claim is made. What the data do show: the IC arm's wall exponent is pinned to about
1.41 +- 0.02 (its counts barely vary between curves), the point estimates differ by 0.14 in the arm's favour, and rho's
exponent is too uncertain at 3 curves x 2 seeds at the new primes to say whether the true gap is 0 or 0.3.

Count exponents: the relation phase grows as p^1.31, close to but 0.025 below 4/3 (the bootstrap interval is narrow
because the counts vary little, it excludes 4/3; it covers sampling over curves and seeds only, not the choice of
factor-base size, which follows `--cc 2.0` and changes in steps). The linear algebra grows as p^1.496, matching 3/2
(interval [1.492, 1.501] contains it). So the LA count scales near p^1.5 and the relation phase near p^1.3; the arm's
wall exponent (1.41) sits between them because the LA share of wall rises with p (about 0.30 to 0.36 at the three old
primes, R9) and per-count wall cost falls slightly (IC wall per `la_mul` 55 / 57 / 52 / 48 / 45 ns).

## Scorecard

| id | prediction | outcome | verdict |
|---|---|---|---|
| E1 | all runs certify, no skip or cap | 36 of 36 runs at the two new primes certified, 0 skips | **pass** |
| E2 | IC/rho 0.18 in [0.11, 0.28] at 524309; 0.17 in [0.10, 0.27] at 1048583 | 0.118 (inside, 0.008 from the lower edge; one curve with a 1040 s rho mean carries it, interval [0.055, 0.487]); 0.184 | **pass** (both, 524309 marginally) |
| E3 | separation does not hold | IC [1.394, 1.433] vs rho [1.277, 1.764]: overlap | **prediction correct; the separation test fails** |
| E4 | relation exponent in [1.25, 1.45]; LA in [1.40, 1.60]; rho in [1.4, 1.6] | 1.308; 1.496; 1.555 | **pass** (all three; 4/3 excluded by the narrow interval, 3/2 contained) |

## Post hoc diagnostic (not in the frozen protocol; `posthoc.py`, `posthoc.json`)

Rho's runtime is random; replacing each curve's realised walk length by its expectation sqrt(pi r / 4) (r from the
instance line) times the prime's measured ns per rho step gives a lower-variance rho reference: IC wall over expected
rho wall is 0.216 / 0.207 / 0.178 / 0.148 / 0.131 at the five primes, falling monotonically, and the expected-rho wall
exponent is 1.61 [1.41, 1.75] (bootstrap over curves). The IC interval [1.394, 1.433] still overlaps it (rho lower 1.415).
Caveat: the expected walk length depends on the curve's cofactor h (1 to 8 in these samples, sqrt(r) scales as
1/sqrt(h)) which the IC arm barely sees, and the new primes have three curves each (h = 4, 1, 8 and 3, 8, 1), so the
fitted slope is sensitive to which h values were drawn. This is context for what a larger sample might show, not a result.

## Limits

One shared VM, one thread, no PMU, walls not isolated from the hypervisor (load 1.00, this process only); three curves
x two seeds at each new prime (six paired runs) against six curves at the old primes; five points over a factor of 16
in p; rho runtime random and curve-dependent; the old and new primes use different instance samples (R2's curves vs a
fresh search); intervals cover curves and seeds, not cofactor, factor-base or host effects; no independent replay; the
IC relation-phase wall is not recorded separately. All figures `[pcc:unverified]`.

## Deviations

* The instance search at 524309 took 1025 s (three curves, mostly the order search; per-curve order search 11.4-12.0 s
  at 524309, 24.0-25.1 s at 1048583, matching the earlier measurement), outside every run's `wall_ns`. Total wall:
  77 min at 524309, 160 min at 1048583.
* `--seeds 2` kept at 1048583 (protocol budget decision made before data: rho alone about 95 min, under 2.5 h; the six rho runs summed to 82 min by mean wall 817 s x 6 = 82 min).
* `analyze.py` was written after `run_all.sh` was frozen and before data were read and was not changed afterwards;
  `posthoc.py` was added after the data and is labelled post hoc. No session log was written (outside the owned path).
* A background watcher I started was stopped by its time limit; the experiment itself was not affected
  (`raw/load.log` shows both invocations exit 0).

## Files
`PROTOCOL.md`, `frozen.sha256`, `run_all.sh`, `raw/run_524309.jsonl`, `raw/run_1048583.jsonl`, `raw/load.log`,
`analyze.py`, `analysis.json`, `tables.md`, `fig_exponent.svg`, `posthoc.py`, `posthoc.json`, `data.sha256`,
`binary.sha256`. Old-prime rows are read from `../genus_ladder_wallclock_vs_rho_20261122/raw/`.
