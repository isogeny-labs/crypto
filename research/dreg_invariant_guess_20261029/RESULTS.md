# Results: the invariant guess is never worse than the raw guess, but does not reliably suffice at ℓ = 8

- **Scoring.** Natively scored by `dreg_linear_guess_score` (`score-output.txt`)
  against `PREREGISTRATION.md`: commit `d0b05eee0`, plus Amendment 1, which
  was made **after** the data and is disclosed as post hoc (it replaces the
  soundness rule; see below).
- **Records.** Every value comes from a committed record in `runs/`.
- **Run, 2026-10-29, 03:08–11:53 UTC.**
  - 42 processes exited 0, and 5 were killed with exit 137 (SIGKILL,
    apparently the 16 GB container's memory). Two container restarts each
    killed one run in flight; both are disclosed in `runs/queue.log`, with
    their empty outputs in `runs/killed-by-restart/`.
  - Peak RSS of the completed runs was 6.1–8.5 GB.
  - Degree-4 scan time: 138–384 s per run.
- **Binary.** One `dreg_linear_guess` build for every run (`runs/binary.sha256`).
  The scorer's and the solution lister's hashes are in `score-binary.sha256`.
- **Host.** The 4-core, 16 GB container, not isolated. Iso Lab EKS is not
  reachable from it (no `aws` CLI or credentials). Closure outcomes are exact
  algebra; timings are indicative only.
- **Report** with graph and PDF: `reports/run-20261029T0308Z/`.

## Controls

- **N (instrument identity): valid.** `{7, 15, 23} = 0` on draw 0 equals the
  committed `research/dreg_closure_guess3_20261023/runs/m3-22-8.d0.G3b0.jsonl`
  on every scan field except `secs`: closure ranks 2,501 and 129,221, no
  refutation.
- **P (positive): valid.** The committed four-guess set `{7, 15, 23, 24} = 0`
  refutes at degree 4 on 4 of 4 draws.
- **V1: valid.** The exactness check is true on every scored run.
- **V3′ soundness (Amendment 1, post hoc): valid.**
  - Satisfiable draw 1 has 6 solutions, which equals the committed count. All
    six vanish on all 44 equations (`runs/solutions-d1.jsonl`, from
    `examples/dreg_chain_solutions.rs`). They are the six orderings of one
    decomposition.
  - They lie in S3 branches 1, 5 and 6. Those three runs were killed (exit
    137) and left no reading.
  - The other five branches refute at degree 4 and hold no solution, so
    every refutation is correct.
  - The registered V3 ("at least one branch does not refute") could not be
    read, because the branches that cannot refute are the ones that ran out
    of memory. V3′ is stricter where both can be read, but it was chosen
    after seeing this.

## The measurement

| draw | S3: guess `a₇ + b₇` (invariant), `c₇`, `e₁[0]` | X3: guess `a₇` (raw), `c₇`, `e₁[0]` |
|--:|---|---|
| 0 | b0–b2 refute; b4 saturates degree 4, no refutation (b3 killed) → does not suffice | b0 saturates, no refutation → does not suffice |
| 3 | b0 saturates, no refutation → does not suffice | b0 saturates, no refutation → does not suffice |
| 5 | **all 8 refute → suffices** | b0 saturates, no refutation → does not suffice |
| 6 | all 8 refute → suffices | all 8 refute → suffices |

- **Registered verdict: mixed.** S3 suffices on 2 of 4 draws, which is
  "split". X3 suffices on 1 of 4, which is "fails". None of H_inv, H_dim,
  H_both or H_rev holds as registered.
- **Paired by draw (descriptive, not registered):**
  - S3 is never worse than X3.
  - It is strictly better on draw 5, where it refutes on all 8 branches and
    X3 fails on its first.
  - It is better on draw 0 by branch count: 3 refute before a failure,
    against X3 failing immediately.
  - They tie on draws 3 and 6.
- **Against the committed raw sets on the same draws:**
  - `{7, 15, 23}` (two raw guesses on the pair, plus `c₇`) suffices on 1 of 4.
  - S3 spends *one* guess on the pair and suffices on 2 of 4.
  - `{7, 15, 23, 24}` (four guesses) suffices on 4 of 4.
- **Predictions.** H_dim was the central guess (about 0.55). It does not
  hold, because S3 does not fail: it is split.

## What it does and does not say

- **The invariant coordinate does work a raw coordinate does not.** That holds
  on one of four draws outright, and partly on a second. With four draws,
  this is a direction, not an established effect. The registered test is
  inconclusive (mixed).
- **The summand-dimension reading is weakened.** That reading (#2749) predicted
  that S3, which leaves both x₁ and x₂ at dimension 8, fails like X3. On
  draws 5 and 6 it refutes on every branch.
- **It does not reliably suffice.** Draws 0 and 3 still fail at three
  guesses.
- **The draw-3 ranks.** On draw 3 both arms fail at branch 0 with the same
  degree-4 closure rank (130,787 of 136,698 columns). The systems differ:
  their closure products are 786,681 and 786,679.
- **Scope.** m = 3, one cell, four unsatisfiable draws, one guess set per arm.
  Other invariant forms (another coordinate of `s`, or two coordinates of
  `s`) are unmeasured.

## What this means for ECC2K-130 (derived)

- **It is the first measured case of a guess choice that beats raw coordinates
  at equal k on the same draws.** For the guess-then-degree-4 oracle that is
  the right lever: fewer guesses for the same degree.
- **It is not enough to move a row.** Even if one invariant guess per summand
  pair replaced two raw guesses everywhere, the extrapolated count at the
  priced m = 4, ℓ = 33 cell would fall from about 105 to roughly 80. The
  break-even is 6. No row moves, none beats rho, and there are no benchmark
  or leaderboard rows.
- **Against the success criterion:** a measured stage-level signal (an
  invariant guess outperforms a raw one on 1–2 of 4 draws). It is not an
  established improvement, because the registered verdict is mixed.

## Successors (not run)

- **More draws at this cell** (8 to 12 unsatisfiable) to settle S3 against
  X3. Each S3 branch costs about 4–6 minutes here; failing draws stop after
  one branch.
- **Two invariant guesses on the pair** (`a₇ + b₇`, `a₆ + b₆`) at k = 3 with
  `c₇`, dropping the intermediate coordinate.
- **The invariant guess at m = 4**, where two pairs share the chain.
- **Satisfiable branches need memory.** Branches holding a solution cannot
  refute and ran past this container's memory. A soundness control on them
  needs a larger host, or the sparse closure with a cap.
