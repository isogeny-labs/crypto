# Results: the invariant guess beats the raw guess at ℓ = 8 (5 of 5 discordant draws, p = 0.031)

- **Scoring.** Natively scored by `dreg_linear_guess_more_score`
  (`score-output.txt`) against `PREREGISTRATION.md` (commit of 2026-10-29,
  frozen before any new draw was generated). No amendments.
- **Records.** Every value comes from a committed record in `runs/`.
- **Run, 2026-10-29, 14:32–21:45 UTC.**
  - 81 processes exited 0, and none exited non-zero.
  - Two runs were killed in flight and repeated: X3.d25.b0 at the session's
    2-hour background limit, and S3.d8.b7 by a container restart. Their
    empty first attempts are kept in `runs/attempts/` and logged in
    `runs/queue.log`.
  - Peak RSS was 7.0–8.6 GB. Degree-4 scan time was 227–415 s per run.
- **Binaries (`runs/binary.sha256`).** The same `dreg_linear_guess` build as
  #2851, plus `dreg_ladder` for the draws. The scorer's hash is in
  `score-binary.sha256`.
- **Host.** The 4-core, 16 GB container, not isolated. Iso Lab EKS is not
  reachable from it. Closure outcomes are exact algebra; timings are
  indicative only.
- **Report** with graph and PDF: `reports/run-20261029T1432Z/`.

## Validity

- **V0, replay: valid.** The generator replays the committed draws 0–6
  exactly: basis, `x(R)`, solution count and outcome.
- **The registered set.** The first 8 unsatisfiable draws from index 7 are 8,
  11, 14, 15, 17, 24, 25 and 27. The 33 draws generated (7–39) are in
  `draws/m3-22-8-more.jsonl`.
- **V1: valid.** Every record's draw and guesses match its name, and the
  exactness check is true on every run.
- **V2: valid.** The positive control `{7, 15, 23, 24} = 0` refutes at
  degree 4 on 8 of 8 draws.
- **Every (arm, draw) is decided.**

## The measurement

| draw | S3: `a₇ + b₇` (invariant) | X3: `a₇` (raw) |
|--:|---|---|
| 8 | **suffices** (8 of 8 refute) | fails at b0 |
| 11 | **suffices** | b0–b5 refute, fails at b6 |
| 14 | fails at b2 | fails at b0 |
| 15 | **suffices** | b0–b5 refute, fails at b6 |
| 17 | b0–b5 refute, fails at b6 | fails at b0 |
| 24 | **suffices** | fails at b0 |
| 25 | fails at b2 | fails at b0 |
| 27 | **suffices** | fails at b0 |

- **S3 suffices on 5 of 8 draws; X3 suffices on 0 of 8.**
- **Discordant draws** (exactly one arm suffices): 5, all of them S3.
- **Registered verdict: S3_better.** One-sided exact sign test: p = 0.031,
  under the pre-registered threshold of 0.05.
- **Secondary, with #2851's four draws:** S3-only on 6 of 6 discordant draws,
  p = 0.016. S3 suffices on 7 of 12 draws in all, X3 on 1 of 12.
- **Paired by draw,** S3 is never worse on any of the 12. Where both fail, S3
  fails at a later branch or the same one: draws 14, 17 and 25 here, and
  draws 0 and 3 of #2851.
- **Prediction.** I gave S3_better a probability of about 0.3; it happened.

## What it does and does not say

- **At equal guess count, guessing a coordinate of the swap invariant
  `s = x₁ + x₂` refutes at degree 4 on more draws than guessing a raw
  summand coordinate.** This holds at m = 3, (22, 8), for this guess set
  (the top coordinate, plus `c₇` and `e₁[0]`).
  - It is a pre-registered result on fresh draws, not the post-hoc reading
    of #2851.
- **Five discordant draws is the smallest count that can reach p ≤ 0.05.**
  The effect is consistent (6 of 6 over both studies). Its size beyond
  "S3 suffices on about 60% of draws, X3 on about 8%" is not estimated
  precisely.
- **The summand-dimension reading is contradicted at this cell.** S3 leaves
  both summands at dimension 8 and suffices on 7 of 12 draws. X3 lowers x₁ to
  dimension 7 and suffices on 1 of 12. What matters is the coordinate of the
  invariant `s`, not the dimension of the summands.
- **S3 does not always suffice.** It fails on 5 of 12 draws. Four guesses
  `{7, 15, 23, 24}` still suffice on every draw measured.
- **Scope.** m = 3, one cell, one guess set per arm. Other invariant guesses,
  ℓ = 9 and m = 4 are unmeasured.

## What this means for ECC2K-130 (derived)

- **A measured stage-level improvement over the baseline, under the success
  criterion.** In the guess-then-degree-4 oracle, an invariant guess is the
  better use of a guess than a raw coordinate, on matched draws, at p = 0.031.
- **It moves no row.** The guess count needed at the priced m = 4, ℓ = 33
  cell is still extrapolated far above the oracle's break-even of 6.
  - This result changes the guess *choice*, not yet the measured *count* at
    larger ℓ.
  - There is no `S`, no rho ratio, and no benchmark or leaderboard row.
- **What it opens:** guessing invariant coordinates as a rule.
  - Measure k at ℓ = 9 with invariant guesses.
  - At m = 4, ask whether the invariant of `(e₁, x₃)` can play the same
    role in the second link.

## Successors (not run)

- **Two invariant guesses on the pair** (`a₇ + b₇`, `a₆ + b₆`) plus `c₇` at
  k = 3, against S3.
- **ℓ = 9 with invariant guesses.** This needs more memory: N = 49, and
  C(46, ≤ 4) = 179,447 columns at k = 3.
- **m = 4.**
