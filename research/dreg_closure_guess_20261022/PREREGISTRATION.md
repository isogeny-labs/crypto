# Pre-registration: do four guessed variables bring ℓ = 8 back to degree 4?

Registered 2026-10-22, before any ℓ = 8 draw below was generated or
measured. It is the successor named by the specialisation study
(`research/dreg_closure_specialise_20261017/RESULTS.md`).

## Why

- **At ℓ = 7 the degree-4 failure is marginal.** The specialisation study
  found that fixing any single variable let the MutantXL closure refute at
  degree 4 on 6 of 8 draws, and both branches of one guess refuted on 5 of
  8.
- **The cost model now has a guess-then-degree-4 option.** With k guessed
  variables the oracle per target is `2^k · C(N−k, ≤4)^w`, against
  `C(N, ≤5)^w` for degree 5. What sets its price is how k grows with ℓ.
- **Two readings of ℓ = 7 predict different k at ℓ = 8:**
  - **K_excess:** k grows like the excess over ℓ = 6 per summand,
    `m·(ℓ−6)`: 6 at m = 3, ℓ = 8. Four guesses do not suffice.
  - **K_marginal:** each step in ℓ costs about one guess per summand above
    ℓ = 7 plus the one that ℓ = 7 needed, `m·(ℓ−7)+1`: 4 at m = 3, ℓ = 8.
    Four guesses suffice.
- **Why k = 4 and no fewer.** The dense closure is capped at 130,000
  columns in this container (16 GB). At m = 3, ℓ = 8, n = 22 the system has
  N = 46 unknowns, so a branch with k guesses has `C(46−k, ≤4)` columns:
  179,447 at k = 0, 136,698 at k = 3 and 124,314 at k = 4. k = 4 is the
  smallest k measurable here. The unspecialised system is therefore not
  measured at degree 4; at ℓ = 7 it did not refute on 8 of 8 draws.

## Draws (frozen before generation)

- **Cell:** m = 3, n = 22, ℓ = 8; N = 46 unknowns, 46 equations, surplus
  S = n − mℓ = −2 (the surplus of the ℓ = 7 cell (19, 7)).
- **Command:** `dreg_ladder --cells 22:8:3 --m 3 --unsat 4 --ffd-max 3
  --seed 20261022`, writing `draws/m3-22-8.jsonl`.
- **Draws:** the first 4 unsatisfiable draws, each certified by exact
  solution count (0 solutions).

## Arms (per draw; each branch a separate process)

Variables for m = 3, ℓ = 8: summand `i` has coordinates `8i … 8i+7`, and
the intermediate point's coordinates start at 24.

| arm | variables fixed | branches | summand dimensions | N after |
|---|---|--:|---|--:|
| G4 | 7, 15, 23, 24 | all 16 value assignments, `G4b<B>`, bit j of B is the j-th variable | 7, 7, 7 (one intermediate coordinate fixed) | 42 |
| G6 (positive control) | 6, 7, 14, 15, 22, 23, all 0 | 1 | 6, 6, 6 | 40 |

- **G4 is the guess.** A one-variable fix on each summand (bringing it to
  ℓ = 7's dimension) and one intermediate coordinate, the fix that worked as
  often as any other at ℓ = 7.
- **G6 is the positive control.** It confines every summand to dimension 6,
  where native cells refute at degree 4 and the ℓ = 7 analogue (arm B)
  refuted on 8 of 8. If G6 fails, specialisation does not emulate a smaller
  ℓ here and G4 is uninformative.
- **Every specialisation of an unsatisfiable system is unsatisfiable**, so
  every branch is a well-posed refutation problem.

## Instrument

- `examples/dreg_closure --fix …` (unchanged since the specialisation
  study), run as `prlimit --as=15000000000 timeout 7200 dreg_closure --to 4
  --max-cols 130000 --draw K --fix …`.
- **Order.** G6 on all draws first, then G4 branch-major (branch 0 on every
  draw, then branch 1, …).
- **Early stop.** Once a draw has a G4 branch that does not refute, its
  remaining branches are skipped: the draw's verdict is already fixed.
- Scored natively by `dreg_score guess runs/*.jsonl`.

## Verdict rules

Per run: **refutes at 4** (closure refutes at degree 3 or 4), **does not**
(degree 4 saturated without `1`) or **unmeasured** (killed or cut).

Per draw: four guesses **suffice** if all 16 G4 branches refute at 4;
**do not suffice** if any branch does not; **incomplete** otherwise.

- **Control valid** when G6 refutes on ≥ 3 of the 4 draws.
- If valid:
  - **K_marginal favoured ("four guesses suffice at ℓ = 8")** if ≥ 3 of the 4
    draws suffice;
  - **K_excess favoured ("four guesses do not suffice")** if ≥ 3 do not;
  - **mixed** otherwise.
- If not valid: **uninformative**, with every run shown.

## Predictions

- G6 refutes: high confidence.
- G4: no prior between K_excess and K_marginal (50/50).

## Scope and class

- **Class:** stage diagnostic. No `S`, no rho ratio, no speed claim.
- **Container:** 4-core, 16 GB, not isolated (`AGENTS.md` §10).
- **What it can move.** Only the extrapolation rule for k(ℓ) in the
  guess-then-degree-4 option. It cannot move any ECC2K-130 row below rho:
  the free-oracle floor binds every row whatever the degree.
- **One guess set.** G4 is one choice of four variables; a failure says
  this set does not suffice, not that no four-variable set does.
- **Conductor:** no task was taken, because the CLI is not installed in
  this container.

## Amendment 1 (2026-10-22, before any closure run)

- **Equation count.** The cell has N = 46 unknowns and **44** equations
  (2n), not 46 as written above. The surplus S = n − mℓ = −2 is as
  registered. Recorded by the generator (`"n_eqs":44`); nothing else
  changes.
- **Draw generation is sharded.** A container restart killed the
  sequential generator after draws 0–2 (draw 0 unsatisfiable, 1 and 2
  satisfiable, kept in `draws/m3-22-8.jsonl`). Each unsatisfiable draw takes
  about 22 minutes single-threaded, so the remaining draws are generated by
  `dreg_ladder --shard I/4 --first-draw 3 --max-draws 16` for I = 0…3, in
  parallel, into `draws/shard-I.jsonl`. `--shard` replays the same seeded
  sequence and measures only draws `≡ I (mod 4)`, so every draw is the one the
  sequential run would produce at that index. The frozen draw set is
  unchanged: the first 4 unsatisfiable draws in draw order, merged into
  `draws/m3-22-8.jsonl`.
