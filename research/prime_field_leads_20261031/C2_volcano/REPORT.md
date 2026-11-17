# C2: one volcano experiment (crater vs floor), solve-cost side

STATUS: IN PROGRESS. The section below was written before the main run and is
not edited afterwards. Results are appended in later sections.

## Frozen before the main run (written 2026-10-31 17:2x, after a one-instance smoke test that showed
## the msolve leg at B = 47 does not finish in 120 s under load and the counted loop runs in < 1 s)

Instances (chosen from the output of `c2_volcano find 3`, before any comparison data):

- Main (M): ell_v = 3, D = -11, p = 932413, t = 1931, N = 930483 = 3^2 * 103387, r = 103387 (prime), B = round(r^(1/3)) = 47.
- Small (S): ell_v = 3, D = -19, p = 105037, t = 647, r = 1657, B = 12 (added so that msolve can run to completion; same design).

Design. Crater = the class-number-one CM curve of discriminant D with order N (volcano depth 2 at ell_v = 3, conductor 9,
Aut = {+-1}); floor = curves reached by two non-backtracking degree-3 Velu steps and certified by a cyclic 3-Sylow subgroup
(crater has full rational E[3], so the test discriminates). The composite isogeny phi (degree 9, coprime to r) maps the
r-subgroup generator and the 32 target multiples [k_i]G exactly to the floor (checked as a homomorphism on a target).
8 crater models (isomorphic curves y^2 = x^3 + a u^4 x + b u^6, u random; they change which abscissae are small) and
8 floor models (distinct floor vertices, then isomorphic rescalings) per instance. Base per model: the first B abscissae
x = 0, 1, 2, ... whose point P satisfies [r]P = O (so the base lies in the prime-order subgroup). The same B on every curve;
largest x used is recorded per model (models.csv).

Counted routine (the only wall-time-free metric used for work): for every pair x1 <= x2 of base abscissae, build the
quartic S_4(x1, x2, z, x_R) in z (resultant of the two S_3 quadratics), form prod_b (z - b) mod q by B monic
reductions, take the gcd degree with q, and on a non-trivial gcd evaluate q at the B base abscissae. Every hit is verified
by explicit group addition (verification arithmetic is on a separate counter and not charged). Unit: field
multiplications (mul and sqr; adds not counted; inversions counted as their square-and-multiply chain).
Cost terms fixed by B alone (coefficient build, B reductions per pair) are reported separately as work_fixed_by_B and are
outside the part of the cost that can depend on the curve; work_variable_gcd_roots is the rest.
work_to_first_relation depends on the probe order and so measures relation yield, which the lead card already covers; it is
reported but not used in the decision.

Second metric (the algebraic one): msolve (DRL, char p, 1 thread, `-v 2`) on the constrained system
S_4(X1,X2,X3,x_R) = 0, prod_b (X_i - b) = 0 for i = 1,2,3. Recorded per F4 round: degree, matrix rows, matrix columns.
Cells on M: targets 0 and 1 of models 0..3 on each side (16 cells), timeout 420 s each, 8 parallel. A cell that times
out is reported with its status; cells are compared on the COMMON PREFIX of F4 rounds (the minimum completed round count over
all cells), because round counts at a timeout depend on host load while the rounds themselves are deterministic.
Cells on S: targets 0 and 1 of all 8 models per side, timeout 300 s, compared at completion.

Decision rule (frozen).
- ratio rho = median over crater models of (per-model median over 32 targets of work_total) divided by the same on the floor.
  Spread = [min, max] of per-model medians on each side.
- "reopens" if rho is outside [0.5, 2] with non-overlapping spreads, OR if the maximal step degree (common prefix on M, completion
  on S) of every crater cell differs from every floor cell by at least 1 in the same direction.
- "null_confirmed" if rho in [0.5, 2] with overlapping spreads and the msolve degrees are not separated.
- Honest limitation stated in advance: for this routine the B-fixed term is about three quarters of the total, so the 2x
  gate on work_total is not reachable by the gcd/root part alone; a null on work_total alone would be weak. The msolve leg is
  the discriminating one.

---

## Results (appended 2026-10-31 by the resuming session; the section above is unchanged)

STATUS: COMPLETE for the frozen decision rule. Nothing was rerun in this session: the counted-work leg finished on M and S before the
host stopped the workflow, and the msolve data on both instances were already on disk. The msolve leg on M did NOT finish (timeouts, below);
the frozen rule compares such cells on the common prefix of F4 rounds, which is what is done here. Files: `M/summary.csv`, `S/summary.csv`,
`*/work_per_target.csv`, `*/msolve_summary.csv`, `*/msolve_trajectories.csv`.

