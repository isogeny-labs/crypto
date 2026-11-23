# Results: a second invariant guess on the pair, in place of `c₇`, does not help (never better on any of 12 draws)

- **Scoring.** Natively scored by `dreg_linear_guess_pair_score`
  (`score-output.txt`) against `PREREGISTRATION.md`, frozen before any D3
  run. No amendments.
- **Records.** Every value comes from a committed record in `runs/` (D3 and
  the identity control). The comparator S3 comes from the committed records of
  `research/dreg_invariant_guess_20261029/` and
  `research/dreg_invariant_guess_more_20261029/`.
- **Run, 2026-10-29 22:07 UTC to 2026-10-30 01:29 UTC.**
  - 39 processes, all exited 0. None was killed or retried, so
    `runs/attempts/` is empty.
  - Peak RSS was 7.0–8.5 GB. Degree-4 scan time was 218–345 s per run.
- **Binary.** The same `dreg_linear_guess` build as #2851 and #2883
  (`runs/binary.sha256`). The scorer's hash is in `score-binary.sha256`.
- **Host.** The 4-core, 16 GB container, not isolated. Iso Lab EKS is not
  reachable from it. Closure outcomes are exact algebra; timings are
  indicative only.
- **Report** with graph and PDF: `reports/run-20261029T2207Z/`.

## Validity

- **Identity control N: valid.** S3 on draw 5 branch 0, re-run here, equals
  the committed `S3.d5.b0` on every scan field except `secs`.
- **Identity and exactness: valid.** Every D3 and comparator S3 record
  matches its registered draw and guesses, and `--check` is true on every
  one.
- **Every draw is decided** for both arms.

## The measurement

| draw | D3: `a₇+b₇`, `a₆+b₆`, `e₁[0]` | S3: `a₇+b₇`, `c₇`, `e₁[0]` |
|--:|---|---|
| 0 | fails at b0 | fails at b4 |
| 3 | fails at b0 | fails at b0 |
| 5 | fails at b4 | **suffices** |
| 6 | suffices | suffices |
| 8 | fails at b0 | **suffices** |
| 11 | fails at b4 | **suffices** |
| 14 | fails at b0 | fails at b2 |
| 15 | fails at b4 | **suffices** |
| 17 | fails at b0 | fails at b6 |
| 24 | suffices | suffices |
| 25 | fails at b0 | fails at b2 |
| 27 | fails at b0 | **suffices** |

- **D3 suffices on 2 of 12 draws; S3 on 7 of 12.**
- **Discordant draws:** 5, all of them S3 (D3-only 0).
- **Registered verdict: no detected difference.**
  - The pre-registered test is two-sided, and 5 of 5 gives p = 0.0625, just
    above 0.05.
  - A one-sided test in S3's favour would give p = 0.031, but it was not the
    registered test and is not used for the verdict.
- **Descriptive.** D3 is never better than S3 on any draw.
  - Where both fail, D3 fails at the same branch or an earlier one: draws 0,
    3, 14, 17 and 25.
  - Every D3 failure is at branch 0 or 4. These are the branches with
    `c = f = 0`, that is `x₁₅ = x₇` and `x₁₄ = x₆`, which set the top two
    coordinates of `s` to zero.
- **Prediction.** I gave D3_better about 0.25 and S3_better about 0.25. The
  result is no detected difference at the registered threshold, and the
  direction is S3's.

## What it does and does not say

- **A second invariant coordinate of the pair is not a substitute for the raw
  coordinate of the third summand.** In this sample it is worse, though the
  difference misses the registered threshold.
- **Together with #2883:** one invariant guess on the pair is the useful part.
  The raw guess `c₇` on x₃ carries weight a second pair guess does not.
- **Reading, not a test.** The D3 failures cluster at `c = f = 0`. That
  suggests the branch where both top coordinates of `s` vanish is the hard
  one.
- **Scope.** m = 3, one cell, 12 draws, one alternative guess set.

## What this means for ECC2K-130 (derived)

- **The guess rule that #2883 supports stays the best measured one:** one
  invariant guess per summand pair, plus raw guesses elsewhere.
- **No row moves and none beats rho.** Stage diagnostic: there is no `S`, no
  rho ratio, and no benchmark or leaderboard row.

## Successors (not run)

- **The invariant of the second link at m = 4**, where `(e₁, x₃)` plays the
  role of the pair. This needs a larger host.
- **ℓ = 9 with S3-type guesses.** About 180k degree-4 columns at k = 3, beyond
  this host's memory.
