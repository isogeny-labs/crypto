# Rotating sweep window for m = 3 rootfind-subtract (protocol, frozen before the run)

## Question
Lead 1 of `research/ic_m3_cost_leads_20261115/LEADS.md`. The m = 3 rootfind-subtract oracle sweeps every
abscissa of the base for each trial until the first hit, and that sweep is 76–99% of the counted cost S
[pcc:unverified]. A decomposition T = P_i + P_j + P_k has three chances to be found, so a trial that examines
only a rotating window of ceil(k/N) of the k abscissae and then takes the next target should find
relations at a higher rate per counted addition. Does `sweep=1/N` lower S at the same recovered logarithm?

## What changed in the code
`RootfindSubtractOracle` (`src/cryptanalysis/ic_framework/plugins.rs`) takes `sweep=1` (default, the old
behaviour) or `sweep=1/N`. With 1/N, trial t examines abscissae `(t * w + j) mod k` for `j < w`,
`w = ceil(k/N)`, with the same batched inversion, charges and hit confirmation as before, restricted to the
window. `sweep` is part of the method identity. `rootfind-subtract-gpu` refuses it. The test
`rootfind_subtract_sweep_window` (`rootfind_gpu.rs`) checks that `sweep=1` reproduces the default report
exactly, that 1/2 and 1/4 recover the planted logarithm with verification, and that malformed values are
refused.

## Arms (spec `specs/sweep.json`)
The 37.4- and 39.3-bit prime_explicit curves, the 3 targets per curve and the target seed of
`research/ic_gpu_rootfind_la_20261115/specs/rf3la.json`, 2 rounds:
- `rf3-f1-F{2048,4096}`: `rootfind-subtract:m=3`, `prime-abscissa:size=a`, `walk-prime-batch-1024`,
  `incremental-gauss-echelon` (the baseline, identical to round 5's `rf3-cpu-echelon-F*`);
- `rf3-f{2,4,8,16}-F{2048,4096}`: the same with `sweep=1/2, 1/4, 1/8, 1/16`;
- `rho-neg` (matched reference) and `rho-neg-aa` (A/A copy).

Counts only, CPU only, L0 (`--cpus none --allow-busy`). S is deterministic per record; wall time is not
read and no wall-time claim is made. The GPU path is not part of this test.

## Predictions (from LEADS.md, frozen)
1. **Replication.** `rf3-f1-F*` gives the same S, trials and recovered logarithm per workload as round 5's
   `rf3-cpu-echelon-F*` records (`research/ic_gpu_rootfind_la_20261115/raw/out_rla.tgz`). If not, the code
   change altered the default path, and the study is void until explained.
2. **S ratio** (mean S of the arm / mean S of the f = 1 arm, per curve and a):

   | log2 r | a | 1/2 | 1/4 | 1/8 | 1/16 |
   |--:|--:|--:|--:|--:|--:|
   | 37.4 | 2048 | 0.59 | 0.46 | 0.41 | 0.38 |
   | 37.4 | 4096 | 0.71 | 0.61 | 0.57 | 0.55 |
   | 39.3 | 2048 | 0.58 | 0.44 | 0.39 | 0.36 |
   | 39.3 | 4096 | 0.61 | 0.48 | 0.43 | 0.41 |

3. **Rows unbiased.** Rows at the stop stay at about 0.917 a for every f, and at most 1.05 a.
4. **Row yield.** Relations per trial match 1 − exp(−μ(1 − (1 − f)³)) within 5%, with μ fitted on the f = 1 arm
   as −ln(1 − relations/trials) per curve and a.
5. **Band.** At w = 0.1 and w = 1 the order f < 1 below f = 1 still holds, with the ratio at w = 1 closer to
   about 0.67 than to the w = 0 ratio (skeptic estimate). The batched inversions per trial are priced at
   w > 0, and the count of inversions rises about 4× at f = 1/16 because trials rise.

## Falsification (frozen)
Lead 1 is false if any of these holds:
- at a = 4096 and f = 1/8, the mean S is above 0.75 of the f = 1 arm's on both curves;
- rows at the stop exceed 1.05 a in any record of a candidate arm;
- any recovered logarithm fails verification (that makes the run `invalid_measurement`, not a negative).

Prediction 4 missing by more than 5% does not falsify the S claim but means the window biases the
relation process; it is reported.

## Reading
Per arm and curve, the mean S, S / matched rho (reported as distance remaining, not a gate), trials,
relations, rows, inversions, and `ecbench band` at w = 0, 0.1, 1 (default and `--no-price-table-work`). An arm
is reported lower than f = 1 only where its S is lower on every workload and the order holds across the
band. Audit: `ecbench verify --replay-all` on the session.

## Not in scope
The GPU oracle, wall time, the base size optimum (Lead 2, a separate grid once this is read), and any
leaderboard or `benchmarks/README.md` row before validation.

## Sources checked
research-kb was not reachable in this container (recorded as a gap, as in the earlier rounds). Repository
notes instead: `research/ic_m3_cost_leads_20261115/LEADS.md` (Lead 1 and its skeptic review),
`research/ic_gpu_rootfind_la_20261115` (the round-5 echelon records replicated here),
`research/ic_la_m3_20261115/PROTOCOL.md` (format).
