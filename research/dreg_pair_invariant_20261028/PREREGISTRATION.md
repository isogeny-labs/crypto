# Pre-registration: does the summand-swap invariant `p = x₁x₂` lower the closure degree of the chained S₃ descent?

Frozen 2026-10-28 before any closure was built on the new arms. The only
thing run before this commit is the construction check below, which builds
no Macaulay matrix.

## Question

The degree-closure studies show the MutantXL closure of the m = 3 chained S₃
system refutes at degree 4 for ℓ ≤ 6. At ℓ = 7 it does not refute at degree 4
(`research/dreg_closure_scale_20261016`, cell (21, 7): 4 of 4 draws). Degree
4 or 5 is the cost driver of every degree-of-regularity row on the
ECC2K-130 descent.

The repository has measured symmetry on the unsplit S₄/S₅ systems, on the
2-torsion frame and on Frobenius orbits of the factor base. It has never
measured symmetry inside the chained descent these studies use (search
summary: `RESEARCH_EXOTIC_COORDINATES.md` §8/§17/§18,
`ic_symmetry_lever_slope_20260929`, `lit2026_w8_summation_models_20261011`,
`pdp_split_slots_20261012`: all constant levers or negatives, none on the
chain).

The chain at m = 3 is `S₃(x₁, x₂, e₁)`, `S₃(e₁, x₃, x(R))`, with

    S₃(x₁, x₂, x₃) = (x₁+x₂)²x₃² + x₁x₂x₃ + (x₁x₂)² + b.

Squaring is F₂-linear, and `x(R)` is a constant. So the second link is
quadratic in the Boolean unknowns, and every cubic monomial of the system
comes from `x₁x₂e₁`. The first link is invariant under swapping the two
summands. That swap is the system's one summand symmetry, and its invariants
are `s = x₁ + x₂`, which is linear, and `p = x₁x₂`. In those invariants the
first link is

    s²e₁² + p·e₁ + p² + b,

which is quadratic and sees the summands only through `s`.

**Question: does adding the invariant `p` as unknowns let the closure refute
at a lower degree than the plain system, and is any gain due to the
invariant rather than to quadratisation alone?**

## Arms (`examples/dreg_invariant.rs`, same draw, same field, same `b = 1`)

| arm | unknowns | equations | max degree |
|---|---|---|---|
| **B** plain chain (`build_decomposition_system`) | `n + 3ℓ` | `2n` | 3 |
| **Q** invariant `p` over a basis of `W = span{v_i v_j : i ≤ j}` | `n + 3ℓ + dim W` | `3n` (two links + `p = x₁x₂`) | 2 |
| **R** matched control: `r = x₁e₁` as `n` free unknowns; cubic term `x₂·r` | `n + 3ℓ + n` | `3n` (two links + `r = x₁e₁`) | 2 |

- `x₁x₂` always lies in W, so each root of B extends to exactly one root of Q
  (and of R), and each root of Q or R restricts to a root of B. The three
  arms have the same satisfiability.
- **R** is the control for "quadratisation alone". It adds a comparable
  number of unknowns (exactly the same at (21, 7)) and leaves every equation
  quadratic, but `r` is not invariant under the swap.
- Q's degree-D row space contains B's link-1 equations at degree 3: B's link
  1 minus Q's is `def·e₁ + def²`. So Q cannot refute at a *higher* degree
  than B for that reason alone. Whether it refutes at a lower degree is open.

## Construction check, run before this commit

- `--check` verifies, as exact polynomial identities on every draw, that
  B-link₁ − Q-link₁ = `def·e₁ + def²` and B-link₁ − R-link₁ = `x₂·def`, and
  that link 2 equals B's term for term.
- It passed on 22 of 22 (arm, draw) systems with this binary
  (`runs/identity-check.jsonl`, `runs/binary.sha256`): Q has 60–63 unknowns,
  R has 62–63, and both have max degree 2.

## Cells and draws (committed, not regenerated)

| cell | file | unsatisfiable draws | satisfiable draws | B committed |
|---|---|---|---|---|
| (22, 6), N_B = 40, S = +4 | `research/dreg_closure_driver_20261016/draws/m3-22-6.jsonl` | 0, 1, 2, 3 | none in file | closure refutes at 4 on 4/4 (driver RESULTS) |
| (21, 7), N_B = 42, S = 0 | `research/dreg_closure_scale_20261016/draws/m3-21-7.jsonl` | 0, 2, 3, 6 | 1, 4, 5 | closure does not refute at 4 on 4/4 (scale RESULTS) |

The 64-variable mask limits the arms to these cells: the next ℓ = 7 cell
would need more than 64 unknowns in Q or R.

## Measurement (`run.sh`)

All dense scans use `closure_refutation_scan` (MutantXL, full), with
`--max-cols 130000`, `prlimit --as=15e9` and a 2 h timeout, one process per
(arm, draw, step).

1. **control:** B, dense, degrees 3–4, every unsatisfiable draw of both cells.
2. **primary:** Q and R, dense, degree 3 (≤ 41,728 columns), every
   unsatisfiable draw.
