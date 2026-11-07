# RESULTS E5-kummer-multiplicative-walk (Kummer-line x-only multiplicative walk)

Directory: /tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/twist_ecdlp/E5-kummer-multiplicative-walk
Every number below is copied from out/summary.txt (analysis v2), out/summary.csv, out/costs.csv, out/results.json or a file in raw/.
Intervals are 95% percentile bootstrap (2000 resamples, setrand(31337), trials resampled within each cell). gp 2.15.4. Reproduce with run.sh.
Scope: scratchpad study; GP implementation; Montgomery curves only; b = 20..40 bits; 2 instances per size. Counted field operations are decisive, GP wall time is secondary.
Preregistration was followed; all deviations/additions are in DEVIATIONS.md (items 1-17). Index calculus is out of scope; the distance to matched rho is reported as a ratio, not as a gate.

## Headline
The Kummer-line multiplicative walk is NOT an improvement over matched additive rho (A0) in counted work. It needs about the same number of steps as A0
(K-pure/A0 = 1.058 [0.959, 1.170]; K-mix/A0 = 0.787 [0.709, 0.872], b >= 28 pooled) but each step costs 21.5 M + 12 S + 1 I against 2 M + 1 S + 1 I,
so total counted cost is above A0 at I/M = 3, 10, 30 for both arms (K-pure also at 100; K-mix/A0 at I/M = 100 is 1.003 [0.904, 1.113], unranked).
The twist gives no cross-curve information (0 cross-curve collisions in 144 trials; step map never crosses the twist class). The preregistered A1 reference
(negation map, only 2-cycles removed) is not a usable baseline for b >= 28 (it solves 114/120 at b28, 9/80 at b32, 0/48 at b36), so every K/A1 ratio is an artefact of A1 failing and is not claimed.
Pre-registered prediction H5 (no improvement) held. Not every sub-prediction held (F1 and F2 formally triggered, see below).

## Positive controls (run first)
| control | result | source |
|---|---|---|
| ladder vs ellmul, E and E' (b=20,24,28, 2 inst, x on E 6054, x on E' 5946, k=2..53) | 624000 tests, 0 mismatches | raw/identity.txt |
| PC1 exhaustive b=10 (n=829), SB set: x-orbit intersection iff a in +-H, all 828 a | 0 mismatches; closure vs znlog 0 mismatches | raw/exhaustive.txt |
| PC1 walks, SB, 400 in-set a, budget 200 sqrt(N)=4069 steps | 400/400 verified (100%); 0/0 out-of-set a (none exist) | raw/exhaustive.txt |
| PC1 S_idx (index 3, N=138): orbit test | 0 mismatches | raw/exhaustive.txt |
| PC1 S_idx walks, 274 in-set a, budget 2349 steps | 165/274 = 60% verified, below the 98% bar (F1 clause); 100 out-of-set a: 0 recovered | raw/exhaustive.txt |
| PC2 orbit size, 20 random P at b=16 x {SA,SB,SC} | 60/60 equal N (ellmul orbit = ladder BFS = N), 0 mismatches | raw/exhaustive.txt |
| PC3 pc1 cell b=24 (K-pure, a in +-H, S_idx, index 3) | inst 1: 40/40, supplementary inst 4: 40/40 (inst 2 degenerate N=2, skipped) | raw/cell_pc1_b24_i1.txt, i4 |
| PC4 A0 mean T ratio b+4 / b (expected 4, band [3.0,5.3]) | 24->28 4.575 [4.084, 5.122]; 28->32 3.460 [3.036, 3.947] (both inside); 32->36 3.843; 36->40 4.547 | out/summary.txt |
| PC5 counters = histogram recomputation | 4152 trials, 0 mismatches (1 successful A-arm trial had one dead-end step counted in T without ops) | out/summary.txt |
PC1 S_idx diagnosis (from the same chk line, not a new experiment): 4027 trails, 1147 reached a DP, 457232 steps; with N=138 and d=3 most trails end in DP-free cycles and are cut at the cap, so the budget is wasted. This is a property of the DP parameters at tiny N, not of the orbit structure (orbit test 0 mismatches).

