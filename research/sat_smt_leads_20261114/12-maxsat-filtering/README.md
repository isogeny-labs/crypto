# Idea 12 pilot: MaxSAT for relation filtering (2026-11-14)

Pilot, toy sizes, one core, not claim data. Every number below is an unverified pilot assertion `[pcc:unverified]`.
Protocol (written before the measured run, with one recorded deviation D1): `PROTOCOL.md`.

## What was run

- Instances: matched RANDOM model, not real decompositions: R = ceil((1+e)N) rows, each a uniform random
  4-subset of N columns (m = 3, m+1 nonzeros), N = 2^5 .. 2^12, e in {5%, 20%, 100%}, seeds in `results.json`.
  66 instances with all arms (N <= 1024), 6 baseline-only instances (N = 2048, 4096).
- Problem (SEL): choose rows S with `|S| - |cols(S)| >= 1` (guarantees a left-kernel vector), minimise `|cols(S)|`;
  then the same greedy Markowitz SGE (density stop: average row weight <= 16) for every arm.
- Baselines: G0 singleton removal; G1 greedy excess removal with clique-style score; G2 the same with
  singleton-cascade lookahead; G3 randomised restarts of G2 for 5 s (equal wall budget to each MaxSAT arm).
  "Greedy pipeline" = G3 selection (it starts from min(G1, G2)).
- Treatment: RC2 (pysat, exact, 5 s) and ANY (Glucose4 model-improving search warm-started from the greedy
  solution, 5 s), both on hard clauses `x_r -> y_c` plus one cardinality network for the excess constraint.
- Validation (`validation.json`): 20/20 tiny instances (N = 10..12) where RC2 = ANY = brute-force optimum;
  planted dense block (12 rows in 9 columns inside a 60-column matrix): RC2 = ANY = 7 columns, greedy G2 = 31
  (positive control: the MaxSAT arms do see a dense sub-structure when one exists). Every arm's output on every
  instance passed the independent verifier (rows inside columns, excess >= 1).
- Commands: `python3 filt.py validate validation.json`; `python3 filt.py run results_raw.json`; `python3 analyze.py`.

## Results [pcc:unverified]

Columns are means over reps. "ANY OPT / improved" = instances where ANY proved optimality / found a selection
with fewer columns than the greedy pipeline. Ratios are best MaxSAT arm divided by greedy pipeline (1.0 = no change).