### Counted work (field multiplications), median over 32 targets, then median over 8 models

| Instance | metric | crater [min,max] | floor [min,max] | ratio crater/floor |
|---|---|---|---|---|
| M (B=47) | work_total | 721402 [721402, 721402] | 721551.25 [721402, 721999] | 0.9998 |
| M | work_variable_gcd_roots | 176565 [176565, 176565] | 176714.25 [176565, 177162] | 0.9992 |
| S (B=12) | work_total | 23035 [23005, 23065] | 23035 [23020, 23035] | 1.0000 |
| S | work_variable_gcd_roots | 10386 [10356, 10416] | 10386 [10371, 10386] | 1.0000 |

rho is inside [0.5, 2] on both instances and the spreads overlap (on M the crater spread is a single value contained in the floor spread).
False hits: 0 on both sides. Decomposable targets (yield, not part of the decision): M crater 196/256, floor 209/256; S crater 195/256,
floor 174/256; median work to first relation on M 207248 vs 153572. Yield is the lead card's question, is not used here, and no
significance was tested.

### msolve (DRL, char p, 1 thread)

- S, 32 cells (16 crater, 16 floor), all completed within 300 s: max step degree is 20 in every cell, crater and floor. Round
  counts differ between cells (168 to 186) on both sides; on the common prefix of 168 rounds the sum of rows x cols is 7767106 to 7803178
  for crater and 7767106 to 7769334 for floor, max matrix 268 x 1142 for every cell.
- M, 16 cells (8 crater, 8 floor): 11 timed out at 420 s (7 crater, 4 floor; per-cell status in `M/msolve_summary.csv`);
  5 completed (crater_3_t1, floor_0_t0, floor_2_t0, floor_3_t0, floor_3_t1) with 2037 to 2089 rounds and max step degree 55.
  Every cell, completed or not, has max step degree 55. Common prefix = 95 rounds (the shortest timed-out cell). Over those 95 rounds
  the sum of rows x cols is 3079461630 in 15 of 16 cells and 3079459926 in crater_3_t1; max rows 4177 in every cell. Over the whole
  run the five completed cells have sum of rows x cols 2.062e10 to 2.775e10, with max matrices 28500 x 28506 (crater_3_t1) against
  26559 x 26442 to 28503 x 28506 (floor); no side separation.
- The timeouts depend on host load (load average 137 to 430 during the original run) and say nothing about the cost of the system.
  The common-prefix comparison does not depend on them. The step degree 55 is reached in round 1, so on this routine max step degree
  carries little information beyond B; the matrix profile is the informative quantity and it is identical to within 1 part in 10^6 on the prefix.

### Decision (frozen rule)

null_confirmed. rho in [0.5, 2] with overlapping spreads on M and S, and msolve step degrees are not separated (S at completion, M on the
common prefix, with the M cells that did finish showing the same degree 55). The stated limitation applies: about three quarters of
work_total is fixed by B, so the work_total gate alone is weak; the msolve leg carries the decision and its M part is a prefix
comparison plus 5 completed cells, not 16 completed cells. No speedup, exponent change or reopening is claimed. The atlas proposals in
`atlas_update.md` stay as proposals (disposition unchanged).

Every attempted cell is listed in the CSV files named above, including the 11 M cells that timed out.

Directories `S_ctrl_a0` and `S_ctrl_b0` hold two extra S-instance control runs (4 cells each, all completed, max step degree 20, identical
trajectories on crater and floor within each directory). Their construction is not described in the frozen text or in the logs kept, so they
are listed here but not interpreted and not used in the decision.

## Rerun 2026-11-01: msolve leg on M to completion (appended; the sections above are unchanged)

Plan `../rerun_20261101/RERUN_PLAN.md`, report `../rerun_20261101/REPORT.md`, data
`../rerun_20261101/C2_M/`. The committed M inputs ran with msolve 0.6.5, exact
sparse/dense kernel (`-l 1`), cap 3600 s. On the S instance this kernel reproduces all
32 cells above, reduced bases included; on M it reproduces all 16 original records over
the rounds they recorded.

All 16 decision cells (models 0 to 3) completed: maximal step degree 55 in every
crater and every floor cell, 2037 to 2089 rounds and the same matrix-profile ranges on
both sides. The 95-round prefix values are those reported above. The 16 cells of models
4 to 7 also ran (driver deviation D1) with the same profile and enter no decision.
**Decision: null_confirmed** stands, now with every msolve cell completed. The
weakness stated in advance (three quarters of the counted work is fixed by B) still
applies to the counted-work leg, which was not rerun.
