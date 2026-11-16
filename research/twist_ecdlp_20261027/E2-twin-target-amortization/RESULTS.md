# RESULTS: E2-twin-target-amortization (E and E' together)

All numbers below are copied from files produced by the frozen code (`raw/*.out` = `out/*.out`, `summary.csv`, `results.csv`, `results.json`,
`raw/analysis.out`).  GP 2.15.4, toy m=2 table decomposition (a controlled stand-in; not a competitive ECDLP algorithm; no discrete log solved).
Driver: `run.sh` (frozen commands of PREREGISTRATION.md section 8).  Frozen files checked: `raw/freeze_check.out` (all OK, re-checked 2026-10-26).

## Verdict
**H_idea (E and E' served by one table at cost(E)(1+eps), eps << 1) is FALSIFIED under the preregistered rule; H_mech (eps ~ 1) holds.**
Instrument valid: PC1 passes P5, L1 and L2 pass.  Falsifier criterion met: in all 9 (bits, mu) cells the median eps >= 0.75 at all three w, and no cell
has a bootstrap upper bound <= 0.25 (0 cells; the support criterion fails).  Several of the secondary numeric predictions (P1, P2, P4, P6, P7) were
partly wrong; they are listed below and do not change the verdict.

## Sample sizes
63 grid runs, 0 missing, 0 failed, 0 capped: 27 at 22 bits, 27 at 26 bits (mu in {0.75,1.5,3} x 3 curves x 3 stream seeds), 8 at 30 bits (mu in {0.75,1.5} x 2 x 2),
1 at 32 bits (stretch, mu 1.5; S0 projection 150 s).  N (factor-base size per type) / W (verified relations collected per curve), curve 1:
22b: 121/291, 242/665, 484/1497; 26b: 305/873, 610/1957, 1219/4331; 30b: 768/2552, 1536/5635; 32b: 2438/9507.
Runs within a cell are not independent (3 curves x 3 stream seeds, same N); intervals below are bootstrap (424242, 10000 resamples) over those runs and are
degenerate-narrow because the op counts are nearly deterministic.  n=4 cells (30b) and the n=1 cell (32b) have a min-max range only (n=1: no interval, reported as -1 by the code).

## Primary metric: eps = C_both / C_E - 1 (JOINT shared-table arm vs E alone), cell medians, [min, max] of the runs; unit: GAE-equivalents (cost(w) = A + K + w * native ops)
| bits | mu | n | eps, w=0 | eps, w=0.1 | eps, w=1 | sav (w=0 / 0.1 / 1) |
|---|---|---|---|---|---|---|
| 22 | 0.75 | 9 | 0.998 [0.955,1.017] | 1.114 [1.072,1.133] | 1.172 [1.130,1.191] | 0 / -0.042 / -0.062 |
| 22 | 1.5 | 9 | 0.978 [0.957,0.999] | 1.081 [1.064,1.098] | 1.116 [1.100,1.131] | 0 / -0.043 / -0.057 |
| 22 | 3 | 9 | 0.998 [0.996,1.001] | 1.085 [1.084,1.087] | 1.107 [1.105,1.108] | 0 / -0.042 / -0.052 |
| 26 | 0.75 | 9 | 0.975 [0.973,0.994] | 1.080 [1.078,1.099] | 1.125 [1.123,1.144] | 0 / -0.037 / -0.053 |
| 26 | 1.5 | 9 | 0.984 [0.979,0.986] | 1.076 [1.072,1.078] | 1.103 [1.100,1.105] | 0 / -0.038 / -0.049 |
| 26 | 3 | 9 | 0.996 [0.996,1.003] | 1.074 [1.074,1.079] | 1.091 [1.091,1.095] | 0 / -0.037 / -0.045 |
| 30 | 0.75 | 4 | 1.007 [1.004,1.011] | 1.103 [1.099,1.106] | 1.139 [1.136,1.143] | 0 / -0.034 / -0.046 |
| 30 | 1.5 | 4 | 0.994 [0.986,1.003] | 1.076 [1.069,1.083] | 1.098 [1.091,1.104] | 0 / -0.034 / -0.043 |
| 32 | 1.5 | 1 | 0.994 | 1.073 | 1.093 | 0 / -0.033 / -0.041 |

Every individual run: eps in [0.80, 1.20] (63/63 at each w).  The joint arm never saved anything: sav is exactly 0 at w=0 (the shared scan is native work, priced 0)
and negative (-0.033 to -0.062) at w = 0.1 and 1 because the frozen accounting sorts one array of 2N^2 keys (n log n comparisons on the larger n) and probes it.
Best-case joint accounting (two separate sorted arrays, stand-alone probe counts, shared scan only; secondary column `epsBest`): epsBest_med = 0.974 to 1.044 over all
cells and w, savBest_med between 0 and 1.6e-4.  So the structural sharing is at most ~1e-4 of the cost (only the O(N) scan), the rest of the gap above 1 is the accounting choice.
Order stability across the w band: eps >= 0.97 and sav <= 0 in every cell at every w, so the conclusion (no amortization) is stable across the band.

## Controls
* **Positive control PC1** (same curve, two target streams sharing one table vs two separate runs; R = C_shared / C_separate), 22b and 26b, n = 9 per cell (w=0): 
  mu=0.75: R = 0.924 (22b), 0.935 (26b); mu=1.5: 0.726, 0.745; mu=3: 0.552, 0.558.  Gate P5 (R(mu=3) <= 0.65, R(0.75) >= 0.85, difference >= 0.20): PASS at all three w, both bit sizes
  (w=1: 0.529/0.875 at 22b, 0.532/0.888 at 26b).  The detector does see a real shared-table saving (~45% at mu=3) where the shared table is useful, and sees none for the E/E' twin.
* **L1** (x-level separation lemma): 22b, full enumeration, 58564 mixed-type pairs: 0 with F_p-rational roots, 0 zero-discriminant, 58564 non-square discriminants;
  E-half and E'-half key sets (58564 each) have intersection 0; 0 E-keys with non-square f, 0 E'-keys with square f.  26b: 10^6 sampled mixed pairs (N=610): 0 rational, 0 zero-disc, intersection 0 (372100 keys each).  PASS.
* **L2** (F_{p^2} separation lemma), seed 777, p = 65521 and 1048573 (both group orders prime), 2000 pairs each (run twice, identical): P in E(F_p), Q in E'(F_p)\{O}: P+Q in E(F_p) in 0/2000 (predicted 0);
  positive control Q = +-P'' (B = O): rational in 2000/2000; Frobenius fixes P in 2000/2000 and sends Q to -Q in 2000/2000.  PASS.
* **NC1** (table of curve 1, targets of curve 2): 18 runs, 14418 wrong-curve key hits, 0 verified relations.  PASS.
* **NC2** (hit rate / analytic y_hat): 126 streams, median 1.006, range [0.966, 1.077], |z| > 3 in 0/126.  PASS (stopping-rule caveat: z mildly anti-conservative).
* Verification: all main-arm hits verified by explicit lifting and group arithmetic: 0 unverified main hits, 0 cross-type hits (E relation using an E'-type abscissa) in 63 runs, 0 capped runs.
* J0 (unpartitioned joint, bits 22 and 26, curve 1): 0 mixed pairs with F_p-rational roots (63 runs record `mixed_with_Fp_root=0`); N^2 mixed pair steps are pure waste.

## Secondary predictions
* P1 (cell median eps in [0.95,1.05]): HELD at w=0 (9/9); FAILED at w=0.1 and w=1 (0/9; eps 1.07 to 1.17, high by the sort/probe pricing above).  Run range [0.80,1.20]: held 63/63.
* P2 (sav in [0, 0.02]/[0, 0.05]; never above 0.10): upper bound held (max sav 0); the lower bound failed at w > 0 (sav negative, 0/9 cells at w=0.1 and 1; 9/9 at w=0).
* P3 (usable by both curves = 0; mixed rational = 0): HELD (L1, J0, cross=0).
* P4 (relation sets equal, 100%): HELD 63/63.  Cost ratio C_A4/C_E - 1 in [0.9,1.1]: FAILED in most cells (held in 0/9 at w=0, 2/9 at w=0.1 and 1).  Median values 0.11 (30b, mu 0.75) to 0.90 (22b, mu 3) at w=0;
  0.22 to 1.02 at w=0.1; 0.26 to 1.04 at w=1.  At small mu the collection, not the build, dominates, so the useless E' half adds less than a doubling; the "~doubling" figure only holds near mu=3.  Single-curve user pays a positive overhead in every cell (never a saving).
* P5 (PC1 gate): HELD (above).
* P6 (R_J0 = C_J0/(C_E+C_E') in [1.3,2.0] at mu >= 1.5): HELD at w=0 (4/4: 1.51 (26b,1.5), 1.55 (22b,1.5), 1.88 (26b,3), 1.89 (22b,3)); FAILED at w=0.1, 1 (1.10 to 1.28, 0/4).  Always > 1 (the unpartitioned joint arm is always worse than separate runs).
* P7 (distance to rho; ratio C_E (relation phase, W relations) / measured rho cost, 10 walks per curve): medians per cell, w=0: 8.7 (22b,0.75), 9.7, 23.7 (22b,3); 23.1, 24.0, 56.0 (26b); 38.1, 36.2 (30b); 45.4 (32b).
  w=1: 88 to 475 (22b), 253 to 1215 (26b), 454 and 624 (30b), 808 (32b).  Predicted band 4 to 40 held in 7/9 cells at w=0, 2/9 at w=0.1, 0/9 at w=1.  Ratio > 1 everywhere: this baseline remains 9x to 1200x above rho for the relation phase alone
  (distance remaining, not a pass/fail; no DLP is solved, so there is no speedup or improvement claim of any kind here).  Nominal 0.886 sqrt(l): 1814 (22b), 7257 (26b), 29033 (30b), 58065 (32b) group operations.

## Conclusions and limits
* One shared table does not serve E and E' at a marginal cost: the mixed pairs give F_{p^2} abscissae that no target can hit, the two key halves are disjoint, and the joint arm costs 0.97 to 1.17 times (1+1) E-alone, i.e. no measurable amortization beyond the O(N) scan (<= ~1e-4 of cost).
* For a single curve the joint table only adds cost.  The mechanism (type-based disjointness) is verified here at the x-level (L1) and at the group level (L2); extension-field F_{q^n} factor bases are NOT tested.
* No improvement over the index-calculus baseline or rho is claimed.  GP wall times are recorded but not used.  Known formatting artefacts: see DEVIATIONS.md (results.json repaired; a few CSV cells carry "x e-5" spacing).
* Plots (recorded values only): `plots/eps_vs_mu.{png,pdf}`, `plots/pc1_vs_twin.{png,pdf}`, `plots/distance_to_rho.{png,pdf}`.
* Reproducibility: the ENTIRE grid, controls, rho references and analysis were re-run end to end by `run.sh` on 2026-10-26 (2-way parallel, about 20 min, into `replicate/`): `results.csv`, `results.json`, `summary.csv` and `raw/analysis.out` are byte-identical to the originals; rho and L1/L2 raw files byte-identical; the run files differ only in wall-clock `_ms` fields (DEVIATIONS.md).

## Operation counts (primary cost metric; medians over the runs of each cell, from `opcounts.csv`, produced by `opcounts.sh` from the raw RESULT lines; nothing recomputed)
Units: Legendre = Legendre-symbol evaluations of the factor-base scan; pair steps = pair-step inversions K per half (each ~10 field multiplications and one sqrt, per PREREGISTRATION.md; the field-multiplication figure is that nominal 10 x K, not separately measured); sqrt = square roots per half; sort cmp = nominal n ceil(log2 n) per sorted array; group adds = target additions R_k+P over the E stream; probes = binary-search probes (standalone E array vs the joint 2N^2-key array). The E' half has the same pair-step count as the E half in every cell (same N).

| bits | mu | n | N | W | Legendre (E/E') | pair steps K (per half) | sqrt (per half) | sort cmp (per half) | sort cmp (joint) | E targets (group adds) | E probes standalone | E probes joint |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 22 | 0.75 | 9 | 121 | 291 | 249 / 240 | 7381 | 7260 | 204974 | 439230 | 40585 | 604196 | 644660 |
| 22 | 1.50 | 9 | 242 | 665 | 516 / 461 | 29403 | 29161 | 937024 | 1991176 | 24592 | 415738 | 440401 |
| 22 | 3.00 | 9 | 484 | 1497 | 1006 / 933 | 117370 | 116886 | 4216608 | 8901728 | 13473 | 255820 | 269320 |
| 26 | 0.75 | 9 | 305 | 873 | 604 / 616 | 46665 | 46360 | 1581425 | 3348900 | 315946 | 5558126 | 5874307 |
| 26 | 1.50 | 9 | 610 | 1957 | 1216 / 1222 | 186355 | 185745 | 7069900 | 14884000 | 180586 | 3539518 | 3720507 |
| 26 | 3.00 | 9 | 1219 | 4331 | 2437 / 2438 | 743590 | 742371 | 31205181 | 65382284 | 98192 | 2124087 | 2222675 |
| 30 | 0.75 | 4 | 768 | 2552 | 1517.5 / 1552.5 | 295296 | 294528 | 11796480 | 24772608 | 2307185 | 46658671.5 | 48963958 |
| 30 | 1.50 | 4 | 1536 | 5635 | 3090 / 3055.5 | 1180416 | 1178880 | 51904512 | 108527616 | 1293093.5 | 28740957 | 30034146.5 |
| 32 | 1.50 | 1 | 2438 | 9507 | 4835 / 4901 | 2973141 | 2970703 | 136708412 | 285304512 | 3452216 | 81442606 | 84895888 |

Reading: the joint arm's sorted array has 2.1x the comparisons (n log n on 2N^2 keys) and 1.07x the probes of the standalone E array, with the same number of E-useful relations (W) in every run.  So in operation counts the twin half adds (K pair steps + sqrt + sort + probes) and removes nothing; this is the same conclusion as eps ~ 1.

Rho reference, group operations (median over 10 walks per curve, `raw_rho_opcounts.txt`: bits, number of walks, median steps (= comparisons), init GAE per attempt, branch-table set-up GAE):

* 22 bits: 30 walks, median 3555.5 steps, init 67, set-up 1072 (x3 curves / x2 at 30b / x1 at 32b pooled)
* 26 bits: 30 walks, median 12217 steps, init 79, set-up 1264 (x3 curves / x2 at 30b / x1 at 32b pooled)
* 30 bits: 20 walks, median 63310.5 steps, init 91, set-up 1456 (x3 curves / x2 at 30b / x1 at 32b pooled)
* 32 bits: 10 walks, median 129578 steps, init 97, set-up 1552 (x3 curves / x2 at 30b / x1 at 32b pooled)

Wall time is secondary and contention-affected (shared 4-core box, up to ~20 agents): the `ms_*` columns in `raw/run_*.out` differ between the 2026-10-21 run and the full 2026-10-26 re-run (see below); op counts do not.
