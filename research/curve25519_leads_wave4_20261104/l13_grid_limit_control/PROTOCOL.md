# L13 protocol: the grid-limit control on the T10/T12 winner's base (k = 20, isogeny factor base)

Wave 4, lead L13. Written 2026-11-04 from existing data only, before any L13 data.
**Status: design (preregistered). Nothing below changes after the first run; deviations go to a
dated addendum.** No git in this lead (the user opens the PR after validation).

**Class.** An end-to-end ecbench control measurement of the algebraic PKM arm: full discrete-log
solves in counted units (GAE, F_p multiplications, F4 mul-adds), priced across the §5a band
(w = 0, 0.1, 1 per unpriced native operation). It is a *control*, not an improvement claim: it
places the best F4 arm against the cheapest possible oracle on its own base. Not a leaderboard
row. Rho remains ahead on every track; the distance remaining is stated, not gated.

Parents: T10 (`research/curve25519_leads_wave3_20261103/t10_seed_and_guess`, winner t = 5,
G = 4, seed 1307: 22 columns, 210 covered, 52,816 F4 mul-adds per target) and T12
(`research/curve25519_t10_winner_ecbench_20261104`, end to end: S(winner) = 26.40 / 100,277 /
1,002,534 at w = 0 / 0.1 / 1; S(winner)/S(t6, G4, seed 0) = 0.0578 [0.047, 0.072] at w ≥ 0.1;
6.06·10⁴× behind rho-neg at w = 0.1, 17.1× at w = 0).

## 1. Question

The F4 oracle `pkm-semaev:m=2,mode=full,guess=4` decides, for a walk point R, whether
R = ±P_i ± P_j for base points on the 22-abscissa isogeny base (|V| = 2^t = 32 tower abscissae,
22 of them on the curve, 44 points). The same decision has two cheaper exact implementations in
the harness, on the very same base and with the same verdicts:

- the **grid** (`engine=exhaustive`, `pkm_semaev.rs::grid`, m = 2): every unordered pair of
  V² against S₃(x₁, x₂, x_R) = 0, **n² + 7n = 1,248 F_p multiplications per target** for n = 32
  (6 per row of the outer loop, 2 per unordered pair including the diagonal; the T12 records
  confirm it: the F4 arm's `pkm_grid_fp_mults_uncharged` is exactly 1,248 per consistent system);
- the **generic pair table** (`mitm:negation_folded=1`, `PairTable::build_negation_folded`):
  506 charged group additions once per solve (Σ_{i<22} 2·(22 − i)), then **one hash lookup per
  target**.

The F4 arm spends 52,816 mul-adds (T10 `mpt`; 51,207 solver mul-adds + 217 specialisation
mults per trial measured in T12) on the same decision. The grid and the table therefore bound,
from below, what any algebraic engine on this base can cost at w > 0, while the base itself
(22 columns, coverage 210/r) fixes the group-operation cost at w = 0.

**Q. How far above its own grid limit is the best F4 arm, end to end, across the band; does
the generic pair table on the same base sit below the grid; and is even the cheapest oracle on
this unprojected base still behind proj-a (the 32-column projected base) at every weight?**

This lead frames the honest position of the algebraic track: the solver's distance to its own
brute-force floor, and the base's distance to the generic baseline.

## 2. Inputs computed now from existing data (no new measurement)

Instruments (frozen; `scripts/run.sh` refuses to run on another hash):
- `/home/user/ctarget/release/ecbench`, sha256
  `54737834d048b22e77399160fa9e159782b1124ea8190d84dba185309229641b` (= T12's and L12's).
  `ecbench plan --spec ecbench/spec.json` accepts the L13 spec (`design/plan_design_check.txt`:
  160 executions, 16 workloads; the grid arm resolves to
  `pkm-semaev:m=2,mode=full,engine=exhaustive`, the table arm to `mitm:negation_folded=1` on
  `prime-isogeny:t=5,seed=1307`). `plan` resolves methods without running the oracle's
  parameter parser; a run-time rejection is handled in section 5.
- Spec `ecbench/spec.json`, sha256 `b83a07cc625d75554e54540b3922d6e367e68aa4d7abeb14e66a2cc7dc9b8faf`.
- Census row (`research/curve25519_isogeny_seed_20261103/data/k20.jsonl`, seed 1307, t = 5):
  columns 22, base_points 44, covered_points 210, J 210.
- Scripts: `scripts/chunked_run.py` is a verbatim copy of T12's (sha256
  `e8edaa81515b400bcb29e402f6ff0d8548001498aef662c23f47daf3d19aabde`). `scripts/pooled_compare.py`
  is T12's copy (sha256 `061b11b0…dfdf9`) with one change: the solver term is recognised by the
  prefix `solver_` instead of `solver_F_p_multiply-adds`, because the grid arm's solver block has
  `op_unit` "F_p multiplications (grid)" and `ops = 0` (its multiplications are on
  `pkm_grid_fp_mults_uncharged`, so nothing is double-priced); this is exactly what
  `ecbench/band.rs::unpriced_counts` does. **Positive control (done, `design/cells.json`):** the
  patched copy reproduces T12's frozen `pooled_compare.jsonl` (12 pairs: ratio and both interval
  ends) with maximum relative difference 0.0.