## Negative controls
- NC1 (K-pure, a not in +-H, 3 cells, 80 trials): 0 cross-side x matches, 0 verified solves (deterministic prediction held). raw/cell_nc1_*.txt
- NC2 (P on E, Q' on E'; b=20,24,28 x 2 inst, 144 trials, budget 30 sqrt(N)): 0 x matches. raw/cell_cross_*.txt. Exhaustive class test on 3 fields (p=4643, 6221, 6101; 37120, 49678, 48762 step evaluations): 0 class crossings, 0 mixed components.
- NC3 (A0 normalised mean T, inst1/inst2): ratio CI contains 1 at 6 of 6 bit lengths (b20 0.990 [0.859, 1.157] ... b40 1.177 [0.787, 1.758]).
- NC4 (wrong a+1 accepted by verification): 0 of 24000 (12 instances x E, E' x 1000); true a verified 24000/24000.

## Primary measurements (K arms SB, E; all K trials solved, 100% success in every K cell)
| quantity | value | n |
|---|---|---|
| R = mean T/sqrt(N), K-pure, b>=24 pooled | 2.029 [1.909, 2.155] (b>=28: 2.049 [1.904, 2.206]) | 10 cells |
| R, K-mix, b>=24 | 1.512 [1.414, 1.609] | 10 cells |
| R, A0, b>=24 | 1.357 [1.282, 1.436] | 10 cells |
| R, K-pure on twist E', b>=24 | 2.069 [1.935, 2.203]; ratio to E (b>=28) 1.002 [0.896, 1.114] | 10 cells |
| slope of log2 mean T vs b: K-pure / K-mix / A0 / K-pure twist | 0.494 [0.482, 0.505] / 0.495 [0.480, 0.506] / 0.496 [0.484, 0.508] / 0.493 [0.479, 0.505] | 12 cells each |
| step ratio K-pure / A0 (b>=28) | 1.058 [0.959, 1.170] | 8 cells |
| step ratio K-mix / A0 (b>=28) | 0.787 [0.709, 0.872] | 8 cells |
| step ratio K-pure / K-mix (b>=28) | 1.345 [1.209, 1.504] | 8 cells |
| measured K-pure step cost (counters) | 21.505 M, 12.003 S, 1.000 I per step (pred. 21.5 / 12 / 1; deviation 0.023%, 0.024%) | all K-pure trials |
| ideal Lucas chain (chainsearch, SB, S=0.8M) | 4.0 differential ops/step, 22.025 M-equiv excl. I (ladder analytic: 6.0 ops, 31.1) | raw/chains.txt |
| GP wall time per step (secondary) | K-pure about 12.1-14.2 us, A0 about 2.8-5.2 us | summary.csv |

Ablations (b=28 inst 1, 60 trials): R K-pure SB 2.057 [1.791, 2.349], SA 2.401 [2.104, 2.700], SC 2.040 [1.766, 2.330]; K-mix SB 1.505 [1.320, 1.688], SA 1.658 [1.455, 1.862], SC 1.479 [1.302, 1.666].
Proper-H (index 3): kmixidx R 2.838 [2.409, 3.285] (inst 1), 2.248 [1.903, 2.586] (inst 4), 40/40 solved; pc1 R 1.621 and 2.282.

## Cost ratios Kummer / A0, b >= 28 pooled, counted cost = M + 0.8 S + (I/M) I
| I/M | K-pure / A0 | K-mix / A0 | ordering |
|---|---|---|---|
| 3 | 6.221 [5.634, 6.875] | 4.625 [4.166, 5.130] | K dearer |
| 10 | 3.397 [3.077, 3.755] | 2.526 [2.275, 2.802] | K dearer |
| 30 | 1.971 [1.785, 2.178] | 1.466 [1.320, 1.625] | K dearer |
| 100 | 1.349 [1.222, 1.491] | 1.003 [0.904, 1.113] | K-pure dearer; K-mix unresolved (K-mix/A0 unranked across the band) |
S = 1.0 M variant at I/M = 3: K-pure 6.437, K-mix 4.786 (summary.txt). Sensitivity with xADD at 3 M (not preregistered, labelled): K-pure/A0 5.765 at I/M = 3, K-mix/A0 4.286 and 0.984 [0.887, 1.091] at 100. Gap to matched rho A0 at I/M = 3: 4.6x (K-mix) and 6.2x (K-pure).
Per b at I/M = 3 (b32/b36/b40): K-mix/A0 4.704 / 4.390 / 4.914; K-pure/A0 7.183 / 5.915 / 6.222.

## A1 and the supplementary a1c reference (read before using any ratio to a negation-map rho)
- Preregistered A1: success 4/40, 5/40 (b32 inst 1, 2), 0/24, 0/24 (b36), b28 56/60 and 58/60, b40 not run (skipped by the 200 s rule). R rises 2.158 (b20) to 40.000 (budget, b36). K/A1 ratios (e.g. K-mix/A1 = 0.268 at I/M = 3) compare against budget-censored A1 and are NOT valid; F5 is not evaluable.
- Supplementary a1c (cycle test over last 8 points, not preregistered): all trials solved at every b, but R = 1.322, 1.552, 1.634, 1.815, 2.549, 6.857 at b = 20..40; K-mix/a1c cost ratio at I/M = 3, b32/b36/b40 = 4.591 / 3.230 / 1.431 (CIs [3.812, 5.563], [2.503, 4.152], [1.054, 1.928]); K-pure/a1c 7.011 / 4.353 / 1.812. K-mix/a1c pooled b>=28: 2.737 [2.379, 3.163] at I/M = 3, 0.596 [0.518, 0.689] at I/M = 100 (unranked across the band). The apparent narrowing at b=36, 40 is a1c degrading (longer fruitless cycles), not a Kummer gain; a1c is not a sound rho reference at those sizes.
- No sound measured negation-map rho exists in this study. A0 is the sound matched reference.

## Verdicts per PREREGISTRATION section 8 (observed bounds)
- H1 / F1: exact structure held (0 orbit mismatches in PC1 SB and S_idx, 0 in 60 PC2 checks, 0 out-of-set recoveries, NC1 clean). F1 is formally TRIGGERED by its walk clause: PC1 S_idx recovery 165/274 = 60% < 98% (DP-free cycles at N=138, see above). PC1 SB walks 400/400.
- H2 / F2: K-pure R in [1.4, 3.0]: 0 of 10 cells (b>=24) have the CI entirely outside; slope CI [0.482, 0.505] inside [0.40, 0.60]; K-pure/A0 and K-mix/A0 CIs not entirely outside their bands. F2 is formally TRIGGERED by the band K-mix/A1 [0.75, 1.15] (observed 0.046 [0.042, 0.049]) which is an artefact of A1 failing; against A0 and a1c the K-mix/A1-band cannot be tested (K-mix/a1c 0.474 [0.412, 0.549]). The point estimate K-mix/A0 0.787 is above the predicted centre 0.71 (CI [0.709, 0.872] just contains it).
- H3 / F3: 0 ladder mismatches, 0 class crossings, 0 cross-curve collisions; twist R ratio 1.002 [0.896, 1.114] (not outside a factor 1.4). Held.
- H4 / F4: per-step counted cost within 0.03% of 21.5 M + 12 S + 1 I (F4 not triggered). Kummer arms above A0 at I/M = 3, 10, 30 (CIs entirely above 1). The sub-claim "no ordering holds across the whole band" is NOT confirmed for K-pure/A0 (dearer at all four I/M, CI above 1); it holds for K-mix/A0 (unranked). No improvement condition (K-mix below A0 at I/M = 3 with upper bound < 1): not met (ratio 4.625).
- H5 / F5: F5 (K below matched A1 at I/M = 3, CI below 1, verified solves only, at b = 32, 36, 40) not met and not evaluable (A1 solved 9/80, 0/48, no b=40 data); against sound A0 the ratio is 4.4 to 4.9 (K-mix) and 5.9 to 7.2 (K-pure) at b >= 32. H5 not falsified.

## Missing / skipped / partial
- a1 at b=40 inst 1, 2: skipped by the preregistered 200 s rule (out/MISSING.txt).
- pc1, nc1, kmixidx at b=24 inst 2: skipped_degenerate_N (N=2, S_idx subgroup of order <= 4); replaced by supplementary instance 4 (DEVIATIONS item 1). No cell timed out.
- Cell trial counts are as preregistered (80/80/60/40/24/12), all trials recorded.

## Operation counts (primary cost metric; added in the finalisation pass, DEVIATIONS items 15-17)
Sums over raw trial records, b >= 28, instances 1 and 2, 272 trials per arm (all solved), from out/opcounts_pooled.txt (ops_table.gp) and out/opcounts_ratios.txt (ops_ratio.gp):
| arm | mean steps/trial | mean M/trial | mean S/trial | mean I/trial | candidate tuples (cross-table x-collisions that were verified; exactly 1 per solved trial) |
|---|---|---|---|---|---|
| A0 (additive rho) | 189621.0 | 379242.0 | 189621.0 | 189621.0 | 272 total |
| K-pure | 200625.1 | 4314448.4 | 2408077.7 | 200625.1 | 272 total |
| K-mix | 152663.5 | 3282008.6 | 1831815.4 | 152663.5 | 272 total |
| K-pure on twist E' | 212539.1 | 4568608.1 | 2549907.8 | 212539.1 | 272 total |
Trial-weighted counted-cost ratios (M + 0.8 S + (I/M) I; weights 60/40/24/12 trials per b, hence not the sqrt(n)-normalised ratios of the cost table above): I/M = 3: K-pure/A0 6.222, K-mix/A0 4.733; 10: 3.398 / 2.585; 30: 1.971 / 1.500; 100: 1.349 / 1.027. They agree with the preregistered normalised ratios to within 0.11 and give the same ordering. Per-cell counts: out/opcounts.csv (86 cells). Polynomial evaluations: none in this method. GP wall time per step stays secondary (summary.csv, measured at load average about 15 on 4 cores, contention affected).

## Replication status
- Full independent replication (replicate/, same scripts, earlier in this session): out/costs.csv and out/results.json byte-identical to the original; summary.csv identical except wall-clock columns; controls raw files byte-identical.
- Reduced replication (run_reduced.sh, 16m54s real under load): 4 control files byte-identical, 28 of 28 cell files identical modulo wall-clock fields, 28 of 28 operation-count rows identical (reduced_check.log).

## Files
Scripts: ops_table.gp ops_ratio.gp build_results_json.sh run_reduced.sh check_reduced.sh (finalisation pass), lib.gp gen_curves.gp check_identity.gp check_exhaustive.gp chainsearch.gp run_cell.gp cellrun.sh analyze.gp (v2) analyze_frozen_v1.gp plot.py run.sh run_supp.sh lib_a1c.gp run_cell_a1c.gp.
Raw: raw/ (curves, identity, exhaustive, chains, 89 cell files + .err + time_*.sec). Final machine-readable results: results.json (top level; = out/results.json + operation counts). Out: out/opcounts.csv, out/opcounts_pooled.txt, out/opcounts_ratios.txt, out/summary.txt, summary.csv, costs.csv, results.json, fig_R_vs_b.{png,svg}, fig_cost_ratio.{png,svg}, MISSING.txt, SUPPLEMENTARY.txt, summary_v1_frozen_analysis.txt, results_v1.json.
