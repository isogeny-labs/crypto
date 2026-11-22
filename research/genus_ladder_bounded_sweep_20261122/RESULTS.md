# Weight-target sweep for the bounded core: results (R12, 2026-11-22) [pcc:unverified]

Protocol, predictions and scripts frozen before data (`PROTOCOL.md`, `frozen.sha256`); binary of HEAD f49709baf2 (`binary.sha256`; the
`genus_ladder` hash equals R11's). Genus 3, p = 32771, 65537, 131101, 262147 on R11's exact curves and seeds (3 curves x 2 seeds), arm c,
unmerged, plain SGE(8,3) and `--bounded W,0.1,3,1` for W in {8.5, 9, 9.5, 10}: 6 arms x 6 instances x 4 primes = 144 runs
(all 144 certify). W = 8 rows are R11's (reference only). Full tables with E, weights, la_mul, relation phase, FPM3 and ratios with
intervals: `tables.md`. Figure: `fig_fpm3_vs_w.svg`. Raw rows `run_P.jsonl`, analysis `analysis.json`, hashes `data.sha256`.
Every figure `[pcc:unverified]`.

## Answer
Yes, in this range and at these counts: a looser weight target beats plain at every prime. At every prime the lowest FPM3 (kappa = 3) is at the
edge of the grid, W = 10 (at 262147 W = 9.5 gives the same counts), where the adaptive excess is small (E = 0.1 at 32771 to 131101, 0.2 at
262147). The ratios to plain (cell means, 95% paired bootstrap) are 0.987 [0.980, 0.994], 0.979 [0.975, 0.987], 0.971 [0.964, 0.977] and
0.971 [0.968, 0.973]. The total FPM3 exponent of the best-W envelope is 1.372 [1.367, 1.375] against plain's 1.380: lower by 0.008
[0.005, 0.012] (paired), not by the 0.02 that was predicted. The minimum is at the edge, so the true optimum may lie at W > 10 (not measured).

## Per-prime best W and ratio (FPM3 kappa = 3 over plain)
| p | best W | E | FPM3 / plain [95%] | la_mul / plain | relation phase / plain | W = 8 (R11) FPM3 / plain |
|--:|--:|--:|--|--:|--:|--:|
| 32771 | 10 | 0.10 | 0.987 [0.980, 0.994] | 0.728 | 1.100 | 1.027 |
| 65537 | 10 | 0.10 | 0.979 [0.975, 0.987] | 0.738 | 1.100 | 1.054 |
| 131101 | 10 | 0.10 | 0.971 [0.964, 0.977] | 0.753 | 1.099 | 1.071 |
| 262147 | 10 (= 9.5) | 0.20 | 0.971 [0.968, 0.973] | 0.639 | 1.201 | 1.082 |

Mean `bd_extra` E per W (probes in tables.md): W = 8.5: 0.1, 0.2, 0.3, 0.4; W = 9: 0.1, 0.1, 0.2, 0.3; W = 9.5: 0.1, 0.1, 0.18, 0.2; W = 10: 0.1, 0.1, 0.1, 0.2
(32771 to 262147). No run hit EMAX. E cannot go below STEP = 0.1: at the small primes several W give identical counts (the first step already
meets W), so those W are one point.

## Exponents over the four primes (least squares in log-log; 95% bootstrap; `re` = W re-selected in every draw; `fx` gives the same intervals here)
| arm | FPM3 | relation phase | la_mul | FPM3 minus plain's (paired) |
|---|--|--|--|--|
| plain SGE(8,3) | 1.380 [1.377, 1.383] | 1.300 [1.297, 1.304] | 1.524 [1.516, 1.533] | - |
| fixed W = 8.5 | 1.405 [1.400, 1.408] | 1.416 | 1.362 | +0.025 [0.021, 0.028] |
| fixed W = 9 | 1.387 [1.383, 1.390] | 1.385 | 1.393 | +0.007 [0.003, 0.010] |
| fixed W = 9.5 | 1.374 [1.370, 1.378] | 1.349 | 1.452 | -0.006 [-0.009, -0.002] |
| fixed W = 10 | 1.372 [1.367, 1.375] | 1.338 | 1.471 | -0.008 [-0.012, -0.005] |
| best-W envelope | 1.372 [1.367, 1.375] | 1.338 [1.335, 1.341] | 1.471 [1.450, 1.488] | -0.008 [-0.012, -0.005] |
| constant-excess arm (W 8.5, 8.5, 9, 9.5; E 0.1, 0.2, 0.2, 0.2) | 1.370 [1.365, 1.373] | 1.338 [1.336, 1.342] | 1.467 [1.447, 1.484] | -0.010 [-0.014, -0.007] |

The envelope equals fixed W = 10 (it picks W = 10 at every prime), so choosing W per prime added nothing beyond one fixed W here. The
constant-excess arm is chosen by E nearest 0.2 on a 0.1 grid; its per-prime ratios are 0.987, 1.014, 0.993, 0.971 (a mixed bag: at 65537
E = 0.2 costs more than E = 0.1 saves).
The trade seen in R11 persists: lowering the weight target buys la_mul exponent (1.36 at W = 8.5 to 1.47 at W = 10 against plain 1.52) at the price of
relation-phase exponent (1.42 to 1.34 against 1.30); the total is flat to within about 0.03, and W = 9.5 to 10 is where it first dips below plain.

## Scorecard
| id | result |
|---|---|
| S1 | **holds**: all 144 runs certified; the 48 plain and unmerged rows equal R11's in every field but wall time (0 mismatches), instance lines equal |
| S2 | **holds**: minimum ratio over W 0.987 [0.980, 0.994], 0.979, 0.971, 0.971 at the four primes (each <= 0.99; at 32771 the margin is 0.003 and the upper end 0.994 exceeds 0.99). Expected best W was 10, 10, 9.5, 9: W = 10 at all four (9.5 ties at 262147); the expectation that S2 fails at 32771 was wrong by a hair |
| S3 | **fails**: envelope exponent 1.372 > limit 1.360; paired difference -0.008 [-0.012, -0.005] (upper end below 0, so the direction holds, the size does not) |
| S4 | **fails** on the relation phase: 1.338 [1.336, 1.342] > 1.32; LA exponent 1.467 [1.447, 1.484] <= 1.47 holds (the upper end 1.484 exceeds it) |

## Limits and deviations
- The best W is the edge of the grid at every prime: the sweep does not locate the optimum; W > 10 (smaller E is impossible at STEP 0.1, so W
  > 10 can only matter at 262147, where E = 0.2) was not run. The pre-registered grid and the W = 8 reference are the only points.
- W was chosen on the same data the ratios and exponents are computed on: the envelope is optimistic (winner's curse). The bootstrap re-selects W per
  draw (`re`), which includes selection noise, but it cannot remove the optimism of picking the best of four on 6 runs per cell; here the best arm
  won in 100 % of the draws at every prime and also equals fixed W = 10, so the optimism is probably small, but it is not measured.
- Ratios below 1 are of 1 to 3 %: counts on 3 curves per prime (6 runs per cell); the intervals cover that sample only. FPM3 uses the
  in-situ kappa = 3; at kappa = 1 the bounded arms are worse (the relation excess dominates), at kappa = 9 better (R11). Counts, not wall time; probe
  and selection additions are not priced (la_sel_add 0.05 to 2 million against 1e7 to 1e9 multiplications and 1e8 to 1e9 additions).
- Shared 4-vCPU host, no isolated run, no replay certificate, no `VALIDATION.json`, no matched-rho band: not a leaderboard or `benchmarks/`
  row and no claim of improvement over the index-calculus baseline beyond these counts. W = 8 reference rows come from R11's data (same binary hash,
  same flags), not rerun.
- No deviation from the frozen protocol; the analysis scripts were tested on a synthetic file derived from R11 before the run (no new data) and ran unchanged.
- The CLAUDE.md session log, the Conductor task and git were not touched (this agent was confined to the owned directory and forbidden git).