| N | excess | reps | G0 cols | G1 cols | G2 cols | G3 cols (restarts) | RC2 OPT / cols | ANY OPT / improved / cols | best MaxSAT/greedy cols | cost_sel ratio | cost_sge ratio (gm) | RC2 conflicts (OPT) | ANY conflicts |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 32 | 5% | 6 | 30.0 | 26.8 | 26.8 | 26.8 (40426) | 0/6 / - | 6/6 / 0 / 26.8 | 1.0000 | 1.0000 | 1.0000 [1.000,1.000] | - | 36160 |
| 32 | 20% | 6 | 31.0 | 24.8 | 24.7 | 24.7 (14128) | 0/6 / - | 5/6 / 0 / 24.7 | 1.0000 | 1.0000 | 1.0000 [1.000,1.000] | - | 77600 |
| 32 | 100% | 6 | 32.0 | 18.0 | 18.0 | 17.7 (2610) | 0/6 / - | 0/6 / 0 / 17.7 | 1.0000 | 1.0000 | 1.0000 [1.000,1.000] | - | 87792 |
| 64 | 5% | 4 | 59.0 | 53.0 | 53.0 | 53.0 (13219) | 0/4 / - | 0/4 / 0 / 53.0 | 1.0000 | 1.0000 | 1.0000 [1.000,1.000] | - | 86094 |
| 64 | 20% | 4 | 61.0 | 44.2 | 44.2 | 44.2 (4096) | 0/4 / - | 0/4 / 0 / 44.2 | 1.0000 | 1.0000 | 1.0000 [1.000,1.000] | - | 71128 |
| 64 | 100% | 4 | 64.0 | 32.8 | 32.8 | 31.2 (756) | 0/4 / - | 0/4 / 0 / 31.2 | 1.0000 | 1.0000 | 1.0000 [1.000,1.000] | - | 64051 |
| 128 | 5% | 4 | 120.8 | 106.5 | 106.2 | 105.5 (3741) | 0/4 / - | 0/4 / 0 / 105.5 | 1.0000 | 1.0000 | 1.0111 [1.000,1.045] | - | 63725 |
| 128 | 20% | 4 | 122.2 | 89.0 | 89.0 | 87.2 (1290) | 0/4 / - | 0/4 / 0 / 87.2 | 1.0000 | 1.0000 | 1.0284 [1.000,1.059] | - | 50791 |
| 128 | 100% | 4 | 127.8 | 58.8 | 58.8 | 56.8 (207) | 0/4 / - | 0/4 / 0 / 56.8 | 1.0000 | 1.0000 | 1.0000 [1.000,1.000] | - | 42510 |
| 256 | 5% | 3 | 232.0 | 197.7 | 197.3 | 195.3 (832) | 0/3 / - | 0/3 / 0 / 195.3 | 1.0000 | 1.0000 | 1.0218 [1.000,1.035] | - | 35697 |
| 256 | 20% | 3 | 246.3 | 174.0 | 173.7 | 171.3 (308) | 0/3 / - | 0/3 / 0 / 171.3 | 1.0000 | 1.0000 | 1.0319 [1.000,1.075] | - | 31967 |
| 256 | 100% | 3 | 255.7 | 115.7 | 116.3 | 111.7 (44) | 0/3 / - | 0/3 / 0 / 111.7 | 1.0000 | 1.0000 | 1.0073 [1.000,1.016] | - | 26699 |
| 512 | 5% | 3 | 463.7 | 385.3 | 385.0 | 382.7 (226) | 0/3 / - | 0/3 / 0 / 382.7 | 1.0000 | 1.0000 | 1.0127 [1.000,1.039] | - | 21597 |
| 512 | 20% | 3 | 480.0 | 333.0 | 332.7 | 328.7 (73) | 0/3 / - | 0/3 / 0 / 328.7 | 1.0000 | 1.0000 | 1.0182 [1.000,1.050] | - | 18486 |
| 512 | 100% | 3 | 510.7 | 226.3 | 227.3 | 222.3 (12) | 0/3 / - | 0/3 / 0 / 222.3 | 1.0000 | 1.0000 | 1.0000 [1.000,1.000] | - | 15261 |
| 1024 | 5% | 2 | 917.0 | 761.0 | 760.5 | 758.0 (62) | 0/2 / - | 0/2 / 0 / 758.0 | 1.0000 | 1.0000 | 1.0000 [1.000,1.000] | - | 10550 |
| 1024 | 20% | 2 | 966.0 | 673.0 | 673.0 | 671.5 (18) | 0/2 / - | 0/2 / 0 / 671.5 | 1.0000 | 1.0000 | 1.0000 [1.000,1.000] | - | 10341 |
| 1024 | 100% | 2 | 1019.5 | 446.5 | 446.5 | 442.5 (2) | 0/2 / - | 0/2 / 0 / 442.5 | 1.0000 | 1.0000 | 1.0031 [1.000,1.006] | - | 6900 |
| 2048 | 5% | 1 | 1844.0 | 1548.0 | 1547.0 | not run | not run | not run | - | - | - | - | - |
| 2048 | 20% | 1 | 1941.0 | 1321.0 | 1321.0 | not run | not run | not run | - | - | - | - | - |
| 2048 | 100% | 1 | 2042.0 | 896.0 | 898.0 | not run | not run | not run | - | - | - | - | - |
| 4096 | 5% | 1 | 3715.0 | 3094.0 | 3092.0 | not run | not run | not run | - | - | - | - | - |
| 4096 | 20% | 1 | 3883.0 | 2674.0 | 2674.0 | not run | not run | not run | - | - | - | - | - |
| 4096 | 100% | 1 | 4087.0 | 1811.0 | 1818.0 | not run | not run | not run | - | - | - | - | - |

