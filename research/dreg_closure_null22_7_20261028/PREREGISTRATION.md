# Pre-registration: is the three-guess failure at ℓ = 8 just ℓ = 7?

Registered 2026-10-28, before any (22, 7) draw was generated or measured.
Successor of `research/dreg_closure_guess3_20261023/` (RESULTS.md there) and the
top-ranked lead of `research/notes/ecc2k130/RESEARCH_ECC2K130_NEW_LEADS_20261023.md`.

## Why

- **The three-guess result.** At m = 3, ℓ = 8 (n = 22), guessing variables 7, 15
  and 23 left every summand in a 7-dimensional subspace or coset, N = 43. The
  degree-4 MutantXL closure then failed to refute on draws 0, 3 and 5 and
  refuted on draw 6 (all 8 branches). The previous study's four-variable guess
  refuted everywhere.
- **The reading, not yet tested.** That 3-set is structurally an ℓ = 7 system, and
  the unspecialised ℓ = 7 draws of the closure-at-scale and closure-driver studies
  (N = 40 and 42) did not refute at degree 4 on 8 of 8 draws. If the guess effect
  is "just ℓ", then an **unspecialised** (n, ℓ) = (22, 7) system, the same N = 43,
  the same 44 equations and the same 136,698 degree-4 columns, should behave
  like G3: mostly not refuting.
- **What the existing ℓ = 7 cells cannot say.** They have n = 19 and 21, so
  N = 40 and 42, with surplus S = n − 3ℓ of −2 and 0. They differ from G3 in n, in
  N and in surplus. This cell removes those confounds except surplus (below).

## Cell and draws (frozen before generation)

- **Cell:** m = 3, (n, ℓ) = (22, 7); N = (m−2)n + mℓ = 43 unknowns, 2n = 44
  equations; S = n − mℓ = +1.
- **Generation:** `dreg_ladder --cells 22:7:3 --m 3 --unsat 4 --ffd-max 3 --seed 20261028`,
  into `draws/m3-22-7.jsonl`. Sharded generation (`--shard I/4`) replays the same
  seeded sequence and is allowed; the draw set is the **first 4 unsatisfiable
  draws in draw order** (exact solution count 0), plus the **first 2 satisfiable
  draws** in draw order for the soundness control.

## Arms

| arm | system | `dreg_closure` flags | N | columns `C(N, ≤4)` |
|---|---|---|--:|--:|
| N0 (the null) | unspecialised, the 4 unsatisfiable draws | `--to 4 --max-cols 140000` | 43 | 136,698 |
| B (positive control) | variables 6, 13, 20 fixed to 0 (one coordinate per summand; dimensions 6, 6, 6) | `--to 4 --max-cols 140000 --fix 6=0,13=0,20=0` | 40 | 102,091 |
| N7 (identity control) | unspecialised `research/dreg_closure_scale_20261016/draws/m3-19-7.jsonl`, draw 0 | `--to 4 --max-cols 130000` | 40 | 102,091 |
| S (soundness) | the 2 satisfiable draws, unspecialised | `--satisfiable --to 4 --max-cols 140000` | 43 | 136,698 |

All runs: `prlimit --as=15000000000 timeout 7200 dreg_closure`, one process at a time.

## Controls (all must hold, else the study is uninformative)

- **N7** reproduces the committed record
  `research/dreg_closure_scale_20261016/runs/fresh-m3-19-7.d0.jsonl` on every
  non-timing field of both scan entries (degree, `xl_refuted`, `closure_refuted`,
  `saturated`, `xl_rank`, `closure_rank`, `columns`, `closure_products`,
  `closure_word_ops`). This checks the instrument on the current binary.
- **B** refutes at degree 4 on at least 3 of the 4 draws. (At ℓ = 7 the same
  arm refuted on 8 of 8 in the specialisation study.)
- **S:** no satisfiable draw is refuted (a refutation of a satisfiable system
  would be a soundness failure).

## Verdict rules (scored by `dreg_score null runs/*.jsonl`)

Per N0 run: refutes at 4, does not (degree 4 saturated without `1`), or
unmeasured (killed, empty or cut; never negative evidence).

With controls valid and no unmeasured N0 run:

- **N0 does not refute on at least 3 of 4 draws → "the pure ℓ = 7 reading".** The
  G3 failure is what an unspecialised ℓ = 7 system does, so G3 says nothing about a
  guess count beyond "guess down to ℓ = 7".
- **N0 refutes on at least 3 of 4 draws → "ℓ = 7-ness alone does not explain the G3
  failure".** An unspecialised (22, 7) system refutes at degree 4 where G3 did
  not, so the failure depends on something else (surplus, the ℓ = 8 structure of
  the fixed system, or the set of guessed variables).
- Otherwise **mixed**.

Any unmeasured N0 run → **incomplete**; controls not valid → **uninformative**.

## Predictions (confidence)

- N7 matches: 0.97 (it matched on the previous binary and this one is the same).
- B refutes on ≥ 3 of 4: 0.9.
- S: no refutation: 0.99.
- **N0 does not refute on ≥ 3 of 4: 0.75.** At N = 40 and 42 it was 8 of 8 not
  refuting; the new cell has more unknowns and the extra +1 surplus is the one
  untested difference.

## Scope, class and limits

- **Class:** stage diagnostic. No `S`, no rho ratio, no speed claim; it decides
  an interpretation of the guess-count result and measures no ECC2K-130 cost.
- **One cell, four draws, one container** (4-core, 16 GB, not isolated).
- **Confound kept, named:** surplus. The earlier ℓ = 7 cells had S = −2 and 0; this
  one has S = +1. If N0 refutes, surplus is the first suspect, and a (21, 7) or
  (20, 7) cell would separate it. The G3 systems also come from an ℓ = 8 draw
  with fixed coordinates, not from a random 7-dimensional subspace, so "same
  structure" is an analogy, not an identity.
- **Memory:** about 7 GB per degree-4 run at 136,698 columns (previous study).
  A killed or empty run is **unmeasured**, never negative evidence; the cap is
  not changed without a written amendment.
- **Binaries:** hashes recorded in `runs/binary.sha256` before the first run.
  `dreg_score` gains a `null` mode (this commit); the `guess` mode output for the
  committed studies is unchanged (checked against their `score-output.txt`).
- **Amendments** are allowed only before the first N0 degree-4 reading and are
  additive afterwards.
- **Conductor** is not installed in this container.
