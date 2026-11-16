# G2-decoupled-AB-projection-arm: results (E3 rerun with a decoupled projection arm and a same-term-count control)

Protocol: `PREREGISTRATION.md` (hash in `PREREG.sha256`, verified by `run.sh`). Deviations and implementation choices: `DEVIATIONS.md` (read item 6: the first frozen run aborted in the Semaev stage, the script was fixed and the whole driver re-run; all numbers below are from the second run, `out/`). Driver: `run.sh` (all 30 gp stages of the run rc=0, plus the v3 `g2_opcounts.gp` stage, `out/timing.txt`; script hashes `SCRIPTS.sha256`, check in `out/scripts_check.txt`). gp 2.15.4, no Rust, no builds. Every number below is in `out/analysis.out`, `out/results.csv`, `out/summarize.out` or `results.json`.

Scope: arm-versus-arm comparison in the Semaev work unit (root-solves x degree) per independent E-relation, on toy fields (p = 101, 211, 401, N <= 64, T = 200 targets per curve-cell). No GAE pricing, no matched-rho number, no cross-method ranking (AGENTS.md 5a not computed). The success criterion (a verified measured improvement over the baseline) is assessed only on the independent E-relation metric, and each cell's gap to 1 is the distance remaining.

## Headline

No decoupled arm and no coupled arm beat the baseline D* (= D_m in every cell). The lowest measured rho_E over all powered primary pooled cells is **1.0069** (family B, p=101, N=20, arm "j=m, k=1"), a distance of **0.0069** from 1, and it sits above 1 from below (99% curve-level interval 1.0069 to 1.0069). The closest decoupled approach is the (j=m, k=1) arm, whose ratio is exactly 1 + 1/W_D(m) (W_D(m) = number of root-solves x signs of D_m): it approaches 1 from above as the factor base grows, and brings no E-relation beyond D_m. The E3 conclusion (the twist route does not reduce the cost of E-relations) is robust to decoupling. Counting (A-set, B-set) pairs makes the twist arm look cheaper in 12 of 22 powered cells, but that is a relabelling and is not an improvement.

## Setting actually run

