# Pre-registration: a second invariant guess on the summand pair, in place of a raw guess on x₃

Frozen 2026-10-29 before any run of the new arm.

## Question

#2851 and #2883 found that at (22, 8), with three guesses, fixing the
swap-invariant coordinate `a₇ + b₇` of `s = x₁ + x₂` (arm **S3**: `a₇ + b₇`,
`c₇`, `e₁[0]`) lets degree 4 refute on more draws than fixing the raw `a₇`
(X3). S3 suffices on 7 of the 12 unsatisfiable draws measured.

**Question: does a second invariant coordinate of the pair, `a₆ + b₆`, in
place of S3's raw guess `c₇` on the third summand, suffice on more draws
than S3?**

- New arm **D3:** `x₁₅ := x₇ + c`, `x₁₄ := x₆ + f`, `x₂₄ = g` (that is,
  `a₇ + b₇`, `a₆ + b₆`, `e₁[0]`).
- Branch `b`: `c = b & 1`, `f = (b >> 1) & 1`, `g = (b >> 2) & 1`.
- D3 differs from S3 in exactly one guess: `a₆ + b₆` instead of `c₇`. Both
  leave 43 occurring variables and 136,698 degree-4 columns.

## Draws and comparator (all committed)

- **Draws.** The 12 unsatisfiable (22, 8) draws already measured:
  - 0, 3, 5, 6 from `research/dreg_invariant_guess_20261029/draws/m3-22-8.jsonl`;
  - 8, 11, 14, 15, 17, 24, 25, 27 from
    `research/dreg_invariant_guess_more_20261029/draws/m3-22-8-more.jsonl`.
- **Comparator S3.** The committed S3 records in those two studies' `runs/`.
  They came from the same `dreg_linear_guess` build (`8376d13a…8dff`), and
  closure outcomes are exact, so S3 is not re-run.
- **Instrument identity (N).** S3 on draw 5 branch 0
  (`--sum 15=7+0 --fix 23=0,24=0`) is re-run here. It must equal the
  committed `research/dreg_invariant_guess_20261029/runs/S3.d5.b0.jsonl` on
  every scan field except `secs`.

## Measurement (`run.sh`)

- Each run is `dreg_linear_guess --to 4 --max-cols 140000 --check`, under
  `prlimit --as=15e9 timeout 7200`, one process at a time.
- Every attempt is kept: an empty earlier attempt is moved to `runs/attempts/`
  before a retry.
- **Early stop:** a draw stops at its first branch that saturates degree 4
  without refuting.
- **Order:** N first, then D3 branch by branch across the 12 draws.

## Validity

- **V1:** every D3 record's draw and guesses match its name, and
  `identity_check` is true.
- **V2:** N matches the committed record.
- **Every (arm, draw) is decided:** it suffices, or it has a failing branch.

## Verdict rules (12 draws)

A draw is *discordant* when exactly one of D3 and S3 suffices.

- With k_D discordant draws where D3 suffices and k_S where S3 suffices
  (d = k_D + k_S), use a **two-sided** exact sign test:
  `p = min(1, 2·P(Bin(d, ½) ≥ max(k_D, k_S)))`.
- **D3_better:** p ≤ 0.05 and k_D > k_S.
- **S3_better:** p ≤ 0.05 and k_S > k_D.
- **No detected difference:** anything else, including d = 0.
- Descriptive: each arm's suffice count, and, where both fail, which fails
  first.

## Predictions (before any D3 run)

- The intermediate coordinate stays in both arms. What D3 gives up is the
  raw `c₇`, which lowers x₃'s dimension; it gains a second constraint on `s`.
- P(D3_better) ≈ 0.25, P(no detected difference) ≈ 0.5, P(S3_better) ≈ 0.25.

## What it would mean for ECC2K-130

- **D3_better:** invariant guesses compose. Two on the pair beat one plus a
  raw guess, which points to a guess rule built from invariant coordinates.
- **S3_better:** the third summand's raw coordinate matters, and one
  invariant guess per pair is the useful part.
- In every case: a stage-level reading of the guess-then-degree-4 oracle's
  guess choice. No `S`, no rho ratio, no leaderboard row.

## Scope

m = 3, one cell, 12 draws, the 4-core 16 GB container, not isolated.

- Iso Lab EKS and the S3 resume checkpoint are not reachable from it.
- ℓ = 9 (about 180k degree-4 columns at k = 3) and m = 4 are out of this
  host's memory.
- Closure outcomes are exact algebra; timings are indicative only.
- Conductor is not installed.
