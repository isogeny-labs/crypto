# Results: an invariant guess on the second link matches the raw guess on x₃ (no detected difference, 6 against 7 of 12 draws)

Standing: claim [pcc:PCC-20261105-dbef6e], an **unverified assertion** until a pcc verdict exists
(AGENTS.md, "Sabotage is assumed").

- **Scoring.** Natively scored by `dreg_second_link_score`
  (`score-output.txt`) against `PREREGISTRATION.md`, frozen before any L3
  run. No amendments.
- **Records.** Every value comes from a committed record:
  - L3 and the identity control in `runs/`;
  - the comparator S3 in `research/dreg_invariant_guess_20261029/` and
    `research/dreg_invariant_guess_more_20261029/`.
- **Run, 2026-11-05 16:42 UTC to 22:33 UTC.**
  - 74 processes started and 73 exited 0.
  - One run, L3 on draw 11 branch 7, was killed with empty output by a
    container restart. It is kept in `runs/attempts/`, and its retry
    completed.
  - Every L3 system has 43 occurring variables (136,698 degree-4 columns).
    The degree-4 scan took 147–351 s per run.
- **Binary.** `runs/binary.sha256`. The scorer's hash is in
  `score-binary.sha256`.
- **Host.** The 4-core, 16 GB container, not isolated. Iso Lab EKS is not
  reachable from it. Closure outcomes are exact algebra; timings are
  indicative only.
- **Report** with graph and PDF: `reports/run-20261105T1642Z/`.

## Validity

- **Identity control N: valid.** The new build's S3 run on draw 5 branch 0
  equals the committed `S3.d5.b0` on every scan field except `secs`.
- **Identity and exactness: valid.** Every L3 and comparator S3 record
  matches its registered draw and guesses. That includes each L3 record's
  second-link sources, re-derived from the draw's basis. `--check` is true on
  every record.
- **Every draw is decided** for both arms.

## The measurement

| draw | L3: `a₇+b₇`, `(e₁+x₃)₂₀`, `e₁[0]` | S3: `a₇+b₇`, `c₇`, `e₁[0]` |
|--:|---|---|
| 0 | fails at b6 | fails at b4 |
| 3 | fails at b0 | fails at b0 |
| 5 | suffices | suffices |
| 6 | suffices | suffices |
| 8 | suffices | suffices |
| 11 | suffices | suffices |
| 14 | fails at b6 | fails at b2 |
| 15 | suffices | suffices |
| 17 | fails at b4 | fails at b6 |
| 24 | suffices | suffices |
| 25 | fails at b2 | fails at b2 |
| 27 | fails at b0 | **suffices** |

- **L3 suffices on 6 of 12 draws; S3 on 7 of 12.**
- **Discordant draws:** 1, draw 27, which favours S3.
- **Registered verdict: no detected difference.** The two-sided exact sign
  test on 1 discordant draw gives p = 1.0.
- **Descriptive.** The two arms agree on 11 of 12 draws.
  - Where both fail, L3 fails later on draws 0 and 14, earlier on draw 17, and
    at the same branch on draws 3 and 25.
  - Draw 27 fails at L3's branch 0, where `c = h = g = 0`.
- **Prediction.** I gave about 0.55 to no detected difference, 0.2 to
  L3_better and 0.25 to S3_better. The outcome is the most likely one.

## What it does and does not say

- **On these draws, a coordinate of the second link's invariant `e₁ + x₃`
  serves as well as the raw coordinate `c₇`.** Contrast #2885: there, a
  second coordinate of the *first* link's invariant in place of `c₇` was
  never better and was worse on 5 draws.
- **Reading, not a test.** The difference from #2885 is the hypothesis this
  study was built on. Taking one guess per link, each on a different link, is
  at least as good as the raw guess. Taking both invariant guesses on one link
  is not.
- **Not shown.** That L3 is better than S3. One cell, 12 draws, and 11 of
  them concordant cannot separate them.

## What this means for ECC2K-130 (derived)

- **One invariant guess per link is a rule that does not lose at m = 3.**
  Unlike the raw guess, it extends along the chain: at m = 4 each link has
  its own swap invariant. That makes m = 4 its natural test, and m = 4 needs
  a larger host than this one.
- **No row moves, and rho is not reached.** Stage diagnostic: there is no `S`, no
  rho ratio, and no benchmark or leaderboard row.

## Successors (not run)

- **m = 4 with one invariant guess per link.** Beyond this host's 16 GB.
- **ℓ = 9 with S3- or L3-type guesses.** About 180k degree-4 columns at
  k = 3, also beyond this host.