- Curves: E3's six (reproduced exactly from the seed rule, asserted in `g2_curves.gp`), extension curves c=3,4 per p (exploratory), one extreme-twist curve per p (c=9; #E' = 91, 183, 365 for p = 101, 211, 401; Hasse lower bounds 82, 183, 362), two even-order curves for NC1 (p=101: #E=96; p=211: #E=216).
- Grid: Family A (m=2) N in {16,24,32,40} for all p, plus {48,64} for p in {101,211} (coupled arm X only for N <= 40); Family B (m=3) p in {101,211}, N in {12,16,20}. 15 curves, 110 curve-cells (curve x N), 200 targets per curve (3,000 distinct targets, 22,000 target rows over the cells), plus 22 primary pooled cells (2 primary curves per p) of which all 22 are powered (smallest D_m total 74). Single-curve cells (primary + extreme): 66, of which 63 powered.
- Metric: rho_E = Z_arm / Z_D*, Z = (work in Semaev units over the 200 targets) / (number of independent E-relations = distinct A x-sets). Intervals: 99% percentile, curve-level resampling of the 2 primary curves per (family, p, N) (B = 10000, seed 9002; at most 3 distinct outcomes, so coarse and conditional on the curve family) and, for single curves, target-level resampling (B = 2000, seed 9001).

## Primary results (primary pooled cells, point estimates; per-cell values with intervals are in `out/analysis.out` and `out/results.csv`)

| arm | family A (m=2): 16 powered cells | family B (m=3): 6 powered cells |
|---|---|---|
| PF(j=m, k=m): decoupled class F, 2m terms, twist work charged | 2.087 (p=101, N=24) to 3.556 (p=211, N=16) | 2.528 (p=101, N=20) to 8.333 (p=211, N=20) |
| PF(j=m, k=1): closest approach | 1.0175 (p=101, N=64) to 1.1111 (p=211, N=16) | 1.0069 (p=101, N=20) to 1.0455 (p=211, N=12) |
| PF best over the sweep (j=1..2m-1, k=2m-j, plus (m,1),(m,m)) | identical to (m,1) in every cell | identical to (m,1) in every cell |
| X coupled, distinct E-sets (12 A cells, N <= 40; 6 B cells) | 209.8 to 916.9 | 390.2 to 2973.8 |
| X coupled, E3 definition (j <= m hits, vs D*) | 136.4 to 851.5 | 85.5 to 234.9 |
| pair-count accounting (j=k=m), a relabelling | 0.0977 (p=101, N=64) to 12.99 (p=401, N=16) | 0.2034 (p=101, N=20) to 5.772 (p=211, N=12) |

P arm on class-S targets (the ECDLP-relevant target): rho_E = 1.000 in every cell (it is D_j with a no-op projection).
Pooled grid (6 primary curves, family A; 4 for B; all E3-grid N pooled by ratio of totals), 99% curve-level interval: family A PF(m,m) = 2.350 [1.872, 3.013], PF(m,1) = 1.0420 [1.0334, 1.0538], X E-sets = 449.3 [346.9, 805.8]; family B PF(m,m) = 4.323 [2.723, 12.49], PF(m,1) = 1.0161 [1.0114, 1.0395], X E-sets = 853.2 [576.1, 3188].
Falsifier scan (H5): over the 22 powered primary pooled cells no arm among PF(m,m), PF(m,1), PF best, X E-sets has an interval end below 1 (smallest upper ends: PF(m,m) 2.18, PF(m,1) and PF best 1.0069, X 232.7); 0 of 63 powered single-curve cells (target-level intervals, primary and extreme) and 0 of 22 exploratory extension-pool cells (c=1..4) have an interval or a point estimate below 1 (lowest extension-pool point 1.0070).
Extreme-twist curves (smallest #E' among the first 400 admissible draws): PF(m,m) 1.476 to 2.528, PF(m,1) 1.0045 to 1.0625, X E-sets 163.1 to 830.5, all above 1 (the registered prediction; Hasse prevents a tiny #E' over F_p).

## Controls

Positive controls (run first):
- Synthetic interval positive control (`g2_pcsynth.gp`, an addition, fixed seed 777): injected arm with true rho 0.5 gives point 0.489, curve-level 99% interval [0.432, 0.550] and target-level [0.463, 0.657]; the null arm (true rho 1) gives intervals [1.000, 1.016] and [0.807, 1.256]. PASS (the interval code reports rho < 1 when the arm is cheaper).
- PC1 injection on real data: baseline work x2 gives rho_P = 0.500 and x0.5 gives 2.000, within 1e-9 in 22 of 22 powered primary pooled cells; the 99% interval excludes 1 on the correct side in 22 of 22.
- PC3 real cheaper-route cell (pair metric): Z_pair(m,m)/Z_D(m) < 1 in 12 of 22 powered primary pooled cells (17 have mean Y_T(m) > 2; all 12 are among them), for example 0.774 (A, p=101, N=24) and 0.0977 (A, p=101, N=64). The detector sees rho < 1 where it exists on the pair metric. Extreme-twist curves: rho_E stays above 1 (see above).
- PC4 counting instrument: D_m mean yield within [0.8, 1.25] of the model in 32 of 32 primary curve-cells with model Y_D >= 0.5 (registered >= 80%).
- PC5 solver vs table: D2, T2, X4, D4 relation sets equal the group-law table sets on 5 cells x 10 targets (0 mismatches; also P-F, P-S and projected-point checks 0), cells (101,12), (101,16), (211,16), (211,24), (401,16). Several cells have few D2 relations (1 in total for p=211 N=16 and N=24, 10 targets), so power there is low.
- E3 instrument check: for the 36 E3 cells, D_m totals and X(j <= m) totals are identical to E3 `results.csv`, and the coupled rho_gen reproduces E3's value (ratio 0.99999996 to 1.0000000 in the 35 cells where E3's value is finite).

Negative controls:
- NC1 (lemma, projection in F_{p^2} arithmetic): odd-order curves (15 curves x 200 class-S targets = 3000): 0 violations (B' = O and A' = R as points); class-F projection recovers (A, B) exactly in 3000 of 3000. Even-order curves (p=101, 211; 400 class-S targets): B' != O in 0 (R - pi R = O identically for R in E(F_p)), A' != R in 368 (targets of even order; boundary of the claim). Decomposition audit (4-subsets of 0..19, 16 sign vectors): odd curves 1,162,800 decompositions, 61,950 with R in E(F_p), 2,232 of those mixed, 0 violations; even-order curves 155,040 decompositions, 9,952 with R in E(F_p), B != O in 1,084 (and A != R in 1,084), 2B != O never.
- NC2 (B redrawn with an independent seed): D counts unchanged in 110 of 110 curve-cells (rho_E(P,F) is B-independent exactly); the pair-count permutation test (alpha 0.01) gives p < 0.01 in 0 of 110 curve-cells (chance expectation 1.1; smallest p = 0.019).
- NC3 timer null (D2 against an identical D2c, 7 interleaved repeats): the 99% t-interval contains 1 in 5 of 5 cells; the largest half-width is 0.331, which is the minimum detectable wall ratio effect here.
- NC4 (E-only factor base, N=24, all 4-sets, actual F_{p^2} group law, 6 primary curves): 0 hits against the 200 class-F targets; 17,599 hits against the class-S targets (the detector sees hits where they exist).

## Prediction outcomes (as registered)

- H1 (P arm on class S is a tie): held. Count differences P-S vs D: 0 over 110 curve-cells (0 per-target count differences, 0 per-target set differences); rho_E(P,S) = 1.000 exactly in all of them. Wall ratio P/D >= 1: PS/D2 median 1.346 to 2.054 (> 1 in 5 of 5 cells; the 99% interval of the 7 per-repeat ratios includes 1 in cell (401,16): [0.951, 1.739]).
- H2 (class F, independent E-relation metric; rho_E(P,F,j,k)/Z_D(j) = 1 + W_T(k)/W_D(j)): the identity held in all 2254 arms tested (curve-cell x j x k with D_j > 0), maximum relative deviation 0 (exact), none <= 1. The magnitude guess partly missed: for j=k=m the per-curve-cell value is 1.50 to 4.80 (family A) and 2.53 to 21.0 (family B), not "1.5 to 3"; for (j=m, k=1) it is 1.016 to 1.100 (A) and 1.007 to 1.042 (B), consistent with the guess of about 1.03 to 1.15 for m=2, N = 16..40. The closest decoupled approach to 1 is predicted from above at distance 1/W_D(m) and measured so (0.0069 at the best cell).
- H3 (pair-count metric, accounting only): held. At least one powered cell has pair-metric rho < 1 (12 of 22); Z_pair(m,m)/Z_X(exact j=m) <= 0.0071 over the 18 powered cells with the coupled arm (registered < 0.2); rank checks (rank of A-set incidence <= Y_D, rank of B-sets <= Y_T, rank of pair rows <= rank A + rank B) violated in 0 targets over 110 curve-cells. Not counted as an improvement.
- H4 (same-term-count control): the registered primary statement Z_D(m2) > Z_D(m) held in 22 of 22 powered primary pooled cells (ratio 2.03 to 19.7). The second corrected rho PF(m,m)/(Z_D(m2)/Z_D(m)) was registered as "not predicted to cross 1" and it does cross: range 0.2066 to 1.790, below 1 in 10 of 22 cells (family A 0.775 to 1.383; family B 0.2066 to 1.790). This compares an arm whose E-relations have m E-terms with a baseline whose relations have 2m E-terms, so it is not the registered baseline and is not an improvement; it shows how a same-term-count baseline would make a twist arm look better.
- H5 (E3 conclusion robust): held. 0 of 22 powered primary pooled cells with a 99% interval entirely below 1 for any independent-metric twist arm; lowest decoupled class-F point value 1.0069, within 0.2 of 1 so the distance is reported (0.0069). The coupled rho reproduces E3 (see instrument check).
- H6 (wall time, exploratory): per-target wall ratios PF/D2 = 2.40 to 7.32 (> 1 in 5 of 5 cells; the registered estimate 1 + N_T/N_E is 1.6 to 5.3, the excess is the projection and call overhead); PF/X4 = 0.00044 to 0.00097 (< 0.1 in 4 of 4 cells with X4 timed); X4/D2 per root-solve = 85.7 to 92.9 (E3: 61.6 to 119.5); per-solve times D2 3.4 to 3.9 us, T2 3.5 to 3.7 us, PF 4.9 to 5.3 us, X4 294 to 331 us. NC3 half-width 0.331 is below the PS/D2 and PF/D2 effects in every cell (PS/D2 - 1 smallest 0.346 vs the cell's NC3 half-width 0.301), so the wall orderings stand; wall numbers come from one 7-repeat run on a shared machine (load average 2.2 to 2.8 across timed blocks).

Outcome: the central prediction (no twist arm is below 1 on the independent E-relation metric; decoupling does not change the E3 conclusion; the P arm on class S is an exact tie) is confirmed. Two secondary registered items did not hold as stated: the H2 magnitude range for j=k=m, and the H4 statement that the same-term-count corrected rho would not cross 1.

## Limits

Toy fields (p <= 401, N <= 64); 200 targets per curve-cell; the registered curve-level intervals use only 2 primary curves per (p, N), so they are coarse; cells share curves, targets and nested factor bases and are strongly correlated (no multiplicity claim); the coupled arm was not run for N > 40; wall times are single 7-repeat runs on a shared 4-core machine; several wall cells have very few D2 relations. No cross-method (rho, GAE) claim is made.

## Operation counts (primary cost metric; added in v3, `g2_opcounts.gp` -> `out/opcounts.csv`, `out/opcounts.out`, `results.json` key `operation_counts`)

All counts are exact functions of the recorded tables (no new sampling); the loop-counted work recorded in `out/tables_*.csv` equals the closed form W_D(j) = C(n, j-1) 2^(j-1) in all 110 curve-cells (0 mismatches, `out/opcounts.out`). Unit: root-solves (Semaev work unit); candidate tuples = C(n, j-1) enumerated (j-1)-subsets per target, times 200 targets. Totals below are over the primary curves c=1,2, all 22 (family, p, N) cells (family A 16 cells x 2 curves, family B 6 x 2 = 44 curve-cells; the coupled arm X exists for 36 of them), from `results.json`:

| arm | family A root-solves | family A candidate tuples | family B root-solves | family B candidate tuples |
|---|---|---|---|---|
| D_m (baseline) | 192,400 | 96,200 | 148,800 | 37,200 |
| P class S (j=m), same count as D_m | 192,400 | 96,200 | 148,800 | 37,200 |
| PF (j=m, k=1) | 198,800 | 102,600 | 151,200 | 39,600 |
| PF (j=m, k=m) | 448,000 | 224,000 | 643,200 | 160,800 |
| D_2m (same term count) | 43,713,600 | 5,464,200 | 2,067,200 | 64,600 |
| X coupled (36 curve-cells only) | 167,270,400 | 20,908,800 | 528,998,400 | 16,531,200 |

Exact per-cell work ratios to D_m (`ratio_to_Dm_rootsolves`): P class S = 1 in every cell; PF (j=m, k=1) 1.00455 to 1.25 over all 110 curve-cells, e.g. 145/144 = 1.006944 in (B, p=101, c=1, N=20), the headline distance 0.0069 from 1; PF (j=m, k=m) 1.214 to 46. Because yield cancels in the decoupled E-relation metric, these work ratios are the rho_E of the analysis (an independent count of the same quantity). Candidate tuples are in the same ratio up to the sign-vector factor.

Projection group operations (model count, per target): 2 elladd + 1 ellneg + 2 double-and-add scalar multiplications by 1/2 mod #E*#E' (bitlength-1 doublings + popcount-1 additions each) = 35 for p=101 c=1 (other curves in `out/opcounts.csv`), plus 2 Frobenius powerings in F_{p^2}. This is a textbook-algorithm model, not PARI's internal count; over the 200 targets it is 7,000 group operations per curve-cell against hundreds to millions of root-solves, so the projection does not change the ordering (arms D_m and P class S differ by this overhead and nothing else). Polynomial evaluations and field multiplications were not instrumented (gp has no counter) and no number is reported for them. Wall times above remain secondary and contention-affected (load average 2.2 to 3.5 on 4 shared cores; a one-cell replication of `g2_tables.gp` took 2m10s here against about 14 s recorded in the run).

Replication: `run.sh` (about 8 min wall in run 2 on the shared machine (env_date start 18:27Z, plot written 18:35Z): timing.txt of run 2 totals 95 s Semaev + 6 s lemma + 54 s analysis + tables run 2 at a time + plots; it overwrites `out/`, so the wall-time rows will change). Reduced check done for this version: `g2_tables.gp` for p=101, c=1, family A re-run into a scratch directory reproduced `pt_p101_c1_A.csv` byte-for-byte and `tables_p101_c1_A.csv` except the `ms` timing column; `g2_opcounts.gp` alone: `timeout 300 gp -q -f g2_opcounts.gp > out/opcounts.out 2>&1` (a few seconds).

## Files

`out/analysis.out`, `out/opcounts.csv`, `out/results.csv` (long format), `out/cellsums.csv`, `out/pt_*.csv`, `out/tables_*.csv`, `out/semaev_*.csv`, `out/lemma.csv`, `out/pcsynth.csv`, `out/summarize.out` (min/max extraction by `out/summarize.sh`), `results.json`, `g2_rho_vs_N.{png,svg,pdf}`, `g2_pair_accounting.{png,svg,pdf}`; first (aborted-semaev) run kept in `out_run1_aborted_semaev/`; development history in `dev/` (see DEVIATIONS.md item 3).