- Seed pairing (checked in T12's records): every arm in a (workload, round) carries the same
  `algorithm_seed`, so arms with the same base and the same exact verdicts walk the same trial
  sequence and find the same relations.

T12 measured means over 32 runs (16 targets × 2 rounds), √r = 362.22, from
`scripts/cells_from_t12.py` → `design/cells.json`:

| arm | mean GAE | mean trials | mean relations | trials/relation | S(w=0) | S(w=0.1) | S(w=1) |
|:--|--:|--:|--:|--:|--:|--:|--:|
| pkm-iso-t5-g4-s1307 (F4 winner) | 9,564 | 7,061 | 10.84 | 651 | 26.40 | 100,277 | 1,002,534 |
| proj-a | 4,576 | 1,554 | 22.25 | 69.8 | 12.63 | 13.63 | 22.55 |
| rho-neg | 559 | — | — | — | 1.544 | 1.654 | 2.651 |

Per-trial unpriced work: F4 arm 51,424 (solver + specialisation); grid 1,248; table 1.
The F4 arm's phase GAE: factor_base 932, oracle_setup 0, relations 8,610, verify 23.

**Model cells for the new arms** (per T12 run: same GAE and trials as the paired F4 run; grid
unpriced = common natives + 1,248 × trials; table GAE + 506, unpriced = common natives + 1
lookup × trials; averaged over the 32 runs):

| arm (modelled) | S(w=0) | S(w=0.1) | S(w=1) |
|:--|--:|--:|--:|
| grid-iso-t5-s1307 | 26.40 | 2,461 | 24,375 |
| mitm-iso-t5-s1307 | 27.80 | 31.76 | 67.39 |

Modelled ratios (ratio = S(b)/S(a)) at w = 0 / 0.1 / 1:

| pair | w = 0 | w = 0.1 | w = 1 |
|:--|--:|--:|--:|
| grid / F4 | 1.000 | 0.0245 | 0.0243 |
| mitm / grid | 1.053 | 0.0129 | 0.0028 |
| grid / proj-a | 2.09 | 181 | 1,081 |
| mitm / proj-a | 2.20 | 2.33 | 2.99 |
| grid / rho-neg | 17.1 | 1,488 | 9,194 |
| F4 / grid (distance above its own grid limit) | 1.00 | 40.7 | 41.1 |

The new session uses new targets (seed base 2026110431), so the absolute S values will differ
from T12's; the ratios between same-base arms do not depend on the targets beyond the per-target
constant costs (932 + 506), and the rules below are written on ratios.

**Model reason for the base comparison (P5, P6).** At w = 0 an IC arm's cost is the walk:
≈ relations × r/covered group operations plus its per-solve constants. proj-a's 32 points are
cofactor-projected into the order-r subgroup, so every one of its ≈ 2 × 32² /2 ≈ 2,000 signed
pair sums lies where the targets live: its coverage sits at the pairs/r ceiling (measured 69.8
trials per relation ≈ r/1,880). The unprojected isogeny base takes its 44 points from the
tower x(R′ + ⟨Q⟩) on the whole curve (order 8r): of its ≈ 22² + 22 = 506 signed pair sums only
210 fall in the subgroup — the rest pay the cofactor cosets — so it needs 651 trials per
relation. Fewer relations (10.8 against 22.3) do not make up for 9.3× more trials per relation.
Hence any oracle on this base, however cheap, is ≈ 2.1–2.2× behind proj-a at w = 0, and the
table arm (whose unpriced work is one lookup a trial, like proj-a's) stays 2.2–3.0× behind it at
every weight.

## 3. Design

### 3.1 Arms (`ecbench/spec.json`, k = 20 curve of T12: p = 1,048,573, r = 131,203, cofactor 8)

| arm | role | factor base | oracle |
|:--|:--|:--|:--|
| rho-neg | reference | — | `rho.negation` |
| proj-a | baseline | `prime-projected-abscissa:size=32,scan_cap=512` | `mitm:negation_folded=1` |
| pkm-iso-t5-g4-s1307 | candidate (T12 winner, unchanged) | `prime-isogeny:t=5,seed=1307` | `pkm-semaev:m=2,mode=full,guess=4` |
| grid-iso-t5-s1307 | candidate (grid limit) | `prime-isogeny:t=5,seed=1307` | `pkm-semaev:m=2,mode=full,engine=exhaustive` |
| mitm-iso-t5-s1307 | candidate (generic table on the base) | `prime-isogeny:t=5,seed=1307` | `mitm:negation_folded=1` |

`guess` is not given on the grid arm: the exhaustive engine enumerates all of V² (no branch
split), which is the brute-force reference the F4 arm's 16 branch systems replace.

### 3.2 Measurement: 16 one-target chunks, 2 rounds, `alternate_measured`

- 16 public targets, `target_seed` base **2026110431**, measurement seed base **2026110432**,
  2 measured rounds, `alternate_measured`, timeout 1,800 s, warmup 0, isolation L0.
- Run through `scripts/chunked_run.py` (T12's Addendum C driver, verbatim): chunk c
  (c = 0 … 15) is a complete one-target session with all 5 arms and both rounds, at
  `target_seed` 2026110431 + c and measurement seed 2026110432 + c; a complete chunk is skipped
  on relaunch, a killed one is moved to `interrupted/` (its `session.json` renamed) and re-run
  from scratch; `--wait --allow-busy` behind `/tmp/crypto-bench.lock` (T11's 6-chunk session
  may hold it; L13 waits). 32 runs per arm, 160 executions in all.
- One heavy process at a time: the driver runs one session at a time; nothing else is launched
  by this lead while it runs.

`scripts/run.sh` executes exactly this, in this order, and is re-launchable after a container
restart; the analysis and `ALL_DONE` are written only when all 16 chunks are complete.

### 3.3 Analysis on the 16 pooled chunks (frozen)

- `ecbench verify --replay 1` on every chunk (`data/ecbench/verify/`).
- `ecbench band --dir <16 chunks> --ranks` at w = 0, 0.1, 1 (`band_pooled.{txt,json}`).
- Paired ratios by `scripts/pooled_compare.py` (K = 16 workloads, df = 15, linearised t, 95%),
  ratio = S(b)/S(a), for (a, b) = (F4, grid), (grid, mitm), (proj-a, grid), (proj-a, mitm),
  (rho-neg, grid), (rho-neg, mitm), (rho-neg, F4), (proj-a, F4), (F4, mitm) at each w
  (`pooled_compare.jsonl`).
- `scripts/identity_check.py` (`identity_check.json`): per (workload, round), `trials`,
  `relations` and `total_gae` across the three same-base arms, the table's 506-addition offset,
  the grid arm's `solver.ops` and `pkm_grid_fp_mults_uncharged = 1,248 × trials`.

## 4. Predictions and decision rules (decided on the files named above; nothing is retuned)

- **P1 (verification).** Every arm verifies 32/32 (`band_pooled.json`: `verified = runs = 32`
  for all five arms) and every chunk's `verify --replay 1` exits 0 (`verify/rc.txt`).
- **P2 (grid limit, the brief's rule).** S(grid)/S(F4 winner) ≤ 0.1 at w = 0.1 and at w = 1,
  with the 95% upper bound < 1 (pooled_compare, a = pkm-iso-t5-g4-s1307, b = grid-iso-t5-s1307).
  Point prediction 0.0245 (w = 0.1) and 0.0243 (w = 1); exploratory interval, not a rule:
  [0.020, 0.030]. Equivalently the F4 arm is ≥ 10× above its own grid limit (model: 41×).
- **P3 (w = 0 identity on the same base).** For every (workload, round) the F4, grid and table
  arms report identical `trials` and identical `relations`, and `total_gae(grid) =
  total_gae(F4)` exactly, so the pooled w = 0 ratio grid/F4 is 1.000 (`identity_check.json`:
  `trials_identical = relations_identical = gae_grid_eq_f4 = pairs = 32`). If
  `seeds_paired = false` (the plan did not pair algorithm seeds across arms), P3 is declared not
  testable as stated and replaced, before looking at the ratio, by: w = 0 ratio grid/F4 in
  [0.9, 1.1] with the 95% interval covering 1.
- **P4 (the generic table sits below the grid; the stated rule).** S(mitm on the base)/S(grid)
  ≤ 0.05 at w = 0.1 (point 0.013) and ≤ 0.01 at w = 1 (point 0.0028), each with the 95% upper
  bound < 1; at w = 0 the pooled ratio is in [1.02, 1.10] (point 1.053: the table's 506 charged
  additions against the walk's ≈ 9,600 GAE), and for every (workload, round)
  `total_gae(mitm) − total_gae(grid)` lies in 506 ± 2 × relations (`mitm_minus_grid_in_band =
  pairs`). Band verdict: "mitm-iso-t5-s1307 vs grid-iso-t5-s1307" **unresolved inside the band**
  (the order flips between w = 0 and w = 0.1).
- **P5 (the grid arm on the unprojected 22-column base is behind proj-a at every weight; the
  stated rule).** S(grid)/S(proj-a) ≥ 1.5 at w = 0 with the 95% lower bound > 1 (point 2.09,
  T12's F4/proj-a = 2.09 [1.78, 2.45] is the same quantity), ≥ 50 at w = 0.1 (point 181) and
  ≥ 300 at w = 1 (point 1,081); band verdict "proj-a < grid-iso-t5-s1307" at every weight.
  Model reason: section 2 (pairs/r ceiling against the cofactor cosets).
- **P6 (the base alone, same oracle as proj-a).** S(mitm on the base)/S(proj-a) ≥ 1.5 at every
  weight with the 95% lower bound > 1 (points 2.20, 2.33, 2.99); band verdict "proj-a <
  mitm-iso-t5-s1307" at every weight. This isolates the factor-base effect from the oracle
  effect: with the oracle held fixed, the 22-column isogeny base loses to the 32-column
  projected base.
- **P7 (rho).** rho-neg is ahead of every IC arm at every weight (band ranks), and the grid
  arm's distance to rho-neg at w = 0.1 is reported as a number (point 1.49·10³×; at w = 0
  17.1×, at w = 1 9.2·10³×). The F4 winner's distance to rho-neg is re-measured on these targets
  (T12: 6.06·10⁴× at w = 0.1).
- **P8 (grid bookkeeping).** In every grid run `solver.ops = 0` and
  `pkm_grid_fp_mults_uncharged = 1,248 × trials` (`grid_solver_ops_zero =
  grid_mults_eq_1248_trials = pairs`), so the grid's work is priced exactly once by the band.

## 5. Failure handling

- A failed prediction is reported as failed, never retuned; the numbers stay in the report.
- ecbench rejects the spec at `plan`: `data/ecbench/GRID_REJECTED` (`rejected_at_plan`) and
  the lead stops with the rejection text quoted. An error outcome on the grid or table arm in
  chunk c00 (for instance the oracle refusing `engine=exhaustive` on this base): `GRID_REJECTED`
  (`error_in_c00`) is written, the error is quoted, and the remaining arms' pooled results are
  reported as what they are (a partial control); no ratio involving a failed arm is quoted.
- Container restarts (every 40–60 min): relaunch `scripts/run.sh`; no killed chunk is cited;
  interrupted chunks stay in `interrupted/`.
- A timeout (1,800 s) on any run makes that run unverified: P1 fails and the arm's row is
  reported with its `verified/runs`; pooled_compare asserts all-verified and will refuse the
  pair, which is reported as "not computable" for that pair.
- P3 fails with `seeds_paired = true` (trials or relations differ between same-base arms): a
  verdict disagreement between F4, the grid and the table on the same base. That is a
  correctness finding about one of the oracles and is reported as such; no ratio from the
  disagreeing pairs is quoted as a cost result.
- The host is shared (4 cores; T11's ecbench session and T12's verify may be in flight): S,
  GAE, F_p multiplications and mul-adds are counted units; no wall-time claim; `wall_ns` is
  recorded and not interpreted.
- Writes only inside this lead directory; no git; no rebuild of the prebuilt binaries.

## 6. Scope and distance remaining

- Nothing here is an improvement: a grid that enumerates V² and a hash table on the same base
  are the floors the algebraic engine has to beat on its own base, not new methods. The result
  is the F4 arm's **distance above its own grid limit** (model 41× at w ≥ 0.1, 1× at w = 0),
  and the base's distance to the generic baseline (model 2.1–3.0× behind proj-a for the
  cheapest oracle on it).
- Distance remaining to rho-neg (model, to be replaced by the measured numbers): the grid limit
  of this base is 17.1× behind at w = 0, 1.5·10³× at w = 0.1, 9.2·10³× at w = 1; the table on
  this base 18×, 19×, 25×; the F4 winner 17.1×, 6.1·10⁴×, 3.8·10⁵× (T12). Rho remains ahead on
  every track at every weight. No leaderboard row; no `benchmarks/` entry; this is a wave-4
  control inside the Curve25519 index-calculus lab.
- Reading intended for the report: the lever that would move the algebraic track past proj-a at
  k = 20 is not the solver alone. Even at zero solver cost the walk on this base (26.4 at w = 0)
  is 2.09× behind proj-a and 17× behind rho-neg; the solver's 41× is on top of that. Coverage
  per column (projection into the subgroup, or a base whose pair sums stay in it) and the solver
  cost per trial are both needed, and this control measures how much each is worth.

## 7. Files

- `ecbench/spec.json` (frozen spec), `design/plan_design_check.txt` (dry `ecbench plan`),
  `design/cells.json`, `design/cells.out` (cells computed from T12 data, with the pooled_compare
  positive control).
- `scripts/run.sh`, `scripts/chunked_run.py`, `scripts/pooled_compare.py`,
  `scripts/identity_check.py`, `scripts/cells_from_t12.py`.
- Outputs under `data/`: `run.log`, `env.txt`, `ecbench.sha256`, `ecbench/spec.frozen.json`,
  `ecbench/plan.txt`, `ecbench/chunks/c00..c15` (+ `specs/`, `interrupted/`, `driver.log`),
  `ecbench/verify/`, `ecbench/band_pooled.{txt,json}`, `ecbench/pooled_compare.jsonl`,
  `ecbench/identity_check.json`, `SHA256SUMS`, `ALL_DONE`.
