# One shared F4 system for the 8 cofactor lifts (`pkm-semaev:lifts=2`): protocol

Frozen 2026-11-02, before any data in this directory. Lead D of the Curve25519
index-calculus effort. Component study of one oracle (`pkm-semaev`), unit =
F4 elimination multiply-adds. No rho arm, no cross-method claim, no ranking, no
leaderboard row (lead A: the algebraic arm is far behind the generic baseline in
a full ecbench solve; any gain here is a component improvement only).

## Question

Lead 2 (`research/curve25519_cofactor_algebraic_20261102`) solved one F4 system
per cofactor lift `R + K`, `K in E[8]` (`lifts=1`) and paid 7.9-8.0x the calls
and multiply-adds per target for 5 hits in 1000 targets (blind: 0). Can the 8
lifts be decided by ONE F4 system per target, cheaper than 8 separate systems,
with the same verdicts?

## Design chosen (`lifts=2`) and why

`pkm-semaev` writes `S_3(x_1, x_2, x_R) = 0` with `x_1, x_2` on the tower
`V` and `x_R` a numeric parameter. For `lifts=2` the third abscissa becomes a
free ring variable `X` (the engine's free variable, as chain mode's `u`), and
the system is

    { S_3(x_1, x_2, X) = 0 ,  P_R(X) = prod over distinct x(R+K) of (X - x(R+K)) = 0 }

plus the summand membership (tower) equations, in ONE `f4_tower` call per guess
branch. The solutions are exactly the union of the solutions of the 8 lift
systems, so F4 refutes the shared system iff every lift system is refuted; the
verdict is therefore the `lifts=1` verdict (`lifts=1` stops at its first
liftable lift; the shared system covers all lifts and the grid reading below
takes them in the same order).

- `P_R` is computed numerically per target from the lift list, which is the
  `lifts=1` list (same additions, same dedupe of equal abscissae
  `x(R+K) = x(-R-K)`, same charged group operations). Its construction costs
  `d(d+1)/2` multiplications (d <= 8 roots), counted as the native counter
  `pkm_shared_poly_fp_mults_uncharged` (unpriced, listed, like the existing
  `pkm_system_fp_mults_uncharged`). The lift additions are charged group
  operations in `ops`.
- After a non-refuted system (consistent, or undecided within the F4 bounds) the
  grid of `V^2` is read for each lift abscissa in the `lifts=1` order and the
  first sum equal to its lift target is returned, so the returned relation is
  identical to `lifts=1`'s. Grid multiplications are listed as
  `pkm_grid_fp_mults_uncharged` (it reads up to 8 grids where `lifts=1` reads
  only those of non-refuted lifts; this extra native work is not hidden: it is
  in `native`).
- A trivial fiber (`h = 1`) makes `lifts=2` the blind oracle.

Alternatives considered. (a) A tower for `X`: the lifts `x(R+<Q>)` are the
fibre of a rational 8-isogeny x-map, i.e. PKM's isogeny-tower form with `t = 3`,
whose top value `c = x(phi(R))` is computed per target. This keeps all relations
quadratic and would bound the degree rise by 3, but needs a cyclic rational
`E[8]` and the Velu chain on `E`, new code and a per-target ring change. It is
not built here; it is the next design to try if the univariate one fails on
degree only. (b) Eliminating `X` (resultant of `S_3` with `P_R`): this is the
product of the 8 systems, degree about 8x, rejected. The univariate `P_R` was
chosen because it is generic over any fibre, reuses the tested free-variable
engine path (chain mode) and is what the coordinator note describes.

Counter-argument stated before data. The shared system lives in a quotient
algebra 8x larger than one lift's (`dim = 8 * 2^(2t)`), and F4 elimination cost
grows faster than linearly in that dimension (lead 2's own cells: cost per
target x7.6 and x14 for dimension x4, an exponent of about 1.5-1.9). Eight
separate systems cost 8 * C(N); one system of dimension 8N costs about
C(8N) = 8^(1.5..1.9) C(N) = 23-52 C(N). The refutation also cannot use `P_R`
before degree 8 (its leading monomial is `X^8`), so the solving degree should
exceed lead 2's.

