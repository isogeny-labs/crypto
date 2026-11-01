# res1eq engineering step: chained restarts (preregistered before data)

Date: 2026-11-01. Branch `claude/res1eq-chain-20261101`. Class if met: **engineering** of a generic collision search; not structural IC, no claim to beat rho.

## Literature / prior work checked
- Repository: `research/ic_residual_dp_eq_20261101` (baseline `residual-fused-eq`), `research/ic_residual_dp_eq_isolab_20261101` (L2 replay; res1eq = 1.591x matched rho at w=0, CI [1.430, 1.789]),
  crypto-autoresearcher lead atlas (`LEAD-ICE-generic-oracle-arity-cost-floor`: generic oracles cannot reach rho; `LEAD-ICP-prime-ic-constant-engineering`).
- Method: distinguished-point rho (van Oorschot-Wiener 1999); restarts that reuse a previous start point plus a random jump are the standard way to avoid a fresh scalar multiplication per walk.
- Collision floor: with B+1 = 2 independent relations needed and collisions Poisson in m^2/2n, the expected cost is Gamma(2.5)/(Gamma(2)Gamma(1.5)) = 1.5 x the first-collision cost (not sqrt(2)), so res1eq at 1.591 is within about 6% of its floor.
  Hence the **ceiling of any step-cost change is about 6%**; this study targets the identifiable part, restart cost.

## Where the cost goes (from the L2 records, res1eq, one target)
421,629 GAE = 410,591 additions + 11,038 doublings. 398,861 steps. 98 restarts at two scalar multiplications each (~11k doublings, ~2.6%); pair-table setup ~4k additions (~1%).
Prediction: chaining the restart saves most of the ~2.6%; the ratio S(chain)/S(eq) is about 0.97 at w=0 and the same at w>0 (priced per-step counters unchanged).

## Candidate
`residual-fused-eq-chain` (mode 5 of `collect_residual_core`): identical to `residual-fused-eq` except every restart after the first starts at `previous start + J_k` (one counted addition, k uniform over the 64 jumps; coefficients tracked) instead of `[a]G + [b]Q` (two scalar multiplications).

## Design
Curve p36 (`r = 68719022539`). Arms: `rho` (rho.negation, reference), `res1eq` (baseline), `res1eqc` (candidate), `res1eqaa` (copy of res1eq; control). Also B = 2: `res2eq`, `res2eqc`.
256 targets, target seed 20261701, measure seed 20261702, 2 rounds + 1 warm-up, alternate_measured, local L0 (counts are deterministic; no wall-clock claim; L2 replay owed if the claim stands).
Positive control: harness verification of every answer; `ecbench verify --replay 12`. A/A (res1eq vs res1eqaa) interval must contain 1. All records verified, else void.

## Claims and decision rule (frozen)
- C1 (engineering, confirmatory): paired 95% interval of S(res1eqc)/S(res1eq) lies below 1 at w = 0, 0.1 and 1. Met iff all three hold. Same statement for B = 2 reported separately.
- C2 (descriptive): distance to matched rho of res1eqc at w = 0, 0.1, 1, and as a multiple of the 1.5 floor.
- No threshold is tuned after data; a null (interval contains 1) is reported as a null.