3. **sound:** Q and R, dense, degree 3, the satisfiable draws 1, 4, 5 of
   (21, 7).
4. **secondary:** for each (arm, draw) that did not refute at degree 3 in
   step 2, the sparse closure at degree 4 (`--sparse 4 --max-entries
   1000000000`). For each arm that runs step 4 in (21, 7), the satisfiable
   draws 1, 4, 5 also run sparse degree 4.
   - A capped run, or one killed by the memory limit or the timeout, is
     **unmeasured**.
   - Dense degree 4 for Q and R needs C(63, ≤ 4) = 637,393 columns and is
     out of reach.

Scoring is native (`dreg_score` is not extended). The verdicts below are read
off the committed JSON lines.

## Validity (all must hold, or the study is reported invalid)

- **V1 identity:** the construction check is true on every system scored.
- **V2 positive control:** B reproduces its committed records: (22, 6)
  refutes at degree 4, not 3, on 4 of 4; (21, 7) saturates degree 4 without
  refuting on 4 of 4.
- **V3 soundness:** no Q or R run refutes a satisfiable draw at any degree
  measured.

## Hypotheses and verdict rules, per cell (4 unsatisfiable draws)

D_X is arm X's measured closure degree on a draw (3, 4, or "> measured").

- **H_lower (invariant lowers the degree):** D_Q < D_B on 4 of 4 draws. At
  (22, 6) this means D_Q = 3. At (21, 7), B does not refute at 4, so this
  means D_Q ≤ 4.
- **H_none:** D_Q ≥ D_B on 4 of 4 draws (at (21, 7): Q does not refute at 4,
  or is unmeasured at 4).
- **mixed:** anything else, reported as counts.
- **Mechanism, read only where H_lower or mixed holds:**
  - "invariant-specific" if D_Q < D_R on ≥ 3 of 4 draws.
  - "quadratisation" if D_R ≤ D_Q on ≥ 3 of 4 draws.
  - "unresolved" otherwise, or if R is unmeasured where Q refuted.
- **Cost reading (stage level only):** columns at the refuting degree,
  `C(N_occ, ≤D)`, Q against B. At (21, 7), B is bounded below by degree 5
  (C(42, ≤ 5) = 974,982). Reported with closure word ops and wall time. No
  `S`, no rho ratio, no leaderboard row.

## Predictions (written before any closure on Q or R)

- **(22, 6):** P(D_Q = 3 on 4 of 4) ≈ 0.4.
  - Q is quadratic with 66 equations in 60–61 unknowns. Its degree-3 span
    holds B's link-1 equations, but not B's degree-4 products by linear
    monomials.
  - My central guess is that Q refutes at 4, not 3: H_none at (22, 6), with
    a column cost about 5.5 times *higher* than B's (C(61, ≤ 4) against
    C(40, ≤ 4)).
- **(21, 7):** P(Q refutes at ≤ 4 on 4 of 4) ≈ 0.25.
- **R:** I expect R to behave like Q (quadratisation, not invariance) with
  probability ≈ 0.6.

## What it would mean for ECC2K-130

- At m = 4 the chain has a second cubic link, `S₃(e₁, x₃, e₂)`. Its cubic
  term `e₁x₃e₂` carries no summand-swap symmetry and would need a
  non-invariant auxiliary of `n` unknowns, which is arm R's construction.
  This study measures whether either kind of auxiliary moves the degree at
  m = 3. It does not price m = 4.
- A degree drop with fewer columns, on matched draws, would be a measured
  stage-level improvement over the index-calculus baseline under the
  success criterion. It would still need a priced, rho-matched row before
  any leaderboard entry.
- A degree drop with *more* columns, or no drop, is a negative and closes
  this lever for the chain at m = 3.

## Scope

m = 3, two cells, 4 unsatisfiable draws each (plus 3 satisfiable draws), one
4-core 16 GB container that is not isolated. Conductor is not installed in
this container, so no task was taken.

## Amendment 1 (2026-10-28T17:12Z, after the first secondary run, before scoring)

- **What happened.** The first secondary run (Q, (22, 6), draw 0, sparse
  degree 4) hit `--max-entries 1000000000` after 6,553 s at a peak RSS of
  12.0 GB. Under the rules above that run is **unmeasured**: a capped run
  reports no refutation reading in either direction.
- **The change.** The secondary step stops there. The remaining 21 runs (15
  unsatisfiable and 6 satisfiable) are not run.
  - At about 1.8 h each they would take about 40 h, and on this 16 GB
    container they would hit the same entry cap: Q and R have 60–63
    unknowns, against the 61 of the run that capped.
  - The second run was stopped 39 s after it started. Its empty outputs are
    in `runs/stopped-by-amendment-1/`.
- **Effect on the verdicts: none.** The verdict rules already fix them
  without degree 4.
  - At (22, 6), H_lower needs D_Q = 3.
  - At (21, 7), the registered H_none explicitly includes "unmeasured at 4".
  - Degree 4 for Q and R is reported as unmeasured on every draw.
- **What I knew when deciding.** No Q or R degree-4 reading exists. The only
  output was `"capped":true`.