## Arms and cells

Instrument: `curve25519_family f4cost 20 <form> <t> <guess> 200 <seed> 600 2 full
<lifts> - <trace>` (positions: lifts = 11, max_nnz = 12 (`-`), trace = 13).
Cells and seeds are lead 2's, unchanged; targets `[a]G` come from the same LCG
on `<seed>`, so every arm sees the same targets:

| form | t | guess | seed |
|:--|:-:|:-:|:-:|
| isogeny | 4 | 0 | 2026111204 |
| isogeny | 5 | 0 | 2026111205 |
| isogeny | 6 | 0 | 2026111206 |
| bits | 5 | 3 | 2026111205 |
| bits | 6 | 3 | 2026111206 |

Arms per cell: `lifts=0`, `lifts=1`, `lifts=2` (trace off). Extra arm: `lifts=2,
trace=1` on the two cheapest cells (isogeny t = 4, 5), compared with their
`lifts=2` arm. Runs are sequential, interleaved by cell (0, 1, 2 in turn), one
process at a time under `ulimit -v 3500000`, `timeout 1500`, per-cell cap 600 s
of target time (checked between targets). A capped cell is a truncated sample:
it is reported with `targets_done` and flagged, its per-target means are over a
prefix of the same target sequence and are NOT paired with an arm that finished;
a failed or killed cell is reported as failed, never dropped. `uptime` is
recorded before every cell. Wall times are not compared (4-core box shared with
another agent); operation counts do not depend on load.

**Equal base size.** The factor base is a function of (form, t, guess) only, so
all arms of a cell have the same base. The analysis asserts equal `columns`,
`covered_points`, `covered_points_all_cosets` and `covered_projected_points`
across the arms of a cell and aborts otherwise. Cells of different t are never
pooled in a ratio.

## Metrics (per cell and arm)

F4 calls per target; multiply-adds per target; max and mean solving degree
(over refuted calls); matrix size = max over calls of rows, columns, non-zeros
(`max_rows_max`, `max_cols_max`, `max_nnz_max`); undecided count (F4 bound or
budget; the verdict is then read on the grid and stays exact); hits, hits
verified (`[8]Sum = [8]R`), `hits_exact` (`Sum = R`); native counters (P_R
mults, grid mults, lift additions, set-up group operations); and the
per-relation cost two ways: (i) on observed hits = multiply-adds / hits
(undefined at 0 hits, never imputed); (ii) at exact coverage = multiply-adds
per target / coverage, with coverage = `covered_points / r` for `lifts=0` (the
subgroup points that are sums of two base points) and
`covered_projected_points / r` for `lifts=1,2` (the subgroup points R with some
R+K a sum of two base points: what an ideal lift oracle sees); the all-coset
count `covered_points_all_cosets / r` is shown as an upper bound. Ratios
`lifts=2 / lifts=1` and `lifts=2 / lifts=0` for calls, multiply-adds, matrix size.

Validity checks (before data): the unit test below; a frozen-binary identity
check; 1-target smoke with `lifts=2`.

## Preregistered predictions and falsifiers

- **D1 (verdicts).** In every cell where lifts=1 and lifts=2 both finish all 200
  targets, hits and hits_verified are equal. Falsifier: any difference.
  (Guaranteed by construction and by the unit test if the code is right.)
- **D2 (calls).** F4 calls per target of lifts=2 equal the number of guess
  branches reached: 1.0 (isogeny cells) and at most 8 (bits, guess 3), against
  about 8x lifts=0's in lifts=1. Falsifier: calls per target above lifts=0's.