rc2_log2conf_per_log2N_e5 slope = None n_points = 0
g2_log2cols_per_log2N_e5 slope = 0.9733090084633531 n_points = 8
rc2_log2conf_per_log2N_e20 slope = None n_points = 0
g2_log2cols_per_log2N_e20 slope = 0.9704171480489117 n_points = 8
rc2_log2conf_per_log2N_e100 slope = None n_points = 0
g2_log2cols_per_log2N_e100 slope = 0.9552115059971933 n_points = 8

Reading the table [pcc:unverified]:

- Selection: on 66/66 instances neither MaxSAT arm returned fewer columns than the G3 greedy pipeline
  (ratio 1.0000 for columns and cost_sel in every cell).
- Exact frontier: RC2 proved optimality on 0/66 measured instances within 5 s (it did on the N <= 12 validation
  instances). ANY proved optimality on 11/12 instances at N = 32 with 5% or 20% excess (mean 36k and 78k conflicts),
  0/6 at N = 32 with 100% excess, and 0/48 for N >= 64. In all 11 proven cases the G3 greedy selection was already optimal.
- cost_sge ratios above 1 (up to 1.075 on single instances) are not a MaxSAT effect: ANY returned exactly the G3
  selection there, and the denominator takes the minimum SGE cost over the G1, G2 and G3 selections. It shows the
  SGE stage has a few percent of noise between near-identical selections, so column count is the cleaner metric.
  At N = 32 and 64 cost_sge is degenerate (the matrix eliminates completely under the density stop).
- Baseline scale (G2, N = 2^5..2^12): kept columns grow as N^0.97, N^0.97, N^0.96 at 5%, 20%, 100% excess;
  at N = 4096 G1/G2 keep 3094/3092, 2674/2674, 1811/1818 columns. G3 restarts gained at most about 5% of columns
  over G1/G2 (N = 64, 100% excess: 32.8 -> 31.2) and usually under 2%.
- SAT units in 5 s: ANY 6.9k (N = 1024) to 88k (N = 32) conflicts per instance; no improving model found anywhere.

## Verdict against the pre-written kill criterion

Kill criterion met, both branches [pcc:unverified]:
(a) measured cost change of the best MaxSAT arm against the greedy pipeline (G3) is 0% in cost_sel at every size
    (threshold 5%); (b) exact solving stops at N = 32 (5-20% excess) and below N = 32 at 100% excess, far before
    2^10, with no anytime gain at any size.
The protocol's prediction (no small dense sub-structures in the uniform model, greedy near-optimal) is consistent
with the 11 proven optima; it is not proven for N >= 64, where the optimum is unknown and only "no model found in
5 s" is measured.

## Not covered / what a full study would need

- Real relation matrices from toy decompositions (only the uniform random model was run). Structure that breaks
  uniformity (large-prime variation, Frobenius/negation orbits, skewed column weights) is where the planted control
  says MaxSAT could matter; that was not tested.
- Merges / fill-in inside the MaxSAT model (only subset selection was modelled; SGE was the same greedy for all arms).
- 50% excess, z3 Optimize, longer budgets than 5 s per arm, stronger encodings (native PB, totalizer variants),
  CP-SAT/ILP with LP bounds, k > 1.
- Rep counts are 2-6 per cell (below 8 for N >= 64); N = 2048 and 4096 have baseline arms only, one seed.
- No slope for exact MaxSAT could be fitted: optimality was proven at one size only (N = 32).

## Files

`PROTOCOL.md`, `filt.py`, `analyze.py`, `validation.json`, `results_raw.json`, `results.json` (per-instance rows,
cell summary, validation), `table.md`, `run.log`, `SESSION_LOG_ENTRY.md`.
