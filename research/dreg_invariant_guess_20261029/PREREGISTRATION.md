# Pre-registration: does an invariant guess do the work of a raw guess at ℓ = 8?

Frozen 2026-10-29 before any closure was built. The only runs before this
commit are the exactness checks below, which build no Macaulay matrix.

## Question

The guess-then-degree-4 oracle (#2429, #2749) fixes raw coordinates until the
MutantXL closure of the m = 3 chained S₃ system refutes at degree 4. At
(n, ℓ) = (22, 8):

- four raw guesses `{7, 15, 23, 24}` refute on 64 of 64 branches
  (`research/dreg_closure_guess_20261022`);
- three raw guesses `{7, 15, 23}` (the top coordinate of each summand) fail on
  3 of 4 draws (`research/dreg_closure_guess3_20261023`).

The count fits `m·(ℓ−7)+1` guesses. Extended linearly, that is about 105 at
the priced m = 4, ℓ = 33 cell, far above the oracle's break-even of 6. So the
lever that matters is guesses per unit of degree reduction.

The first link `S₃(x₁, x₂, e₁)` sees the two summands only through the
swap-invariant `s = x₁ + x₂` and `p = x₁x₂`. A coordinate of `s`, `a₇ + b₇`,
is one linear form. Fixing it costs one guess, where fixing `a₇` and `b₇`
separately costs two. **Question: at equal guess count k = 3, does fixing the
invariant `a₇ + b₇` let degree 4 refute where fixing the raw `a₇` does
not?**

The study `research/dreg_pair_invariant_20261028/` (#2796) added the
invariant `p` as unknowns and found no lower degree at (22, 6). Here the
invariant is guessed instead, so no unknown is added.

## Draws (committed, not regenerated)

`draws/m3-22-8.jsonl` is a byte-identical copy of
`research/dreg_closure_guess_20261022/draws/m3-22-8.jsonl` at commit
`7d6cc37d5` (SHA-256 `95ee147e…5a9b`). That file lives on the open
#2429/#2749 branches, not on `main`, so it is copied here.

- m = 3, (22, 8), N = 46, 44 equations.
- Unsatisfiable draws 0, 3, 5, 6; satisfiable draws 1, 2, 4.
- Variables: summand i owns `[8i, 8i+7]`, so a₇ = 7, b₇ = 15, c₇ = 23. The
  intermediate point owns 24–45.

## Arms (`examples/dreg_linear_guess.rs`, all with three guesses, 8 branches per draw)

| arm | guesses | what each fixes |
|---|---|---|
| **S3** (invariant) | `x₁₅ := x₇ + c`, `x₂₃ = f`, `x₂₄ = g` | the top coordinate of `s`, the top coordinate of x₃, one intermediate coordinate |
| **X3** (raw, matched) | `x₇ = c`, `x₂₃ = f`, `x₂₄ = g` | the top coordinate of x₁ (not x₂), the same two |

- Branch `b` sets `c = b & 1`, `f = (b >> 1) & 1`, `g = (b >> 2) & 1`.
- Both arms leave 43 occurring variables and 136,698 degree-4 columns.
- They differ only in the one guess that touches the first link: the
  invariant form against a raw coordinate.
- X3 leaves x₂ at dimension 8, and S3 leaves both x₁ and x₂ at dimension 8.

### Exactness check, run before this commit

`--check` evaluates every specialised equation against the original at 4,096
random points that satisfy the guesses.

- It is true on all 64 (draw, branch, arm) systems of draws 0, 3, 5, 6.
- A deliberately wrong build, with the substitution constant flipped in the
  point generator, gives `false`.
- The binary hash is in `runs/binary.sha256`.

## Controls

- **N (instrument identity):** `{7, 15, 23} = 0` on draw 0, the committed G3
  branch 0. It must equal `reference/m3-22-8.d0.G3b0.jsonl`, a copy of
  `research/dreg_closure_guess3_20261023/runs/m3-22-8.d0.G3b0.jsonl`, on
  every scan field except `secs`.
- **P (positive):** `{7, 15, 23, 24} = 0` (the committed four-guess set,
  branch 0) on draws 0, 3, 5, 6. It must refute at degree 4 on at least 3 of
  4.
- **Soundness:** all 8 S3 branches on satisfiable draw 1.
  - Some branch contains a solution, so at least one branch must not refute.
  - Every run on a satisfiable draw must also satisfy `--check`.

## Measurement (`run.sh`)

- Each run is `dreg_linear_guess --to 4 --max-cols 140000 --check`, one
  process at a time, under `prlimit --as=15e9 timeout 7200`.
- Order:
  1. `control` (N, then P);
  2. `measure`, which interleaves S3 and X3 branch by branch, draw by draw;
  3. `sound`.
- **Early stop:** an (arm, draw) stops at its first branch that saturates
  degree 4 without refuting, as in #2429. The branches it skips are
  unmeasured.

## Validity

- **V1:** `identity_check` is true on every scored run.
- **V2:** N matches the reference, and P refutes on at least 3 of 4.
- **V3:** at least one S3 branch on draw 1 does not refute.

## Hypotheses and verdict rules

An arm *suffices on a draw* if all 8 branches refute at degree 4 or below. It
*suffices* if it suffices on at least 3 of the 4 draws, and *fails* if it
suffices on at most 1. Two draws is *split*.

- **H_inv:** S3 suffices and X3 fails. The invariant guess does work a raw
  guess cannot.
- **H_dim:** both fail. Refutation tracks summand dimension, which S3 does
  not lower.
- **H_both:** both suffice. Coordinate 24 with any one first-link guess is
  enough, so the invariant is not specific.
- **H_rev:** X3 suffices and S3 fails.
- **Mixed:** anything with a split arm, reported as counts.

## Predictions (before any closure)

- H_dim ≈ 0.55. The ℓ = 7 reading (#2749) says degree 4 fails while a
  summand stays at dimension ≥ 7 to 8, and S3 lowers no summand's
  dimension.
- H_inv ≈ 0.2, H_both ≈ 0.15, H_rev ≈ 0.1.

## What a result would mean for ECC2K-130

- **H_inv:** one invariant guess per summand pair replaces two raw ones. That
  lowers the guess count of the oracle (a stage-level improvement under the
  success criterion). It moves no row unless the count stays near the
  break-even of 6 at ℓ = 33, which this cell cannot show.
- **H_dim:** it supports summand dimension as the driver. It closes invariant
  guessing for the pair at m = 3, ℓ = 8.
- In every case: no `S`, no rho ratio, no leaderboard row.

## Scope

m = 3, one cell, 4 unsatisfiable draws and 1 satisfiable draw, the 4-core
16 GB container, not isolated.

- Iso Lab EKS and the S3 resume checkpoint are not reachable from it: there is
  no `aws` CLI and no credentials.
- Closure outcomes are exact algebra; timings are indicative only.
- Conductor is not installed, so no task was taken.

## Amendment 1 (2026-10-29T12:10Z, AFTER the data; disclosed as post hoc)

- **What happened.** Three of the eight soundness runs, S3 branches 1, 5 and
  6 on satisfiable draw 1, were killed with exit 137 (SIGKILL, apparently the
  container's memory limit) and left no output. The five that finished all
  refute at degree 4.
  - So V3 as registered ("at least one branch does not refute") cannot be
    read: the branches that do not refute have no reading.
- **What was then done.** `examples/dreg_chain_solutions.rs` lists every
  solution of draw 1 with the same enumeration as
  `chained_s3_solution_count_fast`.
  - It found 6 solutions, which equals the committed count. Every one vanishes
    on all 44 equations (`runs/solutions-d1.jsonl`).
  - They lie in S3 branches 1, 5 and 6, exactly the three killed runs. The
    five refuting branches contain no solution.
- **The replacement rule (V3′), decided after seeing the outcome above:**
  - every S3 branch on draw 1 that contains no solution has a record;
  - no refuting branch contains a solution;
  - the solution list matches the committed count and vanishes on the
    system.
  - V3′ is strictly stronger than V3 where both can be read: it requires every
    refutation to be correct, not just one non-refutation. It is still a
    post-hoc rule and is labelled as such in the results.
- **Also disclosed.**
  - S3 draw 0 branch 3 (an unsatisfiable draw) was killed with exit 137 twice.
    The resumed queue re-ran it, and the second attempt overwrote the first
    one's empty output and log.
  - It is unmeasured. Draw 0's verdict rests on branch 4, which saturates
    degree 4 without refuting.