- **D3 (cost, main).** Multiply-adds per target of lifts=2 are HIGHER than
  lifts=1's in every uncapped cell (ratio > 1): one system of an 8x larger
  quotient costs more than eight systems (scaling argument above). Falsifier:
  any uncapped cell with lifts=2 multiply-adds per target below lifts=1's.
  (A capped lifts=2 cell, or a cell where lifts=2 fails to finish, counts as
  support for D3 only if its multiply-adds per target so far also exceed
  lifts=1's; otherwise it is reported as inconclusive.)
- **D4 (solving degree).** Max solving degree of lifts=2 exceeds lifts=0's by
  at least 2 in the isogeny cells, because `P_R` has degree 8. Falsifier: a rise
  of at most 1.
- **Coordinator hypothesis tested alongside** (stated by the task as an example
  and not my forecast): lifts=2 costs less per target than lifts=1 but more
  than lifts=0, with a solving-degree rise of at most 1; refuted by a degree
  rise of 2 or more, or a cost at or above lifts=1. If D3 and D4 hold, this is
  refuted twice.
- **T1 (trace arm).** `trace=1` lowers lifts=2 multiply-adds per target below
  the lifts=2 trace-off arm in both extra cells (learning excluded, reported
  once), as in lead C. Falsifier: no reduction in either cell. A fallback to
  plain F4 is charged.

## Classification (decided before data)

- **Improvement** (component): lifts=2 beats lifts=1 on multiply-adds per target
  in an uncapped cell with equal verdicts and all relations verified. Then
  report against lifts=0 as well; per-relation gain over lifts=0 requires hits
  in both arms or is reported on coverage only, labelled as such.
- Otherwise: **no gain** (a negative result, recorded as such, not an
  improvement); the per-cell numbers stand. Relabelling/accounting is not an
  improvement.

Statistical power: as in lead 2 (lambda about 0.03-0.25 expected exact hits per
200 targets); hit counts are Poisson, the cost side is an exact count.

## Verification of the code (before data)

`cargo test --release --lib pkm_semaev`: all tests pass, including the new
`shared_lift_system_matches_lifts_one_and_the_brute_force_fiber` (h = 8 or 4
curve over F_10007, 40 random plus 12 planted subgroup targets [R with
S = R + K for a base-point sum S]; lifts=2 plain, with guess=1 and with trace=1
against lifts=1 and the brute-force fiber: same verdict, same relation,
`[h]Sum = [h]R`, one F4 call per target and branch, P_R mults listed; h = 1 is
the blind oracle).

## Amendments before data (2026-11-02, after the binary was built, before any protocol cell ran)

1. **Pre-data probes, disclosed.** After D1-D4 and T1 above were written, the
   frozen binary was smoke-tested for identity on isogeny t = 4 (3 targets,
   seed 2026111204, lifts = 0, 1, 2) and probed for cost on 1 target of
   isogeny t = 5 and t = 6 (lifts = 2) to size the caps. Those 1-3-target
   probes showed lifts=2 above lifts=1 and a solving degree of 8 (t = 4), 8
   (t = 5), 10 (t = 6) against 5 for lifts=0 at t = 4. They are not
   protocol data and are not in the tables; they did not change the cells or
   the predictions, but a reader should know they preceded the run.
2. **Prefix-matched re-runs for capped cells.** lifts=2 costs about 1.6e10
   (t = 5) and 2.8e11 (t = 6) multiply-adds per target, so its cells will hit
   the 600 s cap with fewer than 200 targets. For every cell where the lifts=2
   arm is capped (or killed after finishing some targets, which cannot be
   known from a killed run: then see below), lifts=0 and lifts=1 are re-run
   with `targets = targets_done` of the lifts=2 arm and the same seed (the
   LCG gives the same target prefix), so the three arms are paired on the same
   targets. These runs are labelled `prefix` and the 200-target runs stay in
   the tables unchanged. D3 is judged on the prefix-matched rows in a capped
   cell, on the 200-target rows otherwise.
3. **A killed or failed run** (timeout 1500 s, `ulimit -v 3500000` memory
   limit) has no JSON; it is reported as failed with its exit code and the
   cell is stated as "lifts=2 did not finish one target within the limit". It
   counts as support for D3 (cost above lifts=1's by more than the limits
   allow) only in that the limit was hit, and is reported as inconclusive on
   the size of the ratio.
4. The mean solving degree and matrix size come from `solving_degree_max`,
   `solving_degree_mean` and `solver_extra` in the JSON, which this branch
   adds to `f4cost` (the earlier leads' files do not have them).
