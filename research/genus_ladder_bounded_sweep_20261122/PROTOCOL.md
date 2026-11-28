# Weight-target sweep for the bounded core (R12; frozen protocol, 2026-11-22) [pcc:unverified]

Question. R11 held the selected core's mean row weight at W = 8 and the relation excess E had to grow with p
(0.2, 0.3, 0.4, 0.5), which lifted the relation-phase exponent 1.30 -> 1.41; LA exponent fell 1.52 -> 1.37-1.39; total FPM at
kappa = 3 was 1.03-1.10 of plain, FPM3 exponent 1.405 against plain 1.380. Is there a weight target per prime that
beats plain SGE(8,3) in total FPM at kappa = 3, and with it a lower TOTAL exponent? Measurement only; no source change
(binary of HEAD f49709baf2, hashes in `binary.sha256`; the genus_ladder hash equals R11's). Every figure `[pcc:unverified]`.

## Run (`run_all.sh`, frozen)
Genus 3, p = 32771, 65537, 131101, 262147; R11's exact curves (`--curve-seeds` 8/16/31, 64/86/157, 2/14/21, 13/19/23),
`--curves 3 --seeds 2`, arm c, one invocation per prime. Arms per instance: unmerged, plain SGE(8,3) (identity check
against R11), and `--bounded W,0.1,3,1` (STEP 0.1, EMAX 3, cover 1) for W in {8.5, 9, 9.5, 10}. R11's W = 8 rows
(same binary, same flags) are reused as the W = 8 point of the curve in the figure and tables, labelled "from R11";
they are not part of the best-W choice, which is over the four new W only (an envelope with plain and W = 8 included is
reported as a secondary line). Arm name format `C_double_lp+pre+sge(8,3)+bd(W,0.1,3,1)`.

## Estimands (analysis.py, frozen)
Cell mean = mean over curves of the mean over seeds. FPM3 = cantor_mul + 10 cantor_inv + rel_mul + 3 la_mul; relation
phase = cantor_mul + 10 cantor_inv + rel_mul. Exponent = least squares of log cell mean on log p over the four primes.
Intervals: 95% percentile of 4000 hierarchical bootstrap draws (curves with replacement, seeds within the chosen curve,
the same index draw for every arm; seed 20261122).
- Best W at a prime: lowest cell-mean FPM3 among the four new W. Envelope = per-prime best-W FPM3; its exponent over p.
- Constant-excess arm: at each prime the W (of the four new) whose cell-mean `bd_extra` is closest to 0.2 (tie: the smaller
  W); its relation-phase, LA and FPM3 exponents.
- Two interval variants for both: "re-selected" (the best W / constant-excess W is chosen again inside every bootstrap
  draw; includes selection noise) and "fixed" (selection by the point estimates held fixed). The scorecard uses
  "re-selected". Both choose W on the same data they are evaluated on, so the envelope is optimistic (winner's curse).
- Ratio to plain: ratio of cell means with the paired bootstrap interval.

## Predictions (fixed before data)
| id | prediction | falsified if |
|---|---|---|
| S1 | plain and unmerged rows equal R11's rows (counts, every field but wall time); all runs certify | any mismatch or uncertified run |
| S2 | at every p some W has FPM3 <= 0.99 of plain (cell means) | at any p the minimum ratio over the four W > 0.99 |
| S3 | FPM3 exponent of the best-W envelope <= 1.380 - 0.02 = 1.360, and the paired bootstrap interval of (envelope exponent - plain exponent) has its upper end < 0 | point > 1.360 or upper end >= 0 |
| S4 | constant-excess arm: relation-phase exponent <= 1.32 and LA exponent <= 1.47 | either fails |

Expected best W (author's expectation, uncertain): 10 at 32771 and 65537 (smallest E), 9.5 at 131101, 9 at 262147; I
doubt S2 at 32771 (E = 0.1 costs a tenth of a relation phase that is 0.70 of FPM3 there, LA can save at most 0.4 of the
rest), so the S2 and S3 predictions are stated with low confidence. Basis: R11 numbers only (plain, R4 ex(0.3), bd(8)); no
new data seen.

## Limits (declared)
3 curves per prime (6 runs per arm per prime); counts, not wall time; shared 4-vCPU host; selection of W on the same
data (optimistic); adaptive E uses the instance's own relation count (a data-dependent stopping rule, charged; probe
work counted in `la`); STEP 0.1 means E is a multiple of 0.1, so "closest to 0.2" is coarse; no isolated host, no replay,
no `VALIDATION.json`, no leaderboard or `benchmarks/` row (a measurement for the frontier note, not a claim).
