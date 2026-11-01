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
